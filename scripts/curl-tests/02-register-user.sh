#!/usr/bin/env bash
set -e

# Resolve the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Path to the .test_env file
TEST_ENV="$SCRIPT_DIR/.test_env"

echo "=== Register User ==="
# Create a random number to avoid duplicate usernames/emails
RANDOM_NUM=$(( RANDOM % 10000 ))
USERNAME="testuser$RANDOM_NUM"
EMAIL="dimdimhealth+tests$RANDOM_NUM@gmail.com"

curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "user": {
      "username": "'$USERNAME'",
      "email": "'$EMAIL'",
      "password": "securepassword123"
    }
  }' \
  -w "\n\nHTTP Status: %{http_code}\n"

# Overwrite (create or replace) the file
{
    echo "SAVED_USERNAME=$USERNAME"
    echo "SAVED_EMAIL=$EMAIL"
} > "$TEST_ENV"
