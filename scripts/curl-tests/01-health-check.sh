echo "=== Health Check ==="
curl -X GET http://localhost:3000/health \
  -H "Content-Type: application/json" \
  -w "\n\nHTTP Status: %{http_code}\n"