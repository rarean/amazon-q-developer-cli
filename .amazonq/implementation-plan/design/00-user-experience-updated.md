# User Experience Design - Custom Commands (Updated for Consistency)

## Overview

This document defines the user experience for the custom commands feature, following established Amazon Q CLI patterns for consistency with existing features like the knowledge base.

## Command Interface Design

### Primary Commands

**Consistent with Knowledge Base UX Patterns:**

```bash
# Management commands (consistent with /knowledge pattern)
/commands show [--scope=project|user|all] [--namespace=ns]  # Display available commands
/commands add <name> [--scope=project|user] [--namespace=ns] # Create new command
/commands remove <name> [--scope=project|user]             # Remove command  
/commands update <name> [--scope=project|user]             # Update existing command
/commands clear [--scope=project|user]                     # Remove all commands
/commands status                                            # Show command system status

# Direct command execution (Claude Code compatibility)
/project:command-name [arguments]          # Execute project command
/user:command-name [arguments]             # Execute user command
/project:namespace:command [arguments]     # Execute namespaced command
```

### Settings Integration (Consistent with Knowledge Base)

```bash
# Feature enablement (following knowledge base pattern)
q settings chat.enableCommands true        # Enable custom commands feature
q settings chat.enableCommands false       # Disable custom commands feature

# Additional settings
q settings chat.commands.maxExecutionTime 300  # Max execution time in seconds
q settings chat.commands.allowBashExecution true  # Allow bash command execution
```

### Command Execution Examples

```bash
# Basic execution
/project:code-review
/user:security-audit

# With arguments
/project:fix-issue 123
/user:debug "memory leak in authentication module"

# Namespaced commands
/project:frontend:component-review
/user:tools:performance-analysis
```

## User Workflows

### 1. Feature Enablement (Consistent with Knowledge Base)

**Initial Setup Flow:**

```bash
User: /commands show

❌ Commands tool is disabled. Enable it with: q settings chat.enableCommands true

User: q settings chat.enableCommands true
✅ Custom commands feature enabled.

User: /commands show

📁 Project Commands (.amazonq/commands/):
   No commands found.

🌍 User Commands (~/.amazonq/commands/):
   No commands found.

💡 Tip: Create your first command with '/commands add <name>'
```

### 2. Creating Commands (Following Knowledge Base Patterns)

**Interactive Creation Flow:**

```bash
User: /commands add git-helper --scope=project

🚀 Creating new command: git-helper
📁 Scope: Project (.amazonq/commands/)
📝 Opening editor to define your command...

[Editor opens with template]

✅ Command 'git-helper' created successfully!
   Use '/project:git-helper' to execute it.
   
💡 Tip: Use '/commands show git-helper' to see command details.
```

### 3. Discovering Commands (Consistent Visual Style)

**Show Command with Consistent Formatting:**

```bash
User: /commands show

📁 Project Commands (.amazonq/commands/):
   ✅ code-review (v1.0.0) - Comprehensive code review analysis
      Usage: /project:code-review [focus-area]
      Tools: Bash(git:*), FileSystem(read:*)
      Last used: 2 hours ago (executed 15 times)
      
   📂 frontend/
      ✅ component (v1.2.0) - React component analysis
         Usage: /project:frontend:component [component-name]
         Tools: Bash(npm:*), FileSystem(read:src/*)
         Last used: 1 day ago (executed 8 times)

🌍 User Commands (~/.amazonq/commands/):
   ✅ debug-session (v2.1.0) - Interactive debugging helper
      Usage: /user:debug-session [issue-description]
      Tools: Bash(*), FileSystem(read:*)
      Last used: 3 days ago (executed 42 times)

Total: 3 commands available across 2 scopes
```

### 4. Command Execution (Consistent Progress Indicators)

**Execution with Progress (Following Knowledge Base Style):**

```bash
User: /project:git-commit "Add user authentication feature"

🚀 Executing command: git-commit
📝 Arguments: "Add user authentication feature"

Gathering context...
├─ Running: git status
├─ Running: git diff HEAD  
├─ Running: git branch --show-current
└─ Running: git log --oneline -10

Current Git Status:
On branch feature/auth
Changes to be committed:
  modified:   src/auth/mod.rs
  new file:   src/auth/jwt.rs
  modified:   src/main.rs

✅ Commit created successfully: abc123f
```

