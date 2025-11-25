#!/usr/bin/env bash
# DimDim Health - Get Current User Test
# Retrieves the current user's information using stored JWT token

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Resolve script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_ENV="$SCRIPT_DIR/.test_env"

API_BASE_URL="${API_BASE_URL:-http://localhost:3000}"

echo "=========================================="
echo -e "${YELLOW}Get Current User Test${NC}"
echo "=========================================="
echo ""

# Check if test environment file exists
if [ ! -f "$TEST_ENV" ]; then
    echo -e "${RED}Error: Test environment file not found: $TEST_ENV${NC}"
    echo "Run 02-register-user.sh and 03-login-user.sh first."
    exit 1
fi

# Load saved values
# shellcheck source=/dev/null
source "$TEST_ENV"

# Check for access token
if [ -z "${SAVED_ACCESS_TOKEN:-}" ]; then
    echo -e "${YELLOW}Warning: No saved access token found.${NC}"
    echo "Run 02-register-user.sh or 03-login-user.sh first."
    echo ""
    echo "Alternatively, provide a token manually:"
    read -r -p "Enter JWT token (or press Enter to skip): " MANUAL_TOKEN
    
    if [ -z "$MANUAL_TOKEN" ]; then
        echo -e "${RED}No token provided. Aborting.${NC}"
        exit 1
    fi
    SAVED_ACCESS_TOKEN="$MANUAL_TOKEN"
fi

echo "Testing: GET ${API_BASE_URL}/api/user"
echo ""

# Execute the request
RESPONSE=$(curl -s -w "\n%{http_code}" -X GET "${API_BASE_URL}/api/user" \
  -H "Content-Type: application/json" \
  -H "Authorization: Token ${SAVED_ACCESS_TOKEN}")

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
    echo -e "${GREEN}✓ Get current user successful${NC}"
    exit 0
elif [ "$HTTP_CODE" -eq 401 ]; then
    echo -e "${RED}✗ Unauthorized - token may be expired or invalid${NC}"
    exit 1
else
    echo -e "${RED}✗ Request failed${NC}"
    exit 1
fi