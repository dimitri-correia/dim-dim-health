#!/usr/bin/env bash
# DimDim Health - Health Check Test
# Tests the API health endpoint

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

API_BASE_URL="${API_BASE_URL:-http://localhost:3000}"

echo "=========================================="
echo -e "${YELLOW}Health Check Test${NC}"
echo "=========================================="
echo ""
echo "Testing: GET ${API_BASE_URL}/health"
echo ""

# Execute the health check
RESPONSE=$(curl -s -w "\n%{http_code}" -X GET "${API_BASE_URL}/health" \
  -H "Content-Type: application/json")

# Extract HTTP status code (last line) and body (everything else)
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "Response Body:"
echo "$BODY" | jq . 2>/dev/null || echo "$BODY"
echo ""
echo "HTTP Status: $HTTP_CODE"
echo ""

# Evaluate result
if [ "$HTTP_CODE" -eq 200 ]; then
    echo -e "${GREEN}✓ Health check passed${NC}"
    exit 0
else
    echo -e "${RED}✗ Health check failed${NC}"
    exit 1
fi