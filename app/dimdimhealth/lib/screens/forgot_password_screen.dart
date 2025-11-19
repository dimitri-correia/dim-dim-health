import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:email_validator/email_validator.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';

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
      
      final success = await authProvider.forgotPassword(
        _emailController.text.trim(),
      );

      if (mounted) {
        if (success) {
          setState(() {
            _emailSent = true;
          });
        } else {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(authProvider.error ?? 'Request failed'),
              backgroundColor: Colors.red,
            ),
          );
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final authProvider = Provider.of<AuthProvider>(context);

    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topCenter,
            end: Alignment.bottomCenter,
            colors: [
              AppConfig.blueColor,
              AppConfig.blueColor.withOpacity(0.8),
            ],
          ),
        ),
        child: SafeArea(
          child: Center(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24.0),
              child: _emailSent
                  ? _buildSuccessView()
                  : _buildFormView(authProvider),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildFormView(AuthProvider authProvider) {
    return Form(
      key: _formKey,
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          // Logo or App Icon
          const Icon(
            Icons.lock_reset,
            size: 80,
            color: AppConfig.goldColor,
          ),
          const SizedBox(height: 16),
          const Text(
            'Forgot Password',
            style: TextStyle(
              fontSize: 32,
              fontWeight: FontWeight.bold,
              color: AppConfig.goldColor,
            ),
          ),
          const SizedBox(height: 8),
          const Text(
            'Enter your email to reset your password',
            style: TextStyle(
              fontSize: 16,
              color: AppConfig.whiteColor,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 48),
          
          // Email Field
          TextFormField(
            controller: _emailController,
            keyboardType: TextInputType.emailAddress,
            decoration: InputDecoration(
              labelText: 'Email',
              prefixIcon: const Icon(Icons.email),
              filled: true,
              fillColor: AppConfig.whiteColor,
              border: OutlineInputBorder(
                borderRadius: BorderRadius.circular(12),
              ),
            ),
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
          SizedBox(
            width: double.infinity,
            height: 50,
            child: ElevatedButton(
              onPressed: authProvider.isLoading ? null : _handleForgotPassword,
              style: ElevatedButton.styleFrom(
                backgroundColor: AppConfig.goldColor,
                foregroundColor: AppConfig.blueColor,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12),
                ),
              ),
              child: authProvider.isLoading
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
                  : const Text(
                      'Send Reset Link',
                      style: TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
            ),
          ),
          const SizedBox(height: 24),
          
          // Back to Login Link
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
            },
            child: const Text(
              'Back to Login',
              style: TextStyle(
                color: AppConfig.goldColor,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSuccessView() {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        // Success Icon
        Container(
          padding: const EdgeInsets.all(20),
          decoration: const BoxDecoration(
            color: Colors.green,
            shape: BoxShape.circle,
          ),
          child: const Icon(
            Icons.check,
            size: 80,
            color: AppConfig.whiteColor,
          ),
        ),
        const SizedBox(height: 32),
        const Text(
          'Email Sent!',
          style: TextStyle(
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: AppConfig.goldColor,
          ),
        ),
        const SizedBox(height: 16),
        const Text(
          'Check your email for instructions to reset your password.',
          style: TextStyle(
            fontSize: 16,
            color: AppConfig.whiteColor,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 48),
        
        // Back to Login Button
        SizedBox(
          width: double.infinity,
          height: 50,
          child: ElevatedButton(
            onPressed: () {
              Navigator.of(context).pushReplacementNamed('/login');
            },
            style: ElevatedButton.styleFrom(
              backgroundColor: AppConfig.goldColor,
              foregroundColor: AppConfig.blueColor,
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(12),
              ),
            ),
            child: const Text(
              'Back to Login',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
        ),
      ],
    );
  }
}
