# Production Deployment with Internet Access (SFR Box)

This guide explains how to deploy DimDim Health on a Raspberry Pi and make it accessible from the internet using an SFR box (French ISP router).

## Prerequisites

Before starting, ensure you have:
- A Raspberry Pi set up following [DEPLOYMENT.md](DEPLOYMENT.md)
- DimDim Health running locally (accessible at `http://<pi-ip>:3000`)
- Access to your SFR box admin interface
- A domain name (recommended) or use a Dynamic DNS service

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                           INTERNET                                   │
│                              │                                       │
│                              ▼                                       │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                        SFR Box                                  │ │
│  │                   (Public IP / NAT)                             │ │
│  │          Port 80/443 → Raspberry Pi:80/443                      │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                              │                                       │
│                              ▼                                       │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                    Raspberry Pi                                 │ │
│  │   ┌──────────────────────────────────────────────────────────┐ │ │
│  │   │                    Nginx                                   │ │ │
│  │   │   Port 80 (HTTP) → Let's Encrypt / HTTPS redirect         │ │ │
│  │   │   Port 443 (HTTPS) → API (localhost:3000)                 │ │ │
│  │   └──────────────────────────────────────────────────────────┘ │ │
│  │   ┌──────────────────────────────────────────────────────────┐ │ │
│  │   │              Docker Compose Stack                          │ │ │
│  │   │   API :3000 | Worker | PostgreSQL | Redis                 │ │ │
│  │   └──────────────────────────────────────────────────────────┘ │ │
│  └────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

## Step 1: Set a Static IP for Raspberry Pi

First, configure your Raspberry Pi with a static IP address on your local network.

### Option A: Via SFR Box (Recommended)

1. Access your SFR box admin interface: `http://192.168.1.1`
2. Login with your admin credentials
3. Navigate to: **Réseau v4** → **DHCP**
4. Find your Raspberry Pi in the device list
5. Click on it and select **Attribuer une IP statique** (Assign static IP)
6. Choose an IP like `192.168.1.100`
7. Save and reboot the Pi

### Option B: Via Raspberry Pi

Edit the DHCP client configuration:

```bash
sudo nano /etc/dhcpcd.conf
```

Add at the end:

```conf
interface eth0
static ip_address=192.168.1.100/24
static routers=192.168.1.1
static domain_name_servers=192.168.1.1 8.8.8.8
```

Restart networking:

```bash
sudo systemctl restart dhcpcd
```

## Step 2: Configure SFR Box Port Forwarding

### Access SFR Box Admin Interface

1. Open your browser and go to: `http://192.168.1.1`
2. Login with your SFR box credentials
   - Default username: `admin`
   - Default password: First 8 characters of your WiFi key (on the box sticker)

### Configure NAT/Port Forwarding

1. Navigate to: **Réseau v4** → **NAT**
2. Click on **Ajouter une règle** (Add a rule)

#### For HTTP (port 80):

| Field | Value |
|-------|-------|
| Nom | DimDim-HTTP |
| Protocole | TCP |
| Type | Port |
| Port externe | 80 |
| Port interne | 80 |
| Équipement | [Select your Raspberry Pi] |
| IP de destination | 192.168.1.100 |

#### For HTTPS (port 443):

| Field | Value |
|-------|-------|
| Nom | DimDim-HTTPS |
| Protocole | TCP |
| Type | Port |
| Port externe | 443 |
| Port interne | 443 |
| Équipement | [Select your Raspberry Pi] |
| IP de destination | 192.168.1.100 |

3. Click **Valider** (Validate) for each rule
4. Click **Redémarrer la Box** if prompted (some SFR boxes require a restart)

### Verify Port Forwarding

From outside your network (use mobile data), test the ports:

```bash
# Test HTTP
curl -v http://YOUR_PUBLIC_IP:80

# Test HTTPS (after SSL setup)
curl -v https://YOUR_PUBLIC_IP:443
```

You can find your public IP at: https://whatismyip.com

## Step 3: Set Up Dynamic DNS (DDNS)

Since residential connections typically have dynamic public IPs, set up DDNS to have a stable domain name.

### Option A: No-IP (Free)

1. Create an account at https://www.noip.com
2. Create a hostname (e.g., `dimdimhealth.ddns.net`)
3. Install the No-IP client on Raspberry Pi:

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential

