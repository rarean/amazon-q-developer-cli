use std::collections::HashMap;
use std::path::PathBuf;

use crate::database::settings::Setting;
use crate::os::Os;
use crate::util::command_types::{
    CommandError,
    CustomCommand,
};

/// Manages custom commands for the Amazon Q CLI
pub struct CommandManager {
    /// Path to project commands directory (.amazonq/commands/)
    project_commands_dir: PathBuf,
    /// Cache of loaded commands
    cache: HashMap<String, CustomCommand>,
}

impl CommandManager {
    /// Create a new command manager
    pub fn new(os: &Os) -> Result<Self, CommandError> {
        let project_commands_dir = os
            .env
            .current_dir()
            .map_err(CommandError::Io)?
            .join(".amazonq")
            .join("commands");

        Ok(Self {
            project_commands_dir,
            cache: HashMap::new(),
        })
    }

    /// Check if the commands feature is enabled in settings
    pub fn is_enabled(os: &Os) -> bool {
        os.database.settings.get_bool(Setting::EnabledCommands).unwrap_or(false)
    }

    /// Add a new command
    pub fn add_command(&mut self, name: &str, _os: &Os) -> Result<String, CommandError> {
        // Validate command name
        CustomCommand::validate_name(name)?;

        // Create commands directory if it doesn't exist
        std::fs::create_dir_all(&self.project_commands_dir)?;

        let file_path = self.project_commands_dir.join(format!("{}.md", name));

        // Check if command already exists
        if file_path.exists() {
            return Err(CommandError::AlreadyExists(name.to_string()));
        }

        // Create template content
        let template = Self::create_command_template(name);

        // Write template to file
        std::fs::write(&file_path, template)?;

        // Open editor
        Self::open_editor(&file_path)?;

        // Load the command into cache
        let command = CustomCommand::from_file(file_path)?;
        self.cache.insert(name.to_string(), command);

        Ok(format!(
            "✅ Command '{}' created successfully!\n   Use '/project:{}' to execute it.\n\n💡 Tip: Use '/commands show {}' to see command details.",
            name, name, name
        ))
    }

    /// Get a command by name
    pub fn get_command(&mut self, name: &str) -> Result<&CustomCommand, CommandError> {
        // Check cache first
        if !self.cache.contains_key(name) {
            // Try to load from file
            let file_path = self.project_commands_dir.join(format!("{}.md", name));
            if file_path.exists() {
                let command = CustomCommand::from_file(file_path)?;
                self.cache.insert(name.to_string(), command);
            } else {
                return Err(CommandError::NotFound(name.to_string()));
            }
        }

        Ok(self.cache.get(name).unwrap())
    }

    /// Execute a command by name
    pub fn execute_command(&mut self, name: &str) -> Result<String, CommandError> {
        let command = self.get_command(name)?;
        Ok(command.content.clone())
    }

    /// List all available commands
    #[allow(dead_code)]
    pub fn list_commands(&mut self) -> Result<Vec<String>, CommandError> {
        let mut commands = Vec::new();

        if !self.project_commands_dir.exists() {
            return Ok(commands);
        }

        for entry in std::fs::read_dir(&self.project_commands_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    commands.push(name.to_string());
                }
            }
        }

        commands.sort();
        Ok(commands)
    }

    /// Create a command template
    fn create_command_template(name: &str) -> String {
        let display_name = name.replace(['-', '_'], " ");
        format!(
            "# {}\n\n\
            Brief description of what this command does.\n\n\
            ## Instructions\n\n\
            Provide detailed instructions for Amazon Q:\n\n\
            1. Step 1: What to analyze first\n\
            2. Step 2: What to look for\n\
            3. Step 3: How to format the response\n\n\
            ## Context\n\n\
            Any additional context or requirements for this command.\n\n\
            ## Examples\n\n\
            Provide examples of how this command should be used or what output is expected.\n",
            display_name
        )
    }

    /// Open editor for command file
    fn open_editor(file_path: &PathBuf) -> Result<(), CommandError> {
        // Get editor from environment, fallback to sensible defaults
        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| {
                // Platform-specific defaults
                if cfg!(target_os = "windows") {
                    "notepad".to_string()
                } else if cfg!(target_os = "macos") {
                    "open".to_string()
                } else {
                    "nano".to_string()
                }
            });

        let mut cmd = std::process::Command::new(&editor);

        // Special handling for macOS 'open' command
        if editor == "open" {
            cmd.arg("-t"); // Open in text editor
        }

        cmd.arg(file_path);

        let status = cmd
            .status()
            .map_err(|e| CommandError::EditorError(format!("Failed to start editor '{}': {}", editor, e)))?;

        if !status.success() {
            return Err(CommandError::EditorError(format!(
                "Editor '{}' exited with error",
                editor
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_command_name() {
        assert!(CustomCommand::validate_name("valid-name").is_ok());
        assert!(CustomCommand::validate_name("invalid name").is_err());
        assert!(CustomCommand::validate_name("").is_err());
        assert!(CustomCommand::validate_name("help").is_err());
    }
}
