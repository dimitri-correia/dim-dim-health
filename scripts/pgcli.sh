#!/usr/bin/env bash
# Connect to the development PostgreSQL database using pgcli
set -euo pipefail

POSTGRES_USER="dev"
POSTGRES_PASSWORD="dev-db"
POSTGRES_DB="dimdimhealthdev"
DB_PORT="5432"

PGPASSWORD="$POSTGRES_PASSWORD" pgcli -h localhost -p "$DB_PORT" -U "$POSTGRES_USER" -d "$POSTGRES_DB"
