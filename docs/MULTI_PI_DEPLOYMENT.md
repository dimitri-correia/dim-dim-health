# Multi-Raspberry Pi Deployment Guide

This document provides recommendations for distributing DimDim Health services across two Raspberry Pis to optimize performance and resource utilization.

## Why Split Services?

OpenObserve is a powerful observability platform, but it can be resource-intensive:
- **Memory**: OpenObserve can use 500MB-2GB+ RAM depending on data volume
- **CPU**: Log ingestion and querying can spike CPU usage
- **Disk I/O**: Continuous writes for log storage

Running all services on a single Raspberry Pi (especially Pi 3B+ or Pi 4 with 2-4GB RAM) may cause:
- Slow API response times
- Worker job delays
- System instability under load

## Recommended Architecture

### Pi 1: Application Services (Main)
Focus on core application functionality:

| Service | Port | Purpose |
|---------|------|---------|
| PostgreSQL | 5432 | Primary database |
| Redis | 6379 | Job queue |
| API | 3000 | REST API |
| Worker | - | Background jobs |

**Estimated Resources:**
- RAM: 1-2GB
- CPU: Light to moderate
- Storage: Depends on database size

### Pi 2: Observability Services
Dedicated to monitoring and logging:

| Service | Port | Purpose |
|---------|------|---------|
| OpenObserve | 5080 | Logs, metrics, traces |
| (Optional) Prometheus | 9090 | Metrics collection |
| (Optional) Grafana | 3001 | Dashboards |

**Estimated Resources:**
- RAM: 1-3GB (OpenObserve alone)
- CPU: Moderate to high during queries
- Storage: High (logs accumulate over time)

## Network Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          Local Network                                   │
│                                                                          │
│  ┌─────────────────────────────┐    ┌─────────────────────────────────┐ │
│  │      Pi 1 (Application)     │    │      Pi 2 (Observability)       │ │
│  │                             │    │                                  │ │
│  │  ┌───────────────────────┐  │    │  ┌────────────────────────────┐ │ │
│  │  │    Docker Network     │  │    │  │      Docker Network        │ │ │
│  │  │                       │  │    │  │                            │ │ │
│  │  │  ┌─────┐   ┌───────┐  │  │    │  │  ┌────────────────────┐   │ │ │
│  │  │  │ API │   │Worker │  │  │    │  │  │    OpenObserve     │   │ │ │
│  │  │  │:3000│   │       │  │──┼────┼──┼─>│       :5080        │   │ │ │
│  │  │  └──┬──┘   └───┬───┘  │  │    │  │  └────────────────────┘   │ │ │
│  │  │     │         │       │  │    │  │                            │ │ │
│  │  │  ┌──┴─────────┴──┐    │  │    │  └────────────────────────────┘ │ │
│  │  │  │  PostgreSQL   │    │  │    │                                  │ │
│  │  │  │    :5432      │    │  │    │  IP: 192.168.1.102               │ │
│  │  │  └───────────────┘    │  │    └─────────────────────────────────┘ │
│  │  │  ┌───────────────┐    │  │                                        │
│  │  │  │    Redis      │    │  │                                        │
│  │  │  │    :6379      │    │  │                                        │
│  │  │  └───────────────┘    │  │                                        │
│  │  └───────────────────────┘  │                                        │
│  │                             │                                        │
│  │  IP: 192.168.1.101          │                                        │
│  └─────────────────────────────┘                                        │
│                                                                          │
│              Access API: http://192.168.1.101:3000                       │
│              Access Logs: http://192.168.1.102:5080                      │
└─────────────────────────────────────────────────────────────────────────┘
```

## Implementation

### Step 1: Setup Pi 1 (Application Services)

Create `deploy/docker-compose.app.yml`:

```yaml
version: '3.8'

