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
- **Logging**: Tracing with structured logging

### Frontend
- **Framework**: Flutter
- **Platforms**: Web, Android, iOS

## 📚 Documentation

- **[Logging System Guide](LOGGING.md)** - Comprehensive guide to the structured logging system