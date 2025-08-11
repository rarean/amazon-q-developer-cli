---
description: Simulate GitHub Actions build workflow locally
---

# Simulate CI Build

Replicate the GitHub Actions build workflow locally to test the complete CI/CD process before pushing changes.

## Instructions

Follow these steps to simulate the exact GitHub Actions workflow:

### Step 1: Environment Setup
```bash
# Set up environment variables as in GitHub Actions
export CARGO_TERM_COLOR=always
export RUSTFLAGS="-D warnings"
export Q_CLI_ALPHA=1

echo "=== Environment Setup ==="
echo "CARGO_TERM_COLOR: $CARGO_TERM_COLOR"
echo "RUSTFLAGS: $RUSTFLAGS"
echo "Q_CLI_ALPHA: $Q_CLI_ALPHA"
```

### Step 2: Install Rust Toolchain (if needed)
```bash
# Ensure we have the required targets (simulate GitHub Actions setup)
rustup target add x86_64-apple-darwin aarch64-apple-darwin
rustup toolchain install stable
rustup default stable
```

### Step 3: Build Intel Binary (Alpha)
```bash
# Simulate: cargo build --release --target x86_64-apple-darwin --bin chat_cli
echo "=== Building Intel Binary (Alpha) ==="
Q_CLI_ALPHA=1 cargo build --release --target x86_64-apple-darwin --bin chat_cli
```

### Step 4: Build Apple Silicon Binary (Alpha)
```bash
# Simulate: cargo build --release --target aarch64-apple-darwin --bin chat_cli
echo "=== Building Apple Silicon Binary (Alpha) ==="
Q_CLI_ALPHA=1 cargo build --release --target aarch64-apple-darwin --bin chat_cli
```

### Step 5: Create Universal Binary
```bash
# Simulate GitHub Actions universal binary creation
echo "=== Creating Universal Binary ==="
mkdir -p target/universal-apple-darwin/release
lipo -create -output target/universal-apple-darwin/release/q \
  target/x86_64-apple-darwin/release/chat_cli \
  target/aarch64-apple-darwin/release/chat_cli

# Verify universal binary
file target/universal-apple-darwin/release/q
lipo -info target/universal-apple-darwin/release/q
```

### Step 6: Test Binary Execution
```bash
# Simulate GitHub Actions binary test
echo "=== Testing Binary Execution ==="
chmod +x target/universal-apple-darwin/release/q
./target/universal-apple-darwin/release/q --version
```

### Step 7: Verify Alpha Bundle Identifier
```bash
# Simulate GitHub Actions verification step
echo "=== Verifying Alpha Build Configuration ==="
if grep -q "com.amazon.codewhisperer.alpha" crates/chat-cli/src/Info.plist; then
  echo "✅ Alpha bundle identifier confirmed: com.amazon.codewhisperer.alpha"
else
  echo "❌ Alpha bundle identifier not found!"
  cat crates/chat-cli/src/Info.plist
  exit 1
fi
```

### Step 8: Create Alpha DMG Package
```bash
# Simulate GitHub Actions DMG creation
echo "=== Creating Alpha DMG Package ==="
chmod +x scripts/github-actions/create-macos-dmg.sh
Q_CLI_ALPHA=1 ./scripts/github-actions/create-macos-dmg.sh
```

### Step 9: Generate Checksums
```bash
# Simulate GitHub Actions checksum generation
echo "=== Generating Checksums ==="
shasum -a 256 target/universal-apple-darwin/release/q > q-alpha-universal-binary.sha256
shasum -a 256 Amazon-Q-CLI-Alpha-universal.dmg > Amazon-Q-CLI-Alpha-universal.dmg.sha256

echo "=== Alpha Build Checksums ==="
cat q-alpha-universal-binary.sha256
cat Amazon-Q-CLI-Alpha-universal.dmg.sha256
```

### Step 10: Display Results (Simulate Artifact Upload)
```bash
# Simulate what would be uploaded as artifacts
echo "=== Artifacts Ready for Upload ==="
ls -la target/universal-apple-darwin/release/q
ls -la Amazon-Q-CLI-Alpha-universal.dmg
ls -la q-alpha-universal-binary.sha256
ls -la Amazon-Q-CLI-Alpha-universal.dmg.sha256

echo "=== File Sizes ==="
du -h target/universal-apple-darwin/release/q
du -h Amazon-Q-CLI-Alpha-universal.dmg
```

### Step 11: Installation Instructions (Simulate Workflow Output)
```bash
echo "=== macOS Alpha Installation Instructions ==="
echo ""
echo "⚠️  ALPHA VERSION - For Testing Only"
echo "This alpha version runs in parallel with the stable version."
echo ""
echo "### Alpha Version Characteristics:"
echo "- Binary name: q-alpha"
echo "- Bundle ID: com.amazon.codewhisperer.alpha"
echo "- Data Directory: ~/Library/Application Support/amazon-q-alpha/"
echo "- Log Directory: \$TMPDIR/qlog-alpha"
echo "- Can run alongside stable version without conflicts"
echo ""
echo "### Option 1: Alpha DMG Installation (Recommended)"
echo "1. Download Amazon-Q-CLI-Alpha-universal.dmg"
echo "2. Double-click to mount the disk image"
echo "3. Drag 'q-alpha' to Applications folder"
echo "4. Right-click 'q-alpha' → 'Open' (first time only)"
echo "5. Click 'Open' when prompted about unidentified developer"
```

## Validation Steps

After running the simulation, verify:

### ✅ Expected Artifacts:
- `target/universal-apple-darwin/release/q` (universal binary)
- `Amazon-Q-CLI-Alpha-universal.dmg` (alpha DMG)
- `q-alpha-universal-binary.sha256` (binary checksum)
- `Amazon-Q-CLI-Alpha-universal.dmg.sha256` (DMG checksum)

### ✅ Binary Properties:
- Universal binary (Intel + Apple Silicon)
- Alpha bundle identifier in Info.plist
- Functional `--version` command

### ✅ DMG Properties:
- Contains `q-alpha` binary (not `q`)
- Volume name: "Amazon Q CLI Alpha"
- Proper installation instructions

## Cleanup
```bash
# Clean up simulation artifacts
rm -f Amazon-Q-CLI-Alpha-universal.dmg
rm -f q-alpha-universal-binary.sha256
rm -f Amazon-Q-CLI-Alpha-universal.dmg.sha256
rm -rf target/universal-apple-darwin/
```

## Context

This command exactly replicates the GitHub Actions workflow defined in `.github/workflows/build-macos.yml`, allowing you to test the complete CI/CD process locally before pushing changes to the repository.

Use this when:
- Testing workflow changes before committing
- Debugging CI/CD issues locally
- Verifying the complete build process
- Ensuring artifacts match CI/CD expectations
