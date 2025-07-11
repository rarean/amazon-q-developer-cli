# Custom Commands MVP Documentation

## Overview

This directory contains the complete specification and implementation guide for the Custom Commands MVP using arc42 methodology. The MVP focuses on delivering core value with minimal complexity.

## MVP Scope

### ✅ Core Features (MVP)
1. **`/commands add <name>`** - Create custom commands with editor integration
2. **`/project:<name>`** - Execute project-scoped custom commands
3. **Settings Integration** - Feature gated behind `chat.enableCommands`
4. **UX Consistency** - Exact patterns matching knowledge base feature

### ❌ Excluded from MVP
- User-scoped commands (`~/.amazonq/commands/`)
- YAML frontmatter and metadata
- Bash execution (`!` prefix) and file references (`@` prefix)
- Argument substitution (`$ARGUMENTS`)
- Advanced validation and security features
- Background operations and progress tracking

## Document Structure

### 📋 [arc42-mvp-specification.md](arc42-mvp-specification.md)
**Complete arc42 architecture documentation**
- System scope and context
- Solution strategy and building blocks
- Runtime views and deployment
- Quality requirements and risks
- Technical decisions and constraints

### 🛠️ [mvp-implementation-guide.md](mvp-implementation-guide.md)
**Detailed implementation instructions**
- Step-by-step code changes
- File-by-file implementation tasks
- Testing strategy and validation
- 8-day development timeline

### 🚀 [quick-start-guide.md](quick-start-guide.md)
**Developer quick reference**
- Prerequisites and setup
- Implementation phases overview
- Code review checklist
- Common pitfalls and solutions

## Key Architectural Decisions

### ADR-MVP-001: Project Scope Only
- **Decision**: MVP only supports `.amazonq/commands/` (project scope)
- **Rationale**: Reduces complexity, focuses on core value
- **Impact**: Simpler implementation, faster delivery

### ADR-MVP-002: No YAML Frontmatter
- **Decision**: Plain Markdown files without metadata
- **Rationale**: Eliminates parsing complexity for MVP
- **Impact**: No tool permissions or advanced features

### ADR-MVP-003: Knowledge Base Pattern Consistency
- **Decision**: Follow exact knowledge base UX and implementation patterns
- **Rationale**: Consistent user experience, code reuse, faster development
- **Impact**: Reduced development time, familiar interface

## Implementation Timeline

```
Week 1: MVP Development (8 days)
├─ Days 1-2: Foundation (settings, data structures, core logic)
├─ Days 3-4: CLI Integration (subcommands, tool implementation)
├─ Days 5-6: System Integration (tool registration, wiring)
└─ Days 7-8: Testing & Polish (unit tests, integration tests, docs)
```

## Success Metrics

### Functional Requirements
- [ ] Feature can be enabled/disabled via `q settings chat.enableCommands`
- [ ] `/commands add name` creates command file and opens editor
- [ ] `/project:name` executes command content in chat session
- [ ] Error handling for all failure scenarios

### Quality Requirements
- [ ] >90% test coverage for new code
- [ ] Error messages match knowledge base patterns exactly
- [ ] Visual indicators consistent with knowledge base
- [ ] Performance: <2s for command creation, <1s for execution

### Consistency Requirements
- [ ] Settings integration follows exact knowledge base pattern
- [ ] Tool registration uses same pattern as knowledge base
- [ ] CLI structure mirrors knowledge base subcommands
- [ ] Error messages use identical templates and tone

## Development Workflow

### 1. Setup
```bash
# Ensure Amazon Q CLI builds successfully
cargo build -p chat_cli

# Run existing tests to establish baseline
cargo test -p chat_cli
```

### 2. Implementation
Follow the implementation guide phase by phase:
1. **Foundation** - Core data structures and settings
2. **CLI Integration** - User interfaces and commands
3. **System Integration** - Wire components together
4. **Testing & Polish** - Quality assurance and documentation

### 3. Validation
```bash
# Manual testing workflow
q settings chat.enableCommands true
/commands add test-command
/project:test-command

# Automated testing
cargo test -p chat_cli command_manager
cargo test -p chat_cli commands_integration
```

### 4. Code Review
Use the checklist in [quick-start-guide.md](quick-start-guide.md) to ensure:
- Pattern consistency with knowledge base
- Proper error handling and user feedback
- Complete test coverage
- Documentation updates

## File Organization

```
.amazonq/implementation-plan/mvp/
├── README.md                      # This file - MVP overview
├── arc42-mvp-specification.md     # Complete architecture documentation
├── mvp-implementation-guide.md    # Detailed implementation steps
└── quick-start-guide.md           # Developer quick reference
```

## Integration with Full Implementation

This MVP serves as the foundation for the full custom commands implementation:

### MVP → Phase 1 Migration
- Add user-scoped commands (`~/.amazonq/commands/`)
- Implement YAML frontmatter parsing
- Add command metadata and validation

### MVP → Phase 2 Migration  
- Add bash execution and file references
- Implement argument substitution
- Add advanced security features

### MVP → Phase 3 Migration
- Add usage analytics and recommendations
- Implement advanced templates
- Add migration tools and enhanced UX

## Quality Assurance

### Testing Strategy
- **Unit Tests**: All core components (command manager, data structures)
- **Integration Tests**: End-to-end workflows (add → execute)
- **Manual Testing**: User experience validation
- **Consistency Tests**: Pattern matching with knowledge base

### Code Review Focus
- **Pattern Consistency**: Exact match with knowledge base patterns
- **Error Handling**: Comprehensive and user-friendly
- **Security**: Proper input validation and file operations
- **Performance**: Acceptable response times for all operations

## Getting Started

1. **Read the Architecture**: Start with [arc42-mvp-specification.md](arc42-mvp-specification.md)
2. **Follow Implementation Guide**: Use [mvp-implementation-guide.md](mvp-implementation-guide.md)
3. **Use Quick Reference**: Keep [quick-start-guide.md](quick-start-guide.md) handy
4. **Test Continuously**: Run tests after each phase
5. **Validate Consistency**: Check patterns match knowledge base exactly

---

*MVP Documentation Version: 1.0*  
*Created: 2025-07-10*  
*Status: Ready for Implementation*  
*Estimated Effort: 8 days*
