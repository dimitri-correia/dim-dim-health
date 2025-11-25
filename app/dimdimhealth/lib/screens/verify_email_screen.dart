import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';

class VerifyEmailScreen extends StatefulWidget {
  final String? token;

  const VerifyEmailScreen({super.key, this.token});

  @override
  State<VerifyEmailScreen> createState() => _VerifyEmailScreenState();
}

class _VerifyEmailScreenState extends State<VerifyEmailScreen> {
  bool _isVerifying = true;
  bool _verificationSuccess = false;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _verifyEmail();
  }

  Future<void> _verifyEmail() async {
    final token = widget.token;
    if (token == null || token.isEmpty) {
      setState(() {
        _isVerifying = false;
        _errorMessage = 'Invalid verification token. Please request a new verification link.';
      });
      return;
    }

    final authProvider = Provider.of<AuthProvider>(context, listen: false);

    try {
      final success = await authProvider.verifyEmail(token: token);

      if (mounted) {
        setState(() {
          _isVerifying = false;
          _verificationSuccess = success;
          if (!success) {
            _errorMessage = authProvider.error ?? 'Verification failed';
          }
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isVerifying = false;
          _errorMessage = 'Error: $e';
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary),
        child: SafeArea(
          child: Center(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24.0),
              child: _buildContent(),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildContent() {
    if (_isVerifying) {
      return _buildVerifyingView();
    } else if (_verificationSuccess) {
      return _buildSuccessView();
    } else {
      return _buildErrorView();
    }
  }

  Widget _buildVerifyingView() {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const CircularProgressIndicator(
          valueColor: AlwaysStoppedAnimation<Color>(AppConfig.goldColor),
        ),
        const SizedBox(height: 32),
        const Text(
          'Verifying your email...',
          style: TextStyle(
            fontSize: 24,
            fontWeight: FontWeight.bold,
            color: AppConfig.goldColor,
          ),
        ),
        const SizedBox(height: 16),
        const Text(
          'Please wait while we verify your email address.',
          style: TextStyle(fontSize: 16, color: AppConfig.whiteColor),
          textAlign: TextAlign.center,
        ),
      ],
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
          child: const Icon(Icons.check, size: 80, color: AppConfig.whiteColor),
        ),
        const SizedBox(height: 32),
        const Text(
          'Email Verified!',
          style: TextStyle(
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: AppConfig.goldColor,
          ),
        ),
        const SizedBox(height: 16),
        const Text(
          'Your email has been verified successfully. You can now use all features of the app.',
          style: TextStyle(fontSize: 16, color: AppConfig.whiteColor),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 48),

        // Go to Login Button
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
              'Go to Login',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildErrorView() {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        // Error Icon
        Container(
          padding: const EdgeInsets.all(20),
          decoration: const BoxDecoration(
            color: Colors.red,
            shape: BoxShape.circle,
          ),
          child: const Icon(Icons.error_outline, size: 80, color: AppConfig.whiteColor),
        ),
        const SizedBox(height: 32),
        const Text(
          'Verification Failed',
          style: TextStyle(
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: AppConfig.goldColor,
          ),
        ),
        const SizedBox(height: 16),
        Text(
          _errorMessage ?? 'An error occurred during verification.',
          style: const TextStyle(fontSize: 16, color: AppConfig.whiteColor),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 48),

        // Go to Login Button
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
              'Go to Login',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
          ),
        ),
      ],
    );
  }
}
