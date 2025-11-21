#!/bin/bash
# OpenObserve Integration Test Script

set -e

echo "=== OpenObserve Integration Test ==="
echo ""

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null && ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker to run this test."
    exit 1
fi

echo "✅ Docker found"
echo ""

# Check if OpenObserve is already running
if docker ps | grep -q dimdim-health-openobserve; then
    echo "✅ OpenObserve is already running"
else
    echo "📦 Starting OpenObserve..."
    docker-compose up -d openobserve
    
    echo "⏳ Waiting for OpenObserve to be ready (30 seconds)..."
    sleep 30
fi

echo ""
echo "🌐 OpenObserve UI: http://localhost:5080"
echo "   Email: admin@example.com"
echo "   Password: Complexpass#123"
echo ""

# Check if OpenObserve endpoint is configured
if grep -q "^openobserve_endpoint = " config/dev.toml; then
    echo "✅ OpenObserve endpoint is already configured"
else
    echo "📝 Enabling OpenObserve in config/dev.toml..."
    echo "" >> config/dev.toml
    echo "# OpenObserve enabled for testing" >> config/dev.toml
    echo 'openobserve_endpoint = "http://localhost:5080/api/default"' >> config/dev.toml
fi

echo ""
echo "📋 Next Steps:"
echo "1. Open http://localhost:5080 in your browser"
echo "2. Login with the credentials above"
echo "3. Run the API server: cargo run --bin dimdim-health_api"
echo "4. Make some API requests to generate logs"
echo "5. View logs in OpenObserve UI under 'Logs' section"
echo ""
echo "To stop OpenObserve: docker-compose down"
echo ""
