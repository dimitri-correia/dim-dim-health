#!/bin/bash

# Run all API tests in sequence
# This script demonstrates the complete API flow

echo "========================================"
echo "Running all API endpoint tests"
echo "========================================"
echo ""

# 1. Health Check
echo "Step 1: Testing Health Check"
./01-health-check.sh
echo ""
echo "----------------------------------------"
echo ""

# 2. Register User
echo "Step 2: Registering a new user"
echo "Saving token to /tmp/api_token.txt"
RESPONSE=$(./02-register-user.sh)
echo "$RESPONSE"
TOKEN=$(echo "$RESPONSE" | grep -o '"token":"[^"]*"' | sed 's/"token":"//;s/"$//')
echo "$TOKEN" > /tmp/api_token.txt
echo ""
echo "----------------------------------------"
echo ""

# 3. Login User
echo "Step 3: Logging in with the user"
./03-login-user.sh
echo ""
echo "----------------------------------------"
echo ""

# 4. Current User (if token was extracted)
if [ -n "$TOKEN" ]; then
    echo "Step 4: Getting current user information"
    echo "Using extracted token: ${TOKEN:0:20}..."
    curl -X GET http://localhost:3000/api/user \
      -H "Content-Type: application/json" \
      -H "Authorization: Token $TOKEN" \
      -w "\n\nHTTP Status: %{http_code}\n"
else
    echo "Step 4: Skipping current user test (could not extract token)"
    echo "You can manually run: ./04-current-user.sh after updating the token"
fi

echo ""
echo "========================================"
echo "All tests completed!"
echo "Token saved to: /tmp/api_token.txt"
echo "========================================"
