import 'package:flutter/material.dart';
import '../utils/app_config.dart';

/// A styled text form field with consistent appearance across the app.
class AppTextField extends StatelessWidget {
  final TextEditingController controller;
  final String labelText;
  final IconData? prefixIcon;
  final Widget? suffixIcon;
  final bool obscureText;
  final TextInputType? keyboardType;
  final String? Function(String?)? validator;
  final String? helperText;
  final TextStyle? helperStyle;

  const AppTextField({
    super.key,
    required this.controller,
    required this.labelText,
    this.prefixIcon,
    this.suffixIcon,
    this.obscureText = false,
    this.keyboardType,
    this.validator,
    this.helperText,
    this.helperStyle,
  });

  @override
  Widget build(BuildContext context) {
    return TextFormField(
      controller: controller,
      obscureText: obscureText,
      keyboardType: keyboardType,
      decoration: InputDecoration(
        labelText: labelText,
        prefixIcon: prefixIcon != null ? Icon(prefixIcon) : null,
        suffixIcon: suffixIcon,
        filled: true,
        fillColor: AppConfig.whiteColor,
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
        ),
        helperText: helperText,
        helperStyle: helperStyle,
      ),
      validator: validator,
    );
  }
}

/// A styled primary button with consistent appearance.
class AppPrimaryButton extends StatelessWidget {
  final String text;
  final VoidCallback? onPressed;
  final bool isLoading;

  const AppPrimaryButton({
    super.key,
    required this.text,
    this.onPressed,
    this.isLoading = false,
  });

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      height: 50,
      child: ElevatedButton(
        onPressed: isLoading ? null : onPressed,
        style: ElevatedButton.styleFrom(
          backgroundColor: AppConfig.goldColor,
          foregroundColor: AppConfig.blueColor,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
        ),
        child: isLoading
            ? const SizedBox(
                width: 20,
                height: 20,
                child: CircularProgressIndicator(
                  strokeWidth: 2,
                  valueColor: AlwaysStoppedAnimation<Color>(
                    AppConfig.blueColor,
                  ),
                ),
              )
            : Text(
                text,
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                ),
              ),
      ),
    );
  }
}

/// A styled secondary button (outlined) with consistent appearance.
class AppSecondaryButton extends StatelessWidget {
  final String text;
  final IconData? icon;
  final VoidCallback? onPressed;
  final bool isLoading;

  const AppSecondaryButton({
    super.key,
    required this.text,
    this.icon,
    this.onPressed,
    this.isLoading = false,
  });

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      height: 50,
      child: OutlinedButton.icon(
        onPressed: isLoading ? null : onPressed,
        style: OutlinedButton.styleFrom(
          foregroundColor: AppConfig.goldColor,
          side: const BorderSide(
            color: AppConfig.goldColor,
            width: 2,
          ),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
        ),
        icon: icon != null ? Icon(icon) : const SizedBox.shrink(),
        label: isLoading
            ? const SizedBox(
                width: 20,
                height: 20,
                child: CircularProgressIndicator(
                  strokeWidth: 2,
                  valueColor: AlwaysStoppedAnimation<Color>(
                    AppConfig.goldColor,
                  ),
                ),
              )
            : Text(
                text,
                style: const TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                ),
              ),
      ),
    );
  }
}

/// A container with the primary background color used across screens.
class AppBackground extends StatelessWidget {
  final Widget child;

  const AppBackground({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.primary,
      ),
      child: SafeArea(child: child),
    );
  }
}

/// A section title widget with consistent styling.
class AppSectionTitle extends StatelessWidget {
  final String title;
  final Color? color;

  const AppSectionTitle({
    super.key,
    required this.title,
    this.color,
  });

  @override
  Widget build(BuildContext context) {
    return Text(
      title,
      style: TextStyle(
        fontSize: 20,
        fontWeight: FontWeight.bold,
        color: color ?? AppConfig.goldColor,
      ),
    );
  }
}

/// A loading indicator with app colors.
class AppLoadingIndicator extends StatelessWidget {
  final double? size;

  const AppLoadingIndicator({super.key, this.size});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: SizedBox(
        width: size,
        height: size,
        child: const CircularProgressIndicator(
          color: AppConfig.goldColor,
        ),
      ),
    );
  }
}

/// An error state widget with retry button.
class AppErrorState extends StatelessWidget {
  final String message;
  final VoidCallback? onRetry;

  const AppErrorState({
    super.key,
    required this.message,
    this.onRetry,
  });

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(
            Icons.error_outline,
            size: 64,
            color: AppConfig.redColor,
          ),
          const SizedBox(height: 16),
          Text(
            message,
            style: const TextStyle(
              color: AppConfig.whiteColor,
              fontSize: 16,
            ),
            textAlign: TextAlign.center,
          ),
          if (onRetry != null) ...[
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: onRetry,
              child: const Text('Retry'),
            ),
          ],
        ],
      ),
    );
  }
}

/// A styled card with consistent appearance.
class AppCard extends StatelessWidget {
  final Widget child;
  final double elevation;
  final EdgeInsetsGeometry? padding;

  const AppCard({
    super.key,
    required this.child,
    this.elevation = 4,
    this.padding,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: elevation,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
      ),
      child: padding != null
          ? Padding(padding: padding!, child: child)
          : child,
    );
  }
}

/// An information card with icon and description.
class AppInfoCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final String description;

  const AppInfoCard({
    super.key,
    required this.icon,
    required this.title,
    required this.description,
  });

  @override
  Widget build(BuildContext context) {
    return AppCard(
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
    );
  }
}
