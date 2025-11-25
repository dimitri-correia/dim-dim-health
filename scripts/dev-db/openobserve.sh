#!/usr/bin/env bash

set -e

CONTAINER_NAME="openobserve"
IMAGE="public.ecr.aws/zinclabs/openobserve:latest"
HTTP_PORT="5080"
GRPC_PORT="5081"
DATA_DIR="${HOME}/.openobserve/data"

# Create data directory if it doesn't exist
mkdir -p "$DATA_DIR"

# Set proper permissions for podman (use :Z for SELinux context)
chmod 777 "$DATA_DIR"

# Check if container already exists
if podman ps -a --format "{{.Names}}" | grep -q "^${CONTAINER_NAME}$"; then
    echo "Container '$CONTAINER_NAME' already exists."
    
    # Check if it's running
    if podman ps --format "{{.Names}}" | grep -q "^${CONTAINER_NAME}$"; then
        echo "Container is already running."
        echo "OpenObserve UI: http://localhost:${HTTP_PORT}"
        echo "Default credentials - User: root@example.com, Password: Complexpass#123"
        exit 0
    else
        echo "Starting existing container..."
        podman start "$CONTAINER_NAME"
        echo "OpenObserve UI: http://localhost:${HTTP_PORT}"
        echo "Default credentials - User: root@example.com, Password: Complexpass#123"
        exit 0
    fi
fi

echo "Starting OpenObserve with Podman..."
echo "Data directory: $DATA_DIR"

podman run -d \
    --name "$CONTAINER_NAME" \
    -v "$DATA_DIR:/data:Z" \
    -e ZO_DATA_DIR="/data" \
    -e ZO_ROOT_USER_EMAIL="root@example.com" \
    -e ZO_ROOT_USER_PASSWORD="Complexpass#123" \
    -p "${HTTP_PORT}:5080" \
    -p "${GRPC_PORT}:5081" \
    "$IMAGE"

echo ""
echo "OpenObserve started successfully!"
echo "UI: http://localhost:${HTTP_PORT}"
echo "Logs: http://localhost:${HTTP_PORT}"
echo "Default credentials:"
echo "  User: root@example.com"
echo "  Password: Complexpass#123"
echo ""
echo "To stop: podman stop $CONTAINER_NAME"
echo "To remove: podman rm $CONTAINER_NAME"
