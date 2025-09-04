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

    /// Get the currently active theme
    pub fn get_active_theme(&self) -> Option<&BashTheme> {
        self.active_theme.as_ref()
    }

    /// Try to load the default theme
    pub fn load_default_theme(&mut self) -> Result<()> {
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

        if !os.fs.exists(&self.theme_dir) {
            return Ok(themes);
        }

        let mut entries = os.fs.read_dir(&self.theme_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".theme") {
                    let theme_name = name.trim_end_matches(".theme");
                    themes.push(theme_name.to_string());
                }
            }
        }

        themes.sort();
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
}
