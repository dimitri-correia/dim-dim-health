import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:email_validator/email_validator.dart';
import '../services/auth_provider.dart';
import '../widgets/widgets.dart';

class LoginScreen extends StatefulWidget {
  const LoginScreen({super.key});

  @override
  State<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends State<LoginScreen> {
  final _formKey = GlobalKey<FormState>();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    super.dispose();
  }

  Future<void> _handleLogin() async {
    if (_formKey.currentState!.validate()) {
      final authProvider = Provider.of<AuthProvider>(context, listen: false);

      final success = await authProvider.login(
        email: _emailController.text.trim(),
        password: _passwordController.text,
      );

      if (mounted) {
        if (success) {
          Navigator.of(context).pushReplacementNamed('/home');
        } else {
          AppSnackBar.showError(context, authProvider.error ?? 'Login failed');
        }
      }
    }
  }

  Future<void> _handleGuestLogin() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);

    final success = await authProvider.loginAsGuest();

    if (mounted) {
      if (success) {
        Navigator.of(context).pushReplacementNamed('/home');
      } else {
        AppSnackBar.showError(
          context,
          authProvider.error ?? 'Guest login failed',
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final authProvider = Provider.of<AuthProvider>(context);

    return AuthScreenWrapper(
      child: Form(
        key: _formKey,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const AuthHeader(
              title: 'DimDim Health',
              subtitle: 'Login to your account or continue as a guest',
            ),

            // Email Field
            AuthTextField(
              controller: _emailController,
              labelText: 'Email',
              prefixIcon: Icons.email,
              keyboardType: TextInputType.emailAddress,
              validator: (value) {
                if (value == null || value.isEmpty) {
                  return 'Please enter your email';
                }
                if (!EmailValidator.validate(value)) {
                  return 'Please enter a valid email';
                }
                return null;
              },
            ),
            const SizedBox(height: 24),

            // Password Field
            PasswordTextField(
              controller: _passwordController,
              validator: (value) {
                if (value == null || value.isEmpty) {
                  return 'Please enter your password';
                }
                return null;
              },
            ),
            const SizedBox(height: 8),

            // Forgot Password Link
            Align(
              alignment: Alignment.centerLeft,
              child: LinkButton(
                text: 'Forgot Password?',
                onPressed: () {
                  Navigator.of(context).pushNamed('/forgot-password');
                },
              ),
            ),
            const SizedBox(height: 24),

            // Login Button
            PrimaryButton(
              text: 'Login',
              onPressed: _handleLogin,
              isLoading: authProvider.isLoading,
            ),
            const SizedBox(height: 24),

            // Divider
            const OrDivider(),
            const SizedBox(height: 24),

            // Guest Login Button
            SecondaryButton(
              text: 'Continue as Guest',
              icon: Icons.person_outline,
              onPressed: _handleGuestLogin,
              isLoading: authProvider.isLoading,
            ),
            const SizedBox(height: 24),

            // Register Link
            AuthNavigationLink(
              leadingText: "Don't have an account? ",
              linkText: 'Register',
              onPressed: () {
                Navigator.of(context).pushReplacementNamed('/register');
              },
            ),
            const SizedBox(height: 24),
          ],
        ),
      ),
    );
  }
}
