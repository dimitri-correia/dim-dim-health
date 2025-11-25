#!/usr/bin/env bash
# DimDim Health - Forgot Password Test
# Tests the password reset request flow

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
echo -e "${YELLOW}Forgot Password Test${NC}"
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

echo "Requesting password reset for:"
echo "  Email: $SAVED_EMAIL"
echo ""
echo "Testing: POST ${API_BASE_URL}/api/auth/forgot-password"
echo ""

# Execute the forgot password request
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "${API_BASE_URL}/api/auth/forgot-password" \
  -H "Content-Type: application/json" \
  -d "{
    \"email\": \"${SAVED_EMAIL}\"
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
    echo -e "${GREEN}✓ Forgot password request successful${NC}"
    echo ""
    echo "Note: Check your email for the password reset link."
    echo "The API returns success even for non-existent emails (security best practice)."
    exit 0
else
    echo -e "${RED}✗ Forgot password request failed${NC}"
    exit 1
fi