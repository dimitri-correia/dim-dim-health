import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import 'services/auth_provider.dart';

import 'screens/splash_screen.dart';
import 'screens/login_screen.dart';
import 'screens/register_screen.dart';
import 'screens/forgot_password_screen.dart';
import 'screens/reset_password_screen.dart';
import 'screens/verify_email_screen.dart';
import 'screens/home_screen.dart';
import 'screens/weight_screen.dart';
import 'screens/meals_screen.dart';
import 'screens/gym_screen.dart';
import 'screens/manage_watchers_screen.dart';
import 'screens/watching_screen.dart';
import 'screens/settings_screen.dart';

import 'utils/app_config.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (_) => AuthProvider(),
      child: MaterialApp(
        title: AppConfig.appName,
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(
            seedColor: AppConfig.blueColor,
            primary: AppConfig.blueColor,
            secondary: AppConfig.goldColor,
          ),
          useMaterial3: true,
        ),
        home: const AuthWrapper(),
        routes: {
          '/login': (context) => const GuestGuard(child: LoginScreen()),
          '/register': (context) => const GuestGuard(child: RegisterScreen()),
          '/forgot-password': (context) =>
              const GuestGuard(child: ForgotPasswordScreen()),
          '/home': (context) => const AuthGuard(child: HomeScreen()),
          '/weight': (context) => const AuthGuard(child: WeightScreen()),
          '/meals': (context) => const AuthGuard(child: MealsScreen()),
          '/gym': (context) => const AuthGuard(child: GymScreen()),
          '/manage-watchers': (context) =>
              const AuthGuard(child: ManageWatchersScreen()),
          '/watching': (context) => const AuthGuard(child: WatchingScreen()),
          '/settings': (context) => const AuthGuard(child: SettingsScreen()),
        },
        onGenerateRoute: (settings) {
          if (settings.name?.startsWith('/reset-password') ?? false) {
            final uri = Uri.parse(settings.name!);
            final token = uri.queryParameters['token'];
            return MaterialPageRoute(
              builder: (context) =>
                  GuestGuard(child: ResetPasswordScreen(token: token)),
            );
          }
          if (settings.name?.startsWith('/verify-email') ?? false) {
            final uri = Uri.parse(settings.name!);
            final token = uri.queryParameters['token'];
            return MaterialPageRoute(
              builder: (context) =>
                  GuestGuard(child: VerifyEmailScreen(token: token)),
            );
          }
          return null;
        },
      ),
    );
  }
}

class AuthWrapper extends StatefulWidget {
  const AuthWrapper({super.key});

  @override
  State<AuthWrapper> createState() => _AuthWrapperState();
}

class _AuthWrapperState extends State<AuthWrapper> {
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _checkAuthStatus();
  }

  Future<void> _checkAuthStatus() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    await authProvider.loadSavedAuth();

    await Future.delayed(
      Duration(seconds: AppConfig.splashScreenDurationInSeconds),
    );

    setState(() {
      _isLoading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const SplashScreen();
    }

    return Consumer<AuthProvider>(
      builder: (context, authProvider, _) {
        if (authProvider.isAuthenticated) {
          return const HomeScreen();
        } else {
          return const LoginScreen();
        }
      },
    );
  }
}

class AuthGuard extends StatelessWidget {
  final Widget child;

  const AuthGuard({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Selector<AuthProvider, bool>(
      selector: (_, authProvider) => authProvider.isAuthenticated,
      builder: (context, isAuthenticated, _) {
        if (!isAuthenticated) {
          // Redirect to login if not authenticated
          WidgetsBinding.instance.addPostFrameCallback((_) {
            Navigator.of(context).pushReplacementNamed('/login');
          });
          return const SplashScreen();
        }
        return child;
      },
    );
  }
}

class GuestGuard extends StatelessWidget {
  final Widget child;

  const GuestGuard({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Selector<AuthProvider, bool>(
      selector: (_, authProvider) => authProvider.isAuthenticated,
      builder: (context, isAuthenticated, _) {
        if (isAuthenticated) {
          // Redirect to home if already authenticated
          WidgetsBinding.instance.addPostFrameCallback((_) {
            Navigator.of(context).pushReplacementNamed('/home');
          });
          return const SplashScreen();
        }
        return child;
      },
    );
  }
}
