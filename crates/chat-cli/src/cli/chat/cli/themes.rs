use std::path::PathBuf;

use clap::Subcommand;
use crossterm::queue;
use crossterm::style::{
    self,
    Color,
};
use dialoguer::Select;
use themes::{
    ThemeManager,
    ThemeRenderer,
};

use crate::cli::chat::{
    ChatError,
    ChatSession,
    ChatState,
};
use crate::database::settings::Setting;
use crate::os::Os;

/// Theme management commands
#[derive(Clone, Debug, PartialEq, Eq, Subcommand)]
pub enum ThemesSubcommand {
    /// List available themes
    List,
    /// Switch to a theme
    Switch { name: String },
    /// Preview a theme
    Preview { name: String },
    /// Show current theme
    Current,
    /// Interactive theme selection
    Select,
}

impl ThemesSubcommand {
    pub async fn execute(self, os: &mut Os, session: &mut ChatSession) -> Result<ChatState, ChatError> {
        if !Self::is_feature_enabled(os) {
            Self::write_feature_disabled_message(session)?;
            return Ok(Self::default_chat_state());
        }

        match self {
            Self::List => Self::execute_list(os, session).await,
            Self::Switch { ref name } => Self::execute_switch(os, session, name).await,
            Self::Preview { ref name } => Self::execute_preview(os, session, name).await,
            Self::Current => Self::execute_current(os, session).await,
            Self::Select => Self::execute_select(os, session).await,
        }
    }

    fn is_feature_enabled(os: &Os) -> bool {
        os.database.settings.get_bool(Setting::EnabledThemes).unwrap_or(false)
    }

    fn write_feature_disabled_message(session: &mut ChatSession) -> Result<(), std::io::Error> {
        queue!(
            session.stderr,
            style::SetForegroundColor(Color::Red),
            style::Print("\nThemes tool is disabled. Enable it with: /experiment\n"),
            style::SetForegroundColor(Color::Yellow),
            style::Print("💡 Select 'Themes' from the experiment list to enable theme switching.\n\n"),
            style::SetForegroundColor(Color::Reset)
        )
    }

    fn default_chat_state() -> ChatState {
        ChatState::PromptUser {
            skip_printing_tools: false,
        }
    }

    fn get_theme_manager() -> ThemeManager {
        // Use a dummy path since we only use builtin themes
        let themes_dir = PathBuf::from("/dev/null");
        ThemeManager::new(themes_dir)
    }

    async fn execute_list(_os: &mut Os, session: &mut ChatSession) -> Result<ChatState, ChatError> {
        let manager = Self::get_theme_manager();
        let themes = manager.list_themes();

        queue!(
            session.stderr,
            style::SetForegroundColor(Color::Green),
            style::Print("\nAvailable themes:\n"),
            style::SetForegroundColor(Color::Reset)
        )?;

        for theme in themes {
            queue!(session.stderr, style::Print(format!("  • {}\n", theme)))?;
        }
        queue!(session.stderr, style::Print("\n"))?;

        Ok(Self::default_chat_state())
    }

    async fn execute_switch(os: &mut Os, session: &mut ChatSession, name: &str) -> Result<ChatState, ChatError> {
        // Try to load the theme into the session's theme manager
        let theme_loaded = if let Some(ref mut theme_manager) = session.theme_manager {
            theme_manager.load_theme(name).is_ok()
        } else {
            false
        };

        if theme_loaded {
            // Store the current theme in settings
            if let Err(e) = os.database.settings.set(Setting::CurrentTheme, name.to_string()).await {
                return Err(ChatError::Custom(
                    format!("Failed to save theme preference: {}", e).into(),
                ));
            }

            queue!(
                session.stderr,
                style::SetForegroundColor(Color::Green),
                style::Print(format!("\n✓ Switched to theme: {}\n\n", name)),
                style::SetForegroundColor(Color::Reset)
            )?;
        } else {
            queue!(
                session.stderr,
                style::SetForegroundColor(Color::Red),
                style::Print(format!("\n❌ Theme '{}' not found\n\n", name)),
                style::SetForegroundColor(Color::Reset)
            )?;
        }

        Ok(Self::default_chat_state())
    }

