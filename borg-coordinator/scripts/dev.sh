#!/bin/bash

# Development Environment Setup Script

set -e

echo "🛠️  Starting Borg Coordinator in development mode..."

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

# Create necessary directories
mkdir -p workspaces logs config

# Copy environment file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp .env.example .env
fi

# Start development environment
echo "🚀 Starting development environment..."
docker-compose -f docker-compose.dev.yml up --build

echo "🛑 Development environment stopped."