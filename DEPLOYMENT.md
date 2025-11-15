# Production Deployment Guide for Raspberry Pi

This guide explains how to deploy DimDim Health on a Raspberry Pi in your network with zero data loss and graceful shutdown capabilities.

## Architecture Overview

The deployment uses Docker Compose with the following services:
- **PostgreSQL** (database with persistent storage)
- **Redis** (job queue with persistent storage)
- **API** (REST API service)
- **Worker** (background job processor)

## Features

- ✅ **Graceful Shutdown**: Both API and Worker wait for current operations to complete
- ✅ **Zero Data Loss**: All data is persisted to volumes
- ✅ **Rolling Updates**: Deploy new versions without complete downtime
- ✅ **Health Checks**: Automatic monitoring of service health
- ✅ **Auto-restart**: Services automatically restart on failure
- ✅ **ARM64 Support**: Optimized for Raspberry Pi

## Prerequisites

### 1. Raspberry Pi Setup
- Raspberry Pi 3B+ or newer (4GB+ RAM recommended)
- Raspberry Pi OS (64-bit) or Ubuntu Server ARM64
- At least 32GB SD card or SSD (SSD highly recommended)

### 2. Install Docker and Docker Compose

```bash
# Update system
sudo apt-get update && sudo apt-get upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add your user to docker group
sudo usermod -aG docker $USER

# Log out and back in for group changes to take effect

# Install Docker Compose
sudo apt-get install -y docker-compose

# Verify installation
docker --version
docker-compose --version
```

### 3. Clone Repository

```bash
# Clone to /opt for production deployment
sudo mkdir -p /opt/dimdim-health
sudo chown $USER:$USER /opt/dimdim-health
cd /opt/dimdim-health
git clone https://github.com/dimitri-correia/dim-dim-health.git .
```

## Configuration

### 1. Create Common Configuration File

Create `config/common.toml` with your secrets (this file is in .gitignore):

```toml
# Gmail credentials for email sending
gmail_password = "your_gmail_app_password_here"

# JWT secret for authentication (generate a random string)
jwt_secret = "your_secure_random_jwt_secret_here"
```

