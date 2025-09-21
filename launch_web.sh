#!/bin/sh
set -e

# Switch to native Cargo and .cargo config
echo "⏳ Switching to WASM config..."
cp Cargo_wasm.toml Cargo.toml
cp .cargo/config_wasm.toml .cargo/config.toml
# cp -r assets target/wasm32-unknown-unknown/debug/

# Run the app
echo "🚀 Launching WASM build..."
trunk serve