services:
  db:
    image: postgres:18
    container_name: dimdim-health-db
    restart: unless-stopped
    environment:
      POSTGRES_DB: dimdimhealth
      POSTGRES_USER: dimdimhealth
      POSTGRES_PASSWORD: ${DB_PASSWORD:-changeme}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./scripts/init-db.sh:/docker-entrypoint-initdb.d/init-db.sh:ro
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U dimdimhealth -d dimdimhealth"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - dimdim-network

  redis:
    image: redis:7-alpine
    container_name: dimdim-health-redis
    restart: unless-stopped
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data
    ports:
      - "6379:6379"
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - dimdim-network

  api:
    build:
      context: ..
      dockerfile: deploy/Dockerfile.api
    container_name: dimdim-health-api
    restart: unless-stopped
    environment:
      APP_ENV: prod
      RUST_LOG: info
      OPENOBSERVE_URL: ${OPENOBSERVE_URL:-http://192.168.1.102:5080}  # Point to Pi 2
    volumes:
      - ../config:/app/config:ro
    ports:
      - "3000:3000"
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_healthy
    healthcheck:
      test: ["CMD-SHELL", "wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1"]
      interval: 30s
      timeout: 10s
      start_period: 40s
      retries: 3
    networks:
      - dimdim-network
    stop_grace_period: 30s

  worker:
    build:
      context: ..
      dockerfile: deploy/Dockerfile.worker
    container_name: dimdim-health-worker
    restart: unless-stopped
    environment:
      APP_ENV: prod
      RUST_LOG: info
      OPENOBSERVE_URL: ${OPENOBSERVE_URL:-http://192.168.1.102:5080}  # Point to Pi 2
    volumes:
      - ../config:/app/config:ro
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_healthy
    networks:
      - dimdim-network
    stop_grace_period: 60s

networks:
  dimdim-network:
    driver: bridge

volumes:
  postgres_data:
    driver: local
  redis_data:
    driver: local
```

### Step 2: Setup Pi 2 (Observability)

Create `deploy/docker-compose.observability.yml`:

```yaml
version: '3.8'

services:
  openobserve:
    image: public.ecr.aws/zinclabs/openobserve:latest
    container_name: dimdim-health-openobserve
    restart: unless-stopped
    environment:
      ZO_ROOT_USER_EMAIL: ${OPENOBSERVE_EMAIL:-admin@example.com}
      ZO_ROOT_USER_PASSWORD: ${OPENOBSERVE_PASSWORD:?OpenObserve password is required}
      ZO_DATA_DIR: "/data"
    ports:
      - "5080:5080"
    volumes:
      - openobserve_data:/data
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:5080/healthz"]
      interval: 30s
      timeout: 10s
      retries: 5
      start_period: 30s
    networks:
      - observability-network

networks:
  observability-network:
    driver: bridge

volumes:
  openobserve_data:
    driver: local
```

### Step 3: Configure Environment Variables

On Pi 1, create a `.env` file:
```bash
cat > /opt/dimdim-health/.env << EOF
DB_PASSWORD=your_secure_db_password
OPENOBSERVE_URL=http://<Pi2-IP>:5080
EOF
```

On Pi 2, create a `.env` file:
```bash
cat > /opt/dimdim-health/.env << EOF
OPENOBSERVE_EMAIL=admin@example.com
OPENOBSERVE_PASSWORD=your_secure_openobserve_password
EOF
```

### Step 4: Deploy

On Pi 1 (Application):
```bash
cd /opt/dimdim-health
docker-compose -f deploy/docker-compose.app.yml up -d
```

On Pi 2 (Observability):
```bash
cd /opt/dimdim-health
docker-compose -f deploy/docker-compose.observability.yml up -d
```

### Step 5: Configure Log Shipping

Update your application configuration to send logs to Pi 2's OpenObserve instance by setting the `OPENOBSERVE_URL` environment variable to `http://<Pi2-IP>:5080`.

## Resource Comparison

### Single Pi (All Services)

| Resource | Usage | Risk |
|----------|-------|------|
| RAM | 2-4GB | High - may cause OOM |
| CPU | 70-100% under load | High - slow responses |
| Disk I/O | High | Medium - SD card wear |

### Split Deployment (Two Pis)

**Pi 1 (Application):**
| Resource | Usage | Risk |
|----------|-------|------|
| RAM | 1-2GB | Low |
| CPU | 30-60% | Low |
| Disk I/O | Medium | Low |

**Pi 2 (Observability):**
| Resource | Usage | Risk |
|----------|-------|------|
| RAM | 1-3GB | Medium |
| CPU | 20-80% (query dependent) | Medium |
| Disk I/O | High | Medium |

## Considerations

### Pros of Splitting
- **Better Performance**: Core application runs smoothly
- **Isolation**: OpenObserve issues don't affect the app
- **Scalability**: Can upgrade observability Pi independently
- **Reliability**: Application stays responsive during log queries

### Cons of Splitting
- **Network Dependency**: Logs must travel over network
- **Complexity**: Two machines to manage
- **Cost**: Requires second Raspberry Pi
- **Latency**: Slight delay in log delivery

## Alternative: Run Without OpenObserve

If you don't need advanced observability, you can simplify by:
1. Using the basic `docker-compose.yml` without OpenObserve
2. Relying on `docker logs` for basic troubleshooting
3. Adding OpenObserve later when needed

## Recommendations by Pi Model

| Pi Model | RAM | Recommendation |
|----------|-----|----------------|
| Pi 3B+ | 1GB | Split required - won't run all services |
| Pi 4 (2GB) | 2GB | Split recommended |
| Pi 4 (4GB) | 4GB | Can run together, split for best performance |
| Pi 4 (8GB) | 8GB | Can run together comfortably |
| Pi 5 | 4-8GB | Can run together comfortably |

## Summary

For optimal performance with two Raspberry Pis:
1. **Pi 1**: Run PostgreSQL, Redis, API, and Worker
2. **Pi 2**: Run OpenObserve (and any other monitoring tools)

This separation ensures your application remains responsive while still having full observability capabilities.
