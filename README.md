# DIMDIM HEALTH 🏋️‍♂️💪

A comprehensive health and fitness tracking application built with Rust (backend) and Flutter (mobile app).
A health tracking application with API and background worker services.

## Quick Start

### Production Deployment (Raspberry Pi)

For a quick production setup, see [QUICKSTART.md](QUICKSTART.md).

For detailed deployment documentation, see [DEPLOYMENT.md](DEPLOYMENT.md).

One-command setup:
```bash
./scripts/quick-start.sh
```

### Development Setup

For development setup:

```bash
# Start development database
./scripts/dev-db/start-dev-env.sh

# Run API
cargo run --bin dimdim-health_api

# Run Worker (in another terminal)
cargo run --bin dimdim-health_worker
```

## Production Features

We provide a complete production deployment solution with:
- ✅ Docker Compose setup for all services (PostgreSQL, Redis, API, Worker)
- ✅ Graceful shutdown for zero data loss
- ✅ Rolling update deployment strategy
- ✅ Health checks and auto-restart
- ✅ Optimized for Raspberry Pi (ARM64)
- ✅ Backup and restore scripts
- ✅ Optional nginx reverse proxy with SSL

Quick deployment:
```bash
# Initial setup
./scripts/quick-start.sh

# Deploy updates
./scripts/deploy-production.sh

# Backup data
./scripts/backup.sh
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

## Documentation

- [QUICKSTART.md](QUICKSTART.md) - Quick reference for common tasks
- [DEPLOYMENT.md](DEPLOYMENT.md) - Comprehensive deployment guide

## todo

## 📋 Project Overview

DimDim Health is a full-stack health tracking platform designed to help users monitor their fitness journey through:
- **Weight Tracking**: Record and visualize weight changes over time
- **Meal Tracking**: Log daily meals with detailed nutritional information
- **Workout Management**: Plan and track workout routines (coming soon)
- **Social Features**: Share progress with friends and family through user groups

## 🏗️ Architecture

The project consists of three main components:

### Backend (Rust)
- **API Server** (`/api`): RESTful API built with Axum framework
  
- **Worker** (`/worker`): Background job processor using Redis queues

- **Database**: PostgreSQL with SeaORM

### Frontend (Flutter)
- **Mobile App** (`/app/dimdimhealth`): Cross-platform mobile application

## 🚀 Current Status

### ✅ Completed Features

#### Authentication & Security
- [x] User registration and login
- [x] JWT token-based authentication with refresh tokens
- [x] Email verification system
- [x] Password reset functionality
- [x] Secure password hashing with bcrypt

#### User Management
- [x] User profiles with additional information
- [x] User groups for social features
- [x] User watch permissions (share data with friends/family)

#### Database Infrastructure
- [x] PostgreSQL database schema
- [x] Automated migration system
- [x] Background job cleanup (expired tokens via pg_cron)

### 🔨 TODO - Pending Implementation

#### Weight Tracking

#### Meal Tracking

#### Workout Tracking


## 🛠️ Technology Stack

### Backend
- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL with SeaORM
- **Async Runtime**: Tokio
- **Queue**: Redis
- **Email**: Lettre
- **Authentication**: JWT with jsonwebtoken

### Frontend
- **Framework**: Flutter
- **Platforms**: Web, Android, iOS