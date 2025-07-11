# Architectural Decision Records (ADRs) - Custom Commands

## Overview

This document tracks all significant architectural decisions made during the implementation of the custom commands feature. Each decision is documented with context, options considered, and rationale.

---

## ADR-001: Follow Context Command Pattern

**Date**: 2025-07-06
**Status**: Accepted
**Deciders**: Implementation Team

### Context
The Amazon Q CLI already has a well-established `/context` command that manages file-based configuration with global and local scopes. We need to decide whether to follow this pattern or create a new approach for custom commands.

### Decision
We will follow the established `/context` command pattern for consistency and user familiarity.

### Rationale
- **Consistency**: Users already understand the global/local scope concept
- **Code Reuse**: Can leverage existing patterns and utilities
- **Maintainability**: Consistent architecture reduces cognitive load
- **User Experience**: Familiar interface reduces learning curve

### Consequences
- **Positive**: Faster development, consistent UX, easier maintenance
- **Negative**: Constrained by existing patterns, may limit some innovative features
- **Neutral**: Need to adapt context patterns to command-specific requirements

### Implementation Notes
- Use similar CLI structure (`/commands show`, `/commands add`, etc.)
- Follow same scope resolution (local overrides global)
- Use similar file organization patterns
- Leverage existing `Os` abstraction for file operations

---

## ADR-002: Markdown Format for Command Definitions

**Date**: 2025-07-06
**Status**: Accepted
**Deciders**: Implementation Team

### Context
We need to choose a format for storing custom command definitions. Options include JSON, YAML, TOML, or Markdown with structured content.

### Decision
Use Markdown format with structured sections for command definitions.

### Rationale
- **Human Readable**: Markdown is easy to read and edit
- **Rich Content**: Supports formatting, examples, and documentation
- **Familiar**: Developers are comfortable with Markdown
- **Extensible**: Can embed metadata in frontmatter or comments
- **Version Control Friendly**: Text-based format works well with Git

### Consequences
- **Positive**: Easy to create and edit, good documentation support
- **Negative**: Requires parsing logic, less structured than JSON/YAML
- **Neutral**: Need to define standard sections and validation rules

### Implementation Notes
- Required sections: Title (`# Command Name`), Instructions (`## Instructions`)
- Optional sections: Parameters, Examples, Context Requirements
- Metadata can be embedded in YAML frontmatter or HTML comments
- Validation ensures required sections are present

---

## ADR-003: Scope Resolution Strategy

**Date**: 2025-07-06
**Status**: Accepted
**Deciders**: Implementation Team

### Context
When commands exist in both global and local scopes with the same name, we need to determine which takes precedence and how to handle conflicts.

### Decision
Local commands override global commands with the same name, following the context command pattern.

### Rationale
- **Consistency**: Matches existing `/context` behavior
- **Project Specificity**: Local commands are more specific to current project
- **User Expectation**: Users expect local configuration to override global
- **Flexibility**: Allows project-specific customization of global commands

### Consequences
- **Positive**: Intuitive behavior, allows customization, consistent with existing patterns
- **Negative**: May hide global commands unexpectedly
- **Neutral**: Need clear indication of which command is being used

### Implementation Notes
- Display scope indicators in command listings
- Provide `--scope` flag to force specific scope
- Show warnings when local commands override global ones
- Allow users to see both versions with `--expand` flag

---

## ADR-004: Security Model for Command Execution

**Date**: 2025-07-06
**Status**: Accepted
**Deciders**: Implementation Team, Security Review

### Context
Custom commands will execute through Amazon Q, potentially with access to sensitive information. We need a security model to prevent malicious commands while maintaining functionality.

### Decision
Implement a multi-layered security approach with validation, sanitization, and user consent.

### Rationale
- **Defense in Depth**: Multiple security layers reduce risk
- **User Control**: Users can make informed decisions about command execution
- **Flexibility**: Allows legitimate use cases while blocking malicious ones
- **Auditability**: Security events are logged for review

### Consequences
- **Positive**: Reduced security risk, user awareness, audit trail
- **Negative**: Additional complexity, potential false positives
- **Neutral**: May require user interaction for some commands

### Implementation Notes
- Content validation for malicious patterns
- Size limits to prevent resource exhaustion
- User consent for potentially sensitive operations
- Audit logging for all command executions
- Security levels (Standard, Strict, Permissive)

---

## ADR-005: Caching Strategy

**Date**: 2025-07-06
**Status**: Accepted
**Deciders**: Implementation Team

### Context
Command definitions need to be loaded from disk, parsed, and validated. This could be expensive for large numbers of commands or frequent access.

### Decision
Implement in-memory caching with file system watching for invalidation.

### Rationale
- **Performance**: Avoid repeated file I/O and parsing
- **Responsiveness**: Faster command execution and listing
- **Efficiency**: Reduce resource usage for repeated operations
- **Freshness**: File watching ensures cache stays current

### Consequences
- **Positive**: Better performance, reduced I/O, faster user experience
- **Negative**: Memory usage, complexity of cache invalidation
- **Neutral**: Need to handle cache consistency and memory management

### Implementation Notes
- Cache parsed command objects in memory
- Use file system watchers to invalidate cache on changes
- Lazy loading - only cache commands when accessed
- Memory limits to prevent excessive usage
- Cache statistics for monitoring and optimization

---

## ADR-006: Integration with Chat Session

**Date**: 2025-07-06
**Status**: Accepted
**Deciders**: Implementation Team