# Download and install No-IP client
cd /tmp
wget http://www.noip.com/client/linux/noip-duc-linux.tar.gz
tar xzf noip-duc-linux.tar.gz
cd noip-2.1.9-1
sudo make install

# Configure (enter your No-IP credentials)
sudo /usr/local/bin/noip2 -C

# Create systemd service
sudo tee /etc/systemd/system/noip2.service > /dev/null << 'EOF'
[Unit]
Description=No-IP Dynamic DNS Update Client
After=network.target

[Service]
Type=forking
ExecStart=/usr/local/bin/noip2
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable noip2
sudo systemctl start noip2
```

### Option B: DuckDNS (Free)

1. Go to https://www.duckdns.org and login with Google/GitHub
2. Create a subdomain (e.g., `dimdimhealth.duckdns.org`)
3. Note your token

```bash
# Create update script
sudo mkdir -p /opt/duckdns
sudo tee /opt/duckdns/duck.sh > /dev/null << 'EOF'
#!/bin/bash
echo url="https://www.duckdns.org/update?domains=YOUR_SUBDOMAIN&token=YOUR_TOKEN&ip=" | curl -k -o /opt/duckdns/duck.log -K -
EOF

# Replace YOUR_SUBDOMAIN and YOUR_TOKEN
sudo nano /opt/duckdns/duck.sh

# Make executable
sudo chmod +x /opt/duckdns/duck.sh

# Add to crontab (update every 5 minutes)
(crontab -l 2>/dev/null; echo "*/5 * * * * /opt/duckdns/duck.sh >/dev/null 2>&1") | crontab -
```

### Option B: OVH DynHost (If you have an OVH domain)

If you own a domain at OVH, you can use their free DynHost service:

1. Go to OVH Manager → Your Domain → DynHost
2. Create a DynHost identifier
3. Install ddclient:

```bash
sudo apt-get install -y ddclient

# Configure ddclient
sudo tee /etc/ddclient.conf > /dev/null << 'EOF'
protocol=dyndns2
use=web
server=www.ovh.com
login=YOUR_DOMAIN-YOUR_IDENTIFIER
password='YOUR_DYNHOST_PASSWORD'
YOUR_SUBDOMAIN.YOUR_DOMAIN.com
EOF

# Restart ddclient
sudo systemctl restart ddclient
sudo systemctl enable ddclient
```

## Step 4: Install SSL Certificates with Let's Encrypt

### Install Certbot

```bash
sudo apt-get update
sudo apt-get install -y certbot
```

### Stop Nginx temporarily (if running)

```bash
docker-compose -f deploy/docker-compose.nginx.yml stop nginx
```

### Obtain Certificate

```bash
# Replace with your domain
export DOMAIN="your-domain.com"

# Obtain certificate using standalone mode
sudo certbot certonly --standalone \
  --preferred-challenges http \
  --agree-tos \
  --email your-email@example.com \
  -d $DOMAIN
```

### Copy Certificates to Nginx

```bash
# Create SSL directory
sudo mkdir -p /opt/dimdim-health/deploy/nginx/ssl

# Copy certificates
sudo cp /etc/letsencrypt/live/$DOMAIN/fullchain.pem /opt/dimdim-health/deploy/nginx/ssl/
sudo cp /etc/letsencrypt/live/$DOMAIN/privkey.pem /opt/dimdim-health/deploy/nginx/ssl/

