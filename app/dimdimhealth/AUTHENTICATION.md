# Authentication Implementation Documentation

## Overview

This document describes the authentication flow implementation for the DimDim Health Flutter application. The implementation includes user registration, login, password reset, and a protected home screen with proper routing guards.

## Architecture

### State Management
- **Provider Pattern**: Using the `provider` package for global state management
- **AuthProvider**: Central authentication state manager that handles:
  - User authentication state
  - Token management (access & refresh tokens)
  - Secure storage of credentials
  - API calls for auth operations

### Screens

#### 1. Splash Screen (`splash_screen.dart`)
- Displays app logo and branding
- Shows loading indicator
- Acts as initial route while checking authentication status

#### 2. Login Screen (`login_screen.dart`)
Features:
- Email and password input fields
- Email validation using `email_validator` package
- Password visibility toggle
- "Forgot Password?" link
- "Don't have an account? Register" link
- Loading state during authentication
- Error message display via SnackBar

Validation:
- Email format validation
- Password minimum length (6 characters)
- Required field validation

#### 3. Register Screen (`register_screen.dart`)
Features:
- Username, email, and password input fields
- Password confirmation with matching validation
- Password visibility toggles for both password fields
- "Already have an account? Login" link
- Loading state during registration
- Error message display via SnackBar

Validation:
- Username minimum length (3 characters)
- Email format validation
- Password minimum length (6 characters)
- Password confirmation match
- Required field validation

#### 4. Forgot Password Screen (`forgot_password_screen.dart`)
Features:
- Email input for password reset
- Two-state UI: form view and success confirmation
- "Back to Login" navigation
- Loading state during request
- Success confirmation with email sent message

Validation:
- Email format validation
- Required field validation

#### 5. Home Screen (`home_screen.dart`)
Features:
- Welcome card with user information (username, email)
- Logout button in app bar
- Quick action cards for future features:
  - Weight tracking
  - Meal logging
  - Workout planning
  - Profile management
- Gradient background design
- Grid layout for action cards

### Routing & Navigation

#### Route Structure
```dart
Routes:
- '/' (home/root)     -> AuthWrapper (decides login vs home)
- '/login'            -> LoginScreen (protected by GuestGuard)
- '/register'         -> RegisterScreen (protected by GuestGuard)
- '/forgot-password'  -> ForgotPasswordScreen (protected by GuestGuard)
- '/home'             -> HomeScreen (protected by AuthGuard)
```

#### Navigation Guards

**AuthWrapper** (`main.dart`)
- Initial route that displays splash screen
- Loads saved authentication from secure storage
- After splash duration (3 seconds):
  - If authenticated: navigates to HomeScreen
  - If not authenticated: navigates to LoginScreen

**AuthGuard** (`main.dart`)
- Protects authenticated-only routes
- Checks if user is authenticated via AuthProvider
- If not authenticated: redirects to login screen
- Applied to: `/home` route

**GuestGuard** (`main.dart`)
- Protects guest-only routes (login, register, forgot-password)
- Checks if user is already authenticated
- If authenticated: redirects to home screen
- Prevents authenticated users from accessing auth screens
- Applied to: `/login`, `/register`, `/forgot-password` routes

### Authentication Flow

#### 1. App Launch
```
App Start
    ↓
Splash Screen (3 seconds)
    ↓
Check Saved Auth (AuthProvider.loadSavedAuth())
    ↓
├─ Authenticated → Home Screen
└─ Not Authenticated → Login Screen
```

#### 2. Login Flow
```
User enters email & password
    ↓
Form validation
    ↓
AuthProvider.login()
    ↓
API call to /api/users/login
    ↓
├─ Success: Save tokens → Navigate to /home
└─ Error: Show error message
```

#### 3. Registration Flow
```
User enters username, email, password, confirm password
    ↓
Form validation (including password match)
    ↓
AuthProvider.register()
    ↓
API call to /api/users
    ↓
├─ Success: Save tokens → Navigate to /home
└─ Error: Show error message (e.g., "User already exists")
```

#### 4. Forgot Password Flow
```
User enters email
    ↓
Form validation
    ↓
AuthProvider.forgotPassword()
    ↓
API call to /api/auth/forgot-password
    ↓
├─ Success: Show success message → Option to return to login
└─ Error: Show error message
```

#### 5. Logout Flow
```
User clicks logout in Home Screen
    ↓
AuthProvider.logout()
    ↓
Clear tokens from secure storage
    ↓
Navigate to /login
```

### Security Features

1. **Secure Storage**: 
   - Uses `flutter_secure_storage` to encrypt and store tokens
   - Access tokens and refresh tokens stored securely
   - All tokens cleared on logout

2. **Password Handling**:
   - Passwords are obscured by default
   - Toggle visibility option available
   - Minimum length requirements enforced
   - No password stored locally (only tokens)

3. **Route Protection**:
   - AuthGuard prevents unauthorized access to protected routes
   - GuestGuard prevents authenticated users from accessing auth screens
   - Automatic redirection based on authentication state

4. **Form Validation**:
   - Client-side validation before API calls
   - Email format validation
   - Password strength requirements
   - Required field validation

### API Integration

The app communicates with the backend API through the `ApiService` class:

#### Endpoints Used:
- `POST /api/users` - User registration
- `POST /api/users/login` - User login
- `POST /api/auth/forgot-password` - Password reset request

#### Request/Response Models:
All models are defined in `lib/models/user.dart` with JSON serialization:
- `User` - User information
- `LoginRequest` / `LoginResponse` - Login data structures
- `RegisterRequest` / `RegisterUserData` - Registration data structures
- `ForgotPasswordRequest` / `ForgotPasswordResponse` - Password reset structures

### UI/UX Design

#### Color Scheme (from AppConfig):
- Primary (Blue): `#004170` - Main app color
- Secondary (Gold): `#CEAB5D` - Accent color
- Error (Red): `#DA291C` - Error messages
- White: `#FFFFFF` - Text and backgrounds
- Black: `#000000` - Text

#### Design Patterns:
- Gradient backgrounds for visual appeal
- Rounded corners (12px radius) for modern look
- Consistent padding (24px) for spacing
- Material Design 3 components
- Loading indicators for async operations
- SnackBar notifications for feedback

### Testing

#### Widget Tests (`test/widget_test.dart`)
Tests verify:
1. App starts with splash screen
2. Login screen appears after splash
3. Login screen has required elements (email, password, buttons)
4. Register link exists on login screen
5. Forgot password link exists on login screen
6. AuthProvider is accessible in widget tree
7. Initial authentication state is false

#### Running Tests:
```bash
flutter test
```

### Future Enhancements

1. **Enhanced Security**:
   - Implement biometric authentication
   - Add CAPTCHA for registration
   - Implement rate limiting client-side

2. **User Experience**:
   - Remember me functionality
   - Social login integration (Google, Apple)
   - Email verification flow
   - Password strength indicator

3. **Features**:
   - Profile editing
   - Password change within app
   - Two-factor authentication
   - Session management

4. **Testing**:
   - Integration tests for complete flows
   - Mock API service for unit tests
   - Widget tests for all screens

## Development Setup

### Prerequisites
- Flutter SDK 3.10.0 or higher
- Dart SDK (included with Flutter)

### Installation
1. Navigate to the app directory:
   ```bash
   cd app/dimdimhealth
   ```

2. Install dependencies:
   ```bash
   flutter pub get
   ```

3. Generate JSON serialization code:
   ```bash
   flutter pub run build_runner build --delete-conflicting-outputs
   ```

### Running the App

#### Web Development:
```bash
flutter run -d chrome --web-port 8081 --dart-define=API_URL=http://localhost:8080
```

#### Mobile Development:
```bash
# iOS
flutter run -d ios --dart-define=API_URL=http://localhost:8080

# Android
flutter run -d android --dart-define=API_URL=http://localhost:8080
```

### Building for Production

#### Web:
```bash
flutter build web --release --dart-define=API_URL=https://api.dimdimhealth.com
```

#### Mobile:
```bash
# iOS
flutter build ios --release --dart-define=API_URL=https://api.dimdimhealth.com

# Android
flutter build apk --release --dart-define=API_URL=https://api.dimdimhealth.com
```

## Troubleshooting

### JSON Serialization Issues
If you encounter errors related to missing `.g.dart` files:
```bash
flutter pub run build_runner clean
flutter pub run build_runner build --delete-conflicting-outputs
```

### Secure Storage Issues
If secure storage fails on web:
- Secure storage uses browser's localStorage on web
- Clear browser cache and try again

### Navigation Issues
If screens don't navigate properly:
- Check that AuthProvider is properly initialized
- Verify route names match exactly (case-sensitive)
- Use Flutter DevTools to inspect navigation stack

## Conclusion

This authentication implementation provides a solid foundation for the DimDim Health app with:
- ✅ Complete user authentication flow
- ✅ Secure token management
- ✅ Protected routing with guards
- ✅ Form validation and error handling
- ✅ Responsive and attractive UI
- ✅ Proper state management
- ✅ Test coverage

The implementation follows Flutter best practices and is ready for production use with proper backend API integration.
