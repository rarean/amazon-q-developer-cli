---
description: Quick test DMG build for alpha version only
---

# Quick DMG Test

Rapidly build and test an alpha DMG file for development purposes.

## Instructions

Execute these commands in sequence to quickly create and test an alpha DMG:

### Step 1: Build Alpha DMG
```bash
# Clean and build alpha version
rm -f Amazon-Q-CLI-Alpha*.dmg Amazon-Q-CLI-Alpha*.sha256
Q_CLI_ALPHA=1 cargo build --release --bin chat_cli
mkdir -p target/universal-apple-darwin/release
cp target/release/chat_cli target/universal-apple-darwin/release/q
Q_CLI_ALPHA=1 ./scripts/github-actions/create-macos-dmg.sh
```

### Step 2: Quick Verification
```bash
# Generate checksum and verify
shasum -a 256 Amazon-Q-CLI-Alpha-universal.dmg > Amazon-Q-CLI-Alpha-universal.dmg.sha256
echo "=== Alpha DMG Created ==="
ls -la Amazon-Q-CLI-Alpha-universal.dmg
cat Amazon-Q-CLI-Alpha-universal.dmg.sha256
```

### Step 3: Test Mount (Optional)
```bash
# Quick mount test
hdiutil attach Amazon-Q-CLI-Alpha-universal.dmg -mountpoint /tmp/quick-test
ls -la /tmp/quick-test/ | grep q-alpha
/tmp/quick-test/q-alpha --version
hdiutil detach /tmp/quick-test
```

## Expected Output

- ✅ `Amazon-Q-CLI-Alpha-universal.dmg` created
- ✅ Contains `q-alpha` binary
- ✅ Bundle ID: `com.amazon.codewhisperer.alpha`
- ✅ Volume: "Amazon Q CLI Alpha"

## Context

Use this command for rapid iteration during development when you only need to test the alpha version DMG creation process.
