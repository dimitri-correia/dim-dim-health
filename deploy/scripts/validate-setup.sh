#!/bin/bash
# Validation script to check deployment prerequisites and configuration

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_ok() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

ERRORS=0
WARNINGS=0

echo "=========================================="
echo "DimDim Health - Deployment Validation"
echo "=========================================="
echo ""

# Check Docker
if command -v docker &> /dev/null; then
    DOCKER_VERSION=$(docker --version | cut -d' ' -f3 | tr -d ',')
    print_ok "Docker is installed: $DOCKER_VERSION"
else
    print_error "Docker is not installed"
    ((ERRORS++))
fi

# Check Docker Compose
if command -v docker-compose &> /dev/null; then
    COMPOSE_VERSION=$(docker-compose --version | cut -d' ' -f4 | tr -d ',')
    print_ok "Docker Compose is installed: $COMPOSE_VERSION"
else
    print_error "Docker Compose is not installed"
    ((ERRORS++))
fi

# Check if user is in docker group
if groups | grep -q docker; then
    print_ok "User is in docker group"
else
    print_warning "User is not in docker group (may need sudo)"
    ((WARNINGS++))
fi

# Check required files
echo ""
echo "Checking required files..."

if [ -f "deploy/docker-compose.yml" ]; then
    print_ok "deploy/docker-compose.yml exists"
else
    print_error "deploy/docker-compose.yml not found"
    ((ERRORS++))
fi

if [ -f "deploy/Dockerfile.api" ]; then
    print_ok "deploy/Dockerfile.api exists"
else
    print_error "deploy/Dockerfile.api not found"
    ((ERRORS++))
fi

if [ -f "deploy/Dockerfile.worker" ]; then
    print_ok "deploy/Dockerfile.worker exists"
else
    print_error "deploy/Dockerfile.worker not found"
    ((ERRORS++))
fi

if [ -f "config/prod.toml" ]; then
    print_ok "config/prod.toml exists"
else
    print_error "config/prod.toml not found"
    ((ERRORS++))
fi

# Check config/common.toml
echo ""
echo "Checking configuration..."

if [ -f "config/common.toml" ]; then
    print_ok "config/common.toml exists"
    
    # Check for default values
    if grep -q "your_gmail_app_password_here" config/common.toml 2>/dev/null; then
        print_error "config/common.toml contains default gmail_password"
        ((ERRORS++))
    else
        print_ok "gmail_password is configured"
    fi
    
    if grep -q "your_secure_random_jwt_secret_here" config/common.toml 2>/dev/null; then
        print_error "config/common.toml contains default jwt_secret"
        ((ERRORS++))
    else
        print_ok "jwt_secret is configured"
    fi
else
    print_error "config/common.toml not found (required for production)"
    echo "  Create it from template: cp config/common.toml.example config/common.toml"
    ((ERRORS++))
fi

# Check .env file
if [ -f ".env" ]; then
    print_ok ".env file exists"
    
    if grep -q "DB_PASSWORD=changeme" .env 2>/dev/null; then
        print_warning ".env contains default DB_PASSWORD"
        ((WARNINGS++))
    else
        print_ok "DB_PASSWORD is configured"
    fi
else
    print_warning ".env file not found (will use defaults)"
    ((WARNINGS++))
fi

# Check deployment scripts
echo ""
echo "Checking deployment scripts..."

SCRIPTS=("quick-start.sh" "deploy-production.sh" "backup.sh" "restore.sh")
for script in "${SCRIPTS[@]}"; do
    if [ -f "deploy/scripts/$script" ]; then
        if [ -x "deploy/scripts/$script" ]; then
            print_ok "deploy/scripts/$script is executable"
        else
            print_warning "deploy/scripts/$script is not executable (run: chmod +x deploy/scripts/$script)"
            ((WARNINGS++))
        fi
    else
        print_error "deploy/scripts/$script not found"
        ((ERRORS++))
    fi
done

# Check disk space
echo ""
echo "Checking system resources..."

AVAILABLE_SPACE=$(df -BG . | tail -1 | awk '{print $4}' | tr -d 'G')
if [ "$AVAILABLE_SPACE" -gt 10 ]; then
    print_ok "Sufficient disk space available: ${AVAILABLE_SPACE}GB"
else
    print_warning "Low disk space: ${AVAILABLE_SPACE}GB (recommended: 10GB+)"
    ((WARNINGS++))
fi

# Check memory
TOTAL_MEM=$(free -g | awk '/^Mem:/{print $2}')
if [ "$TOTAL_MEM" -ge 2 ]; then
    print_ok "Sufficient memory: ${TOTAL_MEM}GB"
else
    print_warning "Low memory: ${TOTAL_MEM}GB (recommended: 2GB+)"
    ((WARNINGS++))
fi

# Summary
echo ""
echo "=========================================="
if [ $ERRORS -eq 0 ]; then
    print_ok "Validation passed!"
    if [ $WARNINGS -gt 0 ]; then
        echo ""
        print_warning "$WARNINGS warning(s) found - please review"
    fi
    echo ""
    echo "Next steps:"
    echo "  1. Review warnings (if any)"
    echo "  2. Run: ./deploy/scripts/quick-start.sh"
    exit 0
else
    print_error "Validation failed with $ERRORS error(s) and $WARNINGS warning(s)"
    echo ""
    echo "Please fix the errors above before deploying."
    exit 1
fi
