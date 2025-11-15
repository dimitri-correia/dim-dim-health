# DIMDIM HEALTH

A health tracking application with API and background worker services.

## Quick Start

For production deployment on Raspberry Pi, see [DEPLOYMENT.md](DEPLOYMENT.md).

For development setup:

```bash
# Start development database
./scripts/dev-db/start-dev-env.sh

# Run API
cargo run --bin dimdim-health_api

# Run Worker (in another terminal)
cargo run --bin dimdim-health_worker
```

## Production Deployment

We provide a complete production deployment solution with:
- Docker Compose setup for all services (PostgreSQL, Redis, API, Worker)
- Graceful shutdown for zero data loss
- Rolling update deployment strategy
- Health checks and auto-restart
- Optimized for Raspberry Pi (ARM64)

See [DEPLOYMENT.md](DEPLOYMENT.md) for complete instructions.

Quick production setup:
```bash
./scripts/quick-start.sh
```

## Development

### Prerequisites
- Rust 1.85+
- PostgreSQL 18+
- Redis 7+

### Configuration
Configuration files are in the `config/` directory:
- `dev.toml` - Development settings
- `prod.toml` - Production settings
- `common.toml` - Shared secrets (not in git)

Set environment variable `APP_ENV=prod` to use production config.

## Architecture

- **API** (`api/`) - REST API service using Axum
- **Worker** (`worker/`) - Background job processor
- **Entities** (`entities/`) - Database models
- **Migration** (`migration/`) - Database migrations

## todo

check email verif repo