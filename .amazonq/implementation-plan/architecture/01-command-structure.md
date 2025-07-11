# Command Structure and Data Models

## Overview

This document defines the data structures and models used for custom commands, following the established patterns from the `/context` command implementation.

## Core Data Structures

### CommandConfig

```rust
/// Configuration for custom commands, containing command definitions and metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CommandConfig {
    /// Map of command name to command definition
    pub commands: HashMap<String, CustomCommand>,
    
    /// Metadata for the command configuration
    pub metadata: CommandMetadata,
    
    /// Version of the command configuration format
    pub version: String,
}
```

### CustomCommand

```rust
/// Definition of a custom command following Claude Code patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomCommand {
    /// Name of the command (derived from filename)
    pub name: String,
    
    /// Namespace path (from directory structure)
    pub namespace: Option<String>,
    
    /// Command scope (Project or User)
    pub scope: CommandScope,
    
    /// Human-readable description from YAML frontmatter
    pub description: Option<String>,
    
    /// Command content (Markdown format)
    pub content: String,
    
    /// YAML frontmatter metadata
    pub frontmatter: CommandFrontmatter,
    
    /// Command metadata
    pub metadata: CommandMetadata,
    
    /// Execution configuration
    pub execution: ExecutionConfig,
    
    /// Validation rules
    pub validation: ValidationConfig,
}

/// Command scope following Claude Code pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandScope {
    /// Project-specific commands (.amazonq/commands/) - /project: prefix
    Project,
    /// User-global commands (~/.amazonq/commands/) - /user: prefix  
    User,
}

/// YAML frontmatter structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CommandFrontmatter {
    /// Tools this command is allowed to use
    pub allowed_tools: Vec<String>,
    
    /// Brief description of the command
    pub description: Option<String>,
    
    /// Whether this command triggers extended thinking
    pub thinking_mode: Option<bool>,
    
    /// Additional metadata fields
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}
```

### CommandMetadata

```rust
/// Metadata associated with commands
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CommandMetadata {
    /// Creation timestamp
    pub created_at: Option<DateTime<Utc>>,
    
    /// Last modified timestamp
    pub modified_at: Option<DateTime<Utc>>,
    
    /// Author information
    pub author: Option<String>,
    
    /// Command version
    pub version: String,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Usage statistics
    pub usage_stats: UsageStats,
    
    /// File checksum for integrity
    pub checksum: Option<String>,
}
```

### ExecutionConfig

```rust
/// Configuration for command execution
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ExecutionConfig {
    /// Maximum execution time in seconds
    pub timeout_seconds: Option<u64>,
    
    /// Whether to inject current context
    pub inject_context: bool,
    
    /// Whether to preserve conversation history
    pub preserve_history: bool,
    
    /// Pre-execution hooks
    pub pre_hooks: Vec<String>,
    
    /// Post-execution hooks
    pub post_hooks: Vec<String>,
    
    /// Environment variables to set
    pub environment: HashMap<String, String>,
}
```

### ValidationConfig

```rust
/// Validation rules for commands
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ValidationConfig {
    /// Maximum content size in bytes
    pub max_content_size: Option<usize>,
    
    /// Allowed content patterns (regex)
    pub allowed_patterns: Vec<String>,
    
    /// Forbidden content patterns (regex)
    pub forbidden_patterns: Vec<String>,
    
    /// Required metadata fields
    pub required_metadata: Vec<String>,
    
    /// Security validation level
    pub security_level: SecurityLevel,
}
```

### SecurityLevel

```rust
/// Security validation levels
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    #[default]
    Standard,
    Strict,
    Permissive,
}
```

### UsageStats

```rust
/// Usage statistics for commands
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsageStats {
    /// Total execution count
    pub execution_count: u64,
    
    /// Last execution timestamp
    pub last_executed: Option<DateTime<Utc>>,
    
    /// Average execution time in milliseconds
    pub avg_execution_time_ms: Option<u64>,
    
    /// Success rate (0.0 to 1.0)
    pub success_rate: Option<f64>,
}
```

## Command Manager Structure

### CommandManager

```rust
/// Manager for custom commands, following the ContextManager pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandManager {
    /// Maximum command content size
    max_command_size: usize,
    
    /// Current active profile
    pub current_profile: String,
    
    /// Global command configuration
    pub global_config: CommandConfig,
    
    /// Local (project-specific) command configuration
    pub local_config: CommandConfig,
    
    /// Command cache for performance
    #[serde(skip)]
    pub command_cache: HashMap<String, CustomCommand>,
    
    /// Command executor
    #[serde(skip)]
    pub executor: CommandExecutor,
}
```

### CommandExecutor

```rust
/// Executor for custom commands with Claude Code feature support
#[derive(Debug)]
pub struct CommandExecutor {
    /// Execution timeout
    timeout: Duration,
    
    /// Security validator
    validator: CommandValidator,
    
    /// Argument processor for $ARGUMENTS substitution
    arg_processor: ArgumentProcessor,
    
    /// Bash command runner for ! prefix commands
    bash_runner: BashCommandRunner,
    
    /// File reference handler for @ prefix references
    file_includer: FileReferenceHandler,
    
    /// Tool permission manager
    tool_manager: ToolPermissionManager,
}

/// Processes $ARGUMENTS placeholder substitution
#[derive(Debug)]
pub struct ArgumentProcessor {
    /// Pattern for argument substitution
    argument_pattern: Regex,
}

/// Executes bash commands prefixed with !
#[derive(Debug)]
pub struct BashCommandRunner {
    /// Allowed bash commands (security)
    allowed_commands: Vec<Regex>,
    
    /// Execution timeout for bash commands
    timeout: Duration,
}

/// Handles @ file references
#[derive(Debug)]
pub struct FileReferenceHandler {
    /// Maximum file size to include
    max_file_size: usize,
    
    /// Allowed file patterns
    allowed_patterns: Vec<Regex>,
}

/// Manages tool permissions from allowed-tools frontmatter
#[derive(Debug)]
pub struct ToolPermissionManager {
    /// Available tools and their permissions
    available_tools: HashMap<String, ToolPermission>,
}

#[derive(Debug, Clone)]
pub struct ToolPermission {
    /// Tool name (e.g., "Bash")
    pub name: String,
    
    /// Allowed operations (e.g., "git add:*")
    pub operations: Vec<String>,
}
```

### CommandValidator

```rust
/// Validator for command security and format
#[derive(Debug)]
pub struct CommandValidator {
    /// Maximum allowed content size
    max_content_size: usize,
    
    /// Security patterns
    security_patterns: Vec<Regex>,
    
    /// Content sanitizer
    sanitizer: ContentSanitizer,
}
```

## File Format Specifications

### Command Definition Format (Markdown with YAML Frontmatter)

```markdown
---
allowed-tools: 
  - "Bash(git add:*)"
  - "Bash(git status:*)"
  - "Bash(git commit:*)"
description: "Create a git commit based on current changes"
thinking-mode: true
---

# Git Commit Helper

Create a well-structured git commit based on the current repository state.

## Context

- Current git status: !`git status`
- Current git diff (staged and unstaged changes): !`git diff HEAD`
- Current branch: !`git branch --show-current`
- Recent commits: !`git log --oneline -10`

## Your task

Based on the above changes and any provided arguments ($ARGUMENTS), create a single git commit with:

1. A clear, descriptive commit message
2. Proper staging of relevant files
3. Following conventional commit format if applicable

## File Analysis

Review the main configuration: @package.json
Check recent changes in: @src/main.rs

## Instructions

1. Analyze the git status and diff output
2. Determine what changes should be committed
3. Create an appropriate commit message
4. Execute the git commands to stage and commit
```

### Namespace Structure

Commands can be organized in subdirectories for namespacing:

```
.amazonq/commands/
├── optimize.md                    # /project:optimize
├── security-review.md             # /project:security-review
├── frontend/
│   ├── component.md              # /project:frontend:component
│   └── styling.md                # /project:frontend:styling
└── backend/
    ├── api-review.md             # /project:backend:api-review
    └── database/
        └── migration.md          # /project:backend:database:migration

~/.amazonq/commands/
├── code-review.md                # /user:code-review
├── debug.md                      # /user:debug
└── tools/
    ├── benchmark.md              # /user:tools:benchmark
    └── profiling.md              # /user:tools:profiling
```

### Command Execution Syntax

```bash
# Basic command execution
/project:optimize
/user:code-review

# Namespaced commands
/project:frontend:component
/user:tools:benchmark

# Commands with arguments
/project:fix-issue 123
/user:debug "memory leak in parser"

# Arguments are substituted in command content via $ARGUMENTS placeholder
```

### Metadata File Format (.metadata.json)

```json
{
  "version": "1.0.0",
  "commands": {
    "command-name": {
      "file": "command-name.md",
      "checksum": "sha256:...",
      "metadata": {
        "created_at": "2025-07-06T22:00:00Z",
        "modified_at": "2025-07-06T22:00:00Z",
        "author": "username",
        "version": "1.0.0",
        "tags": ["tag1", "tag2"],
        "usage_stats": {
          "execution_count": 42,
          "last_executed": "2025-07-06T22:00:00Z",
          "avg_execution_time_ms": 1500,
          "success_rate": 0.95
        }
      },
      "execution": {
        "timeout_seconds": 300,
        "inject_context": true,
        "preserve_history": true,
        "environment": {}
      },
      "validation": {
        "max_content_size": 10240,
        "security_level": "Standard"
      }
    }
  }
}
```

## Command Resolution Logic

### Scope Resolution Order

1. **Local Commands** (`.amazonq/commands/`) - Highest priority
2. **Global Commands** (`~/.amazonq/commands/`) - Fallback
3. **Built-in Commands** - System defaults

### Name Conflict Resolution

```rust
impl CommandManager {
    /// Resolve command by name, following scope precedence
    pub fn resolve_command(&self, name: &str) -> Option<&CustomCommand> {
        // 1. Check local commands first
        if let Some(cmd) = self.local_config.commands.get(name) {
            return Some(cmd);
        }
        
        // 2. Check global commands
        if let Some(cmd) = self.global_config.commands.get(name) {
            return Some(cmd);
        }
        
        // 3. No command found
        None
    }
}
```

## Integration with Existing Systems

### Profile Integration

Commands can be associated with specific profiles:

```rust
impl TryFrom<&Agent> for CommandConfig {
    type Error = eyre::Report;
    
    fn try_from(agent: &Agent) -> Result<Self, Self::Error> {
        // Load commands specific to this agent/profile
        // Merge with global commands
        // Apply profile-specific overrides
    }
}
```

### Context Integration

Commands can reference and modify context:

```rust
impl CommandExecutor {
    /// Execute command with current context
    pub async fn execute_with_context(
        &self,
        command: &CustomCommand,
        context: &ContextManager,
        session: &mut ChatSession,
    ) -> Result<String, CommandError> {
        // Inject context if configured
        // Execute command through chat session
        // Handle streaming responses
    }
}
```

## Error Handling

### CommandError Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Command not found: {name}")]
    NotFound { name: String },
    
    #[error("Invalid command format: {reason}")]
    InvalidFormat { reason: String },
    
    #[error("Security validation failed: {reason}")]
    SecurityViolation { reason: String },
    
    #[error("Execution timeout after {seconds}s")]
    Timeout { seconds: u64 },
    
    #[error("IO error: {source}")]
    Io { source: std::io::Error },
    
    #[error("Serialization error: {source}")]
    Serialization { source: serde_json::Error },
}
```

## Performance Considerations

### Caching Strategy

- **Command Cache**: In-memory cache of parsed commands
- **Metadata Cache**: Cached metadata for quick lookups
- **File Watching**: Monitor file changes for cache invalidation

### Lazy Loading

- Commands loaded on-demand
- Metadata loaded separately from content
- Streaming for large command outputs

---

*Document Version: 1.0*
*Last Updated: 2025-07-06*
*Dependencies: System Overview, Storage Strategy*
