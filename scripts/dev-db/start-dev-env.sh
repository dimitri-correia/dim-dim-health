#!/usr/bin/env bash
set -e

cargo build --release

POD_NAME="dim-app"
API_IMAGE=" ./target/release/dimdim-health"
WORKER_IMAGE="your-worker-image:latest"

# create pod if not exists
if ! podman pod exists "$POD_NAME"; then
    podman pod create \
      --name "$POD_NAME" \
      -p 3000:3000 \
      -p 5432:5432 \
      -p 6379:6379
fi

# postgres container
if ! podman container exists postgres; then
    podman run -d \
      --name postgres \
      --pod "$POD_NAME" \
      -e POSTGRES_USER=app \
      -e POSTGRES_PASSWORD=app \
      -e POSTGRES_DB=appdb \
      -v pgdata:/var/lib/postgresql/data \
      postgres:16
fi

# redis container
if ! podman container exists redis; then
    podman run -d \
      --name redis \
      --pod "$POD_NAME" \
      -v redis-data:/data \
      redis:7 \
      redis-server --appendonly yes
fi

# api container
if ! podman container exists api; then
    podman run -d \
      --name api \
      --pod "$POD_NAME" \
      -e DATABASE_URL="postgres://app:app@localhost:5432/appdb" \
      -e REDIS_URL="redis://localhost:6379" \
      "$API_IMAGE"
fi

# worker container
if ! podman container exists worker; then
    podman run -d \
      --name worker \
      --pod "$POD_NAME" \
      -e REDIS_URL="redis://localhost:6379" \
      "$WORKER_IMAGE"
fi

echo "App running."
