import 'package:flutter/material.dart';

enum UserProfileImage {
  avatar1,
  avatar2,
  avatar3,
  avatar4,
  avatar5,
}

extension UserProfileImageExtension on UserProfileImage {
  String get displayName {
    switch (this) {
      case UserProfileImage.avatar1:
        return 'Avatar 1';
      case UserProfileImage.avatar2:
        return 'Avatar 2';
      case UserProfileImage.avatar3:
        return 'Avatar 3';
      case UserProfileImage.avatar4:
        return 'Avatar 4';
      case UserProfileImage.avatar5:
        return 'Avatar 5';
    }
  }

  IconData get icon {
    switch (this) {
      case UserProfileImage.avatar1:
        return Icons.person;
      case UserProfileImage.avatar2:
        return Icons.face;
      case UserProfileImage.avatar3:
        return Icons.sentiment_satisfied_alt;
      case UserProfileImage.avatar4:
        return Icons.emoji_emotions;
      case UserProfileImage.avatar5:
        return Icons.account_circle;
    }
  }

  Color get color {
    switch (this) {
      case UserProfileImage.avatar1:
        return Colors.blue;
      case UserProfileImage.avatar2:
        return Colors.green;
      case UserProfileImage.avatar3:
        return Colors.orange;
      case UserProfileImage.avatar4:
        return Colors.purple;
      case UserProfileImage.avatar5:
        return Colors.teal;
    }
  }

  Widget buildAvatar({double size = 40}) {
    return CircleAvatar(
      radius: size / 2,
      backgroundColor: color,
      child: Icon(
        icon,
        color: Colors.white,
        size: size * 0.6,
      ),
    );
  }

  static UserProfileImage fromString(String value) {
    switch (value) {
      case 'avatar1':
        return UserProfileImage.avatar1;
      case 'avatar2':
        return UserProfileImage.avatar2;
      case 'avatar3':
        return UserProfileImage.avatar3;
      case 'avatar4':
        return UserProfileImage.avatar4;
      case 'avatar5':
        return UserProfileImage.avatar5;
      default:
        return UserProfileImage.avatar1;
    }
  }

  String toJson() {
    switch (this) {
      case UserProfileImage.avatar1:
        return 'avatar1';
      case UserProfileImage.avatar2:
        return 'avatar2';
      case UserProfileImage.avatar3:
        return 'avatar3';
      case UserProfileImage.avatar4:
        return 'avatar4';
      case UserProfileImage.avatar5:
        return 'avatar5';
    }
  }
}
