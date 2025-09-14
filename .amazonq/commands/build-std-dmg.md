---
description: Build standard DMG file for Amazon Q CLI using local scripts
---

# Build Standard DMG

Create standard DMG file for Amazon Q CLI using the local build scripts.

## Prerequisites

Before running this command, ensure you have:
- Rust toolchain installed (`rustup`, `cargo`)
- macOS development tools (`xcode-select --install`)
- Required dependencies for DMG creation (`hdiutil`)

## Instructions

Follow these steps to build standard DMG file locally:

### Step 1: Clean Previous Builds
```bash
# Remove any existing DMG files and build artifacts
rm -f Amazon-Q-CLI*.dmg Amazon-Q-CLI*.sha256
rm -r target/universal-apple-darwin/*.*
```
* remove all remaining debug, build, release and tmp files in target dir

### Step 2: Build Standard Version DMG
```bash
# Build standard release binary
cargo build --release --bin chat_cli

# Create universal binary directory structure
mkdir -p target/universal-apple-darwin/release

# Copy standard binary to expected location
cp target/release/chat_cli target/universal-apple-darwin/release/q

# Verify standard bundle identifier was generated
echo "=== Verifying Standard Bundle Identifier ==="
grep -A1 "CFBundleIdentifier" crates/chat-cli/src/Info.plist

# Create standard DMG manually
mkdir -p /tmp/standard-dmg-contents
cp target/universal-apple-darwin/release/q /tmp/standard-dmg-contents/q
hdiutil create -volname "Amazon Q CLI" -srcfolder /tmp/standard-dmg-contents -ov -format UDZO Amazon-Q-CLI-universal.dmg
rm -r /tmp/*

# Generate standard checksum
shasum -a 256 Amazon-Q-CLI-universal.dmg > Amazon-Q-CLI-universal.dmg.sha256

echo "✅ Standard DMG created: Amazon-Q-CLI-universal.dmg"
```

### Step 3: Verify DMG Contents
```bash
# Test mount standard DMG and verify contents
echo "=== Testing Standard DMG ==="
hdiutil attach Amazon-Q-CLI-universal.dmg -mountpoint /tmp/standard-test
ls -la /tmp/standard-test/
/tmp/standard-test/q --version
hdiutil detach /tmp/standard-test
```

### Step 4: Display Results
```bash
# Show created files
echo "=== Created DMG Files ==="
ls -la Amazon-Q-CLI*.dmg Amazon-Q-CLI*.sha256

# Display checksums
echo "=== Checksums ==="
cat Amazon-Q-CLI-universal.dmg.sha256

# Show file sizes
echo "=== File Sizes ==="
du -h Amazon-Q-CLI*.dmg
```

## Expected Results

After successful execution, you should have:

### Standard Version Files:
- `Amazon-Q-CLI-universal.dmg` - Standard DMG package
- `Amazon-Q-CLI-universal.dmg.sha256` - Standard checksum
- Binary name inside DMG: `q`
- Bundle ID: `com.amazon.codewhisperer`
- Volume name: "Amazon Q CLI"

## Verification Steps

### Validate Bundle Identifiers:
```bash
# Extract and verify bundle identifiers from built binaries
# Standard should show: com.amazon.codewhisperer
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


## Context

This command replicates the GitHub Actions build process locally for testing purposes. It uses the same scripts and environment variables that the CI/CD pipeline uses, ensuring consistency between local testing and production builds.

Use this command when:
- Testing DMG creation before pushing to CI/CD
- Creating local test packages for manual testing
- Debugging build issues locally
