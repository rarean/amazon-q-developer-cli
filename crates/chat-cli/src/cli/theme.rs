use clap::{
    Parser,
    Subcommand,
};
use eyre::Result;
use themes::{
    ThemeManager,
    ThemeRenderer,
};

use crate::os::Os;
use crate::util::directories::chat_themes_dir;

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
        let themes_dir = chat_themes_dir(os)?;
        let manager = ThemeManager::new(themes_dir);

        match self.command {
            ThemeSubcommand::List => {
                let themes = manager.list_themes();
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
            ThemeSubcommand::Validate { name } => match manager.validate_theme(&name) {
                Ok(_) => {
                    println!("Theme '{}' is valid", name);
                },
                Err(e) => {
                    eprintln!("Theme '{}' is invalid: {}", name, e);
                },
            },
            ThemeSubcommand::Preview { name } => match manager.load_theme(&name) {
                Ok(template) => {
                    let renderer = ThemeRenderer::new();
                    let rendered = renderer.render_prompt(&template);
                    println!("Preview of theme '{}':", name);
                    print!("{}", rendered);
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

                let rendered = manager.render_context_prompt(&target_path);
                println!("Context-aware prompt for {:?}:", target_path);
                print!("{}", rendered);
            },
        }

        Ok(())
    }
}