**Important**: 
- Never commit `config/common.toml` to git (it's already in .gitignore)
- Use a Gmail App Password (not your regular password)
- Generate a strong random JWT secret: `openssl rand -base64 32`

### 2. Environment Variables (Optional)

You can override settings using environment variables in a `.env` file:

```bash
# Create .env file in the root directory
cat > .env << EOF
# Database password
DB_PASSWORD=your_secure_db_password

# Rust log level (optional)
RUST_LOG=info
EOF
```

### 3. Network Configuration

By default, the API is accessible on port 3000. To access it from other devices:
- Local network: `http://<raspberry-pi-ip>:3000`
- For external access, configure your router's port forwarding

## Deployment

### First Time Deployment

```bash
cd /opt/dimdim-health

# Build images (this may take 15-30 minutes on Raspberry Pi)
docker-compose build

# Start services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f
```

### Automated Deployment Script

For updates with zero downtime and no data loss:

```bash
cd /opt/dimdim-health

# Run the deployment script
./scripts/deploy-production.sh
```

The script will:
1. Pull latest code changes (if using git)
2. Build new Docker images
3. Deploy worker first (finishing current jobs)
4. Run database migrations
5. Deploy API with rolling update
6. Verify health checks
7. Clean up old images

### Manual Rolling Update

If you prefer to update manually:

```bash
# Pull latest changes
git pull

# Rebuild images
docker-compose build

# Update worker (will finish current jobs before stopping)
docker-compose up -d --no-deps worker

# Wait for workers to finish (check logs)
docker-compose logs -f worker

# Update API (will wait for in-flight requests)
docker-compose up -d --no-deps api

# Check health
curl http://localhost:3000/health
```

## Automatic Startup (Systemd)

To automatically start services on boot:

```bash
# Copy systemd service file
sudo cp scripts/dimdim-health.service /etc/systemd/system/

# Edit the WorkingDirectory in the service file if needed
sudo nano /etc/systemd/system/dimdim-health.service

# Reload systemd
sudo systemctl daemon-reload

# Enable service to start on boot
sudo systemctl enable dimdim-health.service

# Start service now
sudo systemctl start dimdim-health.service

# Check status
sudo systemctl status dimdim-health.service
```

## Monitoring and Management

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f api
docker-compose logs -f worker

# Last 100 lines
docker-compose logs --tail=100 api
```

### Check Service Status

```bash
# All services
docker-compose ps

# Detailed information
docker-compose top

# Resource usage
docker stats
```

### Health Checks

```bash
# API health endpoint
curl http://localhost:3000/health

# Check all container health
docker-compose ps
```

### Database Access

```bash
# Connect to PostgreSQL
docker-compose exec db psql -U dimdimhealth -d dimdimhealth

# Create backup
docker-compose exec db pg_dump -U dimdimhealth dimdimhealth > backup.sql

# Restore backup
cat backup.sql | docker-compose exec -T db psql -U dimdimhealth dimdimhealth
```

### Redis Access

```bash
# Connect to Redis CLI
docker-compose exec redis redis-cli

# Check queue length
docker-compose exec redis redis-cli LLEN jobs

# View memory usage
docker-compose exec redis redis-cli INFO memory
```

## Backup and Restore

### Automated Backup Script

Create a backup script:

```bash
#!/bin/bash
BACKUP_DIR="/opt/dimdim-health-backups"
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p $BACKUP_DIR

# Backup database
docker-compose exec -T db pg_dump -U dimdimhealth dimdimhealth | gzip > $BACKUP_DIR/db_$DATE.sql.gz

# Backup Redis data
docker-compose exec -T redis redis-cli BGSAVE
sleep 5
docker cp dimdim-health-redis:/data/dump.rdb $BACKUP_DIR/redis_$DATE.rdb

# Keep only last 7 days of backups
find $BACKUP_DIR -name "*.gz" -mtime +7 -delete
find $BACKUP_DIR -name "*.rdb" -mtime +7 -delete

echo "Backup completed: $DATE"
```

### Setup Automated Daily Backups

```bash
# Create backup script
sudo nano /opt/dimdim-health/scripts/backup.sh
# (paste the script above)
sudo chmod +x /opt/dimdim-health/scripts/backup.sh

# Add to crontab (daily at 2 AM)
(crontab -l 2>/dev/null; echo "0 2 * * * /opt/dimdim-health/scripts/backup.sh") | crontab -
```

## Troubleshooting

### Services Won't Start

```bash
# Check Docker daemon
sudo systemctl status docker

# Check logs for errors
docker-compose logs

# Remove and recreate containers
docker-compose down
docker-compose up -d
```

### Out of Memory

```bash
# Check memory usage
free -h
docker stats

# Reduce worker count in config/prod.toml
number_workers = 2  # Reduce from 3 to 2
```

### Database Connection Issues

```bash
# Check database is running
docker-compose ps db

# Check database logs
docker-compose logs db

# Test connection
docker-compose exec db psql -U dimdimhealth -d dimdimhealth -c "SELECT 1;"
```

### Build Takes Too Long

```bash
# Use cached images when possible
docker-compose build --pull

# Build on a faster machine and export/import
# On fast machine:
docker save dimdim-health-api:latest | gzip > api-image.tar.gz
docker save dimdim-health-worker:latest | gzip > worker-image.tar.gz

# On Raspberry Pi:
gunzip -c api-image.tar.gz | docker load
gunzip -c worker-image.tar.gz | docker load
```

### Port Already in Use

```bash
# Check what's using port 3000
sudo lsof -i :3000

# Change port in docker-compose.yml
ports:
  - "3001:3000"  # Use 3001 externally, 3000 internally
```

## Performance Optimization

### 1. Use SSD Instead of SD Card
SSDs are much faster and more reliable than SD cards for database workloads.

### 2. Adjust Worker Count
Based on your Raspberry Pi model:
- Pi 3B+: 2 workers
- Pi 4 (4GB): 3 workers
- Pi 4 (8GB): 5 workers

Edit `config/prod.toml`:
```toml
number_workers = 2
```

### 3. Enable Swap (if needed)

```bash
# Create 2GB swap file
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Make permanent
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```

### 4. Monitor Resource Usage

```bash
# Install monitoring tools
sudo apt-get install -y htop iotop

# Monitor in real-time
htop
```

## Security Recommendations

1. **Change Default Passwords**: Update all passwords in `config/common.toml`
2. **Firewall**: Configure UFW to restrict access
   ```bash
   sudo ufw allow 22/tcp  # SSH
   sudo ufw allow 3000/tcp  # API
   sudo ufw enable
   ```
3. **HTTPS**: Use a reverse proxy (nginx) with Let's Encrypt for HTTPS
4. **Regular Updates**: Keep system and Docker updated
   ```bash
   sudo apt-get update && sudo apt-get upgrade -y
   ```
5. **Backup Regularly**: Setup automated backups (see Backup section)

## Maintenance

### Update Application

```bash
cd /opt/dimdim-health
./scripts/deploy-production.sh
```

### Update System

```bash
# Update OS packages
sudo apt-get update && sudo apt-get upgrade -y

# Update Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
```

### Clean Up Docker

```bash
# Remove unused images
docker image prune -a -f

# Remove unused volumes (BE CAREFUL!)
docker volume prune -f

# Complete cleanup
docker system prune -a -f
```

## Support

For issues or questions:
- GitHub Issues: https://github.com/dimitri-correia/dim-dim-health/issues
- Check logs: `docker-compose logs -f`
- Review this documentation

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Raspberry Pi                          │
│                                                          │
│  ┌────────────────────────────────────────────────────┐ │
│  │              Docker Compose Network                 │ │
│  │                                                     │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────────┐│ │
│  │  │          │  │          │  │                  ││ │
│  │  │   API    │  │  Worker  │  │    PostgreSQL    ││ │
│  │  │  :3000   │  │          │  │      :5432       ││ │
│  │  │          │  │          │  │                  ││ │
│  │  └────┬─────┘  └────┬─────┘  └────────┬─────────┘│ │
│  │       │             │                   │          │ │
│  │       └─────────────┼───────────────────┘          │ │
│  │                     │                              │ │
│  │                ┌────┴──────┐                       │ │
│  │                │           │                       │ │
│  │                │   Redis   │                       │ │
│  │                │   :6379   │                       │ │
│  │                │           │                       │ │
│  │                └───────────┘                       │ │
│  │                                                     │ │
│  │  Volumes: postgres_data, redis_data                │ │
│  └────────────────────────────────────────────────────┘ │
│                                                          │
│         Access: http://<pi-ip>:3000                      │
└─────────────────────────────────────────────────────────┘
```

## Deployment Flow

```
1. Build Phase
   └─> Build Docker images for API and Worker

2. Worker Update
   ├─> Scale up workers (old + new running)
   ├─> New workers start processing jobs
   ├─> Old workers finish current jobs
   └─> Old workers gracefully shutdown

3. Database Migration
   └─> Run migrations (if any)

4. API Update
   ├─> Start new API container
   ├─> Health check passes
   ├─> Old API finishes in-flight requests
   └─> Old API gracefully shutdown

5. Verification
   └─> Check all services are healthy
```

## Summary

This deployment solution provides:
- **Zero Data Loss**: All data persisted to Docker volumes
- **Graceful Shutdown**: Both API and Worker wait for operations to complete
- **Rolling Updates**: Deploy without complete downtime
- **Easy Management**: Simple scripts and systemd integration
- **Production Ready**: Health checks, auto-restart, and monitoring
- **Raspberry Pi Optimized**: ARM64 support and performance tuning

For a typical deployment, simply run:
```bash
cd /opt/dimdim-health
./scripts/deploy-production.sh
```

The script handles everything automatically with zero data loss!
