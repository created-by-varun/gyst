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

case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        error "❌ Unsupported architecture: $ARCH"
        ;;
esac

# Set install location
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="gyst"
BINARY_PATH="$INSTALL_DIR/$BINARY_NAME"

# Create install directory if needed
if [ ! -d "$INSTALL_DIR" ]; then
    info "📁 Creating install directory..."
    sudo mkdir -p "$INSTALL_DIR"
fi

# Download and install binary
info "⚡ Downloading gyst..."
RELEASE_URL="https://github.com/created-by-varun/gyst/releases/latest/download/gyst-$OS-$ARCH"
sudo curl -#L "$RELEASE_URL" -o "$BINARY_PATH"
sudo chmod +x "$BINARY_PATH"

# Verify installation
if command -v gyst >/dev/null; then
    success "✨ gyst has been installed successfully!"
    info "💡 Run 'gyst --help' to get started"
    info "🔑 Configure your API key with: gyst config --api-key YOUR_API_KEY"
else
    error "❌ Installation failed. Please try again or report the issue."
fi
