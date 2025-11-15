#!/bin/bash
# Restore script for DimDim Health
# This script restores the PostgreSQL database and Redis data from backups

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Configuration
BACKUP_DIR="/opt/dimdim-health-backups"
COMPOSE_FILE="docker-compose.yml"

# Check if backup directory exists
if [ ! -d "$BACKUP_DIR" ]; then
    print_error "Backup directory not found: $BACKUP_DIR"
    exit 1
fi

# List available backups
print_info "Available database backups:"
ls -lht "$BACKUP_DIR"/db_*.sql.gz 2>/dev/null | head -10

echo ""
print_info "Available Redis backups:"
ls -lht "$BACKUP_DIR"/redis_*.rdb 2>/dev/null | head -10

echo ""
read -p "Enter the database backup file name (e.g., db_20240115_120000.sql.gz): " DB_BACKUP
read -p "Enter the Redis backup file name (e.g., redis_20240115_120000.rdb): " REDIS_BACKUP

# Validate files exist
if [ ! -f "$BACKUP_DIR/$DB_BACKUP" ]; then
    print_error "Database backup file not found: $BACKUP_DIR/$DB_BACKUP"
    exit 1
fi

if [ ! -f "$BACKUP_DIR/$REDIS_BACKUP" ]; then
    print_error "Redis backup file not found: $BACKUP_DIR/$REDIS_BACKUP"
    exit 1
fi

# Warning
echo ""
print_warning "WARNING: This will replace all current data with the backup!"
print_warning "Current data will be PERMANENTLY LOST!"
echo ""
read -p "Are you sure you want to continue? (type 'yes' to confirm): " CONFIRM

if [ "$CONFIRM" != "yes" ]; then
    print_info "Restore cancelled."
    exit 0
fi

# Stop services
print_info "Stopping services..."
docker-compose -f "$COMPOSE_FILE" stop api worker

# Restore PostgreSQL database
print_info "Restoring PostgreSQL database..."
if gunzip -c "$BACKUP_DIR/$DB_BACKUP" | docker-compose -f "$COMPOSE_FILE" exec -T db psql -U dimdimhealth dimdimhealth; then
    print_info "Database restored successfully"
else
    print_error "Database restore failed"
    exit 1
fi

# Restore Redis data
print_info "Restoring Redis data..."
docker-compose -f "$COMPOSE_FILE" stop redis
docker cp "$BACKUP_DIR/$REDIS_BACKUP" dimdim-health-redis:/data/dump.rdb
docker-compose -f "$COMPOSE_FILE" start redis

print_info "Waiting for Redis to start..."
sleep 5

# Restart all services
print_info "Restarting all services..."
docker-compose -f "$COMPOSE_FILE" up -d

# Wait for health checks
print_info "Waiting for services to be healthy..."
sleep 20

# Verify services
if docker-compose -f "$COMPOSE_FILE" ps | grep -q "unhealthy"; then
    print_warning "Some services may be unhealthy. Check logs with: docker-compose logs"
else
    print_info "All services are running"
fi

print_info "Restore completed successfully!"
print_info "Please verify your data and test the application."
