#!/usr/bin/env bash

# Check if container exists
if podman ps -a --format "{{.Names}}" | grep -q "^redis\$"; then
    # Container exists, check if it's running
    if podman ps --format "{{.Names}}" | grep -q "^redis\$"; then
        echo "Redis container is already running"
        exit 0
    else
        echo "Starting existing Redis container..."
        podman start redis
    fi
else
    echo "Creating and starting new Redis container..."
    podman run -d --name redis -p 6379:6379 redis:7-alpine
fi