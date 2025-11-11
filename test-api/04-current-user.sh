#!/bin/bash

# Current User Endpoint
# GET /api/user
# 
# This endpoint retrieves the currently authenticated user's information.
# Requires: Authorization header with JWT token in format "Token <jwt_token>"
#
# Expected response: {"user": {"email": "...", "token": "...", "username": "..."}}
#
# Note: Replace YOUR_JWT_TOKEN with the actual token received from login or registration

echo "=== Get Current User ==="
echo "Note: Replace YOUR_JWT_TOKEN with an actual JWT token from login/registration response"
curl -X GET http://localhost:3000/api/user \
  -H "Content-Type: application/json" \
  -H "Authorization: Token YOUR_JWT_TOKEN" \
  -w "\n\nHTTP Status: %{http_code}\n"
