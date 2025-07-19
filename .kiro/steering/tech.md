# Technology Stack

## Build System & Language
- **Primary Language**: Rust (version 1.87.0)
- **Build System**: Cargo workspace with multiple crates
- **Edition**: Rust 2024
- **Toolchain**: Stable channel with rustfmt and clippy components

## Key Dependencies & Frameworks
- **Async Runtime**: Tokio (full features)
- **CLI Framework**: Clap v4 with derive, completion support
- **HTTP Client**: Reqwest with rustls-tls, HTTP/2, JSON support
- **Database**: SQLite via rusqlite with bundled driver
- **Authentication**: AWS SDK (cognito-identity, sso-oidc)
- **Terminal UI**: Crossterm, dialoguer, rustyline for interactive features
- **Serialization**: Serde with JSON support
- **Error Handling**: Eyre, thiserror, color-eyre
- **Logging**: Tracing with subscriber and appender
- **Testing**: Insta for snapshot testing, criterion for benchmarks

## AWS Service Clients
- Amazon CodeWhisperer (streaming and standard)
- Amazon Consolas
- Amazon Q Developer Streaming
- AWS Toolkit Telemetry

## Development Tools
- **Formatter**: rustfmt with custom configuration (120 char width)
- **Linter**: Clippy with extensive warning rules
- **Type Checker**: typos-cli for spell checking
- **Package Manager**: Mise for Node.js (v22) and Python (v3.11)

## Common Commands

### Building
```bash
# Compile and run the main CLI
cargo run --bin chat_cli

# Build release version
cargo build --release

# Run with subcommands
cargo run --bin chat_cli -- login
cargo run --bin chat_cli -- {subcommand}
```

### Testing & Quality
```bash
# Run all tests
cargo test

# Run lints
cargo clippy

# Format code (requires nightly)
cargo +nightly fmt

# Check spelling
typos

# Run benchmarks
cargo bench
```

### Development Setup
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup toolchain install nightly
cargo install typos-cli
```

## Cross-Platform Targets
- x86_64-apple-darwin (Intel Mac)
- aarch64-apple-darwin (Apple Silicon)
- x86_64-unknown-linux-gnu (Linux)
- x86_64-pc-windows-msvc (Windows)
- wasm32-wasip1 (WebAssembly)

## Platform-Specific Dependencies
- **macOS**: Security Framework, Objective-C bindings for system integration
- **Linux**: Nix for system calls, skim for fuzzy selection
- **Windows**: Windows API bindings, registry access