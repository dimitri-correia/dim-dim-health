#!/bin/bash
# Quick start script for first-time setup

set -e

echo "=========================================="
echo "DimDim Health - Quick Start Setup"
echo "=========================================="

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
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

# Check Docker
if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed."
    echo "Install with: curl -fsSL https://get.docker.com -o get-docker.sh && sudo sh get-docker.sh"
    exit 1
fi

# Check Docker Compose
if ! command -v docker-compose &> /dev/null; then
    print_error "docker-compose is not installed."
    echo "Install with: sudo apt-get install -y docker-compose"
    exit 1
fi

print_info "Docker and Docker Compose are installed ✓"

# Check if config/common.toml exists
if [ ! -f "config/common.toml" ]; then
    print_warning "config/common.toml not found. Creating template..."
    
    cat > config/common.toml << 'EOF'
# Common configuration file (not tracked in git)
# Add your secrets here

# Gmail App Password for sending emails
# Get it from: https://myaccount.google.com/apppasswords
gmail_password = "your_gmail_app_password_here"

# JWT Secret for authentication (generate a random string)
# Generate with: openssl rand -base64 32
jwt_secret = "your_secure_random_jwt_secret_here"
EOF

    print_warning "Please edit config/common.toml and add your credentials:"
    echo "  1. Gmail App Password"
    echo "  2. JWT Secret (generate with: openssl rand -base64 32)"
    echo ""
    echo "Then run this script again."
    exit 1
fi

print_info "config/common.toml exists ✓"

# Check if secrets are still default values
if grep -q "your_gmail_app_password_here" config/common.toml || grep -q "your_secure_random_jwt_secret_here" config/common.toml; then
    print_error "config/common.toml still contains default values."
    echo "Please update:"
    echo "  1. gmail_password - Get from https://myaccount.google.com/apppasswords"
    echo "  2. jwt_secret - Generate with: openssl rand -base64 32"
    exit 1
fi

print_info "Configuration looks good ✓"

# Ask user if they want to continue
echo ""
print_warning "This will start building Docker images. On Raspberry Pi, this may take 15-30 minutes."
read -p "Continue? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Setup cancelled."
    exit 0
fi

# Build images
print_info "Building Docker images (this will take a while)..."
docker-compose -f deploy/docker-compose.yml build

# Start services
print_info "Starting services..."
docker-compose -f deploy/docker-compose.yml up -d

# Wait for services to be ready
print_info "Waiting for services to be ready..."
sleep 30

# Check health
print_info "Checking service health..."
if docker-compose -f deploy/docker-compose.yml ps | grep -q "unhealthy"; then
    print_error "Some services are unhealthy. Check logs with: docker-compose -f deploy/docker-compose.yml logs"
    docker-compose -f deploy/docker-compose.yml ps
    exit 1
fi

print_info "=========================================="
print_info "Setup completed successfully!"
print_info "=========================================="
echo ""
print_info "Services are running:"
docker-compose -f deploy/docker-compose.yml ps
echo ""
print_info "API available at: http://localhost:3000"
print_info "API health check: curl http://localhost:3000/health"
echo ""
print_info "Useful commands:"
echo "  View logs:           docker-compose -f deploy/docker-compose.yml logs -f"
echo "  Stop services:       docker-compose -f deploy/docker-compose.yml down"
echo "  Restart services:    docker-compose -f deploy/docker-compose.yml restart"
echo "  Deploy updates:      ./deploy/scripts/deploy-production.sh"
echo ""
print_info "For detailed documentation, see docs/DEPLOYMENT.md"
