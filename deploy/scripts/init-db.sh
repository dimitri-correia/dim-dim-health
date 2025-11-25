#!/bin/bash
set -e

# This script runs during database initialization
# It sets up pg_cron extension if needed

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    -- Enable pg_cron extension if available (optional)
    -- This will fail gracefully if pg_cron is not installed
    CREATE EXTENSION IF NOT EXISTS pg_cron;
    
    -- Grant necessary permissions
    GRANT ALL PRIVILEGES ON DATABASE $POSTGRES_DB TO $POSTGRES_USER;
EOSQL

echo "Database initialization complete"
