#!/bin/bash

set -e

echo "Building sabun..."
cargo build --release

echo "Installing sabun to ~/.local/bin/..."
mkdir -p ~/.local/bin
cp target/release/sabun ~/.local/bin/

echo "sabun installed successfully!"
echo ""
echo "To use as git pager, add to your ~/.gitconfig:"
echo ""
echo "[core]"
echo "    pager = sabun"
echo ""
echo "[interactive]"
echo "    diffFilter = sabun"
echo ""
echo "Make sure ~/.local/bin is in your PATH."