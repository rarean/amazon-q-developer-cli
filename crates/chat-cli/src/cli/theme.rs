use clap::{
    Parser,
    Subcommand,
};
use eyre::Result;

use crate::cli::chat::themes::renderer::ThemeRenderer;
use crate::cli::chat::themes::{
    GitInfo,
    ThemeManager,
};
use crate::os::Os;

#[derive(Debug, Parser, PartialEq)]
pub struct ThemeArgs {
    #[command(subcommand)]
    pub command: ThemeSubcommand,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum ThemeSubcommand {
    /// List available themes
    List,
    /// Switch to a theme
    Switch {
        /// Theme name to switch to
        name: String,
    },
    /// Validate a theme
    Validate {
        /// Theme name to validate
        name: String,
    },
    /// Preview a theme with current git status
    Preview {
        /// Theme name to preview
        name: String,
    },
    /// Show context-aware prompt for current directory
    Context {
        /// Optional path to check (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
}

impl ThemeArgs {
    pub async fn execute(self, os: &mut Os) -> Result<()> {
        let mut manager = ThemeManager::new(os)?;

        match self.command {
            ThemeSubcommand::List => {
                let themes = manager.list_available_themes(os).await?;
                println!("Available themes:");
                for theme in themes {
                    println!("  {}", theme);
                }
            },
            ThemeSubcommand::Switch { name } => {
                match manager.load_theme(&name) {
                    Ok(_) => {
                        println!("Switched to theme: {}", name);
                        // TODO: Save current theme to config
                    },
                    Err(e) => {
                        eprintln!("Error switching theme: {}", e);
                    },
                }
            },
            ThemeSubcommand::Validate { name } => {
                match manager.load_theme(&name) {
                    Ok(_) => {
                        if let Some(theme) = manager.get_active_theme() {
                            // Try to render a test prompt to validate
                            let renderer = ThemeRenderer::new(theme);
                            let git_info = detect_git_info();
                            let _rendered = renderer.render_prompt(Some("test"), false, false, Some(&git_info), None);
                            println!("Theme '{}' is valid", name);
                        } else {
                            eprintln!("Theme '{}' failed to load", name);
                        }
                    },
                    Err(e) => {
                        eprintln!("Theme '{}' is invalid: {}", name, e);
                    },
                }
            },
            ThemeSubcommand::Preview { name } => match manager.load_theme(&name) {
                Ok(_) => {
                    if let Some(theme) = manager.get_active_theme() {
                        let renderer = ThemeRenderer::new(theme);
                        let git_info = detect_git_info();
                        let rendered =
                            renderer.render_prompt(Some("q_cli_default"), false, false, Some(&git_info), Some(48.35));
                        println!("Preview of theme '{}':", name);
                        print!("{}", rendered);
                    } else {
                        eprintln!("Failed to load theme '{}'", name);
                    }
                },
                Err(e) => {
                    eprintln!("Error previewing theme: {}", e);
                },
            },
            ThemeSubcommand::Context { path } => {
                use std::env;
                use std::path::Path;

                let target_path = if let Some(p) = path {
                    Path::new(&p).to_path_buf()
                } else {
                    env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf())
                };

                // Try to load default theme for context rendering
                let _ = manager.load_default_theme();
                if let Some(theme) = manager.get_active_theme() {
                    let renderer = ThemeRenderer::new(theme);
                    let git_info = detect_git_info_for_path(&target_path);
                    let rendered =
                        renderer.render_prompt(Some("q_cli_default"), false, false, Some(&git_info), Some(48.35));
                    println!("Context-aware prompt for {:?}:", target_path);
                    print!("{}", rendered);
                } else {
                    println!("Context-aware prompt for {:?}:", target_path);
                    print!("> ");
                }
            },
        }

        Ok(())
    }
}

/// Detect git information for the current directory
fn detect_git_info() -> GitInfo {
    use std::env;

    let current_dir = env::current_dir().unwrap_or_else(|_| std::path::Path::new(".").to_path_buf());

    // Use our themes crate git detection
    let themes_git_info = themes::GitInfo::detect(&current_dir);

    // Convert to the chat themes GitInfo format
    GitInfo {
        branch: themes_git_info.branch,
        is_dirty: themes_git_info.status.is_some_and(|s| !s.clean),
        is_repo: themes_git_info.is_repo,
    }
}

/// Detect git information for a specific path
fn detect_git_info_for_path(path: &std::path::Path) -> GitInfo {
    // Use our themes crate git detection
    let themes_git_info = themes::GitInfo::detect(path);

    // Convert to the chat themes GitInfo format
    GitInfo {
        branch: themes_git_info.branch,
        is_dirty: themes_git_info.status.is_some_and(|s| !s.clean),
        is_repo: themes_git_info.is_repo,
    }
}
