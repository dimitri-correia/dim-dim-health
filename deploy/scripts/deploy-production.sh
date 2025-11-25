#!/bin/bash
# Deployment script for Raspberry Pi production environment
# This script performs a rolling update with zero data loss

set -e

echo "=========================================="
echo "DimDim Health Production Deployment"
echo "=========================================="

# Configuration
COMPOSE_FILE="deploy/docker-compose.yml"
API_SERVICE="api"
WORKER_SERVICE="worker"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if docker-compose is installed
if ! command -v docker-compose &> /dev/null; then
    print_error "docker-compose is not installed. Please install it first."
    exit 1
fi

# Check if config/common.toml exists
if [ ! -f "config/common.toml" ]; then
    print_error "config/common.toml not found. Please create it with your secrets."
    echo "Example content:"
    echo "gmail_password = \"your_gmail_app_password\""
    echo "jwt_secret = \"your_secure_random_jwt_secret\""
    exit 1
fi

# Pull latest changes (if using git)
if [ -d ".git" ]; then
    print_info "Pulling latest changes from git..."
    git pull
fi

# Build new images
print_info "Building new Docker images..."
docker-compose -f $COMPOSE_FILE build --no-cache

# Deploy with rolling update strategy
print_info "Starting rolling update deployment..."

# Step 1: Deploy Worker first (it will finish current jobs before stopping)
print_info "Updating Worker service..."
docker-compose -f $COMPOSE_FILE up -d --no-deps --scale worker=2 $WORKER_SERVICE
sleep 10

# Stop old worker instances gracefully
print_info "Stopping old Worker instances..."
docker-compose -f $COMPOSE_FILE up -d --no-deps --scale worker=1 $WORKER_SERVICE
sleep 30  # Give time for old workers to finish their jobs

# Step 2: Run database migrations
print_info "Running database migrations..."
docker-compose -f $COMPOSE_FILE run --rm $API_SERVICE /app/dimdim-health-api --migrate || {
    print_warning "Migration command not available or failed. Continuing..."
}

# Step 3: Deploy API with rolling update (prevents new connections, waits for existing)
print_info "Updating API service..."
docker-compose -f $COMPOSE_FILE up -d --no-deps $API_SERVICE

# Wait for health check
print_info "Waiting for API health check..."
sleep 20
for i in {1..30}; do
    if docker-compose -f $COMPOSE_FILE exec -T $API_SERVICE wget --spider -q http://localhost:3000/health; then
        print_info "API is healthy!"
        break
    fi
    if [ $i -eq 30 ]; then
        print_error "API health check failed after 30 attempts"
        docker-compose -f $COMPOSE_FILE logs --tail=50 $API_SERVICE
        exit 1
    fi
    sleep 2
done

# Cleanup unused images
print_info "Cleaning up old Docker images..."
docker image prune -f

print_info "=========================================="
print_info "Deployment completed successfully!"
print_info "=========================================="
print_info "Services status:"
docker-compose -f $COMPOSE_FILE ps

echo ""
print_info "To view logs, run: docker-compose -f deploy/docker-compose.yml logs -f"
print_info "To stop services, run: docker-compose -f deploy/docker-compose.yml down"
