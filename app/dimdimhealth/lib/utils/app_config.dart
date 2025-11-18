class AppConfig {
  // Backend API URL - change this based on your environment
  static const String apiUrl = String.fromEnvironment(
    'API_URL',
    defaultValue: 'http://localhost:8080',
  );
  
  static const String appName = 'DimDim Health';
  static const String appVersion = '1.0.0';
}
