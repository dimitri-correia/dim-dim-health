# Production Deployment - Quick Reference

## Initial Setup (One-Time)

```bash
# 1. Clone repository
sudo mkdir -p /opt/dimdim-health
sudo chown $USER:$USER /opt/dimdim-health
cd /opt/dimdim-health
git clone https://github.com/dimitri-correia/dim-dim-health.git .

# 2. Create secrets file
cat > config/common.toml << EOF
gmail_password = "$(read -sp 'Gmail App Password: ' pwd; echo $pwd)"
jwt_secret = "$(openssl rand -base64 32)"
EOF

# 3. Create environment file
cat > .env << EOF
DB_PASSWORD=$(openssl rand -base64 24)
RUST_LOG=info
EOF

# 4. Start services
./scripts/quick-start.sh
```

## Regular Deployment (Updates)

```bash
cd /opt/dimdim-health
./scripts/deploy-production.sh
```

## Daily Operations

### View Logs
```bash
docker-compose logs -f              # All services
docker-compose logs -f api          # API only
docker-compose logs -f worker       # Worker only
```

### Check Status
```bash
docker-compose ps                   # Service status
curl http://localhost:3000/health   # API health
```

### Restart Services
```bash
docker-compose restart              # All services
docker-compose restart api          # API only
docker-compose restart worker       # Worker only
```

### Stop Services
```bash
docker-compose down                 # Stop all (keeps data)
docker-compose down -v              # Stop and delete volumes (⚠️ data loss)
```

## Backup & Restore

### Manual Backup
```bash
./scripts/backup.sh
```

### Automated Daily Backup (2 AM)
```bash
(crontab -l 2>/dev/null; echo "0 2 * * * /opt/dimdim-health/scripts/backup.sh") | crontab -
```

### Restore from Backup
```bash
./scripts/restore.sh
```

## Troubleshooting

### Services Not Starting
```bash
docker-compose logs                 # Check logs
docker-compose down && docker-compose up -d  # Restart
```

### Out of Memory
```bash
free -h                            # Check memory
docker stats                       # Container usage
# Edit config/prod.toml: number_workers = 2
```

### Database Issues
```bash
docker-compose exec db psql -U dimdimhealth -d dimdimhealth -c "SELECT 1;"
```

### Build Issues (Raspberry Pi)
```bash
# If build takes too long, increase swap
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

## File Structure

```
/opt/dimdim-health/
├── config/
│   ├── common.toml          # Secrets (not in git)
│   ├── dev.toml             # Development config
│   └── prod.toml            # Production config
├── .env                     # Environment variables (not in git)
├── docker-compose.yml       # Basic deployment
├── docker-compose.nginx.yml # With nginx reverse proxy
├── scripts/
│   ├── quick-start.sh       # Initial setup
│   ├── deploy-production.sh # Rolling update
│   ├── backup.sh            # Backup script
│   └── restore.sh           # Restore script
└── DEPLOYMENT.md            # Full documentation
```

## Security Checklist

- [ ] Change DB_PASSWORD in .env
- [ ] Set gmail_password in config/common.toml
- [ ] Set jwt_secret in config/common.toml (use: openssl rand -base64 32)
- [ ] Setup firewall: `sudo ufw allow 22/tcp && sudo ufw allow 3000/tcp && sudo ufw enable`
- [ ] Setup automated backups (cron)
- [ ] Keep system updated: `sudo apt-get update && sudo apt-get upgrade -y`
- [ ] Consider using nginx reverse proxy for SSL/TLS

## Important Notes

⚠️ **Never commit these files:**
- config/common.toml
- .env

✅ **Graceful Shutdown:**
- Worker: 60 seconds to finish jobs
- API: 30 seconds to finish requests
- Use: `docker-compose stop` (not `docker-compose kill`)

✅ **Zero Data Loss:**
- All data stored in Docker volumes
- Volumes persist through restarts
- Regular backups to /opt/dimdim-health-backups

## Support

- Full Documentation: DEPLOYMENT.md
- Check Logs: `docker-compose logs -f`
- GitHub Issues: https://github.com/dimitri-correia/dim-dim-health/issues
