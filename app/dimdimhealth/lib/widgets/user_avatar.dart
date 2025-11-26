import 'package:flutter/material.dart';
import '../utils/app_config.dart';

/// A widget that displays a user's avatar in the app bar.
/// Falls back to displaying the first letter of the username if no image is available.
class UserAvatar extends StatelessWidget {
  final String? profileImage;
  final String? username;
  final double radius;

  const UserAvatar({
    super.key,
    this.profileImage,
    this.username,
    this.radius = 18,
  });

  @override
  Widget build(BuildContext context) {
    return CircleAvatar(
      radius: radius,
      backgroundColor: AppConfig.goldColor,
      backgroundImage: _getBackgroundImage(),
      child: _shouldShowFallback()
          ? Text(
              _getInitial(),
              style: TextStyle(
                color: AppConfig.blueColor,
                fontWeight: FontWeight.bold,
                fontSize: radius * 0.8,
              ),
            )
          : null,
    );
  }

  ImageProvider? _getBackgroundImage() {
    if (profileImage != null && profileImage!.isNotEmpty) {
      // Check if it's a URL (network image) or an asset path
      if (profileImage!.startsWith('http://') ||
          profileImage!.startsWith('https://')) {
        return NetworkImage(profileImage!);
      } else if (profileImage!.startsWith('assets/')) {
        return AssetImage(profileImage!);
      }
    }
    return null;
  }

  bool _shouldShowFallback() {
    return profileImage == null || profileImage!.isEmpty;
  }

  String _getInitial() {
    if (username != null && username!.isNotEmpty) {
      return username![0].toUpperCase();
    }
    return 'U';
  }
}
