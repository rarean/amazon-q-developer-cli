#!/bin/bash

# Create macOS DMG for Amazon Q CLI
# This script creates an unsigned DMG package for distribution

set -euo pipefail

# Configuration
BINARY_PATH="${1:-target/universal-apple-darwin/release/q}"

# Check if this is an alpha build
if [[ "${Q_CLI_ALPHA:-}" == "1" ]]; then
    DMG_NAME="${2:-Amazon-Q-CLI-Alpha-universal.dmg}"
    VOLUME_NAME="Amazon Q CLI Alpha"
    APP_NAME="q-alpha"
else
    DMG_NAME="${2:-Amazon-Q-CLI-universal.dmg}"
    VOLUME_NAME="Amazon Q CLI"
    APP_NAME="q"
fi

# Validate inputs
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

echo "Creating DMG package..."
echo "Binary: $BINARY_PATH"
echo "DMG: $DMG_NAME"
echo "Volume: $VOLUME_NAME"

# Create temporary directory for DMG contents
DMG_TEMP_DIR=$(mktemp -d)
echo "Working directory: $DMG_TEMP_DIR"

# Copy binary to DMG contents
cp "$BINARY_PATH" "$DMG_TEMP_DIR/$APP_NAME"
chmod +x "$DMG_TEMP_DIR/$APP_NAME"

# Create Applications symlink for drag-and-drop installation
ln -s /Applications "$DMG_TEMP_DIR/Applications"

# Copy documentation files if they exist
if [[ -f "README.md" ]]; then
    cp README.md "$DMG_TEMP_DIR/"
fi

if [[ -f "LICENSE.MIT" ]]; then
    cp LICENSE.MIT "$DMG_TEMP_DIR/"
fi

if [[ -f "LICENSE.APACHE" ]]; then
    cp LICENSE.APACHE "$DMG_TEMP_DIR/"
fi

# Create installation instructions
if [[ "${Q_CLI_ALPHA:-}" == "1" ]]; then
cat > "$DMG_TEMP_DIR/INSTALL.txt" << EOF
Amazon Q CLI Alpha Installation Instructions

⚠️  ALPHA VERSION - For Testing Only
This alpha version runs in parallel with the stable version.

1. Drag the 'q-alpha' executable to your Applications folder (or any location in your PATH)
2. Open Terminal
3. If you copied to Applications, add to PATH:
   echo 'export PATH="/Applications:\$PATH"' >> ~/.zshrc
   source ~/.zshrc
4. Run: q-alpha --version

Alpha Version Characteristics:
- Binary name: q-alpha
- Bundle ID: com.amazon.codewhisperer.alpha
- Data directory: ~/Library/Application Support/amazon-q-alpha/
- Can run alongside stable version without conflicts

Note: This is an unsigned application. On first run:
- Right-click the 'q-alpha' executable and select "Open"
- Click "Open" when prompted about unidentified developer
- Alternatively, run: xattr -d com.apple.quarantine /path/to/q-alpha

For more information, visit:
https://github.com/aws/amazon-q-developer-cli
EOF
else
cat > "$DMG_TEMP_DIR/INSTALL.txt" << 'EOF'
Amazon Q CLI Installation Instructions

1. Drag the 'q' executable to your Applications folder (or any location in your PATH)
2. Open Terminal
3. If you copied to Applications, add to PATH:
   echo 'export PATH="/Applications:$PATH"' >> ~/.zshrc
   source ~/.zshrc
4. Run: q --version

Note: This is an unsigned application. On first run:
- Right-click the 'q' executable and select "Open"
- Click "Open" when prompted about unidentified developer
- Alternatively, run: xattr -d com.apple.quarantine /path/to/q

For more information, visit:
https://github.com/aws/amazon-q-developer-cli
EOF
fi

# Remove any existing DMG
rm -f "$DMG_NAME"

echo "Creating DMG with hdiutil..."

# Calculate size needed (binary size + some overhead)
BINARY_SIZE=$(stat -f%z "$BINARY_PATH")
DMG_SIZE_MB=$(( (BINARY_SIZE / 1024 / 1024) + 10 ))  # Add 10MB overhead

echo "Binary size: $BINARY_SIZE bytes"
echo "DMG size: ${DMG_SIZE_MB}MB"

# Create DMG using hdiutil with specified size
hdiutil create \
    -volname "$VOLUME_NAME" \
    -srcfolder "$DMG_TEMP_DIR" \
    -ov \
    -format UDZO \
    -imagekey zlib-level=9 \
    -size "${DMG_SIZE_MB}m" \
    "$DMG_NAME"

# Verify DMG was created
if [[ -f "$DMG_NAME" ]]; then
    echo "✅ DMG created successfully: $DMG_NAME"
    echo "Size: $(du -h "$DMG_NAME" | cut -f1)"
    
    # Test mounting the DMG
    echo "Testing DMG mount..."
    MOUNT_POINT=$(mktemp -d)
    hdiutil attach "$DMG_NAME" -mountpoint "$MOUNT_POINT" -quiet
    
    if [[ -f "$MOUNT_POINT/$APP_NAME" ]]; then
        echo "✅ DMG contents verified"
        # Test the binary
        if "$MOUNT_POINT/$APP_NAME" --version >/dev/null 2>&1; then
            echo "✅ Binary in DMG is functional"
        else
            echo "⚠️  Binary in DMG may have issues"
        fi
    else
        echo "❌ DMG contents missing"
        exit 1
    fi
    
    # Unmount
    hdiutil detach "$MOUNT_POINT" -quiet
    rmdir "$MOUNT_POINT"
else
    echo "❌ Failed to create DMG"
    exit 1
fi

# Clean up
rm -rf "$DMG_TEMP_DIR"

echo "✅ DMG creation complete!"
echo ""
echo "To install:"
echo "1. Double-click $DMG_NAME"
echo "2. Drag 'q' to Applications folder"
echo "3. Right-click 'q' in Applications → Open (first time only)"
echo ""
echo "To distribute:"
echo "- Upload $DMG_NAME to GitHub Releases"
echo "- Include installation instructions"
echo "- Generate SHA256 checksum for verification"
