#!/bin/bash

# Login User Endpoint
# POST /api/users/login
# 
# This endpoint authenticates a user and returns a JWT token.
# Required fields:
# - email: valid email format
# - password: minimum 1 character (validated on registration)
#
# Expected response: {"user": {"email": "...", "token": "...", "username": "..."}}

echo "=== Login User ==="
curl -X POST http://localhost:3000/api/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "user": {
      "email": "testuser@example.com",
      "password": "securepassword123"
    }
  }' \
  -w "\n\nHTTP Status: %{http_code}\n"
