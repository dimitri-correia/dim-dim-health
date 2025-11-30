#!/usr/bin/env bash
# Database migration script: launch db, run migrations, and generate entities
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# PostgreSQL config (matches dev environment)
DB_CONTAINER="dimdim-health-dev-db"
DB_IMAGE="dimdim-postgres-pgcron:18"
DB_VOLUME="dimdim-health-dev-db-data"
POSTGRES_USER="dev"
POSTGRES_PASSWORD="dev-db"
POSTGRES_DB="dimdimhealthdev"
DB_PORT="5432"

DATABASE_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$DB_PORT/$POSTGRES_DB"

echo "=== Database Migration Script ==="

# Build the custom postgres image if it doesn't exist
if ! podman image exists "$DB_IMAGE"; then
    echo "Building custom PostgreSQL image with pg_cron..."
    podman build -t "$DB_IMAGE" -f "$SCRIPT_DIR/Dockerfile.postgres" "$SCRIPT_DIR"
fi

# ===== Start PostgreSQL if not running =====
if podman container exists "$DB_CONTAINER"; then
    status=$(podman inspect -f '{{.State.Status}}' "$DB_CONTAINER" 2>/dev/null || echo "")
    container_image=$(podman inspect -f '{{.Image}}' "$DB_CONTAINER" 2>/dev/null || echo "")
    expected_image=$(podman inspect -f '{{.Id}}' "$DB_IMAGE" 2>/dev/null || echo "")

    if [ "$container_image" != "$expected_image" ]; then
        echo "PostgreSQL container uses wrong image. Removing and recreating..."
        podman rm -f "$DB_CONTAINER"
    elif [ "$status" = "running" ]; then
        echo "PostgreSQL container is already running."
    else
        echo "Starting existing PostgreSQL container..."
        podman start "$DB_CONTAINER"
    fi
fi

if ! podman container exists "$DB_CONTAINER"; then
    echo "Creating and starting PostgreSQL container..."
    podman run --name "$DB_CONTAINER" \
        -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
        -e POSTGRES_USER="$POSTGRES_USER" \
        -e POSTGRES_DB="$POSTGRES_DB" \
        -v "$DB_VOLUME:/var/lib/postgresql/data" \
        -p "$DB_PORT:5432" \
        -d "$DB_IMAGE" \
        -c shared_preload_libraries=pg_cron \
        -c cron.database_name="$POSTGRES_DB"
fi

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
for i in {1..30}; do
    if podman exec "$DB_CONTAINER" pg_isready -U "$POSTGRES_USER" > /dev/null 2>&1; then
        echo "PostgreSQL is ready!"
        break
    fi
    echo "Waiting for PostgreSQL... ($i/30)"
    sleep 1
done

if ! podman exec "$DB_CONTAINER" pg_isready -U "$POSTGRES_USER" > /dev/null 2>&1; then
    echo "PostgreSQL did not become ready in time."
    exit 1
fi

# Ensure pg_cron extension exists
echo "Ensuring pg_cron extension exists..."
podman exec "$DB_CONTAINER" psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "CREATE EXTENSION IF NOT EXISTS pg_cron;" 2>/dev/null || true

# Run migrations
echo ""
echo "Running database migrations..."
cd "$PROJECT_ROOT"
sea-orm-cli migrate up --database-url "$DATABASE_URL" -v

# Generate entities
echo ""
echo "Generating entities from database..."
sea-orm-cli generate entity \
    --database-url "$DATABASE_URL" \
    --output-dir entities/src/db/generated \
    --with-serde "both" \
    -v

echo ""
echo "=== Migration and Entity Generation Complete ==="
echo "Entities have been generated in: entities/src/db/generated"
