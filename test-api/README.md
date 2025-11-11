# API Testing with cURL

This directory contains shell scripts with cURL commands to test all API endpoints of the dim-dim-health application.

## Prerequisites

1. The API server must be running on `http://localhost:3000`
2. Database must be running and accessible
3. Redis must be running (for background jobs)

## Quick Start

To run all tests in sequence:

```bash
chmod +x run-all-tests.sh
./run-all-tests.sh
```

This will execute all endpoints in order and automatically extract/use the JWT token for authenticated requests.

## Available Endpoints

### 1. Health Check (`01-health-check.sh`)

**Endpoint:** `GET /health`

Check if the server and database are running correctly.

```bash
chmod +x 01-health-check.sh
./01-health-check.sh
```

**Expected Response:**
```json
{"status": "ok"}
```

### 2. Register User (`02-register-user.sh`)

**Endpoint:** `POST /api/users`

Create a new user account.

```bash
chmod +x 02-register-user.sh
./02-register-user.sh
```

**Request Body:**
```json
{
  "user": {
    "username": "testuser",
    "email": "testuser@example.com",
    "password": "securepassword123"
  }
}
```

**Validation Rules:**
- `username`: 3-20 characters
- `email`: valid email format
- `password`: minimum 8 characters

**Expected Response:**
```json
{
  "user": {
    "email": "testuser@example.com",
    "token": "eyJ...",
    "username": "testuser"
  }
}
```

### 3. Login User (`03-login-user.sh`)

**Endpoint:** `POST /api/users/login`

Authenticate a user and receive a JWT token.

```bash
chmod +x 03-login-user.sh
./03-login-user.sh
```

**Request Body:**
```json
{
  "user": {
    "email": "testuser@example.com",
    "password": "securepassword123"
  }
}
```

**Expected Response:**
```json
{
  "user": {
    "email": "testuser@example.com",
    "token": "eyJ...",
    "username": "testuser"
  }
}
```

### 4. Get Current User (`04-current-user.sh`)

**Endpoint:** `GET /api/user`

Retrieve information about the currently authenticated user.

```bash
chmod +x 04-current-user.sh
./04-current-user.sh
```

**Required Header:**
```
Authorization: Token YOUR_JWT_TOKEN
```

**Note:** You must replace `YOUR_JWT_TOKEN` in the script with the actual token received from the login or registration response.

**Expected Response:**
```json
{
  "user": {
    "email": "testuser@example.com",
    "token": "eyJ...",
    "username": "testuser"
  }
}
```

## Usage Flow

1. **Start the API server** (from the project root):
   ```bash
   cargo run --bin api
   ```

2. **Check server health**:
   ```bash
   ./01-health-check.sh
   ```

3. **Register a new user**:
   ```bash
   ./02-register-user.sh
   ```
   Save the token from the response.

4. **Login with the user**:
   ```bash
   ./03-login-user.sh
   ```

5. **Get current user info** (update the script with your token first):
   ```bash
   # Edit 04-current-user.sh and replace YOUR_JWT_TOKEN with the actual token
   ./04-current-user.sh
   ```

## Make All Scripts Executable

To make all scripts executable at once:

```bash
chmod +x *.sh
```

## Running All Tests

You can run all tests in sequence using:

```bash
./run-all-tests.sh
```

This script will:
1. Check server health
2. Register a new user and save the token
3. Login with the user
4. Get current user info using the saved token

## Error Responses

The API returns appropriate HTTP status codes:

- `200 OK` - Successful request
- `400 Bad Request` - Invalid request body or validation error
- `401 Unauthorized` - Invalid credentials or missing/invalid token
- `404 Not Found` - Endpoint doesn't exist
- `409 Conflict` - Username or email already exists
- `500 Internal Server Error` - Server error

## Tips

- Use tools like `jq` to format JSON responses:
  ```bash
  ./01-health-check.sh | jq
  ```

- Extract the token from responses:
  ```bash
  TOKEN=$(./02-register-user.sh | jq -r '.user.token')
  echo $TOKEN
  ```

- Use the extracted token in the current user request:
  ```bash
  curl -X GET http://localhost:3000/api/user \
    -H "Content-Type: application/json" \
    -H "Authorization: Token $TOKEN"
  ```
