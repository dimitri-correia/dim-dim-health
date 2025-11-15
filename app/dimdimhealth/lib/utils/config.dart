/// Application configuration
class AppConfig {
  // API Configuration
  // Update this to your backend server URL
  // For local development: 'http://localhost:3000'
  // For production: 'https://your-domain.com'
  static const String apiBaseUrl = 'http://localhost:3000';

  // App Information
  static const String appName = 'DimDim Health';
  static const String appVersion = '1.0.0';

  // Timeouts
  static const Duration apiTimeout = Duration(seconds: 30);
  static const Duration splashScreenDuration = Duration(seconds: 3);

  // Storage Keys (for SharedPreferences)
  static const String keyAccessToken = 'access_token';
  static const String keyRefreshToken = 'refresh_token';
  static const String keyUserData = 'user_data';
}
