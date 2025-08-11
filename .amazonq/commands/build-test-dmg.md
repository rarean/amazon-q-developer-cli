---
description: Build test DMG files for both alpha and stable versions using local scripts
---

# Build Test DMG

Create test DMG files for Amazon Q CLI using the local build scripts. This command supports building both alpha and stable versions with proper binary naming and bundle identifiers.

## Prerequisites

Before running this command, ensure you have:
- Rust toolchain installed (`rustup`, `cargo`)
- macOS development tools (`xcode-select --install`)
- Required dependencies for DMG creation (`hdiutil`)

## Instructions

Follow these steps to build test DMG files locally:

### Step 1: Clean Previous Builds
```bash
# Remove any existing DMG files and build artifacts
rm -f Amazon-Q-CLI*.dmg Amazon-Q-CLI*.sha256
rm -rf target/universal-apple-darwin/
```

### Step 2: Build Alpha Version DMG
```bash
# Build alpha release binary
Q_CLI_ALPHA=1 cargo build --release --bin chat_cli

# Create universal binary directory structure
mkdir -p target/universal-apple-darwin/release

# Copy alpha binary to expected location
cp target/release/chat_cli target/universal-apple-darwin/release/q

# Verify alpha bundle identifier was generated
echo "=== Verifying Alpha Bundle Identifier ==="
grep -A1 "CFBundleIdentifier" crates/chat-cli/src/Info.plist

# Create alpha DMG
Q_CLI_ALPHA=1 ./scripts/github-actions/create-macos-dmg.sh

# Generate alpha checksum
shasum -a 256 Amazon-Q-CLI-Alpha-universal.dmg > Amazon-Q-CLI-Alpha-universal.dmg.sha256

echo "✅ Alpha DMG created: Amazon-Q-CLI-Alpha-universal.dmg"
```

### Step 3: Build Stable Version DMG
```bash
# Build stable release binary
cargo build --release --bin chat_cli

# Copy stable binary to expected location
cp target/release/chat_cli target/universal-apple-darwin/release/q

# Verify stable bundle identifier was generated
echo "=== Verifying Stable Bundle Identifier ==="
grep -A1 "CFBundleIdentifier" crates/chat-cli/src/Info.plist

# Create stable DMG
./scripts/github-actions/create-macos-dmg.sh

# Generate stable checksum
shasum -a 256 Amazon-Q-CLI-universal.dmg > Amazon-Q-CLI-universal.dmg.sha256

echo "✅ Stable DMG created: Amazon-Q-CLI-universal.dmg"
```

### Step 4: Verify DMG Contents
```bash
# Test mount alpha DMG and verify contents
echo "=== Testing Alpha DMG ==="
hdiutil attach Amazon-Q-CLI-Alpha-universal.dmg -mountpoint /tmp/alpha-test
ls -la /tmp/alpha-test/
/tmp/alpha-test/q-alpha --version
hdiutil detach /tmp/alpha-test

# Test mount stable DMG and verify contents
echo "=== Testing Stable DMG ==="
hdiutil attach Amazon-Q-CLI-universal.dmg -mountpoint /tmp/stable-test
ls -la /tmp/stable-test/
/tmp/stable-test/q --version
hdiutil detach /tmp/stable-test
```

### Step 5: Display Results
```bash
# Show created files
echo "=== Created DMG Files ==="
ls -la Amazon-Q-CLI*.dmg Amazon-Q-CLI*.sha256

# Display checksums
echo "=== Checksums ==="
cat Amazon-Q-CLI-Alpha-universal.dmg.sha256
cat Amazon-Q-CLI-universal.dmg.sha256

# Show file sizes
echo "=== File Sizes ==="
du -h Amazon-Q-CLI*.dmg
```

## Expected Results

After successful execution, you should have:

### Alpha Version Files:
- `Amazon-Q-CLI-Alpha-universal.dmg` - Alpha DMG package
- `Amazon-Q-CLI-Alpha-universal.dmg.sha256` - Alpha checksum
- Binary name inside DMG: `q-alpha`
- Bundle ID: `com.amazon.codewhisperer.alpha`
- Volume name: "Amazon Q CLI Alpha"

### Stable Version Files:
- `Amazon-Q-CLI-universal.dmg` - Stable DMG package  
- `Amazon-Q-CLI-universal.dmg.sha256` - Stable checksum
- Binary name inside DMG: `q`
- Bundle ID: `com.amazon.codewhisperer`
- Volume name: "Amazon Q CLI"

## Verification Steps

### Test Parallel Installation:
```bash
# Install both versions to test parallel functionality
# Alpha version will create: ~/Library/Application Support/amazon-q-alpha/
# Stable version will create: ~/Library/Application Support/amazon-q/

# Test data directory isolation
Q_CLI_ALPHA=1 ./target/universal-apple-darwin/release/q --version
./target/universal-apple-darwin/release/q --version

# Check separate data directories are created
ls ~/Library/Application\ Support/amazon-q*/
```

### Validate Bundle Identifiers:
```bash
# Extract and verify bundle identifiers from built binaries
# Alpha should show: com.amazon.codewhisperer.alpha
# Stable should show: com.amazon.codewhisperer
```

## Troubleshooting

### Common Issues:

1. **"Binary not found" error**
   - Ensure you've built the release binary first
   - Check that `target/universal-apple-darwin/release/q` exists

2. **"DMG creation failed" error**
   - Verify you have sufficient disk space
   - Check that `hdiutil` is available
   - Ensure no existing DMG is mounted

3. **"Bundle identifier not found" error**
   - Verify the build completed successfully
   - Check that `crates/chat-cli/src/Info.plist` was generated

4. **Permission errors**
   - Ensure the DMG script is executable: `chmod +x scripts/github-actions/create-macos-dmg.sh`
   - Check write permissions in the current directory

## Context

This command replicates the GitHub Actions build process locally for testing purposes. It uses the same scripts and environment variables that the CI/CD pipeline uses, ensuring consistency between local testing and production builds.

The command supports the alpha build parallel installation feature, creating properly isolated versions that can coexist on the same system without conflicts.

Use this command when:
- Testing DMG creation before pushing to CI/CD
- Verifying alpha/stable version isolation
- Creating local test packages for manual testing
- Debugging build issues locally
