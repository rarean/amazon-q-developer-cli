# Custom Commands MVP - Implementation Guide

## Overview

This guide provides step-by-step implementation instructions for the Custom Commands MVP, focusing on the two core features:
1. **`/commands add <name>`** - Create a new custom command
2. **`/project:<name>`** - Execute a project-scoped custom command

## MVP Scope Definition

### ✅ Included in MVP
- Project-scoped commands only (`.amazonq/commands/`)
- Basic command creation with editor integration
- Simple command execution through chat interface
- Settings integration (`chat.enableCommands`)
- Basic error handling and validation
- Consistent UX with knowledge base patterns

### ❌ Excluded from MVP
- User-scoped commands (`~/.amazonq/commands/`)
- YAML frontmatter and metadata
- Bash execution (`!` prefix)
- File references (`@` prefix)
- Argument substitution (`$ARGUMENTS`)
- Advanced validation and security features
- Background operations and progress tracking

## Implementation Tasks

### Phase 1: Foundation (Days 1-2)

#### Task 1.1: Settings Integration
**File**: `crates/chat-cli/src/database/settings.rs`

```rust
// Add to existing Setting enum
pub enum Setting {
    // ... existing settings
    EnabledKnowledge,
    EnabledCommands,    // NEW: Add this line
}

impl Setting {
    pub fn key(&self) -> &'static str {
        match self {
            // ... existing mappings
            Self::EnabledKnowledge => "chat.enableKnowledge",
            Self::EnabledCommands => "chat.enableCommands",  // NEW: Add this line
        }
    }
}

impl FromStr for Setting {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // ... existing mappings
            "chat.enableKnowledge" => Ok(Self::EnabledKnowledge),
            "chat.enableCommands" => Ok(Self::EnabledCommands),  // NEW: Add this line
            _ => Err(SettingError::UnknownSetting(s.to_string())),
        }
    }
}
```

#### Task 1.2: Data Structures
**File**: `crates/chat-cli/src/util/command_types.rs` (NEW FILE)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CustomCommand {
    pub name: String,
    pub content: String,
    pub file_path: PathBuf,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Command '{0}' not found")]
    NotFound(String),
    
    #[error("Command '{0}' already exists")]
    AlreadyExists(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid command name: {0}")]
    InvalidName(String),
    
    #[error("Invalid command format: {0}")]
    InvalidFormat(String),
    
    #[error("Feature disabled. Enable with: q settings chat.enableCommands true")]
    FeatureDisabled,
}

impl CustomCommand {
    pub fn new(name: String, content: String, file_path: PathBuf) -> Self {
        Self {
            name,
            content,
            file_path,
            created_at: Utc::now(),
        }
    }
    
    pub fn from_file(file_path: PathBuf) -> Result<Self, CommandError> {
        let content = std::fs::read_to_string(&file_path)?;
        let name = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| CommandError::InvalidFormat("Invalid filename".to_string()))?
            .to_string();
            
        Ok(Self::new(name, content, file_path))
    }
}
```

#### Task 1.3: Command Manager Core
**File**: `crates/chat-cli/src/util/command_manager.rs` (NEW FILE)

```rust
use std::collections::HashMap;
use std::path::PathBuf;
use crate::database::settings::Setting;
use crate::os::Os;
use crate::util::command_types::{CustomCommand, CommandError};

pub struct CommandManager {
    project_commands_dir: PathBuf,
    cache: HashMap<String, CustomCommand>,
}

impl CommandManager {
    pub fn new(os: &Os) -> Result<Self, CommandError> {
        let project_commands_dir = os.current_dir().join(".amazonq").join("commands");
        
        Ok(Self {
            project_commands_dir,
            cache: HashMap::new(),
        })
    }
    
    pub fn is_enabled(os: &Os) -> bool {
        os.database
            .settings
            .get_bool(Setting::EnabledCommands)
            .unwrap_or(false)
    }
    
