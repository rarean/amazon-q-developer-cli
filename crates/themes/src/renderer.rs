use std::env;
use std::path::Path;

use crate::git::GitInfo;

pub struct ThemeRenderer {
    git_info: GitInfo,
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

        Self { git_info }
    }

    pub fn new_for_path(path: &Path) -> Self {
        let git_info = if path.exists() {
            GitInfo::detect(path)
        } else {
            GitInfo::empty()
        };

        Self { git_info }
    }

    pub fn render_prompt(&self, template: &str) -> String {
        let mut result = template.to_string();

        // Process conditional formatting first
        result = self.process_conditional_formatting(&result);

        // Replace git variables
        if let Some(branch) = &self.git_info.branch {
            result = result.replace("${GIT_BRANCH}", branch);
        } else {
            result = result.replace("${GIT_BRANCH}", "");
        }

        // Replace git status indicators with colored versions
        if let Some(status) = &self.git_info.status {
            result = result.replace("${GIT_CLEAN}", if status.clean { "\x1b[32m✓\x1b[0m" } else { "" });
            result = result.replace("${GIT_STAGED}", if status.staged { "\x1b[32m●\x1b[0m" } else { "" });
            result = result.replace("${GIT_MODIFIED}", if status.modified { "\x1b[33m±\x1b[0m" } else { "" });
            result = result.replace(
                "${GIT_UNTRACKED}",
                if status.untracked { "\x1b[31m?\x1b[0m" } else { "" },
            );
            result = result.replace("${GIT_AHEAD}", if status.ahead > 0 { "\x1b[36m↑\x1b[0m" } else { "" });
            result = result.replace("${GIT_BEHIND}", if status.behind > 0 { "\x1b[35m↓\x1b[0m" } else { "" });
        } else {
            result = result.replace("${GIT_CLEAN}", "");
            result = result.replace("${GIT_STAGED}", "");
            result = result.replace("${GIT_MODIFIED}", "");
            result = result.replace("${GIT_UNTRACKED}", "");
            result = result.replace("${GIT_AHEAD}", "");
            result = result.replace("${GIT_BEHIND}", "");
        }

        // Process color codes last
        result = self.process_colors(&result);

        result
    }

    fn process_colors(&self, template: &str) -> String {
        template
            .replace("${RED}", RED)
            .replace("${GREEN}", GREEN)
            .replace("${YELLOW}", YELLOW)
            .replace("${BLUE}", BLUE)
            .replace("${MAGENTA}", MAGENTA)
            .replace("${CYAN}", CYAN)
            .replace("${BOLD}", BOLD)
            .replace("${RESET}", RESET)
    }

    fn process_conditional_formatting(&self, template: &str) -> String {
        use regex::Regex;

        let re = Regex::new(r"\$\{([^:}]+):?\+([^}]*)\}").unwrap();

        re.replace_all(template, |caps: &regex::Captures| {
            let var_name = &caps[1];
            let value_if_set = &caps[2];

            let is_set = match var_name {
                "GIT_BRANCH" => self.git_info.branch.is_some(),
                "GIT_CLEAN" => self.git_info.status.as_ref().is_some_and(|s| s.clean),
                "GIT_STAGED" => self.git_info.status.as_ref().is_some_and(|s| s.staged),
                "GIT_MODIFIED" => self.git_info.status.as_ref().is_some_and(|s| s.modified),
                "GIT_UNTRACKED" => self.git_info.status.as_ref().is_some_and(|s| s.untracked),
                "GIT_AHEAD" => self.git_info.status.as_ref().is_some_and(|s| s.ahead > 0),
                "GIT_BEHIND" => self.git_info.status.as_ref().is_some_and(|s| s.behind > 0),
                _ => false,
            };

            if is_set {
                value_if_set.to_string()
            } else {
                String::new()
            }
        })
        .to_string()
    }

    pub fn has_git_repo(&self) -> bool {
        self.git_info.is_repo
    }

    pub fn validate_theme(&self, template: &str) -> Result<(), String> {
        // Check for balanced braces
        let mut brace_count = 0;
        let mut in_variable = false;

        for ch in template.chars() {
            match ch {
                '$' => {
                    // Check if next char is '{'
                    continue;
                },
                '{' if template.chars().nth(template.find(ch).unwrap_or(0).saturating_sub(1)) == Some('$') => {
                    brace_count += 1;
                    in_variable = true;
                },
                '}' if in_variable => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        in_variable = false;
                    }
                },
                _ => continue,
            }
        }

        if brace_count != 0 {
            return Err("Unbalanced braces in theme template".to_string());
        }

        // Validate variable names
        use regex::Regex;
        let var_re = Regex::new(r"\$\{([^}:]+)(?::[^}]*)?\}").unwrap();

        for caps in var_re.captures_iter(template) {
            let var_name = &caps[1];
            match var_name {
                "GIT_BRANCH" | "GIT_CLEAN" | "GIT_STAGED" | "GIT_MODIFIED" | "GIT_UNTRACKED" | "GIT_AHEAD"
                | "GIT_BEHIND" | "RED" | "GREEN" | "YELLOW" | "BLUE" | "MAGENTA" | "CYAN" | "BOLD" | "RESET" => {
                    // Valid variable
                },
                _ => {
                    return Err(format!("Unknown variable: {}", var_name));
                },
            }
        }

        Ok(())
    }
}
