# User-Scoped Commands Requirements

## Introduction

This feature extends the existing custom commands system to support user-scoped commands stored in the global `~/.amazonq/commands/` directory. This allows users to create personal commands that are available across all projects, complementing the existing project-scoped commands.

## Requirements

### Requirement 1: User-Scoped Command Storage

**User Story:** As a developer, I want to create personal commands that are available across all my projects, so that I can reuse common workflows without duplicating command files.

#### Acceptance Criteria

```gherkin
Scenario: Creating a user-scoped command
  Given the commands feature is enabled
  When a user creates a command with user scope
  Then the system should store the command file in `~/.amazonq/commands/` directory

Scenario: Auto-creating user commands directory
  Given the `~/.amazonq/commands/` directory does not exist
  When a user creates their first user-scoped command
  Then the system should create the directory automatically with proper permissions

Scenario: Executing user-scoped commands
  Given a user-scoped command exists
  When a user executes `/user:command-name`
  Then the system should load the command from the user-scoped directory

Scenario: Command scope priority
  Given both user and project commands exist with the same name
  When a user executes `/project:command-name`
  Then the system should prioritize the project-scoped command
```

### Requirement 2: User Command Execution Syntax

**User Story:** As a developer, I want to execute user-scoped commands using `/user:command-name` syntax, so that I can distinguish between personal and project commands.

#### Acceptance Criteria

```gherkin
Scenario: Parsing user command syntax
  Given the commands feature is enabled
  When a user types `/user:command-name`
  Then the system should parse it as a user-scoped command execution

Scenario: Executing user-scoped command
  Given a user-scoped command exists
  When the command is executed
  Then the system should display "🚀 Executing user command: command-name"

Scenario: User command not found
  Given a user-scoped command does not exist
  When a user tries to execute it
  Then the system should display "❌ Command 'command-name' not found in user scope. Use '/commands add command-name --scope user' to create it."

Scenario: Feature disabled for user commands
  Given the commands feature is disabled
  When a user tries to execute a user-scoped command
  Then the system should show the same error message as project commands
```

### Requirement 3: User Command Management

**User Story:** As a developer, I want to manage user-scoped commands through the CLI, so that I can create, view, and organize my personal commands.

#### Acceptance Criteria

```gherkin
Scenario: Creating user-scoped command via CLI
  Given the commands feature is enabled
  When a user runs `/commands add command-name --scope user`
  Then the system should create the command file in `~/.amazonq/commands/`

Scenario: Showing only user-scoped commands
  Given user-scoped commands exist
  When a user runs `/commands show --scope user`
  Then the system should display only user-scoped commands

Scenario: Showing all commands with scope indicators
  Given both user and project commands exist
  When a user runs `/commands show` without scope
  Then the system should display both user and project commands with scope indicators

Scenario: Removing user-scoped command
  Given a user-scoped command exists
  When a user runs `/commands remove command-name --scope user`
  Then the system should remove the command from user scope only
```

### Requirement 4: Namespace Support for User Commands

**User Story:** As a developer, I want to organize my user commands into namespaces using directories, so that I can group related commands and avoid naming conflicts.

#### Acceptance Criteria

```gherkin
Scenario: Namespace command creation
  Given the commands feature is enabled
  When a user creates a command in `~/.amazonq/commands/git/helper.md`
  Then the command should be accessible as `/user:git:helper`

Scenario: Displaying namespaced commands
  Given user-scoped commands exist in namespaces
  When a user runs `/commands show --scope user`
  Then the system should display commands with their full namespace paths

Scenario: Multi-level namespace support
  Given nested directories exist in user commands
  When a user creates commands in nested directories
  Then the system should support multi-level namespaces (e.g., `/user:frontend:components:button`)

Scenario: Empty namespace handling
  Given namespace directories are empty
  When the system discovers commands
  Then it should ignore empty directories during command discovery
```

### Requirement 5: Cross-Platform Compatibility

**User Story:** As a developer working on different operating systems, I want user-scoped commands to work consistently across platforms, so that my personal commands are portable.

#### Acceptance Criteria

```gherkin
Scenario: Cross-platform home directory resolution
  Given the system is running on any operating system
  When the system determines the user commands directory
  Then it should use the appropriate home directory path for the current operating system

Scenario: File permissions respect
  Given user commands are being accessed
  When file operations are performed on user commands
  Then they should respect the operating system's file permissions and security model

Scenario: Path separator handling
  Given commands exist in nested directories
  When commands are discovered across different operating systems
  Then the system should handle path separators correctly across Windows, macOS, and Linux

Scenario: Inaccessible home directory
  Given the user's home directory is not accessible
  When the system tries to access user commands
  Then it should provide a clear error message and graceful degradation
```