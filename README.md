# DIMDIM HEALTH 🏋️‍♂️💪

A comprehensive health and fitness tracking application built with Rust (backend) and Flutter (mobile app).

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
- **Observability**: OpenTelemetry + OpenObserve (optional)

### Frontend
- **Framework**: Flutter
- **Platforms**: Web, Android, iOS

## 📊 Observability with OpenObserve

DimDim Health integrates with **OpenObserve**, a free and open-source observability platform for logs, metrics, and traces.

### Why OpenObserve?
- ✅ **Completely Free**: Open-source (Apache 2.0) and self-hosted
- ✅ **140x Lower Storage Cost**: Compared to Elasticsearch
- ✅ **Easy Setup**: Single binary deployment
- ✅ **High Performance**: Built in Rust, uses 1/4th the resources
- ✅ **Built-in UI**: No need for separate visualization tools

### Quick Start

1. Start OpenObserve with Docker Compose:
   ```bash
   docker-compose up -d openobserve
   ```

2. Enable in your config file (`config/dev.toml`):
   ```toml
   openobserve_endpoint = "http://localhost:5080/api/default"
   ```

3. Access the UI at `http://localhost:5080`
   - **Email**: admin@example.com
   - **Password**: Complexpass#123

📖 **Full Documentation**: See [OPENOBSERVE.md](OPENOBSERVE.md) for detailed setup and usage instructions.