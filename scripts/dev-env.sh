#!/usr/bin/env bash
# Development environment script: starts db, redis, and openobserve with data persistence
# Use --reset flag to clear all data and reapply migrations
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Parse arguments
RESET=false
for arg in "$@"; do
    case $arg in
        --reset)
            RESET=true
            shift
            ;;
        *)
            ;;
    esac
done

# PostgreSQL config
DB_CONTAINER="dimdim-health-dev-db"
DB_IMAGE="dimdim-postgres-pgcron:18"
DB_VOLUME="dimdim-health-dev-db-data"
POSTGRES_USER="dev"
POSTGRES_PASSWORD="dev-db"
POSTGRES_DB="dimdimhealthdev"
DB_PORT="5432"

# Redis config
REDIS_CONTAINER="dimdim-health-dev-redis"
REDIS_VOLUME="dimdim-health-dev-redis-data"
REDIS_PORT="6379"

# OpenObserve config
OO_CONTAINER="openobserve"
OO_HTTP_PORT="5080"
OO_GRPC_PORT="5081"
OO_DATA_DIR="${HOME}/.openobserve/data"

echo "=== Starting Development Environment ==="
if [ "$RESET" = true ]; then
    echo "WARNING: --reset flag detected. ALL DATA WILL BE DELETED!"
    read -p "Are you sure you want to continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 0
    fi
fi

# Build the custom postgres image if it doesn't exist
if ! podman image exists "$DB_IMAGE"; then
    echo "Building custom PostgreSQL image with pg_cron..."
    podman build -t "$DB_IMAGE" -f "$SCRIPT_DIR/Dockerfile.postgres" "$SCRIPT_DIR"
fi

# ===== Reset logic =====
if [ "$RESET" = true ]; then
    echo "Stopping and removing containers..."
    podman rm -f "$DB_CONTAINER" 2>/dev/null || true
    podman rm -f "$REDIS_CONTAINER" 2>/dev/null || true
    podman rm -f "$OO_CONTAINER" 2>/dev/null || true

    echo "Removing volumes..."
    podman volume rm -f "$DB_VOLUME" 2>/dev/null || true
    podman volume rm -f "$REDIS_VOLUME" 2>/dev/null || true
    rm -rf "$OO_DATA_DIR" 2>/dev/null || true
fi

# ===== PostgreSQL =====
start_postgres() {
    if podman container exists "$DB_CONTAINER"; then
        status=$(podman inspect -f '{{.State.Status}}' "$DB_CONTAINER" 2>/dev/null || echo "")
        container_image=$(podman inspect -f '{{.Image}}' "$DB_CONTAINER" 2>/dev/null || echo "")
        expected_image=$(podman inspect -f '{{.Id}}' "$DB_IMAGE" 2>/dev/null || echo "")

        if [ "$container_image" != "$expected_image" ]; then
            echo "PostgreSQL container uses wrong image. Removing and recreating..."
            podman rm -f "$DB_CONTAINER"
        elif [ "$status" = "running" ]; then
            echo "PostgreSQL container is already running."
            return
        else
            echo "Starting existing PostgreSQL container..."
            podman start "$DB_CONTAINER"
            return
        fi
    fi

    echo "Creating and starting PostgreSQL container..."
    podman run --name "$DB_CONTAINER" \
        -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
        -e POSTGRES_USER="$POSTGRES_USER" \
        -e POSTGRES_DB="$POSTGRES_DB" \
        -v "$DB_VOLUME:/var/lib/postgresql" \
        -p "$DB_PORT:5432" \
        -d "$DB_IMAGE" \
        -c shared_preload_libraries=pg_cron \
        -c cron.database_name="$POSTGRES_DB"

    echo "Waiting for PostgreSQL to be ready..."
    for i in {1..30}; do
        if podman exec "$DB_CONTAINER" pg_isready -U "$POSTGRES_USER" > /dev/null 2>&1; then
            echo "PostgreSQL is ready!"
            break
        fi
        echo "Waiting for PostgreSQL... ($i/30)"
        sleep 1
    done

    echo "Creating pg_cron extension..."
    podman exec "$DB_CONTAINER" psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "CREATE EXTENSION IF NOT EXISTS pg_cron;"
    echo "pg_cron extension installed successfully!"
}

# ===== Redis =====
start_redis() {
    if podman container exists "$REDIS_CONTAINER"; then
        status=$(podman inspect -f '{{.State.Status}}' "$REDIS_CONTAINER" 2>/dev/null || echo "")
        if [ "$status" = "running" ]; then
            echo "Redis container is already running."
            return
        else
            echo "Starting existing Redis container..."
            podman start "$REDIS_CONTAINER"
            return
        fi
    fi

    echo "Creating and starting Redis container..."
    podman run --name "$REDIS_CONTAINER" \
        -v "$REDIS_VOLUME:/data" \
        -p "$REDIS_PORT:6379" \
        -d redis:7-alpine \
        redis-server --appendonly yes
}

# ===== OpenObserve =====
start_openobserve() {
    mkdir -p "$OO_DATA_DIR"
    chmod 777 "$OO_DATA_DIR"

    if podman container exists "$OO_CONTAINER"; then
        status=$(podman inspect -f '{{.State.Status}}' "$OO_CONTAINER" 2>/dev/null || echo "")
        if [ "$status" = "running" ]; then
            echo "OpenObserve container is already running."
            return
        else
            echo "Starting existing OpenObserve container..."
            podman start "$OO_CONTAINER"
            return
        fi
    fi

    echo "Creating and starting OpenObserve container..."
    podman run -d \
        --name "$OO_CONTAINER" \
        -v "$OO_DATA_DIR:/data:Z" \
        -e ZO_DATA_DIR="/data" \
        -e ZO_ROOT_USER_EMAIL="admin@example.com" \
        -e ZO_ROOT_USER_PASSWORD="Complexpass#123" \
        -p "$OO_HTTP_PORT:5080" \
        -p "$OO_GRPC_PORT:5081" \
        public.ecr.aws/zinclabs/openobserve:latest
}

# Start all services
start_postgres
start_redis
start_openobserve

# Run migrations if reset was performed
if [ "$RESET" = true ]; then
    echo ""
    echo "Running migrations..."
    DATABASE_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$DB_PORT/$POSTGRES_DB"
    cd "$PROJECT_ROOT"
    sea-orm-cli migrate up --database-url "$DATABASE_URL"
    echo "Migrations applied successfully!"
fi

echo ""
echo "=== Development Environment Ready ==="
echo "PostgreSQL: localhost:$DB_PORT (user: $POSTGRES_USER, password: $POSTGRES_PASSWORD, db: $POSTGRES_DB)"
echo "Redis: localhost:$REDIS_PORT"
echo "OpenObserve: http://localhost:$OO_HTTP_PORT (user: admin@example.com, password: Complexpass#123)"
echo ""
echo "Connection strings:"
echo "  DATABASE_URL=postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:$DB_PORT/$POSTGRES_DB"
echo "  REDIS_URL=redis://localhost:$REDIS_PORT/"
echo ""
echo "To stop services:"
echo "  podman stop $DB_CONTAINER $REDIS_CONTAINER $OO_CONTAINER"
