# User Experience Design - Custom Commands

## Overview

This document defines the user experience for the custom commands feature, including command-line interface design, user workflows, and interaction patterns.

## Command Interface Design

### Primary Commands

Following Claude Code patterns, the interface provides both management and execution:

```bash
# Direct command execution (Claude Code style)
/project:command-name [arguments]          # Execute project command
/user:command-name [arguments]             # Execute user command
/project:namespace:command [arguments]     # Execute namespaced command

# Management commands (Amazon Q CLI style)
/commands show [--scope=project|user|all] [--namespace=ns]  # Display available commands
/commands add <name> [--scope=project|user] [--namespace=ns] # Create new command
/commands edit <scope>:<name>                               # Edit existing command
/commands remove <scope>:<name>                            # Remove command
/commands list [--scope=project|user] [--namespace=ns]     # List with filters
/commands import <file|url> [--scope=project|user]         # Import command
/commands export <scope>:<name> [--output=file]            # Export command
/commands validate <scope>:<name>                          # Validate command
/commands stats [<scope>:<name>]                           # Show usage statistics
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

# Complex example with file references and bash execution
/project:git-commit "Add user authentication feature"
```

## User Workflows

### 1. Creating a New Command

**Interactive Creation Flow:**

```bash
User: /commands add git-helper --scope=project --namespace=tools
Q: I'll help you create a new command called 'git-helper' in the 'tools' namespace.

   This will create: /project:tools:git-helper
   Location: .amazonq/commands/tools/git-helper.md
   
   Opening editor to define your command...
   
   [Editor opens with Claude Code template]
   
   Command created successfully! 
   Use '/project:tools:git-helper' to execute it.
```

**Claude Code Template Structure:**
```markdown
---
allowed-tools:
  - "Bash(git:*)"
description: "Brief description of what this command does"
---

# Git Helper

Brief description of what this command does.

## Context

Include any bash commands to gather context:
- Current status: !`git status`
- Recent commits: !`git log --oneline -5`

## Your task

Provide detailed instructions for Amazon Q:

1. Step 1: What to analyze first
2. Step 2: What to look for  
3. Step 3: How to format the response

Use $ARGUMENTS for dynamic input from the user.

## File References (Optional)

Reference specific files if needed:
- Check configuration: @package.json
- Review main file: @src/main.rs
```

### 2. Discovering Available Commands

**Show Command with Namespaces:**

```bash
User: /commands show --expand

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
         
   📂 backend/
      ✅ api-review (v2.0.0) - API endpoint security review
         Usage: /project:backend:api-review [endpoint]
         Tools: Bash(curl:*), FileSystem(read:api/*)
         Last used: 3 hours ago (executed 23 times)

🌍 User Commands (~/.amazonq/commands/):
   ✅ debug-session (v2.1.0) - Interactive debugging helper
      Usage: /user:debug-session [issue-description]
      Tools: Bash(*), FileSystem(read:*)
      Last used: 3 days ago (executed 42 times)

Total: 4 commands available across 3 namespaces
```

### 3. Executing Commands with Claude Code Features

**Command with Arguments and Bash Execution:**

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

Unstaged changes:
  modified:   README.md

Based on your request to "Add user authentication feature" and the current changes, I'll create a structured commit:

Executing: git add src/auth/mod.rs src/auth/jwt.rs src/main.rs
Executing: git commit -m "feat: add JWT-based user authentication

- Add JWT token generation and validation
- Implement authentication middleware  
- Update main.rs to include auth routes
- Add comprehensive error handling for auth flows"

✅ Commit created successfully: abc123f
```

**Command with File References:**

```bash
User: /project:security-review

🚀 Executing command: security-review

Reading referenced files...
├─ @package.json (dependencies analysis)
├─ @src/auth/mod.rs (authentication logic)
└─ @src/api/routes.rs (API endpoints)

Security Analysis Results:

📋 Dependencies (from package.json):
✅ All dependencies are up to date
⚠️  Found 1 moderate vulnerability in 'lodash' - update recommended

🔐 Authentication (from src/auth/mod.rs):
✅ JWT tokens properly validated
✅ Password hashing using bcrypt
⚠️  Missing rate limiting on login attempts

🌐 API Routes (from src/api/routes.rs):
✅ Input validation implemented
❌ Missing CORS configuration
❌ No request size limits detected

[Detailed analysis continues...]
```

### 4. Managing Commands

**Editing Existing Commands:**

```bash
User: /commands edit code-review

Opening code-review command for editing...
Scope: Local (.amazonq/commands/)
Last modified: 2 hours ago

[Editor opens with current command content]

Command updated successfully!
```

**Removing Commands:**

```bash
User: /commands remove old-command

⚠️  Are you sure you want to remove 'old-command'?
   Scope: Global (~/.amazonq/commands/)
   This action cannot be undone.
   
   Type 'yes' to confirm: yes
   
✅ Command 'old-command' removed successfully.
```

## Error Handling and User Feedback

### Command Not Found

```bash
User: /commands run nonexistent

❌ Command 'nonexistent' not found.

Available commands:
• code-review (local)
• deploy-check (local)  
• debug-session (global)

Use '/commands show' to see all available commands.
```

### Invalid Command Format

```bash
User: /commands validate broken-command

❌ Command validation failed for 'broken-command':

Issues found:
1. Missing required '# Command Name' header
2. Instructions section is empty
3. Content exceeds maximum size limit (15KB > 10KB)

Fix these issues and try again.
```

### Execution Errors

```bash
User: /commands run timeout-command

🚀 Executing command: timeout-command

[Command starts executing...]

❌ Command execution timed out after 300 seconds.

The command may be too complex or require manual intervention.
Consider breaking it into smaller steps or increasing the timeout.
```

## Scope Management

### Local vs Global Commands

**Visual Indicators:**
- 📁 Local commands (project-specific)
- 🌍 Global commands (available everywhere)
- ⚠️ Conflicts (local overrides global)

**Scope Resolution Display:**

```bash
User: /commands show code-review

Command: code-review
├─ 📁 Local: ✅ Available (v1.0.0) - Will be used
└─ 🌍 Global: ✅ Available (v0.8.0) - Overridden by local

Local command takes precedence.
Use '--scope=global' to force global version.
```

## Integration with Existing Features

### Context Integration

Commands can reference current context:

```bash
User: /commands run analyze-context

🚀 Executing command: analyze-context

Using current context:
• README.md (2.1KB)
• src/main.rs (4.5KB)  
• Cargo.toml (0.8KB)

[Analysis begins with current context...]
```

### Profile Integration

Commands respect current profile:

```bash
User: /agent switch rust-expert
Switched to profile: rust-expert

User: /commands show

📁 Profile Commands (rust-expert):
   ✅ rust-review - Rust-specific code review
   ✅ cargo-audit - Security audit for Rust projects

📁 Local Commands:
   ✅ code-review - General code review

🌍 Global Commands:
   ✅ debug-session - Interactive debugging
```

## Accessibility and Usability

### Command Discovery

**Auto-completion Support:**
```bash
User: /commands run <TAB>
code-review    deploy-check    debug-session

User: /commands run code<TAB>
code-review
```

**Search and Filtering:**
```bash
User: /commands list --tags=security
🔍 Commands tagged with 'security':
• code-review (local) - Comprehensive code review analysis
• security-scan (global) - Security vulnerability scanner
```

### Help and Documentation

**Inline Help:**
```bash
User: /commands help run

Usage: /commands run <name> [args...]

Execute a custom command by name.

Arguments:
  <name>     Name of the command to execute
  [args...]  Optional arguments to pass to the command

Examples:
  /commands run code-review
  /commands run deploy --env=staging
  
Use '/commands show <name>' to see command details.
```

**Command-specific Help:**
```bash
User: /commands show code-review --help

Command: code-review (v1.0.0)
Description: Comprehensive code review analysis

Instructions Preview:
> Perform a thorough code review analyzing:
> 1. Code quality and best practices
> 2. Security vulnerabilities
> 3. Performance optimizations
> [...]

Usage: /commands run code-review
Tags: [review, security, quality]
Scope: Local (.amazonq/commands/)
```

## Performance and Responsiveness

### Loading States

```bash
User: /commands run large-analysis

🚀 Executing command: large-analysis
⏳ Loading command definition...
⏳ Preparing execution environment...
⏳ Analyzing codebase (this may take a while)...

[Progress indicators for long-running commands]
```

### Streaming Responses

Commands integrate with Amazon Q's streaming response system:

```bash
User: /commands run code-review

🚀 Executing command: code-review

Analyzing repository structure...
├─ Found package.json - Node.js project detected
├─ Scanning 23 TypeScript files...
└─ Checking dependencies...

Code Quality Assessment:
├─ ✅ Consistent naming conventions
├─ ⚠️  3 files missing error handling
└─ ⚠️  2 potential security issues found

[Response continues streaming...]
```

## Security and Safety

### User Consent

For potentially sensitive operations:

```bash
User: /commands run system-analysis

⚠️  This command will analyze system configuration and may access sensitive information.

Command permissions:
• Read system files
• Access environment variables
• Execute system commands

Continue? [y/N]: y

🚀 Proceeding with system analysis...
```

### Validation Feedback

```bash
User: /commands add suspicious-command

⚠️  Security validation detected potential issues:

1. Command contains shell execution patterns
2. Attempts to access system directories
3. Includes network requests to external URLs

This command has been flagged for review.
Continue anyway? [y/N]: n

Command creation cancelled.
```

---

*Document Version: 1.0*
*Last Updated: 2025-07-06*
*Next Review: After UX Testing*