    async fn execute_preview(_os: &mut Os, session: &mut ChatSession, name: &str) -> Result<ChatState, ChatError> {
        let manager = Self::get_theme_manager();

        match manager.load_theme(name) {
            Ok(template) => {
                let renderer = ThemeRenderer::new();
                let preview = renderer.render_prompt(&template);

                queue!(
                    session.stderr,
                    style::SetForegroundColor(Color::Cyan),
                    style::Print(format!("\nPreview of theme '{}':\n", name)),
                    style::SetForegroundColor(Color::Reset),
                    style::Print(format!("{}\n\n", preview)),
                )?;
            },
            Err(e) => {
                queue!(
                    session.stderr,
                    style::SetForegroundColor(Color::Red),
                    style::Print(format!("\n❌ {}\n\n", e)),
                    style::SetForegroundColor(Color::Reset)
                )?;
            },
        }

        Ok(Self::default_chat_state())
    }

    async fn execute_current(os: &mut Os, session: &mut ChatSession) -> Result<ChatState, ChatError> {
        let current_theme = os
            .database
            .settings
            .get_string(Setting::CurrentTheme)
            .unwrap_or("default".to_string());

        queue!(
            session.stderr,
            style::SetForegroundColor(Color::Blue),
            style::Print(format!("\nCurrent theme: {}\n\n", current_theme)),
            style::SetForegroundColor(Color::Reset)
        )?;

        Ok(Self::default_chat_state())
    }

    async fn execute_select(os: &mut Os, session: &mut ChatSession) -> Result<ChatState, ChatError> {
        let manager = Self::get_theme_manager();
        let themes = manager.list_themes();

        if themes.is_empty() {
            queue!(
                session.stderr,
                style::SetForegroundColor(Color::Yellow),
                style::Print("\nNo themes available.\n\n"),
                style::SetForegroundColor(Color::Reset)
            )?;
            return Ok(Self::default_chat_state());
        }

        let selection: Option<_> = match Select::with_theme(&crate::util::dialoguer_theme())
            .with_prompt("Select a theme")
            .items(&themes)
            .default(0)
            .interact_on_opt(&dialoguer::console::Term::stdout())
        {
            Ok(sel) => {
                let _ = crossterm::execute!(
                    std::io::stdout(),
                    crossterm::style::SetForegroundColor(crossterm::style::Color::Magenta)
                );
                sel
            },
            Err(dialoguer::Error::IO(ref e)) if e.kind() == std::io::ErrorKind::Interrupted => {
                return Ok(Self::default_chat_state());
            },
            Err(e) => return Err(ChatError::Custom(format!("Failed to choose theme: {e}").into())),
        };

        queue!(session.stderr, style::ResetColor)?;

        if let Some(index) = selection {
            // Clear the dialoguer selection line
            queue!(
                session.stderr,
                crossterm::cursor::MoveUp(1),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown),
            )?;

            if index < themes.len() {
                let selected_theme = &themes[index];
                // Remove " (builtin)" suffix if present
                let theme_name = selected_theme.replace(" (builtin)", "");

                // Switch to the selected theme
                Self::execute_switch(os, session, &theme_name).await
            } else {
                Ok(Self::default_chat_state())
            }
        } else {
            Ok(Self::default_chat_state())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_themes_subcommand_variants() {
        let list_cmd = ThemesSubcommand::List;
        assert_eq!(list_cmd, ThemesSubcommand::List);

        let switch_cmd = ThemesSubcommand::Switch {
            name: "dark".to_string(),
        };
        assert_eq!(switch_cmd, ThemesSubcommand::Switch {
            name: "dark".to_string()
        });

        let preview_cmd = ThemesSubcommand::Preview {
            name: "light".to_string(),
        };
        assert_eq!(preview_cmd, ThemesSubcommand::Preview {
            name: "light".to_string()
        });

        let current_cmd = ThemesSubcommand::Current;
        assert_eq!(current_cmd, ThemesSubcommand::Current);

        let select_cmd = ThemesSubcommand::Select;
        assert_eq!(select_cmd, ThemesSubcommand::Select);
    }

    #[test]
    fn test_theme_manager_creation() {
        let manager = ThemesSubcommand::get_theme_manager();
        // Just verify we can create a theme manager without panicking
        let _themes = manager.list_themes();
    }

    #[tokio::test]
    async fn test_theme_feature_enabled_check() {
        use crate::os::Os;

        let mut os = Os::new().await.expect("Failed to create OS instance");

        // Test with themes disabled (default)
        let enabled = ThemesSubcommand::is_feature_enabled(&os);
        assert!(!enabled, "Themes should be disabled by default");

        // Enable themes and test again
        let _ = os.database.settings.set(Setting::EnabledThemes, true).await;
        let enabled = ThemesSubcommand::is_feature_enabled(&os);
        assert!(enabled, "Themes should be enabled after setting");
    }

    /// Integration tests for theme switching functionality
    mod integration_tests {
        use super::*;
        use crate::os::Os;

        #[tokio::test]
        async fn test_theme_settings_persistence() {
            let mut os = Os::new().await.expect("Failed to create OS instance");

            // Enable themes experiment
            let result = os.database.settings.set(Setting::EnabledThemes, true).await;
            assert!(result.is_ok(), "Should be able to enable themes");

            // Set a current theme
            let result = os
                .database
                .settings
                .set(Setting::CurrentTheme, "dark".to_string())
                .await;
            assert!(result.is_ok(), "Should be able to set current theme");

            // Verify the settings persist
            let enabled = os.database.settings.get_bool(Setting::EnabledThemes).unwrap_or(false);
            assert!(enabled, "Themes should remain enabled");

            let current_theme = os
                .database
                .settings
                .get_string(Setting::CurrentTheme)
                .unwrap_or_default();
            assert_eq!(current_theme, "dark", "Current theme should be 'dark'");
        }

        #[tokio::test]
        async fn test_theme_manager_integration() {
            // Test theme manager creation and basic operations
            let manager = ThemesSubcommand::get_theme_manager();

            // List themes should not panic
            let themes = manager.list_themes();

            // Should have themes list (empty or populated, both are valid)
            // Just verify we can get the list without panicking

            // Test loading a theme (this might fail if no themes exist, which is ok)
            if !themes.is_empty() {
                let first_theme = &themes[0];
                let theme_name = first_theme.replace(" (builtin)", "");
                let result = manager.load_theme(&theme_name);
                // We don't assert success here as it depends on theme availability
                // Just verify it doesn't panic
                let _ = result;
            }
        }

        #[test]
        fn test_theme_preview_rendering() {
            let manager = ThemesSubcommand::get_theme_manager();

            // Test that powerline theme can be loaded and rendered
            let powerline_result = manager.load_theme("powerline");
            assert!(powerline_result.is_ok(), "Should be able to load powerline theme");

            // Test that minimal theme can be loaded and rendered
            let minimal_result = manager.load_theme("minimal");
            assert!(minimal_result.is_ok(), "Should be able to load minimal theme");

            // Test that git-enabled theme can be loaded and rendered
            let git_result = manager.load_theme("git-enabled");
            assert!(git_result.is_ok(), "Should be able to load git-enabled theme");
        }

        #[test]
        fn test_powerline_theme_content() {
            let manager = ThemesSubcommand::get_theme_manager();
            let template = manager.load_theme("powerline").expect("Should load powerline theme");

            // Verify powerline theme contains expected elements
            assert!(
                template.contains("${AGENT}"),
                "Powerline theme should contain AGENT variable"
            );
            assert!(
                template.contains("${TOKEN_USAGE}") || template.contains("$TOKEN_USAGE"),
                "Powerline theme should contain TOKEN_USAGE variable"
            );
            assert!(
                template.contains("${GIT_BRANCH"),
                "Powerline theme should contain GIT_BRANCH conditional"
            );
            assert!(
                template.contains("\u{e0b0}"),
                "Powerline theme should contain powerline separator"
            );
            assert!(
                template.contains("\x1b["),
                "Powerline theme should contain ANSI escape sequences"
            );
        }
    }
}
