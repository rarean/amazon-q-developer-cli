use std::path::PathBuf;

use eyre::Result;
use tracing::{
    debug,
    warn,
};

use super::BashTheme;
use super::bash_parser::BashParser;
use crate::os::Os;
use crate::util::directories;

pub struct ThemeManager {
    active_theme: Option<BashTheme>,
    theme_dir: PathBuf,
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new(os: &Os) -> Result<Self> {
        let theme_dir = directories::chat_themes_dir(os)?;

        Ok(Self {
            active_theme: None,
            theme_dir,
        })
    }

    /// Load a theme by name
    pub fn load_theme(&mut self, name: &str) -> Result<()> {
        // First try to load builtin themes from the standalone themes system
        if let Ok(builtin_theme) = Self::load_builtin_theme(name) {
            debug!("Loaded builtin theme: {}", builtin_theme.name);
            self.active_theme = Some(builtin_theme);
            return Ok(());
        }

        // Fallback to loading from theme files
        let theme_path = self.theme_dir.join(format!("{}.theme", name));

        if !theme_path.exists() {
            debug!("Theme file not found: {}", theme_path.display());
            return Ok(()); // Graceful fallback - no theme loaded
        }

        match BashParser::parse_theme_file(&theme_path) {
            Ok(theme) => {
                debug!("Loaded theme: {}", theme.name);
                self.active_theme = Some(theme);
            },
            Err(e) => {
                warn!("Failed to parse theme {}: {}", name, e);
                // Graceful fallback - keep existing theme or none
            },
        }

        Ok(())
    }

    /// Load a builtin theme from the standalone themes system
    fn load_builtin_theme(name: &str) -> Result<BashTheme> {
        // Create a standalone theme manager to access builtin themes
        let standalone_manager = themes::ThemeManager::new(PathBuf::new());

        match standalone_manager.load_theme(name) {
            Ok(template) => {
                // Convert the standalone theme template to a BashTheme
                let mut bash_theme = BashTheme::new(name.to_string());
                bash_theme.git_enabled = template.contains("GIT_BRANCH");
                bash_theme.prompt_template = template;

                // Add standard color variables that themes expect
                bash_theme.set_variable("GREEN".to_string(), "\x1b[32m".to_string());
                bash_theme.set_variable("RED".to_string(), "\x1b[31m".to_string());
                bash_theme.set_variable("YELLOW".to_string(), "\x1b[33m".to_string());
                bash_theme.set_variable("BLUE".to_string(), "\x1b[34m".to_string());
                bash_theme.set_variable("MAGENTA".to_string(), "\x1b[35m".to_string());
                bash_theme.set_variable("CYAN".to_string(), "\x1b[36m".to_string());
                bash_theme.set_variable("RESET".to_string(), "\x1b[0m".to_string());
                bash_theme.set_variable("BOLD".to_string(), "\x1b[1m".to_string());

                Ok(bash_theme)
            },
            Err(e) => Err(eyre::eyre!("Failed to load builtin theme: {}", e)),
        }
    }

    /// Get the currently active theme
    pub fn get_active_theme(&self) -> Option<&BashTheme> {
        self.active_theme.as_ref()
    }

    /// Try to load the default theme
    pub fn load_default_theme(&mut self) -> Result<()> {
        // Try to load "minimal" as the default builtin theme
        if self.load_theme("minimal").is_ok() && self.active_theme.is_some() {
            return Ok(());
        }

        // Fallback to loading "default" from theme files
        self.load_theme("default")
    }

    /// Check if themes directory exists and create if needed
    pub async fn ensure_theme_directory(&self, os: &Os) -> Result<()> {
        if !os.fs.exists(&self.theme_dir) {
            os.fs.create_dir_all(&self.theme_dir).await?;
            debug!("Created themes directory: {}", self.theme_dir.display());
        }
        Ok(())
    }

    /// List available themes
    #[allow(dead_code)] // Will be used in Phase 3 CLI commands
    pub async fn list_available_themes(&self, os: &Os) -> Result<Vec<String>> {
        let mut themes = Vec::new();

        // Add builtin themes
        let standalone_manager = themes::ThemeManager::new(PathBuf::new());
        let builtin_themes = standalone_manager.list_themes();
        for theme in builtin_themes {
            // Remove " (builtin)" suffix if present
            let theme_name = theme.replace(" (builtin)", "");
            themes.push(theme_name);
        }

        // Add file-based themes
        if os.fs.exists(&self.theme_dir) {
            let mut entries = os.fs.read_dir(&self.theme_dir).await?;
            while let Some(entry) = entries.next_entry().await? {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".theme") {
                        let theme_name = name.trim_end_matches(".theme");
                        themes.push(theme_name.to_string());
                    }
                }
            }
        }

        themes.sort();
        themes.dedup(); // Remove duplicates in case builtin and file themes have same name
        Ok(themes)
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            active_theme: None,
            theme_dir: PathBuf::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::Os;

    #[tokio::test]
    async fn test_theme_manager_creation() {
        let os = Os::new().await.unwrap();
        let manager = ThemeManager::new(&os);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_load_nonexistent_theme() {
        let os = Os::new().await.unwrap();
        let mut manager = ThemeManager::new(&os).unwrap();

        // Should not fail, just not load anything
        let result = manager.load_theme("nonexistent");
        assert!(result.is_ok());
        assert!(manager.get_active_theme().is_none());
    }

    #[tokio::test]
    async fn test_load_builtin_theme() {
        let os = Os::new().await.unwrap();
        let mut manager = ThemeManager::new(&os).unwrap();

        // Should be able to load builtin themes
        let result = manager.load_theme("powerline");
        assert!(result.is_ok());

        if let Some(theme) = manager.get_active_theme() {
            assert_eq!(theme.name, "powerline");
            assert!(theme.git_enabled); // powerline theme should have git support
        }
    }
}