# Set permissions
sudo chown -R $USER:$USER /opt/dimdim-health/deploy/nginx/ssl
chmod 600 /opt/dimdim-health/deploy/nginx/ssl/*.pem
```

### Configure Nginx for HTTPS

Edit the nginx configuration to enable HTTPS:

```bash
nano /opt/dimdim-health/deploy/nginx/nginx.conf
```

Update the configuration (uncomment the HTTPS server block and add your domain):

```nginx
events {
    worker_connections 1024;
}

http {
    # Basic settings
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;
    client_max_body_size 20M;

    # Logging
    access_log /var/log/nginx/access.log;
    error_log /var/log/nginx/error.log;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types text/plain text/css text/xml text/javascript application/json application/javascript application/xml+rss;

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;
    limit_req_status 429;

    # Upstream API servers
    upstream api_backend {
        server api:3000;
        keepalive 32;
    }

    # HTTP Server - Redirect to HTTPS
    server {
        listen 80;
        server_name your-domain.com;

        # For Let's Encrypt challenges
        location /.well-known/acme-challenge/ {
            root /var/www/certbot;
        }

        # Redirect all other HTTP to HTTPS
        location / {
            return 301 https://$host$request_uri;
        }
    }

    # HTTPS Server
    server {
        listen 443 ssl http2;
        server_name your-domain.com;

        # SSL certificates
        ssl_certificate /etc/nginx/ssl/fullchain.pem;
        ssl_certificate_key /etc/nginx/ssl/privkey.pem;

        # SSL configuration
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers HIGH:!aNULL:!MD5;
        ssl_prefer_server_ciphers on;
        ssl_session_cache shared:SSL:10m;
        ssl_session_timeout 10m;

        # Security headers
        add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
        add_header X-Frame-Options "SAMEORIGIN" always;
        add_header X-Content-Type-Options "nosniff" always;
        add_header X-XSS-Protection "1; mode=block" always;

        # Health check endpoint
        location /health {
            proxy_pass http://api_backend/health;
            proxy_http_version 1.1;
            proxy_set_header Connection "";
            access_log off;
        }

        # API endpoints
        location / {
            proxy_pass http://api_backend;
            proxy_http_version 1.1;
            
            # Headers
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_set_header Connection "";
            
            # Timeouts
            proxy_connect_timeout 60s;
            proxy_send_timeout 60s;
            proxy_read_timeout 60s;
            
            # Rate limiting
            limit_req zone=api_limit burst=20 nodelay;
        }
    }
}
```

### Set Up Automatic Certificate Renewal

```bash
# Create renewal script
sudo tee /opt/dimdim-health/deploy/scripts/renew-certs.sh > /dev/null << 'EOF'
#!/bin/bash
set -e

DOMAIN="your-domain.com"
DEPLOY_DIR="/opt/dimdim-health"

# Renew certificate
certbot renew --quiet

# Copy new certificates
cp /etc/letsencrypt/live/$DOMAIN/fullchain.pem $DEPLOY_DIR/deploy/nginx/ssl/
cp /etc/letsencrypt/live/$DOMAIN/privkey.pem $DEPLOY_DIR/deploy/nginx/ssl/

# Reload nginx
docker-compose -f $DEPLOY_DIR/deploy/docker-compose.nginx.yml exec nginx nginx -s reload
EOF

sudo chmod +x /opt/dimdim-health/deploy/scripts/renew-certs.sh

# Add to crontab (check daily at 3 AM)
(crontab -l 2>/dev/null; echo "0 3 * * * /opt/dimdim-health/deploy/scripts/renew-certs.sh >> /var/log/cert-renewal.log 2>&1") | crontab -
```

## Step 5: Deploy with Nginx

Use the nginx-enabled docker-compose configuration:

```bash
cd /opt/dimdim-health

# Start all services including nginx
docker-compose -f deploy/docker-compose.nginx.yml up -d

# Verify services are running
docker-compose -f deploy/docker-compose.nginx.yml ps

# Check nginx logs
docker-compose -f deploy/docker-compose.nginx.yml logs nginx
```

## Step 6: Configure Firewall

Secure your Raspberry Pi with UFW:

```bash
# Install UFW
sudo apt-get install -y ufw

# Default policies
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow SSH (important!)
sudo ufw allow 22/tcp

# Allow HTTP and HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Enable firewall
sudo ufw enable

# Check status
sudo ufw status verbose
```

## Step 7: Verify Everything

### Test from Internet

From a device outside your network (mobile data or ask a friend):

```bash
# Test HTTPS
curl -v https://your-domain.com/health

# Should return: {"status":"ok"}
```

### Test SSL Certificate

```bash
# Check SSL certificate details
openssl s_client -connect your-domain.com:443 -servername your-domain.com
```

Or use: https://www.ssllabs.com/ssltest/

## Security Best Practices

### 1. Keep System Updated

```bash
# Update weekly
sudo apt-get update && sudo apt-get upgrade -y

# Enable automatic security updates
sudo apt-get install -y unattended-upgrades
sudo dpkg-reconfigure -plow unattended-upgrades
```

### 2. Secure SSH

```bash
# Disable root login and password authentication
sudo nano /etc/ssh/sshd_config
```

Set:
```
PermitRootLogin no
PasswordAuthentication no  # Only if you have SSH keys setup
```

```bash
sudo systemctl restart sshd
```

### 3. Install Fail2ban

```bash
# Install fail2ban
sudo apt-get install -y fail2ban

# Create local configuration
sudo tee /etc/fail2ban/jail.local > /dev/null << 'EOF'
[DEFAULT]
bantime = 1h
findtime = 10m
maxretry = 5

[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log

[nginx-http-auth]
enabled = true
port = http,https
filter = nginx-http-auth
logpath = /var/log/nginx/error.log
EOF

sudo systemctl restart fail2ban
sudo systemctl enable fail2ban
```

### 4. Monitor Your System

```bash
# Install monitoring tools
sudo apt-get install -y htop iotop nethogs

# Check active connections
sudo netstat -tlnp

# Monitor network usage
sudo nethogs
```

### 5. Regular Backups

Ensure automated backups are configured (see [QUICKSTART.md](QUICKSTART.md)):

```bash
# Setup daily backups at 2 AM
(crontab -l 2>/dev/null; echo "0 2 * * * /opt/dimdim-health/deploy/scripts/backup.sh") | crontab -
```

## Troubleshooting

### Port Forwarding Not Working

1. **Check SFR box firewall**: Some SFR boxes have a separate firewall section. Ensure ports 80/443 are not blocked.

2. **Check if ports are open**:
   ```bash
   # From Pi
   sudo netstat -tlnp | grep -E '80|443'
   ```

3. **Test locally first**:
   ```bash
   curl http://localhost/health
   curl https://localhost/health -k
   ```

4. **Check nginx is running**:
   ```bash
   docker-compose -f deploy/docker-compose.nginx.yml ps nginx
   docker-compose -f deploy/docker-compose.nginx.yml logs nginx
   ```

### DDNS Not Updating

1. **Check No-IP status**:
   ```bash
   sudo systemctl status noip2
   sudo /usr/local/bin/noip2 -S
   ```

2. **Check DuckDNS log**:
   ```bash
   cat /opt/duckdns/duck.log
   # Should show: OK
   ```

### SSL Certificate Issues

1. **Check certificate files exist**:
   ```bash
   ls -la /opt/dimdim-health/deploy/nginx/ssl/
   ```

2. **Verify certificate**:
   ```bash
   openssl x509 -in /opt/dimdim-health/deploy/nginx/ssl/fullchain.pem -text -noout
   ```

3. **Check nginx SSL configuration**:
   ```bash
   docker-compose -f deploy/docker-compose.nginx.yml exec nginx nginx -t
   ```

### SFR Box Specific Issues

1. **DMZ Mode** (Not recommended, but for testing):
   - Navigate to **Réseau v4** → **NAT** → **DMZ**
   - Enable and point to your Raspberry Pi IP
   - This exposes ALL ports - only use for testing

2. **IPv6**: SFR boxes support IPv6. If you have IPv6:
   - Your Pi might be directly accessible without NAT
   - Configure firewall rules for IPv6 as well

3. **UPnP**: Some SFR boxes support UPnP:
   - Navigate to **Réseau v4** → **UPnP**
   - Enable if you want automatic port forwarding (less secure)

### SFR Box Firmware Updates

SFR periodically updates box firmware which might reset settings:
- Document your port forwarding rules
- Re-apply after firmware updates if needed

## Complete Deployment Checklist

- [ ] Raspberry Pi has static IP (192.168.1.100)
- [ ] DimDim Health running locally (`curl http://localhost:3000/health`)
- [ ] SFR box port forwarding configured (80 → Pi, 443 → Pi)
- [ ] Dynamic DNS set up and updating
- [ ] SSL certificates obtained from Let's Encrypt
- [ ] Nginx configured with HTTPS
- [ ] Services started with nginx compose file
- [ ] UFW firewall enabled (22, 80, 443 open)
- [ ] Can access from internet (`curl https://your-domain.com/health`)
- [ ] Fail2ban installed and configured
- [ ] Automatic certificate renewal configured
- [ ] Daily backups configured
- [ ] System automatic updates enabled

## Summary

Your DimDim Health application is now:
- ✅ Accessible from the internet via `https://your-domain.com`
- ✅ Secured with SSL/TLS encryption
- ✅ Protected by rate limiting and firewall
- ✅ Automatically updating IP via DDNS
- ✅ Auto-renewing SSL certificates
- ✅ Backed up daily

For day-to-day operations, see [QUICKSTART.md](QUICKSTART.md).
For detailed local deployment, see [DEPLOYMENT.md](DEPLOYMENT.md).
