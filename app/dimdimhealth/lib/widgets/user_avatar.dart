import 'package:flutter/material.dart';
import '../utils/app_config.dart';

/// A widget that displays a user's avatar in the app bar.
/// Falls back to displaying the first letter of the username if no image is available
/// or if the image fails to load.
class UserAvatar extends StatefulWidget {
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
  State<UserAvatar> createState() => _UserAvatarState();
}

class _UserAvatarState extends State<UserAvatar> {
  bool _imageLoadFailed = false;

  @override
  void didUpdateWidget(UserAvatar oldWidget) {
    super.didUpdateWidget(oldWidget);
    // Reset error state when profile image changes
    if (oldWidget.profileImage != widget.profileImage) {
      _imageLoadFailed = false;
    }
  }

  @override
  Widget build(BuildContext context) {
    final imageUrl = _getValidImageUrl();
    
    if (imageUrl != null && !_imageLoadFailed) {
      return CircleAvatar(
        radius: widget.radius,
        backgroundColor: AppConfig.goldColor,
        backgroundImage: NetworkImage(imageUrl),
        onBackgroundImageError: (_, __) {
          // Mark image as failed and rebuild with fallback
          if (mounted) {
            setState(() {
              _imageLoadFailed = true;
            });
          }
        },
        child: null,
      );
    }
    
    // Fallback to initials
    return CircleAvatar(
      radius: widget.radius,
      backgroundColor: AppConfig.goldColor,
      child: Text(
        _getInitial(),
        style: TextStyle(
          color: AppConfig.blueColor,
          fontWeight: FontWeight.bold,
          fontSize: widget.radius * 0.8,
        ),
      ),
    );
  }

  /// Returns a valid HTTPS image URL, or null if not available.
  /// Only HTTPS URLs are accepted for security reasons.
  String? _getValidImageUrl() {
    if (widget.profileImage != null && widget.profileImage!.isNotEmpty) {
      // Only accept HTTPS URLs for security
      if (widget.profileImage!.startsWith('https://')) {
        return widget.profileImage;
      }
    }
    return null;
  }

  String _getInitial() {
    if (widget.username != null && widget.username!.isNotEmpty) {
      return widget.username![0].toUpperCase();
    }
    return 'U';
  }
}

