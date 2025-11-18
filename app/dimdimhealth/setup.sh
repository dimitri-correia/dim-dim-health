#!/bin/bash

# Script to help developers set up and run the Flutter app

echo "DimDim Health Flutter Web App - Development Helper"
echo "=================================================="
echo ""

# Check if Flutter is installed
if ! command -v flutter &> /dev/null; then
    echo "❌ Flutter is not installed. Please install Flutter first:"
    echo "   https://docs.flutter.dev/get-started/install"
    exit 1
fi

echo "✅ Flutter found: $(flutter --version | head -1)"
echo ""

# Get dependencies
echo "📦 Installing dependencies..."
flutter pub get

if [ $? -ne 0 ]; then
    echo "❌ Failed to install dependencies"
    exit 1
fi

echo ""
echo "🔨 Generating JSON serialization code..."
flutter pub run build_runner build --delete-conflicting-outputs

if [ $? -ne 0 ]; then
    echo "❌ Failed to generate code"
    exit 1
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "To run the app in Chrome:"
echo "  flutter run -d chrome --web-port 8081"
echo ""
echo "To run with a custom API URL:"
echo "  flutter run -d chrome --web-port 8081 --dart-define=API_URL=http://your-api:8080"
echo ""
echo "To build for production:"
echo "  flutter build web --release --dart-define=API_URL=https://your-production-api.com"
echo ""
