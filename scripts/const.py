import pathlib
import os


APP_NAME = "Amazon Q"
CLI_BINARY_NAME = "q"
CHAT_BINARY_NAME = "qchat"
PTY_BINARY_NAME = "qterm"
DESKTOP_BINARY_NAME = "q-desktop"
URL_SCHEMA = "q"
TAURI_PRODUCT_NAME = "q_desktop"
LINUX_PACKAGE_NAME = "amazon-q"

# macos specific
def get_bundle_id():
    """Get bundle ID based on alpha environment variable"""
    if os.environ.get("Q_CLI_ALPHA"):
        return "com.amazon.codewhisperer.alpha"
    return "com.amazon.codewhisperer"

def get_dmg_name():
    """Get DMG name based on alpha environment variable"""
    if os.environ.get("Q_CLI_ALPHA"):
        return "Amazon Q Alpha"
    return APP_NAME

MACOS_BUNDLE_ID = get_bundle_id()
DMG_NAME = get_dmg_name()

# Linux specific
LINUX_ARCHIVE_NAME = "q"
LINUX_LEGACY_GNOME_EXTENSION_UUID = "amazon-q-for-cli-legacy-gnome-integration@aws.amazon.com"
LINUX_MODERN_GNOME_EXTENSION_UUID = "amazon-q-for-cli-gnome-integration@aws.amazon.com"

# cargo packages
CLI_PACKAGE_NAME = "q_cli"
CHAT_PACKAGE_NAME = "chat_cli"
PTY_PACKAGE_NAME = "figterm"
DESKTOP_PACKAGE_NAME = "fig_desktop"
DESKTOP_FUZZ_PACKAGE_NAME = "fig_desktop-fuzz"

DESKTOP_PACKAGE_PATH = pathlib.Path("crates", "fig_desktop")

# AMZN Mobile LLC
APPLE_TEAM_ID = "94KV3E626L"
