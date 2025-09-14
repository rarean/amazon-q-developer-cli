# File Operations
when adding new files or updating existing files make small diff changes/addions.
API interaction needs to be kept small for problematic network connectivity and
potential loss of connection during data exchange between local client and
remote LLM.

Network outages are common reasons for duplicate content and
malformed document syntax. please read full file when making changes to avoid
write and compile errors.

## Patching
when updating modifying or patching files, use line numbers of actual changes
for fs_write operations. this reduces errors and provides better diff views of changes being made.

### Bad example
```bash
🛠️  Using tool: fs_write
 ⋮
 ● Path: crates/acp/Cargo.toml

- 0   : uuid = { version = "1.0", features = ["v4"] }
+    0: bincode = "1.3"
+    1: rusqlite = "0.29"
```

### Good example
```bash
🛠️  Using tool: fs_write
 ⋮
 ● Path: crates/acp/Cargo.toml

  14, 14: agent-client-protocol = "0.1.0"
  15, 15: tokio = { version = "1.0", features = ["full"] }
  16, 16: thiserror = "1.0"
  17, 17: serde_json = "1.0"
  18, 18: rmcp.workspace = true
  19, 19: serde = { version = "1.0", features = ["derive"] }
- 20    : dirs = "5.0"
+     20: dirs = "5.0"
+     21: bincode = "1.3"
+     22: rusqlite = "0.29"
```

## ACP Implementation Guidelines

### CLI Command Structure
- All ACP subcommands must be defined in the `AcpSubcommand` enum
- Each subcommand should have proper documentation and argument definitions
- Match arms in the execute function must handle all enum variants
- Avoid duplicate `execute` method implementations

### Functional Implementation Requirements
- Replace "not yet implemented" placeholder messages with actual functionality
- Use the ACP crate's capabilities (agent, session_manager, file_ops, etc.)
- Implement real server state management, not just println statements
- Validate authentication and server state before operations
- Return meaningful errors when operations cannot be performed

### Compilation Best Practices
- Always build and test after making changes to ensure compilation succeeds
- Fix all compiler errors and warnings before considering implementation complete
- Use proper imports and avoid unused dependencies
- Handle all enum variants in match statements
- Use appropriate error handling with Result types

### Code Quality
- Implement minimal but functional code that actually performs the intended operations
- Use the existing ACP crate modules rather than creating placeholder functionality
- Maintain consistency with existing CLI patterns and error handling
- Provide helpful error messages and user feedback
