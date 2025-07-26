#!/bin/bash

# Borg Coordinator Setup Script

set -e

echo "🤖 Setting up Borg Coordinator..."

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

# Build and start services
echo "🚀 Building and starting services..."
docker-compose up --build -d

# Wait for services to be healthy
echo "⏳ Waiting for services to start..."
sleep 10

# Check if services are running
if docker-compose ps | grep -q "Up"; then
    echo "✅ Borg Coordinator is running!"
    echo ""
    echo "🌐 Dashboard: http://localhost:3000"
    echo "📊 Redis: localhost:6379"
    echo ""
    echo "📋 Useful commands:"
    echo "  - View logs: docker-compose logs -f"
    echo "  - Stop services: docker-compose down"
    echo "  - Restart: docker-compose restart"
    echo ""
else
    echo "❌ Failed to start services. Check logs:"
    docker-compose logs
    exit 1
fi