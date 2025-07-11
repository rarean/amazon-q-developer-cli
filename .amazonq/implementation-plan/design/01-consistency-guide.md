# UX Consistency Guide - Custom Commands & Knowledge Base

## Overview

This document defines the specific patterns and standards that must be followed to ensure UX consistency between the custom commands feature and the existing knowledge base feature.

## Core Consistency Principles

### 1. Feature Enablement Pattern

**Knowledge Base Pattern:**
```bash
q settings chat.enableKnowledge true
```

**Custom Commands Pattern (MUST FOLLOW):**
```bash
q settings chat.enableCommands true
```

**Disabled Feature Response (EXACT MATCH):**
```bash
❌ Commands tool is disabled. Enable it with: q settings chat.enableCommands true
```

### 2. Command Structure Pattern

**Knowledge Base Pattern:**
```bash
/knowledge show
/knowledge add <name> <path>
/knowledge remove <name>
/knowledge update <name> <path>
/knowledge clear
/knowledge status
```

**Custom Commands Pattern (MUST FOLLOW):**
```bash
/commands show
/commands add <name>
/commands remove <name>
/commands update <name>
/commands clear
/commands status
```

### 3. Visual Indicators (EXACT MATCH)

**Scope Indicators:**
- 📁 **Project scope** (local/project-specific)
- 🌍 **User scope** (global/user-wide)

**Status Indicators:**
- ✅ **Success/Available**
- ❌ **Error/Not found**
- ⚠️  **Warning/Conflict**
- 🚀 **Execution/Processing**
- 💡 **Tips/Hints**
- 📊 **Status/Statistics**
- 🗑️  **Destructive operations**

**Progress Indicators:**
```bash
# Tree-style progress (EXACT MATCH)
Gathering context...
├─ Running: git status
├─ Running: git diff HEAD  
└─ Running: git log --oneline -10
```

### 4. Error Message Patterns

**Feature Disabled (EXACT TEMPLATE):**
```bash
❌ {Feature} tool is disabled. Enable it with: q settings {setting} true
```

**Item Not Found (EXACT TEMPLATE):**
```bash
❌ {Item} '{name}' not found in {scope} scope.

Available {scope} {items}:
• {item1}
• {item2}

Use '/{command} show' to see all available {items}.
```

**Validation Error (EXACT TEMPLATE):**
```bash
❌ {Operation} validation failed for '{name}':

Issues found:
1. {issue1}
2. {issue2}

Fix these issues and try again.
```

### 5. Success Message Patterns

**Creation Success (EXACT TEMPLATE):**
```bash
✅ {Item} '{name}' created successfully!
   Use '/{execution-command}' to execute it.
   
💡 Tip: Use '/{command} show {name}' to see {item} details.
```

**Operation Success (EXACT TEMPLATE):**
```bash
✅ Successfully {operation} {count} {items}
```

### 6. Confirmation Patterns

**Destructive Operation (EXACT TEMPLATE):**
```bash
⚠️  This will {action} ALL {scope} {items}. Are you sure? (y/N): 
```

**Clear Operation Response:**
```bash
🗑️  Clearing {scope} {items}...
✅ Successfully removed {count} {scope} {items}

💡 Tip: {Items} can be restored from backups if needed.
```

## Implementation Requirements

### 1. Settings Integration (MANDATORY)

```rust
// In database/settings.rs (EXACT PATTERN)
pub enum Setting {
    // ... existing settings
    EnabledKnowledge,
    EnabledCommands,    // NEW: Follow exact same pattern
}

impl Setting {
    pub fn key(&self) -> &'static str {
        match self {
            Self::EnabledKnowledge => "chat.enableKnowledge",
            Self::EnabledCommands => "chat.enableCommands",  // NEW: Same pattern
        }
    }
}

// In command implementation (EXACT PATTERN)
impl Commands {
    pub fn is_enabled(os: &Os) -> bool {
        os.database
            .settings
            .get_bool(Setting::EnabledCommands)
            .unwrap_or(false)
    }
}
```

### 2. Tool Registration (MANDATORY)

```rust
// In tools/mod.rs (EXACT PATTERN)
pub enum Tool {
    // ... existing tools
    Knowledge(Knowledge),
    Commands(Commands),  // NEW: Same pattern
}

impl Tool {
    pub fn permission_eval(&self) -> PermissionEvalResult {
        match self {
            Tool::Knowledge(_) => PermissionEvalResult::Ask,
            Tool::Commands(_) => PermissionEvalResult::Ask,  // NEW: Same security level
        }
    }
}

// In tool_manager.rs (EXACT PATTERN)
if !Knowledge::is_enabled(os) {
    tool_specs.remove("knowledge");
}
if !Commands::is_enabled(os) {
    tool_specs.remove("commands");  // NEW: Same pattern
}
```

### 3. CLI Structure (MANDATORY)

```rust
// File structure (EXACT PATTERN)
crates/chat-cli/src/
├── cli/chat/tools/knowledge.rs     # Existing
├── cli/chat/tools/commands.rs      # NEW: Same structure
├── cli/chat/cli/knowledge.rs       # Existing  
├── cli/chat/cli/commands.rs        # NEW: Same structure
├── util/knowledge_store.rs         # Existing
└── util/command_manager.rs         # NEW: Same pattern

// CLI enum structure (EXACT PATTERN)
#[derive(Clone, Debug, PartialEq, Eq, Subcommand)]
pub enum CommandsSubcommand {
    Show,
    Add { name: String },
    Remove { name: String },
    Update { name: String },
    Clear,
    Status,
}
```

### 4. Display Formatting (MANDATORY)

**Show Command Output (EXACT TEMPLATE):**
```bash
📁 Project {Items} (.amazonq/{items}/):
   ✅ {name} (v{version}) - {description}
      Usage: /{execution-syntax}
      Tools: {allowed-tools}
      Last used: {time} ago (executed {count} times)

🌍 User {Items} (~/.amazonq/{items}/):
   ✅ {name} (v{version}) - {description}
      Usage: /{execution-syntax}
      Tools: {allowed-tools}
      Last used: {time} ago (executed {count} times)

Total: {count} {items} available across {scope-count} scopes
```

**Status Command Output (EXACT TEMPLATE):**
```bash
📊 {Feature} Status:

📁 Project {Items} (.amazonq/{items}/):
   Total: {count} {items}
   Last updated: {time} ago
   
🌍 User {Items} (~/.amazonq/{items}/):
   Total: {count} {items}  
   Last updated: {time} ago

📈 Usage Statistics (Last 30 days):
   Most used: {name} ({count} executions)
   Recent: {name} (last used {time} ago)
   
✅ All {items} validated successfully
```

### 5. Help System (MANDATORY)

**Help Command Structure (EXACT TEMPLATE):**
```bash
{Feature} - {Brief description}

Usage:
  /{command} show [options]     Display available {items}
  /{command} add <name>         Create new {item}
  /{command} remove <name>      Remove {item}
  /{command} update <name>      Update existing {item}
  /{command} clear              Remove all {items}
  /{command} status             Show system status

{Execution section if applicable}

Examples:
  /{command} add {example-name}
  {execution-example}

For more help: /{command} help <subcommand>
```

## Testing Requirements

### 1. Consistency Tests (MANDATORY)

```rust
#[cfg(test)]
mod consistency_tests {
    use super::*;

    #[test]
    fn test_disabled_feature_message_consistency() {
        let knowledge_msg = "Knowledge tool is disabled. Enable it with: q settings chat.enableKnowledge true";
        let commands_msg = "Commands tool is disabled. Enable it with: q settings chat.enableCommands true";
        
        // Verify same message structure
        assert_eq!(knowledge_msg.split(" with: ").count(), commands_msg.split(" with: ").count());
    }

    #[test]
    fn test_settings_key_consistency() {
        assert_eq!(Setting::EnabledKnowledge.key(), "chat.enableKnowledge");
        assert_eq!(Setting::EnabledCommands.key(), "chat.enableCommands");
        
        // Verify same pattern
        assert!(Setting::EnabledKnowledge.key().starts_with("chat.enable"));
        assert!(Setting::EnabledCommands.key().starts_with("chat.enable"));
    }

    #[test]
    fn test_tool_permission_consistency() {
        let knowledge_perm = Tool::Knowledge(Knowledge::Show).permission_eval();
        let commands_perm = Tool::Commands(Commands::Show).permission_eval();
        
        assert_eq!(knowledge_perm, commands_perm);
    }
}
```

### 2. UX Validation Tests (MANDATORY)

```rust
#[test]
fn test_show_command_format_consistency() {
    // Test that both features use same formatting
    // - Same emoji usage
    // - Same indentation
    // - Same information hierarchy
    // - Same color coding
}

#[test]
fn test_error_message_format_consistency() {
    // Test that error messages follow same template
    // - Same emoji usage
    // - Same message structure
    // - Same suggestion format
}
```

## Documentation Requirements

### 1. User Documentation (MANDATORY)

**Feature Comparison Table (REQUIRED):**
| Feature | Knowledge Base | Custom Commands |
|---------|----------------|-----------------|
| Purpose | Semantic search | Command templates |
| Enable Setting | `chat.enableKnowledge` | `chat.enableCommands` |
| Management | `/knowledge show` | `/commands show` |
| Scope | Project/User | Project/User |
| File Location | `.amazonq/knowledge/` | `.amazonq/commands/` |

**Consistent Examples (REQUIRED):**
```bash
# Enable features
q settings chat.enableKnowledge true
q settings chat.enableCommands true

# View available items
/knowledge show
/commands show

# Add new items
/knowledge add project-docs ./docs
/commands add code-review
```

### 2. Developer Documentation (MANDATORY)

**Pattern Documentation (REQUIRED):**
- Settings integration patterns
- Tool registration patterns
- CLI structure patterns
- Error handling patterns
- Display formatting patterns

## Quality Assurance

### 1. Review Checklist (MANDATORY)

**Before Code Review:**
- [ ] Settings follow exact knowledge base pattern
- [ ] Error messages use exact same templates
- [ ] Visual indicators match exactly
- [ ] Help system follows same structure
- [ ] File organization mirrors knowledge base
- [ ] Tool registration uses same pattern

**Before Release:**
- [ ] Side-by-side UX testing completed
- [ ] Consistency tests pass
- [ ] Documentation updated with consistent examples
- [ ] User acceptance testing validates consistent experience

### 2. Automated Checks (MANDATORY)

```rust
// CI/CD pipeline checks
fn validate_consistency() {
    // Check that error message templates match
    // Check that settings patterns are consistent
    // Check that visual indicators are identical
    // Check that help text follows same structure
}
```

## Maintenance Guidelines

### 1. Future Changes (MANDATORY)

**When updating knowledge base patterns:**
1. Update custom commands to match
2. Run consistency tests
3. Update documentation
4. Validate user experience

**When updating custom commands:**
1. Ensure changes don't break consistency
2. Consider if knowledge base should also change
3. Update shared patterns if applicable

### 2. Monitoring (RECOMMENDED)

- Track user confusion between features
- Monitor support requests for consistency issues
- Gather feedback on unified experience
- Regular UX audits for pattern drift

---

*Document Version: 1.0*
*Last Updated: 2025-07-10*
*Status: Mandatory Implementation Guide*
*Review Required: Before any custom commands implementation*
