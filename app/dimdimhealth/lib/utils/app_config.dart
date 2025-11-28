import 'package:flutter/material.dart';

class AppConfig {
  static const String apiUrl = String.fromEnvironment('API_URL');

  static const String appName = 'DimDim Health';
  static const String appVersion = '1.0.0';

  static const int splashScreenDurationInSeconds = 3;

  // Responsive layout breakpoint (width in pixels)
  static const double desktopBreakpoint = 600.0;

  static const Color blueColor = Color(0xFF004170);
  static const Color redColor = Color(0xFFDA291C);
  static const Color goldColor = Color(0xFFCEAB5D);
  static const Color whiteColor = Color(0xFFFFFFFF);
  static const Color blackColor = Color(0xFF000000);

  // Placeholder avatar images (local assets)
  static const List<String> placeholderAvatars = [
    'assets/avatars/avatar1.png',
    'assets/avatars/avatar2.png',
    'assets/avatars/avatar3.png',
    'assets/avatars/avatar4.png',
    'assets/avatars/avatar5.png',
  ];

  // Default placeholder avatar
  static const String defaultAvatar = 'assets/avatars/avatar1.png';
}
