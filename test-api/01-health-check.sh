#!/bin/bash

# Health Check Endpoint
# GET /health
# 
# This endpoint checks if the server and database are running correctly.
# Expected response: {"status": "ok"}

echo "=== Health Check ==="
curl -X GET http://localhost:3000/health \
  -H "Content-Type: application/json" \
  -w "\n\nHTTP Status: %{http_code}\n"
