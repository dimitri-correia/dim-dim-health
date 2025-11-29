#!/usr/bin/env bash
# Test environment script: starts db and redis for tests with clean/reset data
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# PostgreSQL config
DB_CONTAINER="dimdim-health-test-db"
DB_IMAGE="dimdim-postgres-pgcron:18"
POSTGRES_USER="test"
POSTGRES_PASSWORD="test-db"
POSTGRES_DB="dimdimhealthtest"
DB_PORT="5433"

# Redis config
REDIS_CONTAINER="dimdim-health-test-redis"
REDIS_PORT="6380"

echo "=== Starting Test Environment ==="
echo "This will reset ALL data in test database and redis."

# Build the custom postgres image if it doesn't exist
if ! podman image exists "$DB_IMAGE"; then
    echo "Building custom PostgreSQL image with pg_cron..."
    podman build -t "$DB_IMAGE" -f "$SCRIPT_DIR/Dockerfile.postgres" "$SCRIPT_DIR"
fi

# ===== PostgreSQL =====
if podman container exists "$DB_CONTAINER"; then
    status=$(podman inspect -f '{{.State.Status}}' "$DB_CONTAINER" 2>/dev/null || echo "")
    container_image=$(podman inspect -f '{{.Image}}' "$DB_CONTAINER" 2>/dev/null || echo "")
    expected_image=$(podman inspect -f '{{.Id}}' "$DB_IMAGE" 2>/dev/null || echo "")

    if [ "$container_image" != "$expected_image" ]; then
        echo "PostgreSQL container uses wrong image. Removing and recreating..."
        podman rm -f "$DB_CONTAINER"
    elif [ "$status" = "running" ]; then
        echo "PostgreSQL container is running. Resetting test database..."
        podman exec "$DB_CONTAINER" psql -U "$POSTGRES_USER" -d postgres -c "DROP DATABASE IF EXISTS $POSTGRES_DB;" || true
        podman exec "$DB_CONTAINER" psql -U "$POSTGRES_USER" -d postgres -c "CREATE DATABASE $POSTGRES_DB;"
        podman exec "$DB_CONTAINER" psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "CREATE EXTENSION IF NOT EXISTS pg_cron;"
        echo "PostgreSQL test database reset complete."
    else
        echo "Starting existing PostgreSQL container..."
        podman start "$DB_CONTAINER"
        sleep 2
        echo "Resetting test database..."
        podman exec "$DB_CONTAINER" psql -U "$POSTGRES_USER" -d postgres -c "DROP DATABASE IF EXISTS $POSTGRES_DB;" || true
        podman exec "$DB_CONTAINER" psql -U "$POSTGRES_USER" -d postgres -c "CREATE DATABASE $POSTGRES_DB;"
        podman exec "$DB_CONTAINER" psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "CREATE EXTENSION IF NOT EXISTS pg_cron;"
    fi
fi

if ! podman container exists "$DB_CONTAINER"; then
    echo "Creating and starting PostgreSQL container..."
    podman run --rm --name "$DB_CONTAINER" \
        -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
        -e POSTGRES_USER="$POSTGRES_USER" \
        -e POSTGRES_DB="$POSTGRES_DB" \
        -p "$DB_PORT:5432" \
        -d "$DB_IMAGE" \
        -c shared_preload_libraries=pg_cron \
        -c cron.database_name="$POSTGRES_DB"

    echo "Waiting for PostgreSQL to be ready..."
    for i in {1..30}; do
        if podman exec "$DB_CONTAINER" pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" > /dev/null 2>&1; then
            echo "PostgreSQL is ready!"
            break
        fi
        echo "Waiting for PostgreSQL... ($i/30)"
        sleep 1
    done

    if ! podman exec "$DB_CONTAINER" pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" > /dev/null 2>&1; then
        echo "PostgreSQL did not become ready in time."
        exit 1
    fi

    echo "Creating pg_cron extension..."
    podman exec "$DB_CONTAINER" psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "CREATE EXTENSION IF NOT EXISTS pg_cron;"
fi

# ===== Redis =====
if podman container exists "$REDIS_CONTAINER"; then
    status=$(podman inspect -f '{{.State.Status}}' "$REDIS_CONTAINER" 2>/dev/null || echo "")
    if [ "$status" = "running" ]; then
        echo "Redis container is running. Flushing all data..."
        podman exec "$REDIS_CONTAINER" redis-cli FLUSHALL
    else
        echo "Starting existing Redis container..."
        podman start "$REDIS_CONTAINER"
        sleep 1
        echo "Flushing Redis data..."
        podman exec "$REDIS_CONTAINER" redis-cli FLUSHALL
    fi
else
    echo "Creating and starting Redis container..."
    podman run --rm --name "$REDIS_CONTAINER" \
        -p "$REDIS_PORT:6379" \
        -d redis:7-alpine
    sleep 1
fi

echo ""
echo "=== Test Environment Ready ==="
echo "PostgreSQL: localhost:$DB_PORT (user: $POSTGRES_USER, password: $POSTGRES_PASSWORD, db: $POSTGRES_DB)"
echo "Redis: localhost:$REDIS_PORT"
echo ""
echo "Connection strings:"
echo "  DATABASE_URL=postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$DB_PORT/$POSTGRES_DB"
echo "  REDIS_URL=redis://localhost:$REDIS_PORT/"
