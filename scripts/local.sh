#!/bin/bash

# Run Borg Coordinator locally (Rust Edition)

set -e

echo "🦀 Starting Borg Coordinator locally (Rust Edition)..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed. Please install Rust first."
    echo "Visit: https://rustup.rs/"
    exit 1
fi

# Check if Redis is running
if ! nc -z localhost 6379 2>/dev/null; then
    echo "❌ Redis is not running on localhost:6379"
    echo "Please start Redis first:"
    echo "  - macOS: brew services start redis"
    echo "  - Ubuntu: sudo systemctl start redis"
    echo "  - Docker: docker run -d -p 6379:6379 redis:7-alpine"
    exit 1
fi

echo "✅ Redis is running"

# Create necessary directories
mkdir -p workspaces logs config

# Copy environment file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp .env.example .env
fi

# Start the backend in background
echo "🚀 Starting backend..."
RUST_LOG=info cargo run &
BACKEND_PID=$!

# Wait for backend to start
sleep 3

echo "🖥️  Starting TUI..."
echo ""
echo "📋 TUI Controls:"
echo "  - [i] Create Instance"
echo "  - [t] Create Task"  
echo "  - [r] Refresh"
echo "  - [?] Help"
echo "  - [q] Quit"
echo ""

# Start the TUI
RUST_LOG=info cargo run --bin borg-tui

# Cleanup: kill backend when TUI exits
kill $BACKEND_PID 2>/dev/null || true