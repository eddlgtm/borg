#!/bin/bash

# Borg Coordinator Setup Script (Rust Edition)

set -e

echo "🦀 Setting up Borg Coordinator (Rust Edition)..."

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    echo "Visit: https://docs.docker.com/get-docker/"
    exit 1
fi

# Check if Docker Compose is available
if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Create necessary directories
echo "📁 Creating directories..."
mkdir -p workspaces logs config

# Copy environment file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp .env.example .env
    echo "✏️  Please edit .env file to configure your settings"
fi

# Build and start backend services
echo "🚀 Building and starting backend services..."
docker-compose up --build -d redis borg-coordinator

# Wait for services to be ready
echo "⏳ Waiting for services to start..."
sleep 10

# Check if services are running
if docker-compose ps redis | grep -q "Up" && docker-compose ps borg-coordinator | grep -q "Up"; then
    echo "✅ Backend services are running!"
    echo ""
    echo "🖥️  Starting TUI interface..."
    echo "📋 TUI Controls:"
    echo "  - [i] Create Instance"
    echo "  - [t] Create Task"
    echo "  - [r] Refresh"
    echo "  - [?] Help"
    echo "  - [q] Quit"
    echo ""
    echo "🚀 Launching TUI..."
    
    # Run the TUI interactively
    docker-compose run --rm borg-tui
else
    echo "❌ Failed to start services. Check logs:"
    docker-compose logs
    exit 1
fi