    pub fn add_command(&mut self, name: &str, os: &Os) -> Result<String, CommandError> {
        // Validate command name
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            return Err(CommandError::InvalidName(name.to_string()));
        }
        
        // Create commands directory if it doesn't exist
        std::fs::create_dir_all(&self.project_commands_dir)?;
        
        let file_path = self.project_commands_dir.join(format!("{}.md", name));
        
        // Check if command already exists
        if file_path.exists() {
            return Err(CommandError::AlreadyExists(name.to_string()));
        }
        
        // Create template content
        let template = self.create_command_template(name);
        
        // Write template to file
        std::fs::write(&file_path, template)?;
        
        // Open editor (simplified for MVP)
        self.open_editor(&file_path, os)?;
        
        // Load the command into cache
        let command = CustomCommand::from_file(file_path)?;
        self.cache.insert(name.to_string(), command);
        
        Ok(format!(
            "✅ Command '{}' created successfully!\n   Use '/project:{}' to execute it.",
            name, name
        ))
    }
    
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
    
    pub fn execute_command(&mut self, name: &str) -> Result<String, CommandError> {
        let command = self.get_command(name)?;
        Ok(command.content.clone())
    }
    
    fn create_command_template(&self, name: &str) -> String {
        format!(
            "# {}\n\n\
            Brief description of what this command does.\n\n\
            ## Instructions\n\n\
            Provide detailed instructions for Amazon Q:\n\n\
            1. Step 1: What to analyze first\n\
            2. Step 2: What to look for\n\
            3. Step 3: How to format the response\n\n\
            ## Context\n\n\
            Any additional context or requirements for this command.\n",
            name.replace('-', " ").replace('_', " ")
        )
    }
    
    fn open_editor(&self, file_path: &PathBuf, os: &Os) -> Result<(), CommandError> {
        // Simplified editor opening for MVP
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
        
        let status = std::process::Command::new(editor)
            .arg(file_path)
            .status()?;
            
        if !status.success() {
            return Err(CommandError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Editor exited with error"
            )));
        }
        
        Ok(())
    }
}
```

### Phase 2: CLI Integration (Days 3-4)

#### Task 2.1: Commands CLI Subcommand
**File**: `crates/chat-cli/src/cli/chat/cli/commands.rs` (NEW FILE)

```rust
use clap::Subcommand;
use eyre::Result;
use std::io::Write;
use crossterm::{queue, style::{self, Color}};

use crate::cli::chat::{ChatError, ChatSession, ChatState};
use crate::database::settings::Setting;
use crate::os::Os;
use crate::util::command_manager::CommandManager;

/// Custom commands management
#[derive(Clone, Debug, PartialEq, Eq, Subcommand)]
pub enum CommandsSubcommand {
    /// Add a new custom command
    Add { name: String },
}

impl CommandsSubcommand {
    pub async fn execute(self, os: &Os, session: &mut ChatSession) -> Result<ChatState, ChatError> {
        if !CommandManager::is_enabled(os) {
            Self::write_feature_disabled_message(session)?;
            return Ok(Self::default_chat_state());
        }

        let result = self.execute_operation(os, session).await;
        Self::write_operation_result(session, result)?;
        Ok(Self::default_chat_state())
    }

    fn is_feature_enabled(os: &Os) -> bool {
        CommandManager::is_enabled(os)
    }

    fn write_feature_disabled_message(session: &mut ChatSession) -> Result<(), std::io::Error> {
        queue!(
            session.stderr,
            style::SetForegroundColor(Color::Red),
            style::Print("\n❌ Commands tool is disabled. Enable it with: q settings chat.enableCommands true\n\n"),
            style::SetForegroundColor(Color::Reset)
        )
    }

    fn default_chat_state() -> ChatState {
        ChatState::PromptUser {
            skip_printing_tools: true,
        }
    }

    async fn execute_operation(&self, os: &Os, _session: &mut ChatSession) -> OperationResult {
        match self {
            CommandsSubcommand::Add { name } => Self::handle_add(os, name).await,
        }
    }

    async fn handle_add(os: &Os, name: &str) -> OperationResult {
        let mut manager = match CommandManager::new(os) {
            Ok(manager) => manager,
            Err(e) => return OperationResult::Error(format!("Failed to initialize command manager: {}", e)),
        };

        match manager.add_command(name, os) {
            Ok(message) => OperationResult::Success(message),
            Err(e) => OperationResult::Error(format!("Failed to add command: {}", e)),
        }
    }

    fn write_operation_result(session: &mut ChatSession, result: OperationResult) -> Result<(), std::io::Error> {
        match result {
            OperationResult::Success(msg) => {
                queue!(
                    session.stderr,
                    style::SetForegroundColor(Color::Green),
                    style::Print(format!("\n{}\n\n", msg)),
                    style::SetForegroundColor(Color::Reset)
                )
            },
            OperationResult::Error(msg) => {
                queue!(
                    session.stderr,
                    style::SetForegroundColor(Color::Red),
                    style::Print(format!("\n❌ Error: {}\n\n", msg)),
                    style::SetForegroundColor(Color::Reset)
                )
            },
        }
    }
}

#[derive(Debug)]
enum OperationResult {
    Success(String),
    Error(String),
}
```

#### Task 2.2: Commands Tool Implementation
**File**: `crates/chat-cli/src/cli/chat/tools/commands.rs` (NEW FILE)

```rust
use std::io::Write;
use crossterm::{queue, style::{self, Color}};
use eyre::Result;
use serde::Deserialize;

use super::{InvokeOutput, OutputKind};
use crate::database::settings::Setting;
use crate::os::Os;
use crate::util::command_manager::CommandManager;

/// The Commands tool allows executing custom user-defined commands.
/// This feature can be enabled/disabled via settings:
/// `q settings chat.enableCommands true`
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "command", rename_all = "lowercase")]
pub enum Commands {
    Execute(CommandExecute),
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandExecute {
    pub name: String,
    #[serde(default)]
    pub args: Option<String>,
}

impl Commands {
    /// Checks if the commands feature is enabled in settings
    pub fn is_enabled(os: &Os) -> bool {
        os.database
            .settings
            .get_bool(Setting::EnabledCommands)
            .unwrap_or(false)
    }

    pub async fn validate(&mut self, os: &Os) -> Result<()> {
        match self {
            Commands::Execute(execute) => {
                if execute.name.is_empty() {
                    eyre::bail!("Command name cannot be empty");
                }
                
                // Validate command name format
                if execute.name.contains('/') || execute.name.contains('\\') {
                    eyre::bail!("Invalid command name: {}", execute.name);
                }
                
                Ok(())
            }
        }
    }

    pub async fn queue_description(&self, _os: &Os, updates: &mut impl Write) -> Result<()> {
        match self {
            Commands::Execute(execute) => {
                queue!(
                    updates,
                    style::Print("Executing custom command: "),
                    style::SetForegroundColor(Color::Green),
                    style::Print(&execute.name),
                    style::ResetColor,
                )?;
                
                if let Some(args) = &execute.args {
                    queue!(
                        updates,
                        style::Print(" with arguments: "),
                        style::SetForegroundColor(Color::Blue),
                        style::Print(args),
                        style::ResetColor,
                    )?;
                }
            }
        }
        Ok(())
    }

    pub async fn invoke(&self, os: &Os, _updates: &mut impl Write) -> Result<InvokeOutput> {
        let mut manager = CommandManager::new(os)
            .map_err(|e| eyre::eyre!("Failed to initialize command manager: {}", e))?;

        let result = match self {
            Commands::Execute(execute) => {
                match manager.execute_command(&execute.name) {
                    Ok(content) => {
                        // For MVP, we just return the command content
                        // In full implementation, this would be processed further
                        format!("Executing command '{}':\n\n{}", execute.name, content)
                    },
                    Err(e) => format!("Failed to execute command '{}': {}", execute.name, e),
                }
            }
        };

        Ok(InvokeOutput {
            output: OutputKind::Text(result),
        })
    }
}
```

### Phase 3: Integration (Days 5-6)

#### Task 3.1: Tool Registration
**File**: `crates/chat-cli/src/cli/chat/tools/mod.rs`

```rust
// Add to existing imports
pub mod commands;  // NEW: Add this line

// Add to existing imports
use commands::Commands;  // NEW: Add this line

// Add to TOOL_NAMES array
pub const TOOL_NAMES: &[&str] = &[
    "execute_bash",
    "fs_read",
    "fs_write",
    "use_aws",
    "gh_issue",
    "knowledge",
    "commands",  // NEW: Add this line
    "thinking",
];

// Add to Tool enum
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum Tool {
    ExecuteBash(ExecuteBash),
    FsRead(FsRead),
    FsWrite(FsWrite),
    UseAws(UseAws),
    Custom(CustomTool),
    GhIssue(GhIssue),
    Knowledge(Knowledge),
    Commands(Commands),  // NEW: Add this line
    Thinking(Thinking),
}

// Add to name() method
impl Tool {
    pub fn name(&self) -> &str {
        match self {
            Tool::ExecuteBash(_) => "execute_bash",
            Tool::FsRead(_) => "fs_read",
            Tool::FsWrite(_) => "fs_write",
            Tool::UseAws(_) => "use_aws",
            Tool::Custom(custom_tool) => &custom_tool.name,
            Tool::GhIssue(_) => "gh_issue",
            Tool::Knowledge(_) => "knowledge",
            Tool::Commands(_) => "commands",  // NEW: Add this line
            Tool::Thinking(_) => "thinking (prerelease)",
        }
    }
}

// Add to permission_eval() method
impl Tool {
    pub fn permission_eval(&self) -> PermissionEvalResult {
        match self {
            Tool::ExecuteBash(_) => PermissionEvalResult::Ask,
            Tool::FsRead(_) => PermissionEvalResult::Allow,
            Tool::FsWrite(_) => PermissionEvalResult::Ask,
            Tool::UseAws(_) => PermissionEvalResult::Ask,
            Tool::Custom(_) => PermissionEvalResult::Ask,
            Tool::GhIssue(_) => PermissionEvalResult::Allow,
            Tool::Thinking(_) => PermissionEvalResult::Allow,
            Tool::Knowledge(_) => PermissionEvalResult::Ask,
            Tool::Commands(_) => PermissionEvalResult::Ask,  // NEW: Add this line
        }
    }
}

// Add to invoke() method
impl Tool {
    pub async fn invoke(&self, os: &Os, stdout: &mut impl Write) -> Result<InvokeOutput> {
        match self {
            Tool::ExecuteBash(execute_bash) => execute_bash.invoke(os, stdout).await,
            Tool::FsRead(fs_read) => fs_read.invoke(os, stdout).await,
            Tool::FsWrite(fs_write) => fs_write.invoke(os, stdout).await,
            Tool::UseAws(use_aws) => use_aws.invoke(os, stdout).await,
            Tool::Custom(custom_tool) => custom_tool.invoke(os, stdout).await,
            Tool::GhIssue(gh_issue) => gh_issue.invoke(os, stdout).await,
            Tool::Knowledge(knowledge) => knowledge.invoke(os, stdout).await,
            Tool::Commands(commands) => commands.invoke(os, stdout).await,  // NEW: Add this line
            Tool::Thinking(think) => think.invoke(stdout).await,
        }
    }
}

// Add to queue_description() method
impl Tool {
    pub async fn queue_description(&self, os: &Os, output: &mut impl Write) -> Result<()> {
        match self {
            Tool::ExecuteBash(execute_bash) => execute_bash.queue_description(output),
            Tool::FsRead(fs_read) => fs_read.queue_description(output),
            Tool::FsWrite(fs_write) => fs_write.queue_description(output),
            Tool::UseAws(use_aws) => use_aws.queue_description(output),
            Tool::Custom(custom_tool) => custom_tool.queue_description(output),
            Tool::GhIssue(gh_issue) => gh_issue.queue_description(output),
            Tool::Knowledge(knowledge) => knowledge.queue_description(os, output).await,
            Tool::Commands(commands) => commands.queue_description(os, output).await,  // NEW: Add this line
            Tool::Thinking(thinking) => thinking.queue_description(output),
        }
    }
}

// Add to validate() method
impl Tool {
    pub async fn validate(&mut self, os: &Os) -> Result<()> {
        match self {
            Tool::ExecuteBash(execute_bash) => execute_bash.validate(os).await,
            Tool::FsRead(fs_read) => fs_read.validate(os).await,
            Tool::FsWrite(fs_write) => fs_write.validate(os).await,
            Tool::UseAws(use_aws) => use_aws.validate(os).await,
            Tool::Custom(custom_tool) => custom_tool.validate(os).await,
            Tool::GhIssue(gh_issue) => gh_issue.validate(os).await,
            Tool::Knowledge(knowledge) => knowledge.validate(os).await,
            Tool::Commands(commands) => commands.validate(os).await,  // NEW: Add this line
            Tool::Thinking(think) => think.validate(os).await,
        }
    }
}
```

#### Task 3.2: Tool Manager Integration
**File**: `crates/chat-cli/src/cli/chat/tool_manager.rs`

```rust
// Find the existing tool filtering section and add:
if !crate::cli::chat::tools::commands::Commands::is_enabled(os) {
    tool_specs.remove("commands");  // NEW: Add this line after the knowledge check
}
```

#### Task 3.3: CLI Module Registration
**File**: `crates/chat-cli/src/cli/chat/cli/mod.rs`

```rust
// Add to existing modules
pub mod commands;  // NEW: Add this line

// Add to existing imports
use commands::CommandsSubcommand;  // NEW: Add this line

// Add to ChatSubcommand enum
#[derive(Clone, Debug, PartialEq, Eq, Subcommand)]
pub enum ChatSubcommand {
    // ... existing subcommands
    Knowledge(KnowledgeSubcommand),
    Commands(CommandsSubcommand),  // NEW: Add this line
}

// Add to execute() method
impl ChatSubcommand {
    pub async fn execute(self, os: &Os, session: &mut ChatSession) -> Result<ChatState, ChatError> {
        match self {
            // ... existing matches
            ChatSubcommand::Knowledge(knowledge) => knowledge.execute(os, session).await,
            ChatSubcommand::Commands(commands) => commands.execute(os, session).await,  // NEW: Add this line
        }
    }
}
```

#### Task 3.4: Module Registration
**File**: `crates/chat-cli/src/util/mod.rs`

```rust
// Add to existing modules
pub mod command_manager;  // NEW: Add this line
pub mod command_types;    // NEW: Add this line
```

### Phase 4: Testing and Documentation (Days 7-8)

#### Task 4.1: Unit Tests
**File**: `crates/chat-cli/src/util/command_manager.rs` (add to existing file)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::path::PathBuf;

    fn create_test_os(temp_dir: &TempDir) -> Os {
        // Create a mock Os instance for testing
        // This would need to be implemented based on the existing Os structure
        todo!("Implement test Os creation")
    }

    #[test]
    fn test_command_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let os = create_test_os(&temp_dir);
        
        let manager = CommandManager::new(&os);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_add_command() {
        let temp_dir = TempDir::new().unwrap();
        let os = create_test_os(&temp_dir);
        let mut manager = CommandManager::new(&os).unwrap();
        
        let result = manager.add_command("test-command", &os);
        assert!(result.is_ok());
        
        // Verify file was created
        let expected_path = temp_dir.path().join(".amazonq").join("commands").join("test-command.md");
        assert!(expected_path.exists());
    }

    #[test]
    fn test_invalid_command_name() {
        let temp_dir = TempDir::new().unwrap();
        let os = create_test_os(&temp_dir);
        let mut manager = CommandManager::new(&os).unwrap();
        
        let result = manager.add_command("invalid/name", &os);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommandError::InvalidName(_)));
    }
}
```

#### Task 4.2: Integration Tests
**File**: `crates/chat-cli/tests/commands_integration_test.rs` (NEW FILE)

```rust
use chat_cli::cli::chat::cli::commands::CommandsSubcommand;
use chat_cli::database::settings::Setting;
use tempfile::TempDir;

#[tokio::test]
async fn test_commands_feature_disabled() {
    // Test that commands are properly disabled when setting is false
    todo!("Implement integration test for disabled feature");
}

#[tokio::test]
async fn test_commands_add_and_execute() {
    // Test full workflow: enable feature, add command, execute command
    todo!("Implement integration test for add and execute workflow");
}
```

#### Task 4.3: Documentation
**File**: `crates/chat-cli/README.md` (update existing file)

```markdown
## Custom Commands (MVP)

The Custom Commands feature allows you to create reusable command templates for Amazon Q.

### Enable the Feature

```bash
q settings chat.enableCommands true
```

### Create a Command

```bash
/commands add my-command
```

This will:
1. Create `.amazonq/commands/my-command.md`
2. Open your default editor to define the command
3. Save the command for future use

### Execute a Command

```bash
/project:my-command
```

This will execute the custom command within the current chat session.

### MVP Limitations

- Only project-scoped commands (`.amazonq/commands/`)
- No YAML frontmatter or metadata
- No bash execution or file references
- No argument substitution
```

## Testing Strategy

### Manual Testing Checklist

1. **Feature Enablement**
   - [ ] Feature disabled by default
   - [ ] Proper error message when disabled
   - [ ] Feature enables with setting

2. **Command Creation**
   - [ ] `/commands add test-command` creates file
   - [ ] Editor opens with template
   - [ ] File saved correctly
   - [ ] Proper success message

3. **Command Execution**
   - [ ] `/project:test-command` executes
   - [ ] Command content displayed
   - [ ] Error handling for missing commands

4. **Error Handling**
   - [ ] Invalid command names rejected
   - [ ] Duplicate command names handled
   - [ ] File system errors handled gracefully

### Automated Testing

```bash
# Run unit tests
cargo test -p chat_cli command_manager

# Run integration tests
cargo test -p chat_cli commands_integration

# Run all tests
cargo test -p chat_cli
```

## Deployment Checklist

- [ ] All code changes implemented
- [ ] Unit tests passing
- [ ] Integration tests passing
- [ ] Manual testing completed
- [ ] Documentation updated
- [ ] Feature flag properly implemented
- [ ] Error messages consistent with knowledge base
- [ ] UX patterns match knowledge base exactly

## Success Criteria

The MVP is considered successful when:

1. **Core Functionality Works**
   - Users can enable the feature via settings
   - Users can create commands with `/commands add`
   - Users can execute commands with `/project:name`

2. **UX Consistency Maintained**
   - Error messages match knowledge base patterns
   - Visual indicators consistent
   - Help text follows same format

3. **Quality Standards Met**
   - >90% test coverage
   - All error conditions handled
   - Performance acceptable (<2s for operations)

4. **Security Requirements Met**
   - Feature properly gated behind settings
   - Input validation prevents malicious content
   - File system operations secure

---

*Implementation Guide Version: 1.0*  
*Created: 2025-07-10*  
*Estimated Timeline: 8 days*  
*Status: Ready for Implementation*
