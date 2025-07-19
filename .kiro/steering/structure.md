# Project Structure

## Workspace Organization
This is a Cargo workspace with multiple crates organized under the `crates/` directory. The main binary is `chat_cli` which provides the `q` command-line interface.

## Top-Level Directories

### `/crates/` - Rust Crates
- **`chat-cli/`** - Main CLI application (`q` command)
  - Primary binary: `chat_cli`
  - Test binary: `test_mcp_server` for MCP testing
  - Contains: API clients, auth, database, CLI commands, MCP client, telemetry
- **`amzn-codewhisperer-client/`** - CodeWhisperer API client (standard)
- **`amzn-codewhisperer-streaming-client/`** - CodeWhisperer streaming API client
- **`amzn-consolas-client/`** - Amazon Consolas service client
- **`amzn-qdeveloper-streaming-client/`** - Q Developer streaming client
- **`amzn-toolkit-telemetry-client/`** - Telemetry service client
- **`aws-toolkit-telemetry-definitions/`** - Telemetry schema definitions
- **`semantic-search-client/`** - Semantic search and embedding functionality

### `/scripts/` - Build & Operations
- Python-based build scripts and utilities
- Platform-specific build scripts (e.g., `build-macos.sh`)
- Release verification scripts

### `/docs/` - Documentation
- Technical documentation using mdBook
- Agent format specifications
- Native tools documentation

### `/.amazonq/` - Amazon Q Integration
- Command definitions and examples
- Implementation plans and architecture docs
- MVP specifications and progress tracking

### `/.kiro/` - Kiro IDE Configuration
- Steering rules for AI assistance
- Specifications and requirements

## Key Configuration Files

### Build & Development
- `Cargo.toml` - Workspace configuration and dependencies
- `rust-toolchain.toml` - Rust version and target specifications
- `.rustfmt.toml` - Code formatting rules (120 char width, Rust 2024 style)
- `.mise.toml` - Development environment (Node.js 22, Python 3.11)

### Quality & CI
- `.typos.toml` - Spell checking configuration
- `deny.toml` - Dependency security and licensing checks
- `.lintstagedrc.mjs` - Pre-commit linting rules
- `.husky/pre-commit` - Git hooks

### Cross-Platform
- `Cross.toml` - Cross-compilation configuration
- Platform-specific build configurations in `build-config/`

## Code Organization Patterns

### Client Crates Structure
Each AWS service client follows a consistent pattern:
```
src/
├── client/           # API operation implementations
├── config/           # Client configuration
├── error/            # Error types and handling
├── operation/        # Individual API operations
├── protocol_serde/   # Serialization/deserialization
├── types/            # Data types and models
└── lib.rs           # Public API exports
```

### Main CLI Structure (`chat-cli/src/`)
```
src/
├── api_client/       # AWS service integrations
├── auth/             # Authentication handling
├── aws_common/       # Common AWS utilities
├── cli/              # Command-line interface
├── database/         # SQLite database operations
├── mcp_client/       # Model Context Protocol client
├── os/               # Operating system integrations
├── telemetry/        # Usage analytics
├── util/             # Common utilities
└── main.rs          # Application entry point
```

## Naming Conventions
- Crate names use kebab-case with `amzn-` prefix for AWS services
- Binary names use snake_case (`chat_cli`)
- Module organization follows Rust conventions
- AWS service clients maintain consistent API patterns