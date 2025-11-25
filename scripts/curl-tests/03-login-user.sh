#!/usr/bin/env bash
# DimDim Health - User Login Test
# Logs in with previously registered user credentials

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
echo -e "${YELLOW}User Login Test${NC}"
echo "=========================================="
echo ""

# Check if test environment file exists
if [ ! -f "$TEST_ENV" ]; then
    echo -e "${RED}Error: Test environment file not found: $TEST_ENV${NC}"
    echo "Run 02-register-user.sh first to create test credentials."
    exit 1
fi

# Load saved values
# shellcheck source=/dev/null
source "$TEST_ENV"

if [ -z "${SAVED_EMAIL:-}" ]; then
    echo -e "${RED}Error: SAVED_EMAIL not found in $TEST_ENV${NC}"
    exit 1
fi

echo "Logging in with:"
echo "  Email: $SAVED_EMAIL"
echo ""
echo "Testing: POST ${API_BASE_URL}/api/users/login"
echo ""

# Execute the login request
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "${API_BASE_URL}/api/users/login" \
  -H "Content-Type: application/json" \
  -d "{
    \"user\": {
      \"email\": \"${SAVED_EMAIL}\",
      \"password\": \"${SAVED_PASSWORD:-securepassword123}\"
    }
  }")

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
    echo -e "${GREEN}✓ Login successful${NC}"
    
    # Extract and update tokens in test env
    ACCESS_TOKEN=$(echo "$BODY" | jq -r '.access_token // empty' 2>/dev/null || true)
    REFRESH_TOKEN=$(echo "$BODY" | jq -r '.refresh_token // empty' 2>/dev/null || true)
    
    if [ -n "$ACCESS_TOKEN" ]; then
        # Update tokens in test env file
        sed -i '/^SAVED_ACCESS_TOKEN=/d' "$TEST_ENV" 2>/dev/null || true
        sed -i '/^SAVED_REFRESH_TOKEN=/d' "$TEST_ENV" 2>/dev/null || true
        echo "SAVED_ACCESS_TOKEN=$ACCESS_TOKEN" >> "$TEST_ENV"
        if [ -n "$REFRESH_TOKEN" ]; then
            echo "SAVED_REFRESH_TOKEN=$REFRESH_TOKEN" >> "$TEST_ENV"
        fi
        echo ""
        echo "Tokens updated in: $TEST_ENV"
    fi
    exit 0
else
    echo -e "${RED}✗ Login failed${NC}"
    exit 1
fi
