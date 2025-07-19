# User-Scoped Commands Implementation Plan

- [ ] 1. Enhance CommandScope enum and core data structures
  - Extend existing `CommandScope` enum to include `User` variant
  - Add directory path resolution methods for user scope
  - Update serialization and display implementations
  - Add unit tests for scope resolution and path handling
  - _Requirements: 1.1, 5.1_

- [ ] 2. Implement user command directory management
  - Create user commands directory (`~/.amazonq/commands/`) if it doesn't exist
  - Handle cross-platform home directory resolution
  - Implement proper file permissions for user command files
  - Add error handling for inaccessible home directories
  - _Requirements: 1.1, 1.2, 5.1, 5.2_

- [ ] 3. Extend command parser to support `/user:` syntax
  - Modify existing command parser to recognize `/user:command-name` pattern
  - Add namespace parsing for user commands (e.g., `/user:git:helper`)
  - Update command routing logic to handle user scope
  - Ensure backward compatibility with existing `/project:` syntax
  - _Requirements: 2.1, 2.2, 4.1_

- [ ] 4. Implement namespace resolution for user commands
  - Create namespace resolver to extract namespaces from directory structure
  - Support multi-level namespaces (e.g., `frontend/components/button.md` → `frontend:components:button`)
  - Add namespace validation to prevent invalid characters
  - Handle empty directories during command discovery
  - _Requirements: 4.1, 4.2, 4.3, 4.4_

- [ ] 5. Enhance command manager for dual-scope support
  - Create `DualScopeCommandManager` to handle both user and project commands
  - Implement command resolution with proper scope priority (project over user)
  - Add command listing functionality that includes scope indicators
  - Update caching strategy to handle both scopes efficiently
  - _Requirements: 1.4, 3.3_

- [ ] 6. Update CLI management commands for user scope
  - Extend `/commands add` to support `--scope user` parameter
  - Update `/commands show` to support scope filtering with `--scope user`
  - Implement `/commands remove` with user scope support
  - Add scope indicators in command listings (user/project badges)
  - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [ ] 7. Implement user command execution flow
  - Integrate user command loading with existing execution engine
  - Add proper execution indicators ("🚀 Executing user command: name")
  - Implement error handling for missing user commands
  - Ensure consistent error messages across user and project scopes
  - _Requirements: 2.2, 2.3, 2.4_

- [ ] 8. Add comprehensive error handling for user commands
  - Implement specific error messages for user command scenarios
  - Add graceful degradation when home directory is inaccessible
  - Handle namespace validation errors with helpful messages
  - Ensure error message consistency with existing project command errors
  - _Requirements: 2.3, 5.4_

- [ ] 9. Create integration tests for user-scoped commands
  - Test complete user command creation and execution flow
  - Verify cross-platform compatibility for home directory resolution
  - Test namespace resolution with various directory structures
  - Validate command scope priority (project over user)
  - _Requirements: All requirements validation_

- [ ] 10. Update documentation and help text
  - Add user scope examples to command help text
  - Update CLI documentation with new `--scope` parameter usage
  - Create user guide for organizing commands with namespaces
  - Ensure all error messages provide clear guidance for users
  - _Requirements: User experience and discoverability_