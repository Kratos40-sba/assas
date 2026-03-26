#!/bin/bash
set -e

# ASSAS Windows Build Helper Script
# This script automates cross-compilation from Linux to Windows x86_64.

TARGET="x86_64-pc-windows-gnu"

echo "Checking for rustup target: $TARGET..."
if ! rustup target list | grep -q "$TARGET (installed)"; then
    echo "Installing target $TARGET..."
    rustup target add $TARGET
fi

echo "Checking for mingw-w64..."
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Error: mingw-w64 is not installed. Please install it using:"
    echo "  sudo apt-get install mingw-w64"
    exit 1
fi

echo "Building release binary for Windows..."
cargo build --release --target $TARGET

echo "Packaging... creating release zip..."
RELEASE_DIR="target/$TARGET/release"
ZIP_NAME="assas-windows.zip"

# Create a temporary folder for packaging
mkdir -p "$RELEASE_DIR/package"
cp "$RELEASE_DIR/assas.exe" "$RELEASE_DIR/package/"
cp "scripts/install.ps1" "$RELEASE_DIR/package/"

# Create the zip (using zip command if available, otherwise suggest manual)
if command -v zip &> /dev/null; then
    cd "$RELEASE_DIR/package"
    zip -r "../../$ZIP_NAME" .
    cd - > /dev/null
    echo "Release packaged: target/$TARGET/$ZIP_NAME"
else
    echo "Warning: 'zip' command not found. Please manually zip the contents of $RELEASE_DIR/package"
fi

echo "----------------------------------------------------"
echo "Build Successful!"
echo "Binary located at: target/$TARGET/release/assas.exe"
echo "----------------------------------------------------"
echo "Next Steps for Delivery:"
echo "1. Compress the .exe into a .zip file."
echo "2. Provide instructions to the user to add an exclusion in Windows Defender"
echo "   for the folder where the app will be running."
echo "3. The app is stealthy; it won't show a console when launched."
