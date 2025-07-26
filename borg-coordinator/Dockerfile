FROM node:18-alpine

# Install Claude Code CLI (if available via package manager)
# Note: This assumes Claude Code will be available in the container
# You may need to modify this based on how Claude Code is distributed
RUN apk add --no-cache curl bash

# Create app directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy source code
COPY . .

# Build the TypeScript code
RUN npm run build

# Create directories for logs and workspaces
RUN mkdir -p logs workspaces

# Expose ports
EXPOSE 3000 3001

# Set environment variables
ENV NODE_ENV=production
ENV DASHBOARD_PORT=3000
ENV COMMUNICATION_PORT=3001
ENV REDIS_URL=redis://redis:6379

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/api/instances || exit 1

# Start the application
CMD ["npm", "start"]