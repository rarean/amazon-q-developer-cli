use std::collections::HashMap;
use std::path::PathBuf;

use crate::database::settings::Setting;
use crate::os::Os;
use crate::util::bash_preprocessor::BashPreprocessor;
use crate::util::command_types::{
    CommandError,
    CommandScope,
    CustomCommand,
};

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
mod integration_tests;

/// Manages custom commands for the Amazon Q CLI
pub struct CommandManager {
    /// Path to project commands directory (.amazonq/commands/)
    project_commands_dir: PathBuf,
    /// Path to user commands directory (~/.amazonq/commands/)
    user_commands_dir: PathBuf,
    /// Cache of loaded commands
    cache: HashMap<String, CustomCommand>,
    /// Bash command preprocessor
    bash_preprocessor: BashPreprocessor,
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

        let user_commands_dir = os
            .env
            .home()
            .ok_or_else(|| CommandError::Other("Could not determine home directory".to_string()))?
            .join(".amazonq")
            .join("commands");

        Ok(Self {
            project_commands_dir,
            user_commands_dir,
            cache: HashMap::new(),
            bash_preprocessor: BashPreprocessor::default(),
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

        // Open editor (skip in test mode for performance)
        if !cfg!(test) && std::env::var("EDITOR").unwrap_or_default() != "true" {
            Self::open_editor(&file_path)?;
        }

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

        // This should never fail since we just inserted the key above
        self.cache
            .get(name)
            .ok_or_else(|| CommandError::Other("Failed to retrieve cached command".to_string()))
    }

    /// Execute a command by name
    pub fn execute_command(&mut self, name: &str) -> Result<String, CommandError> {
        let command = self.get_command(name)?;
        Ok(command.content.clone())
    }

    /// Execute command with argument processing and file references
    pub fn execute_command_with_args(
        &mut self,
        name: &str,
        args: Option<&str>,
        os: &Os,
    ) -> Result<String, CommandError> {
        let command = self.get_command(name)?;
        let mut content = command.content.clone();
        let frontmatter = command.frontmatter.clone();

        // Process argument substitution
        if let Some(args) = args {
            content = content.replace("$ARGUMENTS", args);
        } else {
            content = content.replace("$ARGUMENTS", "");
        }

        // Process file references
        content = Self::process_file_references(content, os)?;

        // Process bash commands (NEW)
        content = self
            .bash_preprocessor
            .process_bash_commands(&content, Some(&frontmatter))?;

        // Basic security validation
        Self::validate_command_security(&content)?;

        Ok(content)
    }

    /// Execute user command with argument processing and file references
    pub fn execute_user_command_with_args(
        &mut self,
        name: &str,
        args: Option<&str>,
        os: &Os,
    ) -> Result<String, CommandError> {
        let command = self.get_user_command(name)?;
        let mut content = command.content.clone();
        let frontmatter = command.frontmatter.clone();

        // Process argument substitution
        if let Some(args) = args {
            content = content.replace("$ARGUMENTS", args);
        } else {
            content = content.replace("$ARGUMENTS", "");
        }

        // Process file references
        content = Self::process_file_references(content, os)?;

        // Process bash commands (NEW)
        content = self
            .bash_preprocessor
            .process_bash_commands(&content, Some(&frontmatter))?;

        // Basic security validation
        Self::validate_command_security(&content)?;

        Ok(content)
    }

    /// Get a user command by name
    fn get_user_command(&mut self, name: &str) -> Result<&CustomCommand, CommandError> {
        let cache_key = format!("user:{}", name);

        if !self.cache.contains_key(&cache_key) {
            let command = self.load_user_command(name)?;
            self.cache.insert(cache_key.clone(), command);
        }

        self.cache
            .get(&cache_key)
            .ok_or_else(|| CommandError::NotFound(name.to_string()))
    }

    /// Load a user command from the file system
    fn load_user_command(&self, name: &str) -> Result<CustomCommand, CommandError> {
        // Handle both namespace/name format and simple name format
        let file_path = self.user_commands_dir.join(format!("{}.md", name));

        if !file_path.exists() {
            return Err(CommandError::NotFound(name.to_string()));
        }

        CustomCommand::from_file(file_path)
    }

    /// Process file references in command content
    fn process_file_references(content: String, _os: &Os) -> Result<String, CommandError> {
        use regex::Regex;

        let file_ref_regex =
            Regex::new(r"@([^\s]+)").map_err(|e| CommandError::Other(format!("Regex error: {}", e)))?;
        let mut result = content.clone();

        for captures in file_ref_regex.captures_iter(&content) {
            let file_path = &captures[1];
            let full_match = &captures[0];

            // Try to read the file
            if let Ok(file_content) = std::fs::read_to_string(file_path) {
                result = result.replace(full_match, &file_content);
            }
            // Leave the reference as-is if file can't be read
            // This allows for graceful degradation
        }

        Ok(result)
    }

    /// Basic security validation for command content
    pub fn validate_command_security(content: &str) -> Result<(), CommandError> {
        // List of potentially dangerous patterns
        let dangerous_patterns = [
            "rm -rf",
            "sudo ",
            "chmod 777",
            "chmod +x",
            "../../../", // Path traversal
            "curl -s",   // Potential network access
            "wget ",
            "nc ", // netcat
        ];

        for pattern in &dangerous_patterns {
            if content.contains(pattern) {
                return Err(CommandError::SecurityViolation(format!(
                    "Command contains potentially dangerous pattern: '{}'",
                    pattern
                )));
            }
        }

        Ok(())
    }

    /// List all available commands with full details
    pub fn list_commands_detailed(&mut self, scope: Option<&CommandScope>) -> Result<Vec<CustomCommand>, CommandError> {
        let mut commands = Vec::new();

        // Load project commands if requested or no scope specified
        if (scope.is_none() || scope == Some(&CommandScope::Project)) && self.project_commands_dir.exists() {
            for entry in std::fs::read_dir(&self.project_commands_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Ok(mut command) = CustomCommand::from_file(path) {
                        command.scope = CommandScope::Project;
                        commands.push(command);
                    }
                }
            }
        }

        // Load user/global commands if requested or no scope specified
        if (scope.is_none() || scope == Some(&CommandScope::Global)) && self.user_commands_dir.exists() {
            for entry in std::fs::read_dir(&self.user_commands_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Ok(mut command) = CustomCommand::from_file(path) {
                        command.scope = CommandScope::Global;
                        commands.push(command);
                    }
                }
            }
        }

