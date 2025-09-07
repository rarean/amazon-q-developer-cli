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

        // Handle bash-style conditionals: ${VAR:+text} with proper brace matching
        result = Self::substitute_conditionals(&result, vars);

        // Simple variable substitution: $VAR or ${VAR}
        result = Self::substitute_simple_variables(&result, vars);

        result
    }

    /// Handle conditional substitutions with proper brace matching
    fn substitute_conditionals(template: &str, vars: &HashMap<String, String>) -> String {
        let mut result = String::new();
        let mut chars = template.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' && chars.peek() == Some(&'{') {
                chars.next(); // consume '{'

                // Find the variable name (up to ':' or '}')
                let mut var_name = String::new();
                let mut found_colon = false;

                while let Some(&next_ch) = chars.peek() {
                    if next_ch == ':' {
                        chars.next(); // consume ':'
                        found_colon = true;
                        break;
                    } else if next_ch == '}' {
                        break;
                    } else {
                        var_name.push(chars.next().unwrap());
                    }
                }

                if found_colon && chars.peek() == Some(&'+') {
                    chars.next(); // consume '+'

                    // Find the conditional text with proper brace matching
                    let mut conditional_text = String::new();
                    let mut brace_count = 1; // We're inside the outer braces

                    for next_ch in chars.by_ref() {
                        if next_ch == '{' {
                            brace_count += 1;
                            conditional_text.push(next_ch);
                        } else if next_ch == '}' {
                            brace_count -= 1;
                            if brace_count == 0 {
                                // Found the matching closing brace
                                break;
                            } else {
                                conditional_text.push(next_ch);
                            }
                        } else {
                            conditional_text.push(next_ch);
                        }
                    }

                    // Check if the variable exists and is non-empty
                    if let Some(var_value) = vars.get(&var_name) {
                        if !var_value.is_empty() {
                            // Substitute variables in the conditional text
                            result.push_str(&Self::substitute_simple_variables(&conditional_text, vars));
                        }
                        // If empty or doesn't exist, add nothing
                    }
                } else {
                    // Not a conditional, put back what we consumed
                    result.push('$');
                    result.push('{');
                    result.push_str(&var_name);
                    if found_colon {
                        result.push(':');
                    }
                    // Continue processing normally
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Simple variable substitution without conditionals
    fn substitute_simple_variables(template: &str, vars: &HashMap<String, String>) -> String {
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
            let mut value = line[eq_pos + 1..].trim();

            // Handle inline comments - if the value is quoted, find the closing quote
            // and ignore everything after it
            if value.starts_with('"') {
                if let Some(end_quote) = value[1..].find('"') {
                    value = &value[..end_quote + 2]; // Include both quotes
                }
            } else if value.starts_with('\'') {
                if let Some(end_quote) = value[1..].find('\'') {
                    value = &value[..end_quote + 2]; // Include both quotes
                }
            } else {
                // For unquoted values, stop at the first # (comment)
                if let Some(comment_pos) = value.find('#') {
                    value = value[..comment_pos].trim();
                }
            }

            Some((key, value.to_string()))
        } else {
            None
        }
    }

    /// Remove quotes from a value and process escape sequences
    fn unquote(value: &str) -> String {
        let trimmed = value.trim();
        let unquoted = if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            &trimmed[1..trimmed.len() - 1]
        } else {
            trimmed
        };

        // Process escape sequences
        Self::process_escape_sequences(unquoted)
    }

    /// Process escape sequences in a string (e.g., \033 -> actual escape character)
    fn process_escape_sequences(input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.peek() {
                    Some('0') => {
                        // Handle octal escape sequences like \033
                        chars.next(); // consume '0'
                        let mut octal = String::new();

                        // Collect up to 3 octal digits
                        for _ in 0..3 {
                            if let Some(&digit) = chars.peek() {
                                if digit.is_ascii_digit() && digit <= '7' {
                                    octal.push(digit);
                                    chars.next();
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        if !octal.is_empty() {
                            if let Ok(code) = u8::from_str_radix(&octal, 8) {
                                result.push(code as char);
                            } else {
                                result.push('\\');
                                result.push('0');
                                result.push_str(&octal);
                            }
                        } else {
                            result.push('\\');
                            result.push('0');
                        }
                    },
                    Some('n') => {
                        chars.next();
                        result.push('\n');
                    },
                    Some('t') => {
                        chars.next();
                        result.push('\t');
                    },
                    Some('r') => {
                        chars.next();
                        result.push('\r');
                    },
                    Some('\\') => {
                        chars.next();
                        result.push('\\');
                    },
                    Some('"') => {
                        chars.next();
                        result.push('"');
                    },
                    Some('\'') => {
                        chars.next();
                        result.push('\'');
                    },
                    _ => {
                        result.push(ch);
                    },
                }
            } else {
                result.push(ch);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_conditional_substitution() {
        let mut vars = HashMap::new();
        vars.insert("GIT_BRANCH".to_string(), "main".to_string());
        vars.insert("GIT_CLEAN".to_string(), "✓".to_string());
        vars.insert("Q_GIT_PREFIX".to_string(), "(".to_string());
        vars.insert("Q_GIT_SUFFIX".to_string(), ")".to_string());

        // Test conditional with existing variable
        let template = "${GIT_BRANCH:+ ($GIT_BRANCH)}";
        let result = BashParser::substitute_variables(template, &vars);
        assert_eq!(result, " (main)");

        // Test conditional with non-existent variable
        let template2 = "${NONEXISTENT:+ (branch)}";
        let result2 = BashParser::substitute_variables(template2, &vars);
        assert_eq!(result2, "");

        // Test complex conditional
        let template3 = "${GIT_BRANCH:+ $Q_GIT_PREFIX$GIT_BRANCH$GIT_CLEAN$Q_GIT_SUFFIX}";
        let result3 = BashParser::substitute_variables(template3, &vars);
        assert_eq!(result3, " (main✓)");
    }

    #[test]
    fn test_parse_assignment_with_inline_comments() {
        // Test quoted value with inline comment
        assert_eq!(
            BashParser::parse_assignment("Q_AGENT_COLOR=\"\\033[36m\"      # cyan"),
            Some(("Q_AGENT_COLOR".to_string(), "\"\\033[36m\"".to_string()))
        );

        // Test single quoted value with inline comment
        assert_eq!(
            BashParser::parse_assignment("Q_AGENT_COLOR='\\033[36m'      # cyan"),
            Some(("Q_AGENT_COLOR".to_string(), "'\\033[36m'".to_string()))
        );

        // Test unquoted value with inline comment
        assert_eq!(
            BashParser::parse_assignment("Q_GIT_ENABLED=true # enable git"),
            Some(("Q_GIT_ENABLED".to_string(), "true".to_string()))
        );

        // Test value without comment
        assert_eq!(
            BashParser::parse_assignment("Q_PROMPT_SYMBOL=\">\""),
            Some(("Q_PROMPT_SYMBOL".to_string(), "\">\"".to_string()))
        );
    }

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

        // Test escape sequence processing
        assert_eq!(BashParser::unquote("\"\\033[36m\""), "\x1b[36m");
        assert_eq!(BashParser::unquote("\"\\033[0m\""), "\x1b[0m");
        assert_eq!(BashParser::unquote("\"\\n\""), "\n");
        assert_eq!(BashParser::unquote("\"\\t\""), "\t");
    }

    #[test]
    fn test_process_escape_sequences() {
        assert_eq!(BashParser::process_escape_sequences("\\033[36m"), "\x1b[36m");
        assert_eq!(BashParser::process_escape_sequences("\\033[0m"), "\x1b[0m");
        assert_eq!(BashParser::process_escape_sequences("\\n"), "\n");
        assert_eq!(BashParser::process_escape_sequences("\\t"), "\t");
        assert_eq!(BashParser::process_escape_sequences("\\\\"), "\\");
        assert_eq!(BashParser::process_escape_sequences("normal text"), "normal text");
        assert_eq!(
            BashParser::process_escape_sequences("\\033[36mcolored\\033[0m"),
            "\x1b[36mcolored\x1b[0m"
        );
    }

    #[test]
    fn test_end_to_end_theme_rendering() {
        // Create a simple theme content
        let theme_content = r#"THEME_NAME="test"
Q_AGENT_COLOR="\033[36m"
RESET_COLOR="\033[0m"
PROMPT="$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR > ""#;

        // Parse the theme
        let mut theme = BashTheme::new("test".to_string());

        // Manually parse the content (simulating what parse_theme_file would do)
        for line in theme_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = BashParser::parse_assignment(line) {
                match key.as_str() {
                    "THEME_NAME" => theme.name = BashParser::unquote(&value),
                    "PROMPT" => theme.prompt_template = BashParser::unquote(&value),
                    _ => {
                        theme.set_variable(key, BashParser::unquote(&value));
                    },
                }
            }
        }

        // Create context variables
        let mut context_vars = std::collections::HashMap::new();
        context_vars.insert("Q_AGENT".to_string(), "test-agent".to_string());

        // Add theme variables
        for (key, value) in &theme.variables {
            context_vars.insert(key.clone(), value.clone());
        }

        // Render the prompt
        let rendered = BashParser::substitute_variables(&theme.prompt_template, &context_vars);

        // Check that escape sequences are properly processed
        assert!(
            rendered.contains('\x1b'),
            "Rendered prompt should contain actual escape sequences"
        );
        assert!(
            rendered.contains("[test-agent]"),
            "Rendered prompt should contain the agent name"
        );
        assert!(
            !rendered.contains("\\033"),
            "Rendered prompt should not contain literal \\033"
        );

        println!("Rendered prompt: {:?}", rendered);
        println!("Rendered prompt (display): {}", rendered);
    }

    #[test]
    fn test_git_enabled_theme_nested_braces() {
        // Test the simplified git-enabled theme template
        let template = "${GREEN}➜${RESET} ${GIT_BRANCH:+${BLUE}git:(${GIT_BRANCH})${RESET} }> ";

        let mut vars = HashMap::new();
        vars.insert("GREEN".to_string(), "\x1b[32m".to_string());
        vars.insert("RESET".to_string(), "\x1b[0m".to_string());
        vars.insert("BLUE".to_string(), "\x1b[34m".to_string());
        vars.insert("GIT_BRANCH".to_string(), "develop".to_string());

        let result = BashParser::substitute_variables(template, &vars);

        // Should contain the git branch with proper formatting
        assert!(
            result.contains("git:(develop)"),
            "Result should contain 'git:(develop)', got: {}",
            result
        );
        assert!(
            !result.contains("${"),
            "Result should not contain unsubstituted variables: {}",
            result
        );
        assert!(
            !result.contains("} } } } } } }>"),
            "Result should not contain malformed braces: {}",
            result
        );
        assert!(result.ends_with("> "), "Result should end with '> '");

        println!("Git-enabled theme result: {}", result);
    }

    #[test]
    fn test_powerline_theme_format() {
        // Test the powerline theme template
        let template = "agent:[${AGENT}] usage:${TOKEN_USAGE} ${GIT_BRANCH:+git:(${GIT_BRANCH})}> ";

        let mut vars = HashMap::new();
        vars.insert("AGENT".to_string(), "default".to_string());
        vars.insert("TOKEN_USAGE".to_string(), "(48.35%)".to_string());
        vars.insert("GIT_BRANCH".to_string(), "develop".to_string());

        let result = BashParser::substitute_variables(template, &vars);

        // Should match the expected format
        assert_eq!(result, "agent:[default] usage:(48.35%) git:(develop)> ");

        println!("Powerline theme result: {}", result);
    }

    #[test]
    fn test_parse_theme_file_error_handling() {
        use std::path::Path;

        // Test with non-existent file (line 13-14)
        let result = BashParser::parse_theme_file(Path::new("/nonexistent/file.theme"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_assignment_edge_cases() {
        // Test line without equals sign (line 246)
        assert_eq!(BashParser::parse_assignment("invalid_line"), None);

        // Test empty key (line 254-256)
        assert_eq!(
            BashParser::parse_assignment("=value"),
            Some(("".to_string(), "value".to_string()))
        );

        // Test quoted value with no closing quote (line 259-260)
        assert_eq!(
            BashParser::parse_assignment("KEY=\"unclosed"),
            Some(("KEY".to_string(), "\"unclosed".to_string()))
        );
    }

    #[test]
    fn test_substitute_conditionals_edge_cases() {
        let vars = HashMap::new();

        // Test malformed conditional without closing brace (line 129)
        let template = "${VAR:+unclosed";
        let result = BashParser::substitute_conditionals(template, &vars);
        // The function should return empty string when it can't find closing brace
        assert_eq!(result, "");

        // Test conditional with empty variable (line 166-176)
        let mut vars = HashMap::new();
        vars.insert("EMPTY".to_string(), "".to_string());
        let template = "${EMPTY:+should_not_show}";
        let result = BashParser::substitute_conditionals(template, &vars);
        assert_eq!(result, "");
    }

    #[test]
    fn test_process_escape_sequences_error_cases() {
        // Test invalid octal sequence (line 271-273)
        assert_eq!(BashParser::process_escape_sequences("\\099"), "\\099");

        // Test incomplete octal sequence (line 279-281)
        assert_eq!(BashParser::process_escape_sequences("\\0"), "\\0");

        // Test backslash at end of string (line 283-285)
        assert_eq!(BashParser::process_escape_sequences("test\\"), "test\\");

        // Test unknown escape sequence (line 287-288)
        assert_eq!(BashParser::process_escape_sequences("\\x"), "\\x");
    }
}
