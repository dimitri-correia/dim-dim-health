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
  - User authentication and authorization (JWT-based)
  - Email verification and password reset
  - User management and social features
  - Health data endpoints (weight, meals)
  
- **Worker** (`/worker`): Background job processor using Redis queues
  - Email sending (verification, password reset)
  - Scheduled tasks for token cleanup

- **Database**: PostgreSQL with SeaORM
  - Comprehensive migration system
  - User, authentication, and health data models

### Frontend (Flutter)
- **Mobile App** (`/app/dimdimhealth`): Cross-platform mobile application
  - iOS and Android support
  - User-friendly interface for health tracking

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
- [ ] POST endpoint to create weight records
- [ ] GET endpoint to retrieve weight history
- [ ] GET endpoint to retrieve weight statistics (trends, averages)
- [ ] DELETE endpoint to remove weight records
- [ ] Weight repository implementation
- [ ] Weight data visualization in mobile app

#### Meal Tracking
- [ ] POST endpoint to create meals
- [ ] GET endpoint to retrieve meal history
- [ ] PUT endpoint to update meals
- [ ] DELETE endpoint to remove meals
- [ ] Food item management endpoints (CRUD)
- [ ] Meal item endpoints (link foods to meals)
- [ ] Nutritional calculations and summaries
- [ ] Meal repository implementation
- [ ] Food database/API integration
- [ ] Meal logging interface in mobile app

#### Workout Tracking
- [ ] Database schema for workout sessions
- [ ] Database schema for exercises
- [ ] Database schema for workout plans
- [ ] POST endpoint to create workout sessions
- [ ] GET endpoint to retrieve workout history
- [ ] Exercise library management
- [ ] Workout repository implementation
- [ ] Workout logging interface in mobile app
- [ ] Progress tracking and analytics

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
- **Platforms**: iOS, Android, Linux, Windows

## 📦 Project Structure

```
dim-dim-health/
├── api/              # REST API server
├── worker/           # Background job processor
├── entities/         # Database entities and models
├── migration/        # Database migrations
├── app/              # Flutter mobile application
├── config/           # Configuration files
├── scripts/          # Development scripts
└── images/           # Project assets and images
```

## 🔧 Setup & Development

### Prerequisites
- Rust (latest stable)
- PostgreSQL
- Redis
- Flutter SDK (for mobile app)

### Backend Setup

1. Install dependencies:
```bash
cargo build
```

2. Configure environment:
```bash
cp config/dev.toml config/local.toml
# Edit local.toml with your database and Redis URLs
```

3. Run migrations:
```bash
cd migration
cargo run
```

4. Start the API server:
```bash
cd api
cargo run
```

5. Start the worker:
```bash
cd worker
cargo run
```

### Frontend Setup

1. Navigate to the Flutter app:
```bash
cd app/dimdimhealth
```

2. Get dependencies:
```bash
flutter pub get
```

3. Run the app:
```bash
flutter run
```

## 📝 API Documentation

API runs on `http://localhost:3000` by default.

### Implemented Endpoints
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login
- `POST /api/auth/refresh` - Refresh access token
- `POST /api/auth/logout` - User logout
- `POST /api/auth/verify-email` - Email verification
- `POST /api/auth/forgot-password` - Request password reset
- `POST /api/auth/reset-password` - Reset password
- `GET /api/health` - Server health check

### Coming Soon
- Weight tracking endpoints
- Meal tracking endpoints
- Workout tracking endpoints

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source and available under the MIT License.