#!/usr/bin/env bash
set -euo pipefail

CONTAINER=dimdim-health-test-db
IMAGE=postgres:18
POSTGRES_USER=test
POSTGRES_PASSWORD=test-db
POSTGRES_DB=dimdimhealthtest
PORT=5433

if podman container exists "$CONTAINER"; then
  status=$(podman inspect -f '{{.State.Status}}' "$CONTAINER" 2>/dev/null || echo "")
  if [ "$status" = "running" ]; then
    echo "Container '$CONTAINER' is already running."
  else
    echo "Starting existing container '$CONTAINER'..."
    podman start "$CONTAINER"
  fi

  # Drop and recreate the test database
  echo "Resetting test database..."
  podman exec -u postgres "$CONTAINER" psql -U "$POSTGRES_USER" -d postgres -c "DROP DATABASE IF EXISTS $POSTGRES_DB;"
  podman exec -u postgres "$CONTAINER" psql -U "$POSTGRES_USER" -d postgres -c "CREATE DATABASE $POSTGRES_DB;"
  echo "Database reset complete."

else
  echo "Creating and starting container '$CONTAINER'..."
  podman run --name "$CONTAINER" \
    -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
    -e POSTGRES_USER="$POSTGRES_USER" \
    -e POSTGRES_DB="$POSTGRES_DB" \
    -p "$PORT:5432" \
    -d "$IMAGE"
fi

# wait until Postgres accepts connections
echo "Waiting for Postgres to be ready..."
for i in {1..30}; do
  if podman exec "$CONTAINER" pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" > /dev/null 2>&1; then
    echo "Postgres ready!"
    break
  fi
  sleep 1
done

# fallback if Postgres never becomes ready
if ! podman exec "$CONTAINER" pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" > /dev/null 2>&1; then
  echo "Postgres did not become ready in time."
  exit 1
fi
