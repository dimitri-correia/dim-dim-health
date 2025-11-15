# DimDim Health - Flutter App

A Flutter mobile application for health tracking with user authentication.

## Features

- **Splash Screen**: Welcome screen with app logo
- **User Registration**: Create new account with username, email, and password
- **User Login**: Secure login with email and password
- **Password Reset**: Forgot password flow with email verification and token-based reset
- **Home Dashboard**: User profile display after successful authentication

## Getting Started

### Prerequisites

- Flutter SDK (version 3.10.0 or higher)
- Dart SDK
- Android Studio / Xcode for mobile development
- Running backend API server

### Installation

1. Navigate to the app directory:
```bash
cd app/dimdimhealth
```

2. Install dependencies:
```bash
flutter pub get
```

3. Update the API base URL in `lib/services/api_service.dart`:
```dart
static const String baseUrl = 'http://your-backend-url:3000';
```

4. Run the app:
```bash
flutter run
```

## Project Structure

```
lib/
├── main.dart                          # App entry point
├── models/
│   └── user_model.dart               # User data models
├── screens/
│   ├── splash_screen.dart            # Splash screen
│   ├── login_screen.dart             # Login page
│   ├── register_screen.dart          # Registration page
│   ├── forgot_password_screen.dart   # Forgot password page
│   ├── reset_password_screen.dart    # Reset password with token
│   └── home_screen.dart              # Home dashboard
└── services/
    └── api_service.dart              # API communication layer
assets/
└── images/                            # App images
```

## API Integration

The app integrates with the following backend endpoints:

- `POST /api/users` - Register new user
- `POST /api/users/login` - Login user
- `POST /api/auth/forgot-password` - Request password reset
- `POST /api/auth/reset-password` - Reset password with token

## Dependencies

- `http: ^1.1.0` - HTTP client for API calls
- `shared_preferences: ^2.2.2` - Local storage for user data
- `cupertino_icons: ^1.0.8` - iOS-style icons

## Development

To contribute to this project:

1. Make changes to the code
2. Test thoroughly on both iOS and Android
3. Run `flutter analyze` to check for issues
4. Format code with `flutter format .`
5. Submit a pull request

For help getting started with Flutter development, view the
[online documentation](https://docs.flutter.dev/).
