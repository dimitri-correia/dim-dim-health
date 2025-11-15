# Environment Configuration Examples

## Local Development

For local backend development on your computer:

```dart
// lib/utils/config.dart
static const String apiBaseUrl = 'http://localhost:3000';
```

## Android Emulator

When testing on Android emulator with backend on your host machine:

```dart
// lib/utils/config.dart
static const String apiBaseUrl = 'http://10.0.2.2:3000';
```

Note: `10.0.2.2` is the special IP address that maps to the host machine from Android emulator.

## iOS Simulator

When testing on iOS simulator with backend on your host machine:

```dart
// lib/utils/config.dart
static const String apiBaseUrl = 'http://localhost:3000';
```

Or use your computer's local network IP:

```dart
static const String apiBaseUrl = 'http://192.168.1.100:3000';
```

## Physical Device (Same Network)

When testing on a physical device connected to the same WiFi network:

1. Find your computer's local IP address:
   - Mac/Linux: `ifconfig | grep inet`
   - Windows: `ipconfig`

2. Update config:
```dart
static const String apiBaseUrl = 'http://192.168.1.100:3000';
```

3. Ensure firewall allows connections on port 3000

## Production

For production deployment:

```dart
// lib/utils/config.dart
static const String apiBaseUrl = 'https://api.dimdimhealth.com';
```

## Environment-Specific Builds

For more advanced setups, you can use different config files:

### lib/utils/config_dev.dart
```dart
class AppConfig {
  static const String apiBaseUrl = 'http://localhost:3000';
  static const String appName = 'DimDim Health (Dev)';
}
```

### lib/utils/config_prod.dart
```dart
class AppConfig {
  static const String apiBaseUrl = 'https://api.dimdimhealth.com';
  static const String appName = 'DimDim Health';
}
```

Then import the appropriate config based on build flavor or environment variables.

## Testing API Connectivity

To verify your backend is accessible:

1. **From command line:**
   ```bash
   curl http://localhost:3000/health
   ```

2. **From your app**, check the error messages:
   - "Connection refused" → Backend not running or wrong URL
   - "Network unreachable" → Device not on same network
   - "Timeout" → Firewall blocking connection

3. **Enable debugging in API service:**
   Add print statements in `lib/services/api_service.dart` to see request/response details.

## Security Notes

- Never commit production API URLs with sensitive data
- Use environment variables for CI/CD pipelines
- Consider using a package like `flutter_dotenv` for production apps
- Always use HTTPS in production
