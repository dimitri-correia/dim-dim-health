set -euo pipefail

CONTAINER=dimdim-health-dev-db
IMAGE=postgres:18
POSTGRES_USER=dev
POSTGRES_PASSWORD=dev-db
POSTGRES_DB=dimdimhealthdev
PORT=5432

if podman container exists "$CONTAINER"; then
  status=$(podman inspect -f '{{.State.Status}}' "$CONTAINER" 2>/dev/null || echo "")
  if [ "$status" = "running" ]; then
    echo "Container '$CONTAINER' is already running."
    exit 0
  else
    echo "Starting existing container '$CONTAINER'..."
    podman start "$CONTAINER"
    exit $?
  fi
else
  echo "Creating and starting container '$CONTAINER'..."
  podman run --name "$CONTAINER" \
    -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
    -e POSTGRES_USER="$POSTGRES_USER" \
    -e POSTGRES_DB="$POSTGRES_DB" \
    -p "$PORT:$PORT" \
    -d "$IMAGE"
fi