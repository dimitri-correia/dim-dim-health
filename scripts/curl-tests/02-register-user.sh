#!/usr/bin/env bash
# DimDim Health - User Registration Test
# Creates a new test user and saves credentials for subsequent tests

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Resolve the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_ENV="$SCRIPT_DIR/.test_env"

API_BASE_URL="${API_BASE_URL:-http://localhost:3000}"

echo "=========================================="
echo -e "${YELLOW}User Registration Test${NC}"
echo "=========================================="
echo ""

# Create a random number to avoid duplicate usernames/emails
RANDOM_NUM=$(( RANDOM % 10000 ))
USERNAME="testuser$RANDOM_NUM"
EMAIL="dimdimhealth+tests$RANDOM_NUM@gmail.com"

echo "Creating user:"
echo "  Username: $USERNAME"
echo "  Email: $EMAIL"
echo ""
echo "Testing: POST ${API_BASE_URL}/api/users"
echo ""

# Execute the registration request
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "${API_BASE_URL}/api/users" \
  -H "Content-Type: application/json" \
  -d "{
    \"user\": {
      \"username\": \"${USERNAME}\",
      \"email\": \"${EMAIL}\",
      \"password\": \"securepassword123\"
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
    echo -e "${GREEN}✓ Registration successful${NC}"
    
    # Save credentials for subsequent tests
    {
        echo "# Test environment variables"
        echo "# Generated at: $(date -Iseconds)"
        echo "SAVED_USERNAME=$USERNAME"
        echo "SAVED_EMAIL=$EMAIL"
        echo "SAVED_PASSWORD=securepassword123"
        
        # Extract and save tokens if present
        ACCESS_TOKEN=$(echo "$BODY" | jq -r '.access_token // empty' 2>/dev/null || true)
        REFRESH_TOKEN=$(echo "$BODY" | jq -r '.refresh_token // empty' 2>/dev/null || true)
        
        if [ -n "$ACCESS_TOKEN" ]; then
            echo "SAVED_ACCESS_TOKEN=$ACCESS_TOKEN"
        fi
        if [ -n "$REFRESH_TOKEN" ]; then
            echo "SAVED_REFRESH_TOKEN=$REFRESH_TOKEN"
        fi
    } > "$TEST_ENV"
    
    echo ""
    echo "Credentials saved to: $TEST_ENV"
    exit 0
else
    echo -e "${RED}✗ Registration failed${NC}"
    exit 1
fi