        commands.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(commands)
    }

    /// Clear the command cache
    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Test accessor methods for integration tests
    #[cfg(test)]
    pub fn get_project_commands_dir(&self) -> &std::path::PathBuf {
        &self.project_commands_dir
    }

    #[cfg(test)]
    pub fn get_cache(&self) -> &std::collections::HashMap<String, crate::util::command_types::CustomCommand> {
        &self.cache
    }

    #[cfg(test)]
    pub fn clear_cache_for_test(&mut self) {
        self.cache.clear();
    }

    /// Create a CommandManager for testing with custom directories
    #[cfg(test)]
    pub fn new_for_test(project_commands_dir: PathBuf, user_commands_dir: PathBuf) -> Self {
        Self {
            project_commands_dir,
            user_commands_dir,
            cache: HashMap::new(),
            bash_preprocessor: BashPreprocessor::default(),
        }
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
        let display_name = name
            .replace(['-', '_'], " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        let mut result = first.to_uppercase().collect::<String>();
                        result.push_str(chars.as_str());
                        result
                    },
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

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
    pub fn open_editor(file_path: &PathBuf) -> Result<(), CommandError> {
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

    // Phase 2 functionality tests
    #[test]
    fn test_argument_substitution() {
        let content = "Help with: $ARGUMENTS and more $ARGUMENTS";
        let args = "git commits";

        let processed = content.replace("$ARGUMENTS", args);
        assert_eq!(processed, "Help with: git commits and more git commits");

        // Test with no arguments
        let processed_empty = content.replace("$ARGUMENTS", "");
        assert_eq!(processed_empty, "Help with:  and more ");
    }

    #[test]
    fn test_security_validation() {
        let _manager = CommandManager {
            project_commands_dir: PathBuf::new(),
            user_commands_dir: PathBuf::new(),
            cache: HashMap::new(),
            bash_preprocessor: BashPreprocessor::default(),
        };

        // Test dangerous patterns
        let dangerous_patterns = [
            "rm -rf /tmp",
            "sudo apt install",
            "chmod 777 file",
            "chmod +x script",
            "../../../etc/passwd",
            "curl -s malicious.com",
            "wget http://bad.com",
            "nc -l 1234",
        ];

        for pattern in &dangerous_patterns {
            let content = format!("Execute: {}", pattern);
            assert!(
                CommandManager::validate_command_security(&content).is_err(),
                "Pattern '{}' should be detected as dangerous",
                pattern
            );
        }

        // Test safe content
        let safe_content = "Please help me with git commands and best practices";
        assert!(CommandManager::validate_command_security(safe_content).is_ok());
    }

    #[test]
    fn test_file_reference_regex() {
        use regex::Regex;

        let file_ref_regex = Regex::new(r"@([^\s]+)").unwrap();
        let content = "Read @file1.txt and @file2.md for context";

        let matches: Vec<&str> = file_ref_regex
            .captures_iter(content)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();

        assert_eq!(matches, vec!["file1.txt", "file2.md"]);

        // Test single file reference
        let single_content = "Analyze @README.md";
        let single_matches: Vec<&str> = file_ref_regex
            .captures_iter(single_content)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();

        assert_eq!(single_matches, vec!["README.md"]);
    }

    #[test]
    fn test_command_name_parsing() {
        // Test namespace parsing for user commands
        let name_with_namespace = "frontend/component";
        assert!(name_with_namespace.contains('/'));

        let simple_name = "git-helper";
        assert!(!simple_name.contains('/'));

        // Test cache key generation
        let cache_key = format!("user:{}", name_with_namespace);
        assert_eq!(cache_key, "user:frontend/component");
    }

    #[test]
    fn test_user_command_cache_key() {
        let name = "test-command";
        let cache_key = format!("user:{}", name);
        assert_eq!(cache_key, "user:test-command");

        let namespaced_name = "frontend/component";
        let namespaced_cache_key = format!("user:{}", namespaced_name);
        assert_eq!(namespaced_cache_key, "user:frontend/component");
    }

    #[tokio::test]
    async fn test_add_command_creates_file() {
        use tempfile::TempDir;

        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let commands_dir = temp_dir.path().join(".amazonq").join("commands");

        // Create Os instance for testing
        let os = crate::os::Os::new().await.unwrap();

        // Create command manager with temp directory
        let mut manager = CommandManager {
            project_commands_dir: commands_dir.clone(),
            user_commands_dir: temp_dir.path().join(".amazonq").join("user_commands"),
            cache: HashMap::new(),
            bash_preprocessor: BashPreprocessor::default(),
        };

        // Test adding a command
        let command_name = "test-command";

        // Mock the editor opening by setting a dummy editor that does nothing
        unsafe {
            std::env::set_var("EDITOR", "true"); // 'true' command does nothing and exits successfully
        }

        let result = manager.add_command(command_name, &os);

        // Restore original editor
        unsafe {
            std::env::remove_var("EDITOR");
        }

        // Check that the operation succeeded
        assert!(result.is_ok(), "add_command should succeed: {:?}", result);

        // Check that the file was created
        let expected_file_path = commands_dir.join(format!("{}.md", command_name));
        assert!(expected_file_path.exists(), "Command file should be created");

        // Check that the file has the expected template content
        let file_content = std::fs::read_to_string(&expected_file_path).expect("Should be able to read created file");

        // Verify the template structure
        assert!(
            file_content.contains("# Test Command"),
            "Should contain formatted title"
        );
        assert!(
            file_content.contains("## Instructions"),
            "Should contain Instructions section"
        );
        assert!(file_content.contains("## Context"), "Should contain Context section");
        assert!(file_content.contains("## Examples"), "Should contain Examples section");
        assert!(
            file_content.contains("Brief description of what this command does"),
            "Should contain description placeholder"
        );

        // Check that the command was loaded into cache
        assert!(manager.cache.contains_key(command_name), "Command should be cached");

        // Check that the success message is correct
        let success_message = result.unwrap();
        assert!(success_message.contains("✅ Command 'test-command' created successfully!"));
        assert!(success_message.contains("Use '/project:test-command' to execute it"));
    }

    #[tokio::test]
    async fn test_add_command_duplicate_error() {
        use tempfile::TempDir;

        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let commands_dir = temp_dir.path().join(".amazonq").join("commands");
        std::fs::create_dir_all(&commands_dir).expect("Failed to create commands dir");

        // Create Os instance for testing
        let os = crate::os::Os::new().await.unwrap();

        // Create command manager with temp directory
        let mut manager = CommandManager {
            project_commands_dir: commands_dir.clone(),
            user_commands_dir: temp_dir.path().join(".amazonq").join("user_commands"),
            cache: HashMap::new(),
            bash_preprocessor: BashPreprocessor::default(),
        };

        let command_name = "duplicate-command";
        let file_path = commands_dir.join(format!("{}.md", command_name));

        // Create the file first
        std::fs::write(&file_path, "existing content").expect("Failed to create existing file");

        // Try to add the same command
        let result = manager.add_command(command_name, &os);

        // Should return an error
        assert!(result.is_err(), "add_command should fail for duplicate command");

        match result.unwrap_err() {
            CommandError::AlreadyExists(name) => {
                assert_eq!(name, command_name, "Error should contain the command name");
            },
            other => panic!("Expected AlreadyExists error, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_add_command_invalid_name() {
        use tempfile::TempDir;

        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let commands_dir = temp_dir.path().join(".amazonq").join("commands");

        // Create Os instance for testing
        let os = crate::os::Os::new().await.unwrap();

        // Create command manager with temp directory
        let mut manager = CommandManager {
            project_commands_dir: commands_dir,
            user_commands_dir: temp_dir.path().join(".amazonq").join("user_commands"),
            cache: HashMap::new(),
            bash_preprocessor: BashPreprocessor::default(),
        };

        // Test with invalid command names
        let invalid_names = ["invalid name", "help", "", "command/with/slash"];

        for invalid_name in &invalid_names {
            let result = manager.add_command(invalid_name, &os);
            assert!(
                result.is_err(),
                "add_command should fail for invalid name: '{}'",
                invalid_name
            );
        }
    }
}