### 5. Error Handling (Consistent with Knowledge Base)

**Feature Disabled:**

```bash
User: /commands show

❌ Commands tool is disabled. Enable it with: q settings chat.enableCommands true
```

**Command Not Found:**

```bash
User: /project:nonexistent

❌ Command 'nonexistent' not found in project scope.

Available project commands:
• code-review
• git-helper
• deploy-check

Use '/commands show' to see all available commands.
```

**Validation Errors:**

```bash
User: /commands add invalid-command

❌ Command validation failed for 'invalid-command':

Issues found:
1. Missing required YAML frontmatter
2. Instructions section is empty
3. Content exceeds maximum size limit (15KB > 10KB)

Fix these issues and try again.
```

## Status and Management (Consistent with Knowledge Base)

### Status Command (Following Knowledge Base Pattern)

```bash
User: /commands status

📊 Custom Commands Status:

📁 Project Commands (.amazonq/commands/):
   Total: 5 commands
   Last updated: 2 hours ago
   
🌍 User Commands (~/.amazonq/commands/):
   Total: 12 commands  
   Last updated: 1 day ago

📈 Usage Statistics (Last 30 days):
   Most used: code-review (45 executions)
   Recent: git-helper (last used 10 minutes ago)
   
✅ All commands validated successfully
```

### Clear Operation (Consistent Confirmation Pattern)

```bash
User: /commands clear --scope=project

⚠️  This will remove ALL project commands. Are you sure? (y/N): y

🗑️  Clearing project commands...
✅ Successfully removed 5 project commands

💡 Tip: Commands can be restored from backups if needed.
```

## Visual Consistency Standards

### Icons and Indicators (Matching Knowledge Base)

- 📁 **Project scope** (local commands)
- 🌍 **User scope** (global commands)  
- 📂 **Namespaces** (command folders)
- ✅ **Success states**
- ❌ **Error states**
- ⚠️  **Warning states**
- 🚀 **Execution states**
- 💡 **Tips and hints**
- 📊 **Status information**
- 🗑️  **Destructive operations**

### Progress Indicators (Consistent Style)

```bash
# Tree-style progress (matching knowledge base)
Gathering context...
├─ Running: git status
├─ Running: git diff HEAD  
└─ Running: git log --oneline -10

# Status with emojis
✅ Command executed successfully
⏳ Command execution in progress...
❌ Command execution failed
```

### Color Coding (Consistent with Knowledge Base)

- **Green**: Success states, available commands
- **Red**: Error states, disabled features
- **Yellow**: Warning states, pending operations
- **Blue**: Information, command names
- **Cyan**: Headers, section titles

## Integration with Existing Features

### Context Integration (Consistent Behavior)

```bash
User: /context add README.md
✅ Added README.md to context

User: /project:analyze-context

🚀 Executing command: analyze-context

Using current context:
• README.md (2.1KB)

[Analysis begins with current context...]
```

### Profile Integration (Consistent Scoping)

```bash
User: q profile switch rust-expert
Switched to profile: rust-expert

User: /commands show

📁 Profile Commands (rust-expert):
   ✅ rust-review - Rust-specific code review
   ✅ cargo-audit - Security audit for Rust projects

📁 Project Commands:
   ✅ code-review - General code review

🌍 User Commands:
   ✅ debug-session - Interactive debugging
```

## Accessibility and Discoverability

### Help System (Consistent with Knowledge Base)

```bash
User: /commands help

Custom Commands - Create and execute reusable command templates

Usage:
  /commands show [options]     Display available commands
  /commands add <name>         Create new command
  /commands remove <name>      Remove command
  /commands update <name>      Update existing command
  /commands clear              Remove all commands
  /commands status             Show system status

Execution:
  /project:<name> [args]       Execute project command
  /user:<name> [args]          Execute user command

Examples:
  /commands add code-review
  /project:code-review
  /user:debug-session "memory issue"

For more help: /commands help <subcommand>
```

### Auto-completion (Consistent Behavior)

```bash
User: /commands <TAB>
show    add    remove    update    clear    status

User: /project:<TAB>
code-review    git-helper    deploy-check

User: /user:<TAB>
debug-session    security-audit    performance-check
```

---

*Document Version: 2.0 - Updated for UX Consistency*
*Last Updated: 2025-07-10*
*Changes: Aligned with knowledge base UX patterns*
