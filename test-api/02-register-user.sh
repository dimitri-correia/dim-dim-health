#!/bin/bash

# Register User Endpoint
# POST /api/users
# 
# This endpoint creates a new user account.
# Required fields:
# - username: 3-20 characters
# - email: valid email format
# - password: minimum 8 characters
#
# Expected response: {"user": {"email": "...", "token": "...", "username": "..."}}

echo "=== Register User ==="
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "user": {
      "username": "testuser",
      "email": "testuser@example.com",
      "password": "securepassword123"
    }
  }' \
  -w "\n\nHTTP Status: %{http_code}\n"
