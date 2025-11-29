import 'package:flutter/material.dart';
import '../utils/app_config.dart';

/// Helper class for showing consistent snackbars across the app.
class AppSnackBar {
  /// Shows an error snackbar with red background.
  static void showError(BuildContext context, String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: AppConfig.redColor,
      ),
    );
  }

  /// Shows a success snackbar with green background.
  static void showSuccess(BuildContext context, String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: Colors.green,
      ),
    );
  }

  /// Shows an info snackbar with default styling.
  static void showInfo(BuildContext context, String message, {Duration? duration}) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        duration: duration ?? const Duration(seconds: 2),
      ),
    );
  }
}
