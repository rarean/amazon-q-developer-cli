use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::renderer::ThemeRenderer;

pub struct ThemeManager {
    themes_dir: PathBuf,
    builtin_themes: HashMap<String, &'static str>,
}

impl ThemeManager {
    pub fn new(themes_dir: PathBuf) -> Self {
        let mut builtin_themes = HashMap::new();
        builtin_themes.insert("minimal".to_string(), "> ");
        builtin_themes.insert("powerline".to_string(), "${BLUE}${BOLD}❯${RESET} ${GIT_BRANCH:+${CYAN}⎇ ${GIT_BRANCH}${RESET} }${GIT_STAGED}${GIT_MODIFIED}${GIT_UNTRACKED}${GIT_AHEAD}${GIT_BEHIND}${GIT_CLEAN:+ }${YELLOW}❯${RESET} ");
        builtin_themes.insert("git-enabled".to_string(), "${GREEN}➜${RESET} ${GIT_BRANCH:+${BLUE}git:(${GIT_BRANCH})${RESET} }${GIT_STAGED:+${GIT_STAGED} }${GIT_MODIFIED:+${GIT_MODIFIED} }${GIT_UNTRACKED:+${GIT_UNTRACKED} }${GIT_AHEAD:+${GIT_AHEAD} }${GIT_BEHIND:+${GIT_BEHIND} }${GIT_CLEAN:+${GIT_CLEAN} }> ");

        Self {
            themes_dir,
            builtin_themes,
        }
    }

    pub fn list_themes(&self) -> Vec<String> {
        let mut themes = Vec::new();

        // Add builtin themes
        for name in self.builtin_themes.keys() {
            themes.push(format!("{} (builtin)", name));
        }

        // Add user themes
        if self.themes_dir.exists() {
            if let Ok(entries) = fs::read_dir(&self.themes_dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".theme") {
                            let theme_name = name.strip_suffix(".theme").unwrap_or(name);
                            themes.push(theme_name.to_string());
                        }
                    }
                }
            }
        }

        themes.sort();
        themes
    }

    pub fn load_theme(&self, name: &str) -> Result<String, String> {
        // Check builtin themes first
        if let Some(template) = self.builtin_themes.get(name) {
            return Ok(template.to_string());
        }

        // Check user themes
        let theme_path = self.themes_dir.join(format!("{}.theme", name));
        if theme_path.exists() {
            fs::read_to_string(&theme_path).map_err(|e| format!("Failed to read theme file: {}", e))
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }

    pub fn validate_theme(&self, name: &str) -> Result<(), String> {
        let template = self.load_theme(name)?;
        let renderer = ThemeRenderer::new();
        renderer.validate_theme(&template)
    }

    pub fn install_theme(&self, name: &str, template: &str) -> Result<(), String> {
        // Validate theme first
        let renderer = ThemeRenderer::new();
        renderer.validate_theme(template)?;

        // Create themes directory if it doesn't exist
        if !self.themes_dir.exists() {
            fs::create_dir_all(&self.themes_dir).map_err(|e| format!("Failed to create themes directory: {}", e))?;
        }

        // Write theme file
        let theme_path = self.themes_dir.join(format!("{}.theme", name));
        fs::write(&theme_path, template).map_err(|e| format!("Failed to write theme file: {}", e))?;

        Ok(())
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
    fn test_install_theme_invalid_template() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        let result = manager.install_theme("test", "${INVALID_VAR}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown variable: INVALID_VAR"));
    }

    #[test]
    fn test_install_theme_unbalanced_braces() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        let result = manager.install_theme("test", "${RED");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unbalanced braces"));
    }

    #[test]
    fn test_install_theme_success() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        let result = manager.install_theme("test", "${RED}> ${RESET}");
        assert!(result.is_ok());

        let loaded = manager.load_theme("test");
        assert!(loaded.is_ok());
        assert_eq!(loaded.unwrap(), "${RED}> ${RESET}");
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
    fn test_list_themes_with_user_themes() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::new(temp_dir.path().to_path_buf());

        // Install a user theme
        manager.install_theme("custom", "${BLUE}> ${RESET}").unwrap();

        let themes = manager.list_themes();
        assert!(themes.contains(&"custom".to_string()));
        assert!(themes.contains(&"minimal (builtin)".to_string()));
    }
}
