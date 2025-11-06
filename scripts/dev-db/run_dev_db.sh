set -euo pipefail

CONTAINER=dimdim-health-dev-db
IMAGE=dimdim-postgres-pgcron:18
POSTGRES_USER=dev
POSTGRES_PASSWORD=dev-db
POSTGRES_DB=dimdimhealthdev
PORT=5432
VOLUME=dimdim-health-dev-db-data

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
        exit 0
    else
        echo "Starting existing container '$CONTAINER'..."
        podman start "$CONTAINER"
        exit $?
    fi
fi

if ! podman container exists "$CONTAINER"; then
    echo "Creating and starting container '$CONTAINER'..."
    podman run --name "$CONTAINER" \
        -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
        -e POSTGRES_USER="$POSTGRES_USER" \
        -e POSTGRES_DB="$POSTGRES_DB" \
        -v "$VOLUME:/var/lib/postgresql" \
        -p "$PORT:$PORT" \
        -d "$IMAGE" \
        -c shared_preload_libraries=pg_cron \
        -c cron.database_name="$POSTGRES_DB"
    
    # Wait for PostgreSQL to be ready
    echo "Waiting for PostgreSQL to be ready..."
    for i in {1..30}; do
        if podman exec "$CONTAINER" pg_isready -U "$POSTGRES_USER" > /dev/null 2>&1; then
            echo "PostgreSQL is ready!"
            break
        fi
        echo "Waiting for PostgreSQL... ($i/30)"
        sleep 1
    done
    
    # Create the pg_cron extension
    echo "Creating pg_cron extension..."
    podman exec "$CONTAINER" psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "CREATE EXTENSION IF NOT EXISTS pg_cron;"
    
    echo "pg_cron extension installed successfully!"
fi