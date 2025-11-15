# Flutter App Quick Start Guide

## Prerequisites

Before running the Flutter app, ensure you have:

1. **Flutter SDK** installed (version 3.10.0 or higher)
   ```bash
   flutter --version
   ```

2. **Backend API Server** running
   - The backend should be running on `http://localhost:3000` or update the URL in `lib/utils/config.dart`

## Setup Instructions

1. **Navigate to the Flutter app directory:**
   ```bash
   cd app/dimdimhealth
   ```

2. **Install dependencies:**
   ```bash
   flutter pub get
   ```

3. **Configure the API URL (if needed):**
   Edit `lib/utils/config.dart` and update:
   ```dart
   static const String apiBaseUrl = 'http://your-backend-url:3000';
   ```

4. **Run the app:**
   
   For Android:
   ```bash
   flutter run
   ```
   
   For iOS:
   ```bash
   flutter run -d ios
   ```
   
   For web:
   ```bash
   flutter run -d chrome
   ```

## App Flow

1. **Splash Screen** (3 seconds)
   - Shows DimDim Health logo
   - Auto-navigates to login

2. **Login Screen**
   - Enter email and password
   - Links to Register and Forgot Password

3. **Register Screen**
   - Enter username (3-20 characters)
   - Enter valid email
   - Enter password (min 8 characters)
   - Confirm password

4. **Forgot Password**
   - Enter email to receive reset token
   - Or enter token directly if you have one

5. **Reset Password**
   - Enter reset token from email
   - Enter new password (min 8 characters)
   - Confirm new password

6. **Home Screen**
   - Shows user profile
   - Logout option

## Testing the App

### Test User Registration:
1. Open the app
2. Wait for splash screen
3. Click "Register" on login screen
4. Fill in all fields:
   - Username: `testuser`
   - Email: `test@example.com`
   - Password: `password123`
   - Confirm Password: `password123`
5. Click "Register"

### Test User Login:
1. On login screen, enter:
   - Email: `test@example.com`
   - Password: `password123`
2. Click "Login"

### Test Password Reset:
1. Click "Forgot Password?" on login screen
2. Enter email: `test@example.com`
3. Check your email for reset token
4. Enter token and new password
5. Click "Reset Password"

## Troubleshooting

### "Connection refused" error:
- Ensure backend server is running
- Check API URL in `lib/utils/config.dart`
- Verify backend is accessible from your device/emulator

### "Failed to load image" error:
- Run `flutter pub get` to ensure assets are included
- Check that `assets/images/` directory contains all images

### Build errors:
```bash
flutter clean
flutter pub get
flutter run
```

## Development Mode

To enable hot reload during development:
```bash
flutter run
```
Then press:
- `r` to hot reload
- `R` to hot restart
- `q` to quit

## Platform-Specific Notes

### Android:
- Minimum SDK version: Check `android/app/build.gradle`
- For local backend testing, use `10.0.2.2:3000` instead of `localhost:3000`

### iOS:
- Requires Xcode installed
- May need to configure signing in Xcode

### Web:
- CORS must be enabled on backend for web testing
- Use Chrome for best compatibility

## Need Help?

Check the main README.md for more detailed documentation about the project structure and API integration.
