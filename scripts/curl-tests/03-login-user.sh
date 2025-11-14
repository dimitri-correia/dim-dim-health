#!/usr/bin/env bash
set -e

# Resolve script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_ENV="$SCRIPT_DIR/.test_env"

# Load saved values
source "$TEST_ENV"

echo "=== Login User ==="

curl -X POST http://localhost:3000/api/users/login \
  -H "Content-Type: application/json" \
  -d "{
    \"user\": {
      \"email\": \"${SAVED_EMAIL}\",
      \"password\": \"securepassword123\"
    }
  }" \
  -w "\n\nHTTP Status: %{http_code}\n"
