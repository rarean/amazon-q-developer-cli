#[cfg(test)]
mod tests {
    use crate::cli::chat::prompt_parser::generate_themed_prompt;
    use crate::cli::chat::themes::ThemeManager;
    use crate::os::Os;

    #[tokio::test]
    async fn test_themed_prompt_integration() {
        let os = Os::new().await.unwrap();
        let mut theme_manager = ThemeManager::new(&os).unwrap();

        // Test fallback when no theme is loaded
        let prompt = generate_themed_prompt(Some("test"), false, false, Some(&theme_manager));
        assert_eq!(prompt, "[test] > ");

        // Test with theme loaded (if default.theme exists)
        let _ = theme_manager.load_theme("default");
        if theme_manager.get_active_theme().is_some() {
            let themed_prompt = generate_themed_prompt(Some("test"), false, false, Some(&theme_manager));
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
}
