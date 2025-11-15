#!/bin/bash
# Automated backup script for DimDim Health
# This script backs up the PostgreSQL database and Redis data

set -e

# Configuration
BACKUP_DIR="/opt/dimdim-health-backups"
DATE=$(date +%Y%m%d_%H%M%S)
COMPOSE_FILE="docker-compose.yml"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create backup directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

# Check if services are running
if ! docker-compose -f "$COMPOSE_FILE" ps | grep -q "Up"; then
    print_error "Services are not running. Cannot perform backup."
    exit 1
fi

print_info "Starting backup at $DATE"

# Backup PostgreSQL database
print_info "Backing up PostgreSQL database..."
if docker-compose -f "$COMPOSE_FILE" exec -T db pg_dump -U dimdimhealth dimdimhealth | gzip > "$BACKUP_DIR/db_$DATE.sql.gz"; then
    print_info "Database backup completed: db_$DATE.sql.gz"
else
    print_error "Database backup failed"
    exit 1
fi

# Backup Redis data
print_info "Backing up Redis data..."
if docker-compose -f "$COMPOSE_FILE" exec -T redis redis-cli BGSAVE > /dev/null; then
    sleep 5  # Wait for background save to complete
    
    if docker cp dimdim-health-redis:/data/dump.rdb "$BACKUP_DIR/redis_$DATE.rdb" 2>/dev/null; then
        print_info "Redis backup completed: redis_$DATE.rdb"
    else
        print_error "Redis backup copy failed"
    fi
else
    print_error "Redis BGSAVE command failed"
fi

# Calculate backup sizes
DB_SIZE=$(du -h "$BACKUP_DIR/db_$DATE.sql.gz" 2>/dev/null | cut -f1)
REDIS_SIZE=$(du -h "$BACKUP_DIR/redis_$DATE.rdb" 2>/dev/null | cut -f1)

print_info "Backup sizes: Database=$DB_SIZE, Redis=$REDIS_SIZE"

# Cleanup old backups (keep last 7 days)
print_info "Cleaning up old backups (keeping last 7 days)..."
find "$BACKUP_DIR" -name "db_*.sql.gz" -mtime +7 -delete
find "$BACKUP_DIR" -name "redis_*.rdb" -mtime +7 -delete

# Count remaining backups
BACKUP_COUNT=$(find "$BACKUP_DIR" -name "db_*.sql.gz" | wc -l)
print_info "Total backups: $BACKUP_COUNT"

print_info "Backup completed successfully at $(date)"
