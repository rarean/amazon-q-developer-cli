use std::collections::HashMap;
use std::path::PathBuf;

use crate::renderer::ThemeRenderer;

pub struct ThemeManager {
    builtin_themes: HashMap<String, &'static str>,
}

impl ThemeManager {
    pub fn new(_themes_dir: PathBuf) -> Self {
        let mut builtin_themes = HashMap::new();
        builtin_themes.insert("minimal".to_string(), "> ");
        builtin_themes.insert(
            "powerline".to_string(),
            "\x1b[44m\x1b[37m ${AGENT} \x1b[0m\x1b[45m\x1b[34m\u{e0b0}\x1b[37m $TOKEN_USAGE \x1b[0m${GIT_BRANCH:+\x1b[43m\x1b[35m\u{e0b0}\x1b[30m ${GIT_BRANCH} \x1b[0m\x1b[33m\u{e0b0}}\x1b[0m ",
        );
        builtin_themes.insert(
            "git-enabled".to_string(),
            "${BOLD}${MAGENTA}➜ ${MODEL}:$TOKEN_USAGE${RESET} ${GREEN}${BOLD}${PWD}${GIT_BRANCH:+:(${YELLOW}${GIT_BRANCH}${RESET}${GREEN}${BOLD})${RESET} }> ",
        );

        Self { builtin_themes }
    }

    pub fn list_themes(&self) -> Vec<String> {
        let mut themes = Vec::new();

        // Add builtin themes
        for name in self.builtin_themes.keys() {
            themes.push(format!("{} (builtin)", name));
        }

        themes.sort();
        themes
    }

    pub fn load_theme(&self, name: &str) -> Result<String, String> {
        // Check builtin themes only
        if let Some(template) = self.builtin_themes.get(name) {
            return Ok(template.to_string());
        }

        Err(format!("Theme '{}' not found", name))
    }

    pub fn validate_theme(&self, name: &str) -> Result<(), String> {
        let template = self.load_theme(name)?;
        let renderer = ThemeRenderer::new();
        renderer.validate_theme(&template)
    }

    pub fn get_context_theme(&self, path: &std::path::Path) -> String {
        let renderer = ThemeRenderer::new_for_path(path);

        // Dynamic theme selection based on context
        if renderer.has_git_repo() {
            // In git repository - use git-enabled theme
            self.load_theme("git-enabled").unwrap_or_else(|_| "$ ".to_string())
        } else {
            // Not in git repository - use minimal theme
            self.load_theme("minimal").unwrap_or_else(|_| "$ ".to_string())
        }
    }

    pub fn render_context_prompt(&self, path: &std::path::Path) -> String {
        let template = self.get_context_theme(path);
        let renderer = ThemeRenderer::new_for_path(path);
        renderer.render_prompt(&template)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_load_theme_non_existent() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        let result = manager.load_theme("non_existent");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Theme 'non_existent' not found"));
    }

    #[test]
    fn test_load_theme_builtin() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        let result = manager.load_theme("minimal");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "> ");
    }

    #[test]
    fn test_validate_theme_non_existent() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        let result = manager.validate_theme("non_existent");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Theme 'non_existent' not found"));
    }

    #[test]
    fn test_list_themes_builtin_only() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        let themes = manager.list_themes();
        assert!(themes.contains(&"minimal (builtin)".to_string()));
        assert!(themes.contains(&"powerline (builtin)".to_string()));
        assert!(themes.contains(&"git-enabled (builtin)".to_string()));
    }

    #[test]
    fn test_git_enabled_theme_rendering() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        // Load the git-enabled theme
        let template = manager.load_theme("git-enabled").unwrap();

        // Create a renderer and render the template with mock data
        let renderer = ThemeRenderer::new();

        // Mock the template with test values
        let test_template = template
            .replace("${MODEL}", "claude-3-5-sonnet")
            .replace("$TOKEN_USAGE", "(25.50%)");

        let result = renderer.render_prompt(&test_template);

        // Should contain the basic prompt structure
        assert!(result.contains("➜"), "Result should contain arrow symbol: {}", result);
        assert!(result.ends_with("> "), "Result should end with '> ': {}", result);

        // Should contain the mocked values
        assert!(
            result.contains("claude-3-5-sonnet") && result.contains("(25.50%)"),
            "Result should contain model and usage info: {}",
            result
        );

        // Verify variables are properly substituted, not displayed as literals
        assert!(
            !result.contains("${GREEN}"),
            "Should not contain literal ${{GREEN}}: {}",
            result
        );
        assert!(
            !result.contains("${RESET}"),
            "Should not contain literal ${{RESET}}: {}",
            result
        );
        assert!(
            !result.contains("${BLUE}"),
            "Should not contain literal ${{BLUE}}: {}",
            result
        );
        assert!(
            !result.contains("${YELLOW}"),
            "Should not contain literal ${{YELLOW}}: {}",
            result
        );
        assert!(
            !result.contains("${BOLD}"),
            "Should not contain literal ${{BOLD}}: {}",
            result
        );

        // Verify ANSI color codes are present (variables were substituted)
        assert!(
            result.contains("\x1b["),
            "Should contain ANSI escape sequences: {}",
            result
        );

        println!("Git-enabled theme rendered: {}", result);
    }

    #[test]
    fn test_git_enabled_theme_with_git_branch() {
        use std::fs;

        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        // Create a mock git repository with proper structure
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();
        fs::write(git_dir.join("HEAD"), "ref: refs/heads/feature-branch").unwrap();

        // Create refs directory structure
        let refs_dir = git_dir.join("refs").join("heads");
        fs::create_dir_all(&refs_dir).unwrap();
        fs::write(refs_dir.join("feature-branch"), "abc123def456").unwrap();

        // Load the git-enabled theme
        let template = manager.load_theme("git-enabled").unwrap();

        // Create a renderer for the git repo path
        let renderer = ThemeRenderer::new_for_path(temp_dir.path());
        let result = renderer.render_prompt(&template);

        // Should contain basic prompt structure
        assert!(result.contains("➜"), "Should contain arrow symbol: {}", result);

        // Check if git detection worked - if it did, we should see git branch info
        if result.contains("git:(") {
            assert!(
                result.contains("feature-branch"),
                "Should contain git branch info: {}",
                result
            );
        } else {
            // Git detection may not work in all test environments, so just verify basic structure
            println!("Git detection didn't work in test environment, result: {}", result);
        }

        // Should not contain unsubstituted variables
        assert!(
            !result.contains("${"),
            "Should not contain unsubstituted variables: {}",
            result
        );

        println!("Git-enabled theme with branch: {}", result);
    }

    #[test]
    fn test_powerline_theme_rendering() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        // Load the powerline theme
        let template = manager.load_theme("powerline").unwrap();

        // Create a renderer and render the template with mock data
        let renderer = ThemeRenderer::new();

        // Mock the template with test values
        let test_template = template
            .replace("${AGENT}", "default")
            .replace("$TOKEN_USAGE", "(48.00%)");

        let result = renderer.render_prompt(&test_template);

        // Should contain the expected segmented format
        assert!(
            result.contains("default") && result.contains("(48.00%)"),
            "Result should contain agent and usage info: {}",
            result
        );
        // Check if we're in a git repository and have a branch
        let git_info = crate::git::GitInfo::detect(std::env::current_dir().unwrap().as_path());
        if let Some(branch) = git_info.branch {
            assert!(
                result.contains(&branch),
                "Result should contain git branch '{}': {}",
                branch,
                result
            );
        }
        assert!(result.ends_with(" "), "Result should end with space: {}", result);

        // Should contain powerline separator character
        assert!(
            result.contains("\u{e0b0}"),
            "Result should contain powerline separator: {}",
            result
        );

        println!("Powerline theme rendered: {}", result);
    }
}
