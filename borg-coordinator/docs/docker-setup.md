# Docker Setup Guide

This guide explains how to run Borg Coordinator using Docker for easy setup and deployment.

## Overview

The Docker setup includes:
- **Application Container**: Runs the Borg Coordinator Node.js application
- **Redis Container**: Provides the task queue and caching layer
- **Volume Mounts**: Persists workspaces, logs, and configuration
- **Health Checks**: Ensures services are running properly

## Quick Start

### 1. One-Command Setup

```bash
./scripts/setup.sh
```

This script will:
- Check Docker installation
- Create necessary directories
- Copy environment template
- Build and start all services
- Verify everything is running

### 2. Manual Setup

```bash
# Clone and enter directory
cd borg-coordinator

# Start services
docker-compose up --build -d

# Check status
docker-compose ps
```

## Docker Compose Services

### Production (`docker-compose.yml`)

```yaml
services:
  redis:
    image: redis:7-alpine
    ports: ["6379:6379"]
    volumes: [redis_data:/data]
    
  borg-coordinator:
    build: .
    ports: ["3000:3000", "3001:3001"]
    volumes:
      - ./workspaces:/app/workspaces
      - ./logs:/app/logs
      - ~/.claude:/root/.claude:ro
    depends_on: [redis]
```

### Development (`docker-compose.dev.yml`)

```yaml
services:
  borg-coordinator-dev:
    build:
      dockerfile: Dockerfile.dev
    volumes:
      - .:/app                    # Live code reload
      - /app/node_modules         # Preserve container node_modules
    command: npm run dev
```

## Volume Mounts

### Application Volumes
- `./workspaces:/app/workspaces` - Claude instance workspaces
- `./logs:/app/logs` - Application logs
- `./config:/app/config` - Configuration files

### Claude Code Integration
- `~/.claude:/root/.claude:ro` - Claude Code credentials (read-only)

### Data Persistence
- `redis_data:/data` - Redis data persistence

## Environment Variables

Set these in your `.env` file or docker-compose environment section:

```env
# Core Configuration
CLAUDE_CODE_PATH=claude
REDIS_URL=redis://redis:6379
DASHBOARD_PORT=3000
COMMUNICATION_PORT=3001
LOG_LEVEL=info

# Optional Configuration
TASK_TIMEOUT=300000
WORKSPACES_DIR=/app/workspaces
```

## Development Workflow

### Start Development Environment

```bash
# Start with hot reload
./scripts/dev.sh

# Or manually
docker-compose -f docker-compose.dev.yml up --build
```

### Development Features
- **Hot Reload**: Code changes trigger automatic restarts
- **Volume Mounting**: Local files sync with container
- **Debug Logging**: Enhanced logging for development

### Debugging

```bash
# View live logs
docker-compose logs -f borg-coordinator-dev

# Enter container for debugging
docker-compose exec borg-coordinator-dev sh

# Check Redis
docker-compose exec redis redis-cli ping
```

## Production Deployment

### Build and Deploy

```bash
# Production build
docker-compose up --build -d

# Check health
docker-compose ps
curl http://localhost:3000/api/instances
```

### Health Monitoring

Both services include health checks:

```bash
# Check container health
docker-compose ps

# View health check logs
docker inspect borg-coordinator --format='{{.State.Health}}'
```

### Log Management

```bash
# View all logs
docker-compose logs

# Follow specific service
docker-compose logs -f redis

# Log rotation (for production)
docker-compose logs --tail=100 borg-coordinator
```

## Common Commands

### Service Management

```bash
# Start services
docker-compose up -d

# Stop services
docker-compose down

# Restart specific service
docker-compose restart borg-coordinator

# Rebuild and restart
docker-compose up --build -d
```

### Data Management

```bash
# Backup Redis data
docker-compose exec redis redis-cli SAVE
docker cp $(docker-compose ps -q redis):/data/dump.rdb ./backup/

# Clear all data
docker-compose down -v  # Removes volumes

# Reset workspaces
rm -rf workspaces/*
docker-compose restart borg-coordinator
```

### Scaling

```bash
# Scale coordinator instances (advanced)
docker-compose up -d --scale borg-coordinator=2

# Note: You'll need load balancing for multiple instances
```

## Troubleshooting

### Common Issues

1. **Port Conflicts**
   ```bash
   # Check what's using ports
   lsof -i :3000
   lsof -i :3001
   lsof -i :6379
   ```

2. **Volume Permission Issues**
   ```bash
   # Fix permissions
   sudo chown -R $(whoami):$(whoami) workspaces logs
   ```

3. **Redis Connection Failed**
   ```bash
   # Check Redis health
   docker-compose exec redis redis-cli ping
   
   # Restart Redis
   docker-compose restart redis
   ```

4. **Claude Code Not Found**
   ```bash
   # Check if Claude Code is accessible in container
   docker-compose exec borg-coordinator which claude
   
   # Or specify full path in .env
   CLAUDE_CODE_PATH=/usr/local/bin/claude
   ```

### Service Health

```bash
# Check all services
docker-compose ps

# Service-specific health
curl http://localhost:3000/api/instances
docker-compose exec redis redis-cli ping

# Container resource usage
docker stats
```

### Log Analysis

```bash
# Error logs only
docker-compose logs borg-coordinator 2>&1 | grep ERROR

# Recent logs with timestamps
docker-compose logs -t --since="1h" borg-coordinator

# Follow logs from specific service
docker-compose logs -f redis
```

## Advanced Configuration

### Custom Networks

```yaml
networks:
  borg-network:
    driver: bridge
    ipam:
      config:
        - subnet: 172.20.0.0/16
```

### Resource Limits

```yaml
services:
  borg-coordinator:
    deploy:
      resources:
        limits:
          memory: 1G
          cpus: '0.5'
```

### Multiple Environments

```bash
# Production
docker-compose -f docker-compose.yml up -d

# Staging
docker-compose -f docker-compose.yml -f docker-compose.staging.yml up -d

# Development
docker-compose -f docker-compose.dev.yml up -d
```

## Security Considerations

1. **Container Security**
   - Run as non-root user in production
   - Use specific image tags, not `latest`
   - Regular security updates

2. **Network Security**
   - Isolate containers in custom networks
   - Only expose necessary ports
   - Use secrets for sensitive data

3. **Data Security**
   - Encrypt volumes in production
   - Regular backups
   - Secure Claude Code credentials

4. **Access Control**
   - Implement authentication for dashboard
   - Use reverse proxy for HTTPS
   - Monitor container access logs