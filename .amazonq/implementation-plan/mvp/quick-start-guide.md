# Custom Commands MVP - Quick Start Guide

## 🚀 Getting Started

This guide helps you implement the Custom Commands MVP in 8 days with just two core features:
1. **`/commands add <name>`** - Create custom commands
2. **`/project:<name>`** - Execute custom commands

## 📋 Prerequisites

- Rust development environment set up
- Amazon Q CLI codebase cloned and building
- Familiarity with the knowledge base implementation patterns

## 🎯 MVP Goals

### ✅ What We're Building
- Project-scoped commands only (`.amazonq/commands/`)
- Simple command creation with editor
- Basic command execution
- Settings integration (`chat.enableCommands`)
- Consistent UX with knowledge base

### ❌ What We're NOT Building
- User-scoped commands
- YAML frontmatter
- Bash execution or file references
- Argument substitution
- Advanced validation

## 📁 File Structure Overview

```
crates/chat-cli/src/
├── database/settings.rs              # Add EnabledCommands setting
├── util/
│   ├── mod.rs                        # Register new modules
│   ├── command_types.rs              # NEW: Data structures
│   └── command_manager.rs            # NEW: Core logic
├── cli/chat/
│   ├── cli/
│   │   ├── mod.rs                    # Register commands subcommand
│   │   └── commands.rs               # NEW: CLI interface
│   ├── tools/
│   │   ├── mod.rs                    # Register commands tool
│   │   └── commands.rs               # NEW: Tool implementation
│   └── tool_manager.rs               # Add tool filtering
```

## 🔧 Implementation Phases

### Phase 1: Foundation (Days 1-2)
**Focus**: Core data structures and settings

**Key Files**:
- `database/settings.rs` - Add `EnabledCommands`
- `util/command_types.rs` - Data structures and errors
- `util/command_manager.rs` - Core command management logic

**Validation**: Settings integration works, basic command creation

### Phase 2: CLI Integration (Days 3-4)
**Focus**: User interfaces

**Key Files**:
- `cli/chat/cli/commands.rs` - `/commands add` subcommand
- `cli/chat/tools/commands.rs` - Tool for command execution

**Validation**: `/commands add` creates files, basic tool structure

### Phase 3: Integration (Days 5-6)
**Focus**: Wire everything together

**Key Files**:
- `cli/chat/tools/mod.rs` - Register commands tool
- `cli/chat/tool_manager.rs` - Enable/disable tool
- `cli/chat/cli/mod.rs` - Register CLI subcommand

**Validation**: End-to-end workflow works

### Phase 4: Testing & Polish (Days 7-8)
**Focus**: Quality assurance

**Key Files**:
- Unit tests for all components
- Integration tests for workflows
- Documentation updates

**Validation**: All tests pass, documentation complete

## 🎨 UX Consistency Requirements

### Error Messages (MUST match knowledge base exactly)
```rust
// Feature disabled
"❌ Commands tool is disabled. Enable it with: q settings chat.enableCommands true"

// Command not found  
"❌ Command 'name' not found in project scope.\n\nUse '/commands add name' to create it."

// Success message
"✅ Command 'name' created successfully!\n   Use '/project:name' to execute it."
```

### Visual Indicators (MUST match knowledge base)
- 📁 Project scope indicators
- ✅ Success states
- ❌ Error states  
- 💡 Tips and hints

## 🧪 Testing Strategy

### Manual Testing Workflow
```bash
# 1. Test feature disabled (default state)
/commands add test
# Expected: Error message about enabling feature

# 2. Enable feature
q settings chat.enableCommands true

# 3. Test command creation
/commands add git-helper
# Expected: Editor opens with template, file created

# 4. Test command execution
/project:git-helper
# Expected: Command content displayed in chat

# 5. Test error cases
/project:nonexistent
# Expected: Command not found error
```

### Automated Testing
```bash
# Run during development
cargo test -p chat_cli command_manager
cargo test -p chat_cli commands_integration

# Final validation
cargo test -p chat_cli
```

## 🔍 Code Review Checklist

### Settings Integration
- [ ] `EnabledCommands` added to `Setting` enum
- [ ] Key mapping added (`"chat.enableCommands"`)
- [ ] FromStr implementation updated
- [ ] Feature check follows knowledge base pattern

### Data Structures
- [ ] `CustomCommand` struct with required fields
- [ ] `CommandError` enum with proper error types
- [ ] Error messages match knowledge base tone
- [ ] Proper serialization/deserialization

### Command Manager
- [ ] Singleton pattern like `KnowledgeStore`
- [ ] File operations use existing `Os` abstraction
- [ ] Error handling consistent with knowledge base
- [ ] Template creation follows markdown standards

### CLI Integration
- [ ] Subcommand structure matches knowledge base
- [ ] Error handling and user feedback consistent
- [ ] Visual formatting matches knowledge base exactly
- [ ] Help text follows same patterns

### Tool Integration
- [ ] Tool registration follows exact knowledge base pattern
- [ ] Permission evaluation same as knowledge base
- [ ] Tool filtering in `tool_manager.rs`
- [ ] Validation and execution methods implemented

## 🚨 Common Pitfalls

### 1. Pattern Inconsistency
**Problem**: Deviating from knowledge base patterns  
**Solution**: Copy exact patterns, don't innovate in MVP

### 2. Over-Engineering
**Problem**: Adding features not in MVP scope  
**Solution**: Stick to the two core features only

### 3. Error Handling
**Problem**: Generic or inconsistent error messages  
**Solution**: Follow knowledge base error message templates exactly

### 4. File Operations
**Problem**: Not using existing `Os` abstraction  
**Solution**: Use `os.current_dir()` and existing file utilities

## 📚 Reference Implementation

### Settings Pattern (Copy from knowledge base)
```rust
// In database/settings.rs
EnabledKnowledge,    // Existing
EnabledCommands,     // NEW: Add this

// In key() method
Self::EnabledKnowledge => "chat.enableKnowledge",
Self::EnabledCommands => "chat.enableCommands",  // NEW

// In from_str() method  
"chat.enableKnowledge" => Ok(Self::EnabledKnowledge),
"chat.enableCommands" => Ok(Self::EnabledCommands),  // NEW
```

### Tool Registration Pattern (Copy from knowledge base)
```rust
// In tools/mod.rs
pub mod knowledge;   // Existing
pub mod commands;    // NEW

use knowledge::Knowledge;   // Existing  
use commands::Commands;     // NEW

// Add to TOOL_NAMES, Tool enum, and all methods
```

### CLI Pattern (Copy from knowledge base)
```rust
// In cli/mod.rs
Knowledge(KnowledgeSubcommand),   // Existing
Commands(CommandsSubcommand),     // NEW

// In execute() method
ChatSubcommand::Knowledge(knowledge) => knowledge.execute(os, session).await,
ChatSubcommand::Commands(commands) => commands.execute(os, session).await,  // NEW
```

## 🎉 Success Criteria

The MVP is ready when:

- [ ] Feature can be enabled/disabled via settings
- [ ] `/commands add name` creates a command file and opens editor
- [ ] `/project:name` executes the command content
- [ ] Error messages match knowledge base exactly
- [ ] All tests pass (>90% coverage)
- [ ] Manual testing checklist completed
- [ ] Code review checklist completed

## 📞 Getting Help

If you get stuck:

1. **Check Knowledge Base Implementation**: Look at how knowledge base does it
2. **Review Consistency Guide**: `.amazonq/implementation-plan/design/01-consistency-guide.md`
3. **Run Tests**: `cargo test -p chat_cli` to catch issues early
4. **Manual Testing**: Follow the testing workflow above

---

*Quick Start Guide Version: 1.0*  
*Created: 2025-07-10*  
*Estimated Timeline: 8 days*  
*Status: Ready for Development*
