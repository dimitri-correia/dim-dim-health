import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/auth_provider.dart';
import '../widgets/widgets.dart';

class ResetPasswordScreen extends StatefulWidget {
  final String? token;

  const ResetPasswordScreen({super.key, this.token});

  @override
  State<ResetPasswordScreen> createState() => _ResetPasswordScreenState();
}

class _ResetPasswordScreenState extends State<ResetPasswordScreen> {
  final _formKey = GlobalKey<FormState>();
  final _passwordController = TextEditingController();
  final _confirmPasswordController = TextEditingController();
  bool _resetSuccess = false;

  @override
  void dispose() {
    _passwordController.dispose();
    _confirmPasswordController.dispose();
    super.dispose();
  }

  Future<void> _handleResetPassword() async {
    if (_formKey.currentState!.validate()) {
      final token = widget.token;
      if (token == null || token.isEmpty) {
        if (mounted) {
          AppSnackBar.showError(
            context,
            'Invalid reset token. Please request a new reset link.',
          );
        }
        return;
      }

      final authProvider = Provider.of<AuthProvider>(context, listen: false);

      try {
        final success = await authProvider.resetPassword(
          token: token,
          newPassword: _passwordController.text.trim(),
        );

        if (mounted) {
          if (success) {
            setState(() {
              _resetSuccess = true;
            });
          } else {
            AppSnackBar.showError(
              context,
              authProvider.error ?? 'Reset failed',
            );
          }
        }
      } catch (e) {
        if (mounted) {
          AppSnackBar.showError(context, 'Error: $e');
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return AuthScreenWrapper(
      child: _resetSuccess ? _buildSuccessView() : _buildFormView(),
    );
  }

  Widget _buildFormView() {
    return Consumer<AuthProvider>(
      builder: (context, authProvider, _) {
        return Form(
          key: _formKey,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const AuthHeader(
                title: 'Reset Password',
                subtitle: 'Enter your new password',
                showLogo: false,
                icon: Icons.lock_reset,
              ),

              // New Password Field
              PasswordTextField(
                controller: _passwordController,
                labelText: 'New Password',
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a password';
                  }
                  if (value.length < 8) {
                    return 'Password must be at least 8 characters';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),

              // Confirm Password Field
              PasswordTextField(
                controller: _confirmPasswordController,
                labelText: 'Confirm Password',
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please confirm your password';
                  }
                  if (value != _passwordController.text) {
                    return 'Passwords do not match';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 32),

              // Submit Button
              PrimaryButton(
                text: 'Reset Password',
                onPressed: _handleResetPassword,
                isLoading: authProvider.isLoading,
              ),
              const SizedBox(height: 24),

              // Back to Login Link
              LinkButton(
                text: 'Back to Login',
                onPressed: () {
                  Navigator.of(context).pushReplacementNamed('/login');
                },
              ),
            ],
          ),
        );
      },
    );
  }

  Widget _buildSuccessView() {
    return SuccessView(
      title: 'Password Reset!',
      message: 'Your password has been reset successfully. You are now logged in.',
      buttonText: 'Go to Home',
      onButtonPressed: () {
        Navigator.of(context).pushReplacementNamed('/home');
      },
    );
  }
}
