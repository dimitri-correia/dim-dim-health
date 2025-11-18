# DimDim Health - Flutter Web App

This is the Flutter web application for DimDim Health.

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
flutter run -d chrome --web-port 8081 --dart-define=API_URL=http://your-api-url:8080
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
