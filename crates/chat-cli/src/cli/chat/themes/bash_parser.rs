use std::collections::HashMap;
use std::fs;
use std::path::Path;

use eyre::Result;

use super::BashTheme;

pub struct BashParser;

impl BashParser {
    /// Parse a bash-style theme file
    pub fn parse_theme_file(path: &Path) -> Result<BashTheme> {
        let content = fs::read_to_string(path)?;
        let mut theme = BashTheme::new(
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
        );

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse variable assignments: VAR="value" or VAR=value
            if let Some((key, value)) = Self::parse_assignment(line) {
                match key.as_str() {
                    "THEME_NAME" => theme.name = Self::unquote(&value),
                    "PROMPT" => theme.prompt_template = Self::unquote(&value),
                    "Q_GIT_ENABLED" => {
                        theme.git_enabled = Self::unquote(&value).to_lowercase() == "true";
                    },
                    // Git configuration variables
                    "Q_GIT_PREFIX" | "Q_GIT_SUFFIX" | "Q_GIT_CLEAN" | "Q_GIT_DIRTY" | "Q_GIT_STAGED"
                    | "Q_GIT_UNTRACKED" | "Q_GIT_AHEAD" | "Q_GIT_BEHIND" => {
                        theme.set_variable(key, Self::unquote(&value));
                    },
                    _ => {
                        theme.set_variable(key, Self::unquote(&value));
                    },
                }
            }
        }

        Ok(theme)
    }

    /// Substitute variables in a template string with git support
    pub fn substitute_variables(template: &str, vars: &HashMap<String, String>) -> String {
        let mut result = template.to_string();

        // Sort variables by key length (longest first) to avoid partial substitutions
        let mut sorted_vars: Vec<_> = vars.iter().collect();
        sorted_vars.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        // Simple variable substitution: $VAR or ${VAR}
        for (key, value) in sorted_vars {
            let patterns = [
                format!("${{{}}}", key), // ${VAR}
                format!("${}", key),     // $VAR
            ];

            for pattern in &patterns {
                result = result.replace(pattern, value);
            }
        }

        result
    }

    /// Get default git configuration variables
    #[allow(dead_code)]
    pub fn get_default_git_vars() -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("Q_GIT_PREFIX".to_string(), " (".to_string());
        vars.insert("Q_GIT_SUFFIX".to_string(), ")".to_string());
        vars.insert("Q_GIT_CLEAN".to_string(), "✓".to_string());
        vars.insert("Q_GIT_DIRTY".to_string(), "±".to_string());
        vars.insert("Q_GIT_STAGED".to_string(), "●".to_string());
        vars.insert("Q_GIT_UNTRACKED".to_string(), "?".to_string());
        vars.insert("Q_GIT_AHEAD".to_string(), "↑".to_string());
        vars.insert("Q_GIT_BEHIND".to_string(), "↓".to_string());
        vars
    }

    /// Parse a variable assignment line
    fn parse_assignment(line: &str) -> Option<(String, String)> {
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let value = line[eq_pos + 1..].trim().to_string();
            Some((key, value))
        } else {
            None
        }
    }

    /// Remove quotes from a value
    fn unquote(value: &str) -> String {
        let trimmed = value.trim();
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            trimmed.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_substitute_variables() {
        let mut vars = HashMap::new();
        vars.insert("AGENT".to_string(), "test-agent".to_string());
        vars.insert("COLOR".to_string(), "\u{001b}[36m".to_string());

        let template = "$COLOR[$AGENT] > ";
        let result = BashParser::substitute_variables(template, &vars);
        assert_eq!(result, "\u{001b}[36m[test-agent] > ");
    }

    #[test]
    fn test_parse_assignment() {
        assert_eq!(
            BashParser::parse_assignment("VAR=value"),
            Some(("VAR".to_string(), "value".to_string()))
        );
        assert_eq!(
            BashParser::parse_assignment("VAR=\"quoted value\""),
            Some(("VAR".to_string(), "\"quoted value\"".to_string()))
        );
        assert_eq!(BashParser::parse_assignment("invalid line"), None);
    }

    #[test]
    fn test_unquote() {
        assert_eq!(BashParser::unquote("\"quoted\""), "quoted");
        assert_eq!(BashParser::unquote("'single'"), "single");
        assert_eq!(BashParser::unquote("unquoted"), "unquoted");
    }

    #[test]
    fn test_git_variables() {
        let git_vars = BashParser::get_default_git_vars();
        assert_eq!(git_vars.get("Q_GIT_CLEAN"), Some(&"✓".to_string()));
        assert_eq!(git_vars.get("Q_GIT_DIRTY"), Some(&"±".to_string()));
    }
}
