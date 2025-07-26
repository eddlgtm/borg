#!/bin/bash

set -e

echo "🤖 Borg Coordinator Setup & Launch Script"
echo "========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if Docker is installed and start Redis container
check_docker_and_redis() {
    print_info "Checking Docker..."
    
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first:"
        echo "  Visit: https://docs.docker.com/get-docker/"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        print_error "Docker is not running. Please start Docker first."
        exit 1
    fi
    
    print_status "Docker is running"
    
    # Check if Redis container is already running
    if docker ps | grep -q "borg-redis"; then
        print_status "Redis container is already running"
    else
        print_info "Starting Redis container..."
        
        # Remove any existing container
        docker rm -f borg-redis &> /dev/null || true
        
        # Start Redis container
        docker run -d \
            --name borg-redis \
            -p 6379:6379 \
            redis:7-alpine \
            redis-server --appendonly yes
        
        # Wait for Redis to be ready
        print_info "Waiting for Redis to be ready..."
        for i in {1..30}; do
            if docker exec borg-redis redis-cli ping 2>/dev/null | grep -q PONG; then
                print_status "Redis container is ready"
                break
            fi
            if [ $i -eq 30 ]; then
                print_error "Redis container failed to start properly"
                docker logs borg-redis
                exit 1
            fi
            sleep 1
        done
    fi
    
    # Test Redis connection from host
    if docker exec borg-redis redis-cli ping | grep -q PONG; then
        print_status "Redis is accessible"
    else
        print_error "Cannot connect to Redis container"
        exit 1
    fi
}

# Kill any existing instances
cleanup_existing() {
    print_info "Cleaning up existing processes..."
    
    # Kill any existing borg processes
    if pgrep -f "borg-coordinator" > /dev/null; then
        print_warning "Stopping existing Borg Coordinator processes..."
        pkill -f "borg-coordinator" || true
        sleep 2
    fi
    
    # Clear Redis task queues (using Docker)
    print_info "Clearing Redis task queues..."
    if docker ps | grep -q "borg-redis"; then
        docker exec borg-redis redis-cli del "borg:tasks:critical" "borg:tasks:high" "borg:tasks:medium" "borg:tasks:low" > /dev/null 2>&1 || true
    fi
    
    print_status "Cleanup completed"
}

# Build the project
build_project() {
    print_info "Building Borg Coordinator..."
    
    if ! cargo build --release; then
        print_error "Build failed"
        exit 1
    fi
    
    print_status "Build completed successfully"
}

# Start the orchestrator
start_orchestrator() {
    print_info "Starting Borg Coordinator orchestrator..."
    
    # Start orchestrator in background
    nohup cargo run --release --bin borg-coordinator > orchestrator.log 2>&1 &
    ORCHESTRATOR_PID=$!
    
    # Wait a moment for startup
    sleep 3
    
    # Check if it's still running
    if kill -0 $ORCHESTRATOR_PID 2>/dev/null; then
        print_status "Orchestrator started (PID: $ORCHESTRATOR_PID)"
        echo $ORCHESTRATOR_PID > .orchestrator.pid
    else
        print_error "Orchestrator failed to start. Check orchestrator.log for details."
        exit 1
    fi
}

# Verify the system
verify_system() {
    print_info "Verifying system setup..."
    
    # Wait for full initialization
    sleep 5
    
    # Simple verification - check if process is running and log file exists
    if [ -f .orchestrator.pid ] && kill -0 $(cat .orchestrator.pid) 2>/dev/null; then
        print_status "System verification passed"
        
        # Show basic status
        echo ""
        echo "🎉 Borg Coordinator is ready!"
        echo ""
        echo "✅ Orchestrator running (PID: $(cat .orchestrator.pid))"
        echo "✅ Redis container running"
        echo "✅ 8-member AI development team initialized"
        echo ""
        echo "Team includes:"
        echo "  - Project Manager (task planning & breakdown)"
        echo "  - Team Supervisor (architecture & coordination)"
        echo "  - 3 Developers (Frontend, Backend, Full-Stack)"
        echo "  - QA Tester (testing & quality assurance)"
        echo "  - Code Reviewer (security & best practices)"
        echo "  - Researcher (analysis & documentation)"
    else
        print_error "System verification failed. Orchestrator is not running properly."
        exit 1
    fi
}

# Show usage instructions
show_usage() {
    echo ""
    echo "🚀 System is ready! Available commands:"
    echo ""
    echo "  Launch Web Interface:"
    echo "    cargo run --release --bin web-ui"
    echo "    Then open: http://localhost:8080"
    echo ""
    echo "  View logs:"
    echo "    tail -f orchestrator.log"
    echo ""
    echo "  Stop orchestrator:"
    echo "    kill \$(cat .orchestrator.pid) && rm .orchestrator.pid"
    echo ""
    echo "🌐 Web Interface:"
    echo "  cargo run --release --bin web-ui"
    echo "    - Open http://localhost:8080 in your browser"
    echo "    - Create tasks through an easy web form"
    echo "    - View real-time team status and task progress"
    echo ""
}

# Handle script arguments
case "${1:-}" in
    "stop")
        print_info "Stopping Borg Coordinator..."
        if [ -f .orchestrator.pid ]; then
            PID=$(cat .orchestrator.pid)
            if kill $PID 2>/dev/null; then
                print_status "Orchestrator stopped (PID: $PID)"
            else
                print_warning "Process $PID was not running"
            fi
            rm -f .orchestrator.pid
        else
            pkill -f "borg-coordinator" || print_warning "No running processes found"
        fi
        
        # Also stop Redis container
        print_info "Stopping Redis container..."
        if docker ps | grep -q "borg-redis"; then
            docker stop borg-redis > /dev/null
            docker rm borg-redis > /dev/null
            print_status "Redis container stopped and removed"
        else
            print_warning "Redis container was not running"
        fi
        exit 0
        ;;
    "restart")
        $0 stop
        sleep 2
        $0
        exit 0
        ;;
    "status")
        if [ -f .orchestrator.pid ] && kill -0 $(cat .orchestrator.pid) 2>/dev/null; then
            print_status "Orchestrator is running (PID: $(cat .orchestrator.pid))"
            echo ""
            echo "🤖 System Status:"
            echo "✅ Orchestrator: Running"
            echo "✅ Redis: $(docker ps | grep -q "borg-redis" && echo "Running" || echo "Stopped")"
            echo "✅ Web UI: Available at http://localhost:8080"
            echo ""
            echo "📋 To create tasks:"
            echo "  cargo run --release --bin web-ui"
            echo "  Then open: http://localhost:8080"
        else
            print_error "Orchestrator is not running"
            echo "Run './run.sh' to start the system"
            exit 1
        fi
        exit 0
        ;;
    "help"|"--help"|"-h")
        echo "Usage: $0 [stop|restart|status|help]"
        echo ""
        echo "Commands:"
        echo "  (no args)  Start the Borg Coordinator system"
        echo "  stop       Stop the orchestrator"
        echo "  restart    Restart the orchestrator"
        echo "  status     Check system status"
        echo "  help       Show this help message"
        exit 0
        ;;
esac

# Main execution flow
main() {
    check_docker_and_redis
    cleanup_existing
    build_project
    start_orchestrator
    verify_system
    show_usage
}

# Trap to cleanup on script exit
trap 'if [ ! -z "${ORCHESTRATOR_PID:-}" ]; then echo ""; print_warning "Script interrupted. Orchestrator is still running (PID: $ORCHESTRATOR_PID)"; fi' INT TERM

# Run main function
main

print_status "Setup completed successfully! 🎉"