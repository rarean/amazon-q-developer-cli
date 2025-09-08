use std::env;
use std::path::{
    Path,
    PathBuf,
};

use crate::git::GitInfo;

pub struct ThemeRenderer {
    git_info: GitInfo,
    current_dir: PathBuf,
}

// ANSI color codes
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

impl Default for ThemeRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeRenderer {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        let git_info = GitInfo::detect(&current_dir);

        Self { git_info, current_dir }
    }

    pub fn new_for_path(path: &Path) -> Self {
        let git_info = if path.exists() {
            GitInfo::detect(path)
        } else {
            GitInfo::empty()
        };

        Self {
            git_info,
            current_dir: path.to_path_buf(),
        }
    }

    pub fn has_git_repo(&self) -> bool {
        self.git_info.is_repo
    }

    pub fn validate_theme(&self, template: &str) -> Result<(), String> {
        // Basic validation - check for balanced braces
        let open_braces = template.matches("${").count();
        let close_braces = template.matches('}').count();

        if open_braces != close_braces {
            return Err("Unbalanced braces in theme template".to_string());
        }

        // Check for unknown variables
        let valid_vars = [
            "RED",
            "GREEN",
            "YELLOW",
            "BLUE",
            "MAGENTA",
            "CYAN",
            "RESET",
            "BOLD",
            "GIT_BRANCH",
            "GIT_CLEAN",
            "GIT_STAGED",
            "GIT_MODIFIED",
            "GIT_UNTRACKED",
            "GIT_AHEAD",
            "GIT_BEHIND",
            "PWD",
            "AGENT",
            "USAGE",
        ];

        let re = regex::Regex::new(r"\$\{([^}:]+)").unwrap();
        for caps in re.captures_iter(template) {
            let var_name = &caps[1];
            if !valid_vars.contains(&var_name) {
                return Err(format!("Unknown variable: {}", var_name));
            }
        }

        Ok(())
    }

    pub fn render_prompt(&self, template: &str) -> String {
        let mut result = template.to_string();

        // Process conditional formatting first
        result = self.process_conditional_formatting(&result);

        // Replace git variables
        result = self.replace_git_variables(&result);

        // Replace agent and usage variables
        result = self.replace_agent_variables(&result);

        // Replace color variables
        result = self.replace_color_variables(&result);

        result
    }

    fn process_conditional_formatting(&self, template: &str) -> String {
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

                    // Check if the variable has a value
                    let has_value = match var_name.as_str() {
                        "GIT_BRANCH" => self.git_info.branch.is_some(),
                        "GIT_CLEAN" => self.git_info.status.as_ref().is_some_and(|s| s.clean),
                        "GIT_STAGED" => self.git_info.status.as_ref().is_some_and(|s| s.staged),
                        "GIT_MODIFIED" => self.git_info.status.as_ref().is_some_and(|s| s.modified),
                        "GIT_UNTRACKED" => self.git_info.status.as_ref().is_some_and(|s| s.untracked),
                        "GIT_AHEAD" => self.git_info.status.as_ref().is_some_and(|s| s.ahead > 0),
                        "GIT_BEHIND" => self.git_info.status.as_ref().is_some_and(|s| s.behind > 0),
                        _ => false,
                    };

                    if has_value {
                        // Process the conditional text recursively for nested variables
                        let processed_text = self.replace_color_variables(&conditional_text);
                        let processed_text = self.replace_git_variables(&processed_text);
                        result.push_str(&processed_text);
                    }
                    // If empty or doesn't exist, add nothing
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

    fn replace_git_variables(&self, template: &str) -> String {
        let mut result = template.to_string();

        // Replace PWD (current directory with ~ substitution)
        let pwd = if let Some(home) = std::env::var_os("HOME") {
            let home_path = std::path::Path::new(&home);
            if let Ok(relative) = self.current_dir.strip_prefix(home_path) {
                format!("~/{}", relative.display())
            } else {
                self.current_dir.to_string_lossy().to_string()
            }
        } else {
            self.current_dir.to_string_lossy().to_string()
        };
        result = result.replace("${PWD}", &pwd);

        // Replace git branch
        if let Some(branch) = &self.git_info.branch {
            result = result.replace("${GIT_BRANCH}", branch);
        } else {
            result = result.replace("${GIT_BRANCH}", "");
        }

        // Replace git status indicators
        if let Some(status) = &self.git_info.status {
            result = result.replace("${GIT_CLEAN}", if status.clean { "✓" } else { "" });
            result = result.replace("${GIT_STAGED}", if status.staged { "●" } else { "" });
            result = result.replace("${GIT_MODIFIED}", if status.modified { "✚" } else { "" });
            result = result.replace("${GIT_UNTRACKED}", if status.untracked { "?" } else { "" });
            result = result.replace(
                "${GIT_AHEAD}",
                &if status.ahead > 0 {
                    format!("↑{}", status.ahead)
                } else {
                    String::new()
                },
            );
            result = result.replace(
                "${GIT_BEHIND}",
                &if status.behind > 0 {
                    format!("↓{}", status.behind)
                } else {
                    String::new()
                },
            );
        } else {
            // No git status available
            result = result.replace("${GIT_CLEAN}", "");
            result = result.replace("${GIT_STAGED}", "");
            result = result.replace("${GIT_MODIFIED}", "");
            result = result.replace("${GIT_UNTRACKED}", "");
            result = result.replace("${GIT_AHEAD}", "");
            result = result.replace("${GIT_BEHIND}", "");
        }

        result
    }

    fn replace_color_variables(&self, template: &str) -> String {
        template
            .replace("${RED}", RED)
            .replace("${GREEN}", GREEN)
            .replace("${YELLOW}", YELLOW)
            .replace("${BLUE}", BLUE)
            .replace("${MAGENTA}", MAGENTA)
            .replace("${CYAN}", CYAN)
            .replace("${RESET}", RESET)
            .replace("${BOLD}", BOLD)
    }

    fn replace_agent_variables(&self, template: &str) -> String {
        let mut result = template.to_string();

        result = self.replace_agent_variable(&result);
        result = self.replace_model_variable(&result);
        result = self.replace_token_usage_variable(&result);

        result
    }

    fn replace_agent_variable(&self, template: &str) -> String {
        let agent = std::env::var("Q_AGENT").unwrap_or_else(|_| "default".to_string());
        template.replace("${AGENT}", &agent)
    }

    fn replace_model_variable(&self, template: &str) -> String {
        let model = std::env::var("Q_MODEL").unwrap_or_else(|_| "unknown".to_string());
        template.replace("${MODEL}", &model)
    }

    fn replace_token_usage_variable(&self, template: &str) -> String {
        let token_usage = std::env::var("Q_TOKEN_USAGE").unwrap_or_else(|_| "(25.50%)".to_string());
        template
            .replace("${TOKEN_USAGE}", &token_usage)
            .replace("$TOKEN_USAGE", &token_usage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_basic_template() {
        let renderer = ThemeRenderer::new();
        let result = renderer.render_prompt("${GREEN}> ${RESET}");
        assert_eq!(result, "\x1b[32m> \x1b[0m");
    }

    #[test]
    fn test_validate_theme() {
        let renderer = ThemeRenderer::new();
        assert!(renderer.validate_theme("${GREEN}> ${RESET}").is_ok());
        assert!(renderer.validate_theme("${GREEN> ${RESET}").is_err());
    }

    #[test]
    fn test_model_variable_substitution() {
        let renderer = ThemeRenderer::new();

        // Test with environment variable set
        std::env::set_var("Q_MODEL", "claude-3");
        let result = renderer.render_prompt("Model: ${MODEL}");
        assert_eq!(result, "Model: claude-3");

        // Test with environment variable unset
        std::env::remove_var("Q_MODEL");
        let result = renderer.render_prompt("Model: ${MODEL}");
        assert_eq!(result, "Model: unknown");
    }

    #[test]
    fn test_token_usage_variable_substitution() {
        let renderer = ThemeRenderer::new();

        // Test with environment variable set
        std::env::set_var("Q_TOKEN_USAGE", "(75.25%)");
        let result1 = renderer.render_prompt("Usage: ${TOKEN_USAGE}");
        let result2 = renderer.render_prompt("Usage: $TOKEN_USAGE");
        assert_eq!(result1, "Usage: (75.25%)");
        assert_eq!(result2, "Usage: (75.25%)");

        // Test with environment variable unset
        std::env::remove_var("Q_TOKEN_USAGE");
        let result1 = renderer.render_prompt("Usage: ${TOKEN_USAGE}");
        let result2 = renderer.render_prompt("Usage: $TOKEN_USAGE");
        assert_eq!(result1, "Usage: (25.50%)");
        assert_eq!(result2, "Usage: (25.50%)");
    }

    #[test]
    fn test_pwd_variable_substitution() {
        let temp_dir = std::env::temp_dir().join("pwd_test");
        let _ = std::fs::create_dir_all(&temp_dir);

        let renderer = ThemeRenderer::new_for_path(&temp_dir);
        let result = renderer.render_prompt("Current: ${PWD}");

        assert!(result.contains(&temp_dir.to_string_lossy().to_string()));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
