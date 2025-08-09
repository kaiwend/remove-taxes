#!/bin/bash

set -e

echo "Building remove-tax in release mode..."
cargo build --release

INSTALL_DIR="${1:-/usr/local/bin}"

echo "Installing to $INSTALL_DIR..."
if [[ "$INSTALL_DIR" == "/usr/local/bin" || "$INSTALL_DIR" == "/usr/bin" ]]; then
    echo "Note: Installing to $INSTALL_DIR requires sudo privileges"
    sudo mkdir -p "$INSTALL_DIR"
    sudo cp target/release/remove-tax "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/remove-tax"
else
    mkdir -p "$INSTALL_DIR"
    cp target/release/remove-tax "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/remove-tax"
fi

echo "Installation complete!"
echo ""
echo "Installed to: $INSTALL_DIR/remove-tax"
echo "Make sure $INSTALL_DIR is in your PATH."
echo ""
echo "Usage: remove-tax <number1> [number2 ...] [--rate <percentage>]"
echo "Environment variable: DEFAULT_VAT_RATE (default: 19)"