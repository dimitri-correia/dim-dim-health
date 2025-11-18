# DimDim Health - Flutter Web App

This is the Flutter web application for DimDim Health, a comprehensive health and fitness tracking platform.

## Features

- **Splash Screen**: Fast-loading splash screen with the health logo
- **User Authentication**:
  - Register new account
  - Login with existing credentials
  - Forgot/Reset password functionality
- **Form Navigation**: Easy form filling with Tab key navigation and Enter key submission
- **Responsive Design**: Optimized for web browsers
- **State Management**: Using Provider for efficient state management
- **Secure Storage**: Tokens stored securely using flutter_secure_storage

## Getting Started

### Prerequisites

- Flutter SDK (3.10.0 or higher)
- A running instance of the DimDim Health backend API

### Installation

1. Install dependencies:
```bash
flutter pub get
```

2. Generate JSON serialization code:
```bash
flutter pub run build_runner build --delete-conflicting-outputs
```

### Running the App

For web development:
```bash
flutter run -d chrome --web-port 8081
```

Or with a custom API URL:
```bash
flutter run -d chrome --web-port 8081 --dart-define=API_URL=http://your-api-url:8080
```

### Building for Production

```bash
flutter build web --release --dart-define=API_URL=https://your-production-api.com
```

## Configuration

The API URL can be configured in two ways:

1. **Environment Variable** (recommended for production):
   ```bash
   flutter run --dart-define=API_URL=http://localhost:8080
   ```

2. **Default Value**: Edit `lib/utils/app_config.dart` to change the default API URL

## Backend Requirements

Ensure your backend API is running and accessible. The backend should have CORS enabled for:
- `http://localhost:8081` (Flutter web default port)

## Form Navigation

All forms support efficient keyboard navigation:
- **Tab**: Move to the next field
- **Shift+Tab**: Move to the previous field
- **Enter**: Submit the form (when on the last field or submit button)

## Project Structure

```
lib/
├── main.dart                 # App entry point
├── models/                   # Data models
│   └── user.dart
├── screens/                  # UI screens
│   ├── splash_screen.dart
│   ├── login_screen.dart
│   ├── register_screen.dart
│   ├── forgot_password_screen.dart
│   └── home_screen.dart
├── services/                 # Business logic
│   ├── api_service.dart
│   └── auth_provider.dart
└── utils/                    # Utilities
    └── app_config.dart
```

## API Endpoints Used

- `POST /api/users` - Register new user
- `POST /api/users/login` - User login
- `POST /api/auth/forgot-password` - Request password reset
- `POST /api/auth/reset-password` - Reset password with token
- `POST /api/auth/refresh-token` - Refresh access token
- `POST /api/auth/logout` - Logout user
- `GET /api/user` - Get current user info

## Troubleshooting

### CORS Issues
If you encounter CORS errors, ensure the backend CORS configuration includes your Flutter web URL (default: `http://localhost:8081`).

### Build Runner Issues
If JSON serialization fails, try:
```bash
flutter pub run build_runner clean
flutter pub run build_runner build --delete-conflicting-outputs
```

## License

This project is part of the DimDim Health platform.