### Context
Custom commands need to execute within the context of an Amazon Q chat session, potentially accessing conversation history and context.

### Decision
Integrate CommandManager with ChatSession, similar to ContextManager integration.

### Rationale
- **Consistency**: Follows established pattern from context management
- **Access**: Commands can access current conversation state
- **Integration**: Seamless execution within existing chat flow
- **Context**: Commands can leverage current context and history

### Consequences
- **Positive**: Rich command execution environment, consistent architecture
- **Negative**: Tight coupling with chat system, complexity of integration
- **Neutral**: Need to manage lifecycle and state properly

### Implementation Notes
- Add CommandManager field to ChatSession
- Initialize during session creation
- Integrate with existing profile system
- Handle errors gracefully without breaking chat session
- Support streaming responses for command output

---

## ADR-007: Error Handling Strategy

**Date**: 2025-07-06
**Status**: Accepted
**Deciders**: Implementation Team

### Context
The custom commands system will have many potential failure points: file I/O, parsing, validation, execution, etc. We need a consistent error handling approach.

### Decision
Use structured error types with context and recovery suggestions, following existing CLI patterns.

### Rationale
- **Consistency**: Matches existing error handling in the codebase
- **User Experience**: Clear error messages help users resolve issues
- **Debugging**: Structured errors provide context for troubleshooting
- **Recovery**: Suggestions help users fix problems

### Consequences
- **Positive**: Better user experience, easier debugging, consistent patterns
- **Negative**: More code for error handling, potential over-engineering
- **Neutral**: Need to balance detail with simplicity

### Implementation Notes
- Use `thiserror` for structured error types
- Include context and suggestions in error messages
- Graceful degradation when possible
- Log errors for debugging while showing user-friendly messages
- Recovery mechanisms where appropriate

---

## ADR-008: Testing Strategy

**Date**: 2025-07-06
**Status**: Accepted
**Deciders**: Implementation Team

### Context
The custom commands feature involves file system operations, CLI interactions, and integration with existing systems. We need a comprehensive testing strategy.

### Decision
Multi-layered testing approach with unit tests, integration tests, and end-to-end tests.

### Rationale
- **Coverage**: Different test types catch different classes of bugs
- **Confidence**: Comprehensive testing reduces risk of regressions
- **Maintainability**: Good tests make refactoring safer
- **Documentation**: Tests serve as executable documentation

### Consequences
- **Positive**: Higher quality, safer refactoring, better documentation
- **Negative**: More code to maintain, longer build times
- **Neutral**: Need to balance test coverage with development speed

### Implementation Notes
- Unit tests for all core logic and data structures
- Integration tests for file system operations and CLI interactions
- End-to-end tests for complete user workflows
- Mock external dependencies where appropriate
- Use temporary directories for file system tests
- Target >90% code coverage for Phase 1, >95% for production

---

## Decision Status Legend

- **Proposed**: Decision is suggested but not yet agreed upon
- **Accepted**: Decision is agreed upon and will be implemented
- **Deprecated**: Decision is no longer valid and should not be used
- **Superseded**: Decision has been replaced by a newer decision

## Review Process

1. **Proposal**: New decisions are proposed with context and options
2. **Discussion**: Team discusses trade-offs and implications
3. **Decision**: Team agrees on the approach to take
4. **Documentation**: Decision is recorded with rationale
5. **Implementation**: Decision is implemented in code
6. **Review**: Decisions are reviewed periodically for relevance

## ADR-009: Claude Code Integration and Syntax Compatibility

**Date**: 2025-07-06 (Updated)
**Status**: Accepted
**Deciders**: Implementation Team

### Context
After reviewing the existing `.amazonq/commands/README.md`, we discovered this feature is based on Claude Code's slash command system. We need to decide whether to maintain compatibility with Claude Code's syntax and features or create a simplified Amazon Q-specific version.

### Decision
Implement full compatibility with Claude Code's slash command syntax and feature set while maintaining Amazon Q CLI management capabilities.

### Rationale
- **User Familiarity**: Users coming from Claude Code will have zero learning curve
- **Feature Completeness**: Claude Code's system is mature and well-designed
- **Migration Path**: Easy migration of existing Claude Code commands
- **Proven Patterns**: Leverages battle-tested command execution patterns
- **Rich Functionality**: Supports advanced features like bash execution and file references

### Consequences
- **Positive**: Full feature parity, easy migration, proven UX patterns
- **Negative**: Increased complexity, more security considerations, longer development time
- **Neutral**: Need to support dual syntax (execution + management)

### Implementation Notes
- Support Claude Code execution syntax: `/project:name`, `/user:namespace:name`
- Support Amazon Q management syntax: `/commands show`, `/commands add`
- Implement all Claude Code features: arguments, bash execution, file references, tool permissions
- Use `.amazonq/commands/` instead of `.claude/commands/` for Amazon Q branding
- Maintain YAML frontmatter compatibility for metadata and tool permissions

### Updated Requirements
- **Namespace Support**: Directory-based namespacing (`frontend/component.md` → `/project:frontend:component`)
- **Argument Processing**: `$ARGUMENTS` placeholder substitution
- **Bash Execution**: `!` prefix for bash command execution with tool permissions
- **File References**: `@` prefix for file inclusion
- **Tool Permissions**: YAML frontmatter `allowed-tools` validation
- **Thinking Mode**: Extended thinking keyword support

---

*Last Updated: 2025-07-06*
*Next Review: End of Phase 1*
*Total Decisions: 9*
