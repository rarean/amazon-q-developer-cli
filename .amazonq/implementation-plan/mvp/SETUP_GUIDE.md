# Custom Commands MVP - Development Setup Guide

## Prerequisites

Before starting the implementation, you need to set up your development environment.

### System Requirements

**For macOS:**
- Xcode 13 or later
- Homebrew

**For Linux (Ubuntu/Debian):**
- Build tools and system libraries

**For Windows:**
- WSL2 recommended (follow Windows setup in README)

## Step 1: Clone the Repository

```bash
git clone https://github.com/aws/amazon-q-developer-cli.git
cd amazon-q-developer-cli
```

## Step 2: Install Dependencies

### Option A: Automated Setup (Recommended)
```bash
npm run setup
```

### Option B: Manual Setup

#### 1. Install Platform Dependencies

**For macOS:**
```bash
# Xcode command line tools (if not already installed)
xcode-select --install

# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

**For Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential pkg-config jq dpkg curl wget cmake clang \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
  libdbus-1-dev libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev \
  valac libibus-1.0-dev libglib2.0-dev sqlite3 libxdo-dev protobuf-compiler
```

#### 2. Install Rust Toolchain

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Set stable as default
rustup default stable

# Install nightly for formatting
rustup toolchain install nightly

# Install additional tools
cargo install typos-cli
```

**For macOS, add targets:**
```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

#### 3. Install mise (for Python/Node management)

```bash
# Install mise
curl https://mise.run | sh

# Add to your shell profile
# For zsh:
echo 'eval "$(mise activate zsh)"' >> "${ZDOTDIR-$HOME}/.zshrc"
source ~/.zshrc

# For bash:
echo 'eval "$(mise activate bash)"' >> ~/.bashrc
source ~/.bashrc

# Install Python and Node
mise trust
mise install
```

#### 4. Install Node Dependencies

```bash
pnpm install --ignore-scripts
```

## Step 3: Verify Setup

### Test the Build
```bash
# Build the project
cargo build --workspace

# Run tests to ensure everything works
cargo test -p chat_cli

# Test the CLI
cargo run --bin chat_cli -- --help
```

### Test Q Chat
```bash
# Start a chat session
cargo run --bin chat_cli -- chat

# In the chat, try:
/help
```

## Step 4: Development Tools Setup

### Code Formatting
```bash
# Format Rust code
cargo +nightly fmt

# Check for issues
cargo clippy --locked --workspace --color always -- -D warnings
```

### Pre-commit Hooks
The pnpm install should have set up pre-commit hooks automatically.

## Step 5: Familiarize with Codebase

### Add Project Context to Q Chat
```bash
# Start Q chat
cargo run --bin chat_cli -- chat

# Add codebase context
/context add codebase-summary.md

# Ask questions like:
# "What does this crate do?"
# "Where is the knowledge base implemented?"
# "How do tools work in this codebase?"
```

### Explore Key Files
```bash
# Look at the knowledge base implementation (our reference pattern)
ls -la crates/chat-cli/src/cli/chat/tools/knowledge.rs
ls -la crates/chat-cli/src/cli/chat/cli/knowledge.rs
ls -la crates/chat-cli/src/util/knowledge_store.rs

# Check settings implementation
ls -la crates/chat-cli/src/database/settings.rs

# Look at tool registration
ls -la crates/chat-cli/src/cli/chat/tools/mod.rs
ls -la crates/chat-cli/src/cli/chat/tool_manager.rs
```

## Troubleshooting

### Common Issues

**Rust not found after installation:**
```bash
source ~/.cargo/env
# or restart your terminal
```

**Build failures on macOS:**
```bash
# Make sure Xcode command line tools are installed
xcode-select --install

# Update Rust
rustup update
```

**Build failures on Linux:**
```bash
# Make sure all system dependencies are installed
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

**mise not working:**
```bash
# Make sure it's in your PATH
export PATH="$HOME/.local/bin:$PATH"

# Reload shell configuration
source ~/.bashrc  # or ~/.zshrc
```

### Verification Commands

```bash
# Check Rust installation
rustc --version
cargo --version

# Check required tools
typos --version
mise --version

# Check project builds
cargo check --workspace

# Check tests pass
cargo test -p chat_cli --lib
```

## Next Steps

Once your environment is set up:

1. **Read the MVP Documentation**:
   - [arc42-mvp-specification.md](arc42-mvp-specification.md)
   - [mvp-implementation-guide.md](mvp-implementation-guide.md)

2. **Start Implementation**:
   - Begin with Phase 1: Foundation
   - Follow the step-by-step guide

3. **Test Continuously**:
   ```bash
   # Run tests after each change
   cargo test -p chat_cli
   
   # Test your changes
   cargo run --bin chat_cli -- chat
   ```

## Development Workflow

```bash
# 1. Make changes
# 2. Format code
cargo +nightly fmt

# 3. Check for issues
cargo clippy --locked --workspace --color always -- -D warnings

# 4. Run tests
cargo test -p chat_cli

# 5. Test manually
cargo run --bin chat_cli -- chat
```

---

*Setup Guide Version: 1.0*  
*Created: 2025-07-10*  
*Status: Ready for Use*
