#!/usr/bin/env bash
# Deployment script: run podman deployment with everything included
# Uses podman-compose to run the full stack
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
COMPOSE_FILE="$PROJECT_ROOT/deploy/docker-compose.yml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

echo "=========================================="
echo "DimDim Health Podman Deployment"
echo "=========================================="

# Check if podman-compose or docker-compose is available
COMPOSE_CMD=""
if command -v podman-compose &> /dev/null; then
    COMPOSE_CMD="podman-compose"
elif command -v docker-compose &> /dev/null; then
    COMPOSE_CMD="docker-compose"
else
    print_error "Neither podman-compose nor docker-compose is installed."
    echo "Install with: pip install podman-compose"
    exit 1
fi

print_info "Using: $COMPOSE_CMD"

# Check if config/common.toml exists
if [ ! -f "$PROJECT_ROOT/config/common.toml" ]; then
    print_warning "config/common.toml not found. Creating template..."
    
    cat > "$PROJECT_ROOT/config/common.toml" << 'EOF'
# Common configuration file (not tracked in git)
# Add your secrets here

# Gmail App Password for sending emails
# Get it from: https://myaccount.google.com/apppasswords
gmail_password = "your_gmail_app_password_here"

# JWT Secret for authentication (generate a random string)
# Generate with: openssl rand -base64 32
jwt_secret = "your_secure_random_jwt_secret_here"
EOF

    print_error "Please edit config/common.toml and add your credentials:"
    echo "  1. Gmail App Password"
    echo "  2. JWT Secret (generate with: openssl rand -base64 32)"
    exit 1
fi

# Check if secrets are still default values
if grep -q "your_gmail_app_password_here" "$PROJECT_ROOT/config/common.toml" || grep -q "your_secure_random_jwt_secret_here" "$PROJECT_ROOT/config/common.toml"; then
    print_error "config/common.toml still contains default values."
    echo "Please update:"
    echo "  1. gmail_password - Get from https://myaccount.google.com/apppasswords"
    echo "  2. jwt_secret - Generate with: openssl rand -base64 32"
    exit 1
fi

print_info "Configuration check passed."

# Build images
print_info "Building images..."
cd "$PROJECT_ROOT"
$COMPOSE_CMD -f "$COMPOSE_FILE" build

# Start all services
print_info "Starting all services..."
$COMPOSE_CMD -f "$COMPOSE_FILE" up -d

# Wait for services to be ready
print_info "Waiting for services to be ready..."
sleep 15

# Check health
print_info "Checking service health..."
$COMPOSE_CMD -f "$COMPOSE_FILE" ps

echo ""
print_info "=========================================="
print_info "Deployment completed!"
print_info "=========================================="
echo ""
print_info "Services:"
echo "  API:         http://localhost:3000"
echo "  OpenObserve: http://localhost:5080"
echo "  Database:    localhost:5432"
echo "  Redis:       localhost:6379"
echo ""
print_info "Useful commands:"
echo "  View logs:     $COMPOSE_CMD -f $COMPOSE_FILE logs -f"
echo "  Stop services: $COMPOSE_CMD -f $COMPOSE_FILE down"
echo "  Restart:       $COMPOSE_CMD -f $COMPOSE_FILE restart"
