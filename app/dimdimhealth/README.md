# DimDim Health - Flutter Web App

This is the Flutter web application for DimDim Health.

## ✨ Recent Updates

### Authentication System Implementation
The app now includes a complete authentication flow with:
- ✅ User Registration
- ✅ User Login
- ✅ Password Reset (Forgot Password)
- ✅ Protected Routing
- ✅ Secure Token Storage
- ✅ Home Dashboard

See [AUTHENTICATION.md](AUTHENTICATION.md) for detailed documentation and [AUTH_FLOW_DIAGRAM.md](AUTH_FLOW_DIAGRAM.md) for visual flow diagrams.

## Getting Started

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
flutter run -d chrome --web-port 8081 --dart-define=API_URL=http://localhost:8080
```

For mobile development:
```bash
# iOS
flutter run -d ios --dart-define=API_URL=http://localhost:8080

# Android
flutter run -d android --dart-define=API_URL=http://localhost:8080
```

### Running Tests

```bash
flutter test
```

### Building for Production

```bash
flutter build web --release --dart-define=API_URL=https://your-production-api.com
```

### Build Runner Issues
If JSON serialization fails, try:
```bash
flutter pub run build_runner clean
flutter pub run build_runner build --delete-conflicting-outputs
```

## 📁 Project Structure

```
lib/
├── main.dart                          # App entry, routing, guards
├── models/
│   ├── user.dart                      # User data models
│   └── user.g.dart                    # Generated JSON serialization
├── screens/
│   ├── splash_screen.dart             # Initial loading screen
│   ├── login_screen.dart              # Login UI
│   ├── register_screen.dart           # Registration UI
│   ├── forgot_password_screen.dart    # Password reset UI
│   └── home_screen.dart               # Main authenticated screen
├── services/
│   ├── api_service.dart               # API client
│   └── auth_provider.dart             # State management
└── utils/
    └── app_config.dart                # App configuration
```

## 📖 Documentation

- **[AUTHENTICATION.md](AUTHENTICATION.md)** - Complete authentication implementation guide
- **[AUTH_FLOW_DIAGRAM.md](AUTH_FLOW_DIAGRAM.md)** - Visual flow diagrams and architecture

## 🧪 Testing

The app includes widget tests for:
- App initialization and splash screen
- Login screen UI elements
- Navigation links
- Authentication state management

Run tests with: `flutter test`

## 🔐 Security

- Secure token storage using flutter_secure_storage
- Form validation on all inputs
- Password obscuring with visibility toggle
- Protected routes with authentication guards
- No sensitive data stored in plain text
