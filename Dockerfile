FROM rust:1.75-alpine AS builder

# Install system dependencies
RUN apk add --no-cache musl-dev

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache bash

# Create app directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/borg-coordinator /usr/local/bin/borg-coordinator

# Create directories for logs and workspaces
RUN mkdir -p logs workspaces

# Set environment variables
ENV RUST_LOG=info
ENV REDIS_URL=redis://redis:6379

# Health check for Redis connection
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD borg-coordinator --redis-url $REDIS_URL --help > /dev/null || exit 1

# Start the core application
CMD ["borg-coordinator"]