#[cfg(test)]
mod tests {
    use crate::cli::chat::prompt_parser::generate_themed_prompt;
    use crate::cli::chat::themes::renderer::ThemeRenderer;
    use crate::cli::chat::themes::{
        GitInfo,
        ThemeManager,
    };
    use crate::os::Os;

    #[tokio::test]
    async fn test_themed_prompt_integration() {
        let os = Os::new().await.unwrap();
        let mut theme_manager = ThemeManager::new(&os).unwrap();

        // Test fallback when no theme is loaded
        let prompt = generate_themed_prompt(Some("test"), false, false, Some(&theme_manager), None);
        assert_eq!(prompt, "[test] > ");

        // Test with theme loaded (if default.theme exists)
        let _ = theme_manager.load_theme("default");
        if theme_manager.get_active_theme().is_some() {
            let themed_prompt = generate_themed_prompt(Some("test"), false, false, Some(&theme_manager), None);
            // Should contain ANSI color codes if theme is loaded
            assert!(themed_prompt.contains("\u{001b}[36m") || themed_prompt == "[test] > ");
        }
    }

    #[tokio::test]
    async fn test_theme_directory_creation() {
        let os = Os::new().await.unwrap();
        let theme_manager = ThemeManager::new(&os).unwrap();

        // Should not fail even if directory doesn't exist
        let result = theme_manager.ensure_theme_directory(&os).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_theme_switching_changes_prompt() {
        let os = Os::new().await.expect("Failed to create OS instance");
        let mut manager = ThemeManager::new(&os).expect("Failed to create theme manager");

        // Load minimal theme first
        manager.load_theme("minimal").expect("Should load minimal theme");
        let minimal_theme = manager.get_active_theme().expect("Should have active theme");
        let minimal_renderer = ThemeRenderer::new(minimal_theme);
        let minimal_prompt = minimal_renderer.render_prompt(None, false, false, None, None);

        // Load powerline theme
        manager.load_theme("powerline").expect("Should load powerline theme");
        let powerline_theme = manager.get_active_theme().expect("Should have active theme");
        let powerline_renderer = ThemeRenderer::new(powerline_theme);
        let powerline_prompt = powerline_renderer.render_prompt(
            Some("default"),
            false,
            false,
            Some(&GitInfo {
                branch: Some("main".to_string()),
                is_dirty: false,
                is_repo: true,
            }),
            Some(50.0),
        );

        // Verify the prompts are different
        assert_ne!(
            minimal_prompt, powerline_prompt,
            "Different themes should produce different prompts"
        );

        // Verify minimal theme characteristics
        assert_eq!(minimal_prompt, "> ", "Minimal theme should be simple");

        // Verify powerline theme characteristics
        assert!(powerline_prompt.contains("default"), "Powerline should contain agent");
        assert!(powerline_prompt.contains("50"), "Powerline should contain usage");
        assert!(powerline_prompt.contains("main"), "Powerline should contain git branch");
        assert!(
            powerline_prompt.contains("\u{e0b0}"),
            "Powerline should contain separator"
        );
    }

    #[tokio::test]
    async fn test_builtin_themes_produce_different_prompts() {
        let os = Os::new().await.expect("Failed to create OS instance");
        let mut manager = ThemeManager::new(&os).expect("Failed to create theme manager");

        let git_info = GitInfo {
            branch: Some("feature".to_string()),
            is_dirty: false,
            is_repo: true,
        };

        // Test minimal theme
        manager.load_theme("minimal").expect("Should load minimal theme");
        let minimal_prompt = manager
            .get_active_theme()
            .map(|theme| {
                ThemeRenderer::new(theme).render_prompt(Some("test"), false, false, Some(&git_info), Some(25.0))
            })
            .unwrap_or_default();

        // Test powerline theme
        manager.load_theme("powerline").expect("Should load powerline theme");
        let powerline_prompt = manager
            .get_active_theme()
            .map(|theme| {
                ThemeRenderer::new(theme).render_prompt(Some("test"), false, false, Some(&git_info), Some(25.0))
            })
            .unwrap_or_default();

        // Test git-enabled theme
        manager
            .load_theme("git-enabled")
            .expect("Should load git-enabled theme");
        let git_prompt = manager
            .get_active_theme()
            .map(|theme| {
                ThemeRenderer::new(theme).render_prompt(Some("test"), false, false, Some(&git_info), Some(25.0))
            })
            .unwrap_or_default();

        // All themes should produce different prompts
        assert_ne!(minimal_prompt, powerline_prompt, "Minimal and powerline should differ");
        assert_ne!(minimal_prompt, git_prompt, "Minimal and git-enabled should differ");
        assert_ne!(powerline_prompt, git_prompt, "Powerline and git-enabled should differ");

        // Verify each theme has expected characteristics
        assert_eq!(minimal_prompt, "> ", "Minimal should be simple");
        assert!(
            powerline_prompt.contains("\u{e0b0}"),
            "Powerline should have separators"
        );
        assert!(git_prompt.contains("➜"), "Git-enabled should have arrow");
    }

    #[tokio::test]
    async fn test_git_enabled_theme_variable_substitution() {
        let os = Os::new().await.expect("Failed to create OS instance");
        let mut manager = ThemeManager::new(&os).expect("Failed to create theme manager");

        // Load git-enabled theme
        manager
            .load_theme("git-enabled")
            .expect("Should load git-enabled theme");
        let theme = manager.get_active_theme().expect("Should have active theme");
        let renderer = ThemeRenderer::new(theme);

        let git_info = GitInfo {
            branch: Some("develop".to_string()),
            is_dirty: false,
            is_repo: true,
        };

        let prompt = renderer.render_prompt(Some("default"), false, false, Some(&git_info), None);

        println!("Git-enabled theme prompt: {:?}", prompt);
        println!("Git-enabled theme prompt (display): {}", prompt);

        // Verify variables are properly substituted, not displayed as literals
        assert!(!prompt.contains("${GREEN}"), "Should not contain literal ${{GREEN}}");
        assert!(!prompt.contains("${RESET}"), "Should not contain literal ${{RESET}}");
        assert!(!prompt.contains("${BLUE}"), "Should not contain literal ${{BLUE}}");
        assert!(!prompt.contains("${YELLOW}"), "Should not contain literal ${{YELLOW}}");

        // Verify expected content is present
        assert!(prompt.contains("➜"), "Should contain arrow symbol");

        // Check for git branch info (accounting for color codes)
        assert!(
            prompt.contains("git:(") && prompt.contains("develop") && prompt.contains(")"),
            "Should contain git branch info with develop branch"
        );

        assert!(prompt.ends_with("> "), "Should end with prompt symbol");

        // Verify ANSI color codes are present (variables were substituted)
        assert!(prompt.contains("\x1b["), "Should contain ANSI escape sequences");
    }
}
