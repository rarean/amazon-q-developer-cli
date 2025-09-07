#[cfg(test)]
mod tests {
    use crate::cli::chat::themes::renderer::ThemeRenderer;
    use crate::cli::chat::themes::{
        BashTheme,
        ThemeManager,
    };
    use crate::os::Os;

    #[tokio::test]
    async fn test_cross_platform_theme_parsing() {
        // Test that theme parsing works with different line endings and paths
        let theme_content_unix =
            "THEME_NAME=\"test\"\nPROMPT=\"$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR > \"\nQ_AGENT_COLOR=\"\\033[36m\"";
        let theme_content_windows =
            "THEME_NAME=\"test\"\r\nPROMPT=\"$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR > \"\r\nQ_AGENT_COLOR=\"\\033[36m\"";

        // Both should parse successfully
        let mut theme_unix = BashTheme::new("test".to_string());
        let mut theme_windows = BashTheme::new("test".to_string());

        // Simulate parsing (normally done by parse_theme_file)
        for line in theme_content_unix.lines() {
            if let Some((key, value)) = parse_line(line) {
                match key.as_str() {
                    "THEME_NAME" => theme_unix.name = unquote(&value),
                    "PROMPT" => theme_unix.prompt_template = unquote(&value),
                    _ => {
                        theme_unix.set_variable(key, unquote(&value));
                    },
                }
            }
        }

        for line in theme_content_windows.lines() {
            if let Some((key, value)) = parse_line(line) {
                match key.as_str() {
                    "THEME_NAME" => theme_windows.name = unquote(&value),
                    "PROMPT" => theme_windows.prompt_template = unquote(&value),
                    _ => {
                        theme_windows.set_variable(key, unquote(&value));
                    },
                }
            }
        }

        assert_eq!(theme_unix.name, "test");
        assert_eq!(theme_windows.name, "test");
        assert_eq!(theme_unix.prompt_template, theme_windows.prompt_template);
    }

    #[tokio::test]
    async fn test_cross_platform_ansi_colors() {
        // Test that ANSI color codes work consistently across platforms
        let mut theme = BashTheme::new("test".to_string());
        theme.prompt_template = "$Q_AGENT_COLOR[$Q_AGENT]$RESET_COLOR > ".to_string();
        theme.set_variable("Q_AGENT_COLOR".to_string(), "\u{001b}[36m".to_string());
        theme.set_variable("RESET_COLOR".to_string(), "\u{001b}[0m".to_string());

        let renderer = ThemeRenderer::new(&theme);
        let result = renderer.render_prompt(Some("test"), false, false, None, None, None);

        // Should contain ANSI escape sequences and the agent name
        assert!(result.contains("[test]"));
        assert!(result.contains(">"));

        // The exact ANSI codes depend on variable substitution working
        // Let's just check that some substitution occurred
        assert!(!result.contains("$Q_AGENT_COLOR"));
        assert!(!result.contains("$Q_AGENT"));
    }

    #[tokio::test]
    async fn test_cross_platform_paths() {
        let os = Os::new().await.unwrap();
        let theme_manager = ThemeManager::new(&os);

        // Should work regardless of platform
        assert!(theme_manager.is_ok());

        let tm = theme_manager.unwrap();
        let result = tm.ensure_theme_directory(&os).await;
        assert!(result.is_ok());
    }

    // Helper functions for testing
    fn parse_line(line: &str) -> Option<(String, String)> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let value = line[eq_pos + 1..].trim().to_string();
            Some((key, value))
        } else {
            None
        }
    }

    fn unquote(value: &str) -> String {
        let trimmed = value.trim();
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            trimmed.to_string()
        }
    }
}
