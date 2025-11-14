echo "=== Get Current User ==="
echo "Note: Replace YOUR_JWT_TOKEN with an actual JWT token from login/registration response"
curl -X GET http://localhost:3000/api/user \
  -H "Content-Type: application/json" \
  -H "Authorization: Token YOUR_JWT_TOKEN" \
  -w "\n\nHTTP Status: %{http_code}\n"