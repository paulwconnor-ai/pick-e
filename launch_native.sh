#!/bin/sh
set -e

# Switch to native Cargo and .cargo config
echo "⏳ Switching to native config..."
cp Cargo_native.toml Cargo.toml
cp .cargo/config_native.toml .cargo/config.toml
cp -r assets target/debug/

# Check for --build-only flag
if [ "$1" = "--build-only" ]; then
    echo "🔨 Building only (no run)..."
    cargo build || exit $?
else
    echo "🚀 Launching native build..."
    cargo run
fi
