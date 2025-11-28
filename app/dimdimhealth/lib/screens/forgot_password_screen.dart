import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:email_validator/email_validator.dart';
import '../services/auth_provider.dart';
import '../widgets/widgets.dart';

class ForgotPasswordScreen extends StatefulWidget {
  const ForgotPasswordScreen({super.key});

  @override
  State<ForgotPasswordScreen> createState() => _ForgotPasswordScreenState();
}

class _ForgotPasswordScreenState extends State<ForgotPasswordScreen> {
  final _formKey = GlobalKey<FormState>();
  final _emailController = TextEditingController();
  bool _emailSent = false;

  @override
  void dispose() {
    _emailController.dispose();
    super.dispose();
  }

  Future<void> _handleForgotPassword() async {
    if (_formKey.currentState!.validate()) {
      final authProvider = Provider.of<AuthProvider>(context, listen: false);

      try {
        final success = await authProvider.forgotPassword(
          _emailController.text.trim(),
        );

        if (mounted) {
          if (success) {
            setState(() {
              _emailSent = true;
            });
          } else {
            AppSnackBar.showError(
              context,
              authProvider.error ?? 'Request failed',
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
      child: _emailSent ? _buildSuccessView() : _buildFormView(),
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
                title: 'Forgot Password',
                subtitle: 'Enter your email to reset your password',
                showLogo: false,
                icon: Icons.lock_reset,
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
              const SizedBox(height: 32),

              // Submit Button
              PrimaryButton(
                text: 'Send Reset Link',
                onPressed: _handleForgotPassword,
                isLoading: authProvider.isLoading,
              ),
              const SizedBox(height: 24),

              // Back to Login Link
              LinkButton(
                text: 'Back to Login',
                onPressed: () {
                  Navigator.of(context).pop();
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
      title: 'Email Sent!',
      message: 'Check your email for instructions to reset your password.',
      buttonText: 'Back to Login',
      onButtonPressed: () {
        Navigator.of(context).pushReplacementNamed('/login');
      },
    );
  }
}
