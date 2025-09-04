use std::collections::HashMap;

use super::bash_parser::BashParser;
use super::{
    BashTheme,
    GitInfo,
};

pub struct ThemeRenderer<'a> {
    theme: &'a BashTheme,
}

impl<'a> ThemeRenderer<'a> {
    pub fn new(theme: &'a BashTheme) -> Self {
        Self { theme }
    }

    /// Render a prompt using the theme
    pub fn render_prompt(
        &self,
        agent: Option<&str>,
        warning: bool,
        tangent_mode: bool,
        git_info: Option<&GitInfo>,
        token_usage_percent: Option<f32>,
    ) -> String {
        let mut context_vars =
            self.build_context_variables(agent, warning, tangent_mode, git_info, token_usage_percent);

        // Add theme variables
        for (key, value) in &self.theme.variables {
            context_vars.insert(key.clone(), value.clone());
        }

        BashParser::substitute_variables(&self.theme.prompt_template, &context_vars)
    }

    /// Build context variables for the current prompt state
    fn build_context_variables(
        &self,
        agent: Option<&str>,
        warning: bool,
        tangent_mode: bool,
        git_info: Option<&GitInfo>,
        token_usage_percent: Option<f32>,
    ) -> HashMap<String, String> {
        let mut vars = HashMap::new();

        // Core Q CLI variables
        if let Some(agent_name) = agent {
            vars.insert("Q_AGENT".to_string(), agent_name.to_string());
        } else {
            vars.insert("Q_AGENT".to_string(), String::new());
        }

        if warning {
            vars.insert("Q_WARNING".to_string(), "1".to_string());
        }

        if tangent_mode {
            vars.insert("Q_TANGENT".to_string(), "1".to_string());
        }

        // Token usage variable
        if let Some(usage) = token_usage_percent {
            vars.insert("TOKEN_USAGE".to_string(), format!("({:.2}%)", usage));
        } else {
            vars.insert("TOKEN_USAGE".to_string(), String::new());
        }

        // Git variables (if enabled and available)
        if self.theme.git_enabled {
            if let Some(git) = git_info {
                if git.is_repo {
                    if let Some(branch) = &git.branch {
                        vars.insert("GIT_BRANCH".to_string(), branch.clone());
                        vars.insert("Q_GIT_BRANCH".to_string(), branch.clone());
                    }

                    // Set git status variables
                    if git.is_dirty {
                        vars.insert(
                            "GIT_DIRTY".to_string(),
                            self.theme
                                .get_variable("Q_GIT_DIRTY")
                                .unwrap_or(&"±".to_string())
                                .clone(),
                        );
                        vars.insert(
                            "GIT_MODIFIED".to_string(),
                            self.theme
                                .get_variable("Q_GIT_DIRTY")
                                .unwrap_or(&"±".to_string())
                                .clone(),
                        );
                        vars.insert("GIT_CLEAN".to_string(), String::new());
                    } else {
                        vars.insert(
                            "GIT_CLEAN".to_string(),
                            self.theme
                                .get_variable("Q_GIT_CLEAN")
                                .unwrap_or(&"✓".to_string())
                                .clone(),
                        );
                        vars.insert("GIT_DIRTY".to_string(), String::new());
                        vars.insert("GIT_MODIFIED".to_string(), String::new());
                    }

                    // Set other git status variables (empty for now since we don't have detailed git status)
                    vars.insert("GIT_STAGED".to_string(), String::new());
                    vars.insert("GIT_UNTRACKED".to_string(), String::new());
                    vars.insert("GIT_AHEAD".to_string(), String::new());
                    vars.insert("GIT_BEHIND".to_string(), String::new());

                    vars.insert(
                        "Q_GIT_STATUS".to_string(),
                        if git.is_dirty { "dirty" } else { "clean" }.to_string(),
                    );

                    // Format git info using theme variables
                    let git_info_str = self.format_git_info(git);
                    vars.insert("Q_GIT_INFO".to_string(), git_info_str);
                }
            }
        }

        vars
    }

    /// Format git information using theme configuration
    fn format_git_info(&self, git: &GitInfo) -> String {
        if !git.is_repo {
            return String::new();
        }

        let prefix = self.theme.get_variable("Q_GIT_PREFIX").map_or("", |s| s.as_str());
        let suffix = self.theme.get_variable("Q_GIT_SUFFIX").map_or("", |s| s.as_str());
        let branch = git.branch.as_deref().unwrap_or("unknown");

        let status_symbol = if git.is_dirty {
            self.theme
                .get_variable("Q_GIT_DIRTY_SYMBOL")
                .map_or("✗", |s| s.as_str())
        } else {
            self.theme
                .get_variable("Q_GIT_CLEAN_SYMBOL")
                .map_or("✓", |s| s.as_str())
        };

        format!("{}{} {}{}", prefix, branch, status_symbol, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_theme() -> BashTheme {
        let mut theme = BashTheme::new("test".to_string());
        theme.prompt_template = "$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR $Q_PROMPT_SYMBOL ".to_string();
        theme.set_variable("Q_AGENT_COLOR".to_string(), "\u{001b}[36m".to_string());
        theme.set_variable("RESET_COLOR".to_string(), "\u{001b}[0m".to_string());
        theme.set_variable("Q_PROMPT_SYMBOL".to_string(), ">".to_string());
        theme
    }

    #[test]
    fn test_render_basic_prompt() {
        let theme = create_test_theme();
        let renderer = ThemeRenderer::new(&theme);

        let result = renderer.render_prompt(Some("test-agent"), false, false, None, None);
        assert_eq!(result, "\u{001b}[36m[test-agent]\u{001b}[0m > ");
    }

    #[test]
    fn test_render_prompt_no_agent() {
        let theme = create_test_theme();
        let renderer = ThemeRenderer::new(&theme);

        let result = renderer.render_prompt(None, false, false, None, None);
        assert_eq!(result, "\u{001b}[36m[]\u{001b}[0m > ");
    }

    #[test]
    fn test_render_prompt_with_git_variables() {
        let mut theme = BashTheme::new("test".to_string());
        theme.prompt_template =
            "$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR ${GIT_BRANCH:+($GIT_BRANCH$GIT_CLEAN)} > ".to_string();
        theme.set_variable("Q_AGENT_COLOR".to_string(), "\u{001b}[36m".to_string());
        theme.set_variable("RESET_COLOR".to_string(), "\u{001b}[0m".to_string());
        theme.set_variable("Q_GIT_CLEAN".to_string(), "✓".to_string());
        theme.git_enabled = true;

        let renderer = ThemeRenderer::new(&theme);

        let git_info = GitInfo {
            branch: Some("main".to_string()),
            is_dirty: false,
            is_repo: true,
        };

        let result = renderer.render_prompt(Some("test-agent"), false, false, Some(&git_info), None);

        // Should contain the git branch and clean symbol
        assert!(result.contains("main"));
        assert!(result.contains("✓"));
        assert!(result.contains("test-agent"));
    }

    #[test]
    fn test_render_prompt_with_dirty_git_status() {
        let mut theme = BashTheme::new("test".to_string());
        theme.prompt_template =
            "$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR ${GIT_BRANCH:+($GIT_BRANCH$GIT_DIRTY)} > ".to_string();
        theme.set_variable("Q_AGENT_COLOR".to_string(), "\u{001b}[36m".to_string());
        theme.set_variable("RESET_COLOR".to_string(), "\u{001b}[0m".to_string());
        theme.set_variable("Q_GIT_DIRTY".to_string(), "±".to_string());
        theme.git_enabled = true;

        let renderer = ThemeRenderer::new(&theme);

        let git_info = GitInfo {
            branch: Some("feature".to_string()),
            is_dirty: true,
            is_repo: true,
        };

        let result = renderer.render_prompt(Some("test-agent"), false, false, Some(&git_info), None);

        // Should contain the git branch and dirty symbol
        assert!(result.contains("feature"));
        assert!(result.contains("±"));
        assert!(!result.contains("✓")); // Should not contain clean symbol
    }

    #[test]
    fn test_render_prompt_no_git_repo() {
        let mut theme = BashTheme::new("test".to_string());
        theme.prompt_template = "$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR ${GIT_BRANCH:+($GIT_BRANCH)} > ".to_string();
        theme.set_variable("Q_AGENT_COLOR".to_string(), "\u{001b}[36m".to_string());
        theme.set_variable("RESET_COLOR".to_string(), "\u{001b}[0m".to_string());
        theme.git_enabled = true;

        let renderer = ThemeRenderer::new(&theme);

        let git_info = GitInfo {
            branch: None,
            is_dirty: false,
            is_repo: false,
        };

        let result = renderer.render_prompt(Some("test-agent"), false, false, Some(&git_info), None);

        // Should not contain git information when not in a repo
        assert!(!result.contains("("));
        assert!(!result.contains(")"));
        assert!(result.contains("test-agent"));
    }

    #[test]
    fn test_render_prompt_with_token_usage() {
        let theme = BashTheme {
            name: "test".to_string(),
            prompt_template: "$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR $TOKEN_USAGE$Q_PROMPT_SYMBOL ".to_string(),
            variables: HashMap::new(),
            git_enabled: false,
        };

        let renderer = ThemeRenderer::new(&theme);
        let result = renderer.render_prompt(Some("test-agent"), false, false, None, Some(48.35));

        // Should contain the token usage percentage
        assert!(result.contains("(48.35%)"));
        assert!(result.contains("test-agent"));
    }

    #[test]
    fn test_render_prompt_without_token_usage() {
        let theme = BashTheme {
            name: "test".to_string(),
            prompt_template: "$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR $TOKEN_USAGE$Q_PROMPT_SYMBOL ".to_string(),
            variables: HashMap::new(),
            git_enabled: false,
        };

        let renderer = ThemeRenderer::new(&theme);
        let result = renderer.render_prompt(Some("test-agent"), false, false, None, None);

        // Should not contain token usage when not provided
        assert!(!result.contains("("));
        assert!(!result.contains("%)"));
        assert!(result.contains("test-agent"));
    }
}
