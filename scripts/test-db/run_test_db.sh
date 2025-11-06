#!/usr/bin/env bash
set -euo pipefail

CONTAINER=dimdim-health-test-db
IMAGE=dimdim-postgres-pgcron:18
POSTGRES_USER=test
POSTGRES_PASSWORD=test-db
POSTGRES_DB=dimdimhealthtest
PORT=5433

# Build the custom image if it doesn't exist
if ! podman image exists "$IMAGE"; then
    echo "Building custom PostgreSQL image with pg_cron..."
    podman build -t "$IMAGE" -f scripts/dev-db/Dockerfile .
fi

if podman container exists "$CONTAINER"; then
    status=$(podman inspect -f '{{.State.Status}}' "$CONTAINER" 2>/dev/null || echo "")
    
    # Check if container is using the correct image
    container_image=$(podman inspect -f '{{.Image}}' "$CONTAINER" 2>/dev/null || echo "")
    expected_image=$(podman inspect -f '{{.Id}}' "$IMAGE" 2>/dev/null || echo "")
    
    if [ "$container_image" != "$expected_image" ]; then
        echo "Container exists but uses wrong image. Removing and recreating..."
        podman rm -f "$CONTAINER"
    elif [ "$status" = "running" ]; then
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
  podman run --rm --name "$CONTAINER" \
    -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
    -e POSTGRES_USER="$POSTGRES_USER" \
    -e POSTGRES_DB="$POSTGRES_DB" \
    -p "$PORT:5432" \
    -d "$IMAGE" \
    -c shared_preload_libraries=pg_cron \
    -c cron.database_name="$POSTGRES_DB"
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

# Create the pg_cron extension
echo "Creating pg_cron extension..."
podman exec "$CONTAINER" psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "CREATE EXTENSION IF NOT EXISTS pg_cron;"

echo "pg_cron extension installed successfully!"
