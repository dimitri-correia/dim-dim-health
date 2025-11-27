import 'package:flutter/material.dart';

class UserAvatar extends StatelessWidget {
  final String? profileImage;

  const UserAvatar({super.key, this.profileImage});

  @override
  Widget build(BuildContext context) {
    final imagePath = "avatars/${profileImage!.toLowerCase()}.png";

    return CircleAvatar(backgroundImage: AssetImage(imagePath), radius: 20);
  }
}
