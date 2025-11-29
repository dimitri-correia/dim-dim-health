import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/auth_provider.dart';
import '../widgets/widgets.dart';

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
    return AuthScreenWrapper(
      child: _buildContent(),
    );
  }

  Widget _buildContent() {
    if (_isVerifying) {
      return const LoadingView(
        title: 'Verifying your email...',
        message: 'Please wait while we verify your email address.',
      );
    } else if (_verificationSuccess) {
      return SuccessView(
        title: 'Email Verified!',
        message: 'Your email has been verified successfully. You can now use all features of the app.',
        buttonText: 'Go to Login',
        onButtonPressed: () {
          Navigator.of(context).pushReplacementNamed('/login');
        },
      );
    } else {
      return ErrorView(
        title: 'Verification Failed',
        message: _errorMessage ?? 'An error occurred during verification.',
        buttonText: 'Go to Login',
        onButtonPressed: () {
          Navigator.of(context).pushReplacementNamed('/login');
        },
      );
    }
  }
}
