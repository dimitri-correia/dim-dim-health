import 'package:flutter/material.dart';
import '../utils/app_config.dart';

/// Header component for authentication screens with logo and title.
class AuthHeader extends StatelessWidget {
  final String title;
  final String subtitle;
  final bool showLogo;
  final IconData? icon;

  const AuthHeader({
    super.key,
    required this.title,
    required this.subtitle,
    this.showLogo = true,
    this.icon,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        if (showLogo)
          Image.asset(AppConfig.healthImagePath, width: 80, height: 80)
        else if (icon != null)
          Icon(
            icon,
            size: 80,
            color: AppConfig.goldColor,
          ),
        const SizedBox(height: 16),
        Text(
          title,
          style: const TextStyle(
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: AppConfig.goldColor,
          ),
        ),
        const SizedBox(height: 8),
        Text(
          subtitle,
          style: const TextStyle(
            fontSize: 16,
            color: AppConfig.whiteColor,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 48),
      ],
    );
  }
}

/// Divider with "OR" text for authentication screens.
class OrDivider extends StatelessWidget {
  const OrDivider({super.key});

  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Expanded(child: Divider(color: AppConfig.whiteColor)),
        Padding(
          padding: EdgeInsets.symmetric(horizontal: 16),
          child: Text(
            'OR',
            style: TextStyle(
              color: AppConfig.whiteColor,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
        Expanded(child: Divider(color: AppConfig.whiteColor)),
      ],
    );
  }
}

/// Navigation link row for authentication screens (e.g., "Don't have an account? Register").
class AuthNavigationLink extends StatelessWidget {
  final String leadingText;
  final String linkText;
  final VoidCallback onPressed;

  const AuthNavigationLink({
    super.key,
    required this.leadingText,
    required this.linkText,
    required this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(
          leadingText,
          style: const TextStyle(
            color: AppConfig.whiteColor,
            fontSize: 16,
          ),
        ),
        TextButton(
          onPressed: onPressed,
          child: Text(
            linkText,
            style: const TextStyle(
              color: AppConfig.goldColor,
              fontWeight: FontWeight.bold,
              fontSize: 16,
            ),
          ),
        ),
      ],
    );
  }
}

/// Section title for forms and settings screens.
class SectionTitle extends StatelessWidget {
  final String title;
  final Color color;

  const SectionTitle({
    super.key,
    required this.title,
    this.color = AppConfig.goldColor,
  });

  @override
  Widget build(BuildContext context) {
    return Text(
      title,
      style: TextStyle(
        fontSize: 20,
        fontWeight: FontWeight.bold,
        color: color,
      ),
    );
  }
}

/// Info card with icon and description text.
class InfoCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final String description;

  const InfoCard({
    super.key,
    required this.icon,
    required this.title,
    required this.description,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(icon, color: AppConfig.blueColor),
                const SizedBox(width: 8),
                Text(
                  title,
                  style: const TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                    color: AppConfig.blueColor,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              description,
              style: TextStyle(
                fontSize: 14,
                color: Colors.grey[700],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
