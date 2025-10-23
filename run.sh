#!/bin/bash

# Dark Emperor - Rust Edition
# Quick start script

echo "🏰 Starting Dark Emperor server..."
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo is not installed."
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build and run the server
echo "Building project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "🚀 Starting server on http://127.0.0.1:3000"
    echo "   Press Ctrl+C to stop the server"
    echo ""
    echo "🌐 Open your browser to: http://127.0.0.1:3000"
    echo ""

    # Run the server
    cargo run --release
else
    echo ""
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi