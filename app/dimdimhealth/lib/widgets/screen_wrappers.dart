import 'package:flutter/material.dart';
import '../utils/app_config.dart';

/// A wrapper component for authentication-related screens.
/// Provides consistent styling with primary color background and centered content.
class AuthScreenWrapper extends StatelessWidget {
  final Widget child;
  final bool useGradient;

  const AuthScreenWrapper({
    super.key,
    required this.child,
    this.useGradient = false,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: useGradient
            ? BoxDecoration(
                gradient: LinearGradient(
                  begin: Alignment.topCenter,
                  end: Alignment.bottomCenter,
                  colors: [
                    Theme.of(context).colorScheme.primary,
                    Theme.of(context).colorScheme.primaryContainer,
                  ],
                ),
              )
            : BoxDecoration(color: Theme.of(context).colorScheme.primary),
        child: SafeArea(
          child: Center(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24.0),
              child: child,
            ),
          ),
        ),
      ),
    );
  }
}

/// A standard screen layout used for authenticated screens.
/// Includes app bar and primary color background.
class AppScreenWrapper extends StatelessWidget {
  final Widget child;
  final PreferredSizeWidget? appBar;
  final Widget? floatingActionButton;
  final Future<void> Function()? onRefresh;

  const AppScreenWrapper({
    super.key,
    required this.child,
    this.appBar,
    this.floatingActionButton,
    this.onRefresh,
  });

  @override
  Widget build(BuildContext context) {
    Widget body = Container(
      width: double.infinity,
      height: double.infinity,
      decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary),
      child: SafeArea(child: child),
    );

    if (onRefresh != null) {
      body = Container(
        width: double.infinity,
        height: double.infinity,
        decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary),
        child: SafeArea(
          child: RefreshIndicator(
            onRefresh: onRefresh!,
            child: child,
          ),
        ),
      );
    }

    return Scaffold(
      appBar: appBar,
      body: body,
      floatingActionButton: floatingActionButton,
    );
  }
}

/// Standard app bar used across the application.
class AppStandardAppBar extends StatelessWidget implements PreferredSizeWidget {
  final String title;
  final List<Widget>? actions;
  final Widget? leading;

  const AppStandardAppBar({
    super.key,
    required this.title,
    this.actions,
    this.leading,
  });

  @override
  Widget build(BuildContext context) {
    return AppBar(
      title: Text(title),
      backgroundColor: AppConfig.blueColor,
      foregroundColor: AppConfig.goldColor,
      leading: leading,
      actions: actions,
    );
  }

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}
