#!/bin/bash

set -e # Exit on error

# Print colored output
info() {
    printf "\033[36m%s\033[0m\n" "$1"
}

success() {
    printf "\033[32m%s\033[0m\n" "$1"
}

error() {
    printf "\033[31m%s\033[0m\n" "$1"
    exit 1
}

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Verify supported platform
if [ "$OS" != "darwin" ]; then
    error "❌ This installer only supports macOS currently"
fi

# Detect architecture
if [ "$ARCH" != "arm64" ] && [ "$ARCH" != "x86_64" ]; then
    error "❌ Unsupported architecture $ARCH"
fi

# Set install location
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="gyst"
BINARY_PATH="$INSTALL_DIR/$BINARY_NAME"

# Create install directory if needed
if [ ! -d "$INSTALL_DIR" ]; then
    info "📁 Creating install directory..."
    sudo mkdir -p "$INSTALL_DIR"
fi

# Set binary name based on architecture
BINARY="gyst-darwin-${ARCH}"

# GitHub repository information
REPO="created-by-varun/gyst"
LATEST_RELEASE_TAG="v0.1.2"

# Download URLs
BINARY_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE_TAG/$BINARY"
CHECKSUM_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE_TAG/$BINARY.sha256"

# Create temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

info "⚡ Downloading gyst..."

# Download binary and checksum
curl -fsSL "$BINARY_URL" -o "$BINARY"
curl -fsSL "$CHECKSUM_URL" -o "$BINARY.sha256"

# Verify checksum
info "🔒 Verifying checksum..."
shasum -a 256 -c "$BINARY.sha256"

# Make binary executable
chmod +x "$BINARY"

# Move binary to /usr/local/bin
info "📦 Installing gyst..."
sudo mv "$BINARY" "$BINARY_PATH"

# Clean up
cd - > /dev/null
rm -rf "$TMP_DIR"

# Verify installation
if command -v gyst >/dev/null; then
    success "✨ gyst has been installed successfully!"
    info "💡 Run 'gyst --help' to get started"
else
    error "❌ Installation failed. Please try again or report the issue."
fi
