#!/bin/sh
set -e

# Switch to native Cargo and .cargo config
echo "⏳ Switching to WASM config..."
cp Cargo_wasm.toml Cargo.toml
cp .cargo/config_wasm.toml .cargo/config.toml

# Build the app
echo "🚀 Building WASM build..."
trunk build --release

# Run the app
echo "🚀 Launching WASM build..."
cd dist
python3 -m http.server 8080

