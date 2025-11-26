import 'package:flutter/material.dart';

class AppConfig {
  static const String apiUrl = String.fromEnvironment('API_URL');

  static const String appName = 'DimDim Health';
  static const String appVersion = '1.0.0';

  static const int splashScreenDurationInSeconds = 3;

  static const Color blueColor = Color(0xFF004170);
  static const Color redColor = Color(0xFFDA291C);
  static const Color goldColor = Color(0xFFCEAB5D);
  static const Color whiteColor = Color(0xFFFFFFFF);
  static const Color blackColor = Color(0xFF000000);

  // Placeholder avatar images (online URLs)
  static const List<String> placeholderAvatars = [
    'https://i.pravatar.cc/150?img=1',
    'https://i.pravatar.cc/150?img=2',
    'https://i.pravatar.cc/150?img=3',
    'https://i.pravatar.cc/150?img=4',
    'https://i.pravatar.cc/150?img=5',
    'https://i.pravatar.cc/150?img=6',
    'https://i.pravatar.cc/150?img=7',
    'https://i.pravatar.cc/150?img=8',
  ];

  // Default placeholder avatar
  static const String defaultAvatar = 'https://i.pravatar.cc/150?img=1';
}
