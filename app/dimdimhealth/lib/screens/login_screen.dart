import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:email_validator/email_validator.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';

class LoginScreen extends StatefulWidget {
  const LoginScreen({super.key});

  @override
  State<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends State<LoginScreen> {
  final _formKey = GlobalKey<FormState>();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  bool _obscurePassword = true;

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
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(authProvider.error ?? 'Login failed'),
              backgroundColor: AppConfig.redColor,
            ),
          );
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
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(authProvider.error ?? 'Guest login failed'),
            backgroundColor: AppConfig.redColor,
          ),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final authProvider = Provider.of<AuthProvider>(context);

    return Scaffold(
      body: Container(
        decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary),
        child: SafeArea(
          child: Center(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24.0),
              child: Form(
                key: _formKey,
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Image.asset('images/health.jpg', width: 80, height: 80),
                    const SizedBox(height: 16),
                    const Text(
                      'DimDim Health',
                      style: TextStyle(
                        fontSize: 32,
                        fontWeight: FontWeight.bold,
                        color: AppConfig.goldColor,
                      ),
                    ),
                    const SizedBox(height: 8),
                    const Text(
                      'Login to your account or continue as a guest',
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
                        labelStyle: const TextStyle(color: AppConfig.goldColor),
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
                    const SizedBox(height: 24),

                    // Password Field
                    TextFormField(
                      controller: _passwordController,
                      obscureText: _obscurePassword,
                      decoration: InputDecoration(
                        labelText: 'Password',
                        labelStyle: const TextStyle(color: AppConfig.goldColor),
                        prefixIcon: const Icon(Icons.lock),
                        suffixIcon: IconButton(
                          icon: Icon(
                            _obscurePassword
                                ? Icons.visibility_off
                                : Icons.visibility,
                          ),
                          onPressed: () {
                            setState(() {
                              _obscurePassword = !_obscurePassword;
                            });
                          },
                        ),
                        filled: true,
                        fillColor: AppConfig.whiteColor,
                        border: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(12),
                        ),
                      ),
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
                      child: TextButton(
                        onPressed: () {
                          Navigator.of(context).pushNamed('/forgot-password');
                        },
                        child: const Text(
                          'Forgot Password?',
                          style: TextStyle(color: AppConfig.goldColor),
                        ),
                      ),
                    ),
                    const SizedBox(height: 24),

                    // Login Button
                    SizedBox(
                      width: double.infinity,
                      height: 50,
                      child: ElevatedButton(
                        onPressed: authProvider.isLoading ? null : _handleLogin,
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
                                'Login',
                                style: TextStyle(
                                  fontSize: 18,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                      ),
                    ),
                    const SizedBox(height: 24),

                    // Divider
                    const Row(
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
                    ),
                    const SizedBox(height: 24),

                    // Guest Login Button
                    SizedBox(
                      width: double.infinity,
                      height: 50,
                      child: OutlinedButton.icon(
                        onPressed: authProvider.isLoading
                            ? null
                            : _handleGuestLogin,
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
                        icon: const Icon(Icons.person_outline),
                        label: authProvider.isLoading
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
                            : const Text(
                                'Continue as Guest',
                                style: TextStyle(
                                  fontSize: 16,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                      ),
                    ),
                    const SizedBox(height: 24),

                    // Register Link
                    Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        const Text(
                          "Don't have an account? ",
                          style: TextStyle(
                            color: AppConfig.whiteColor,
                            fontSize: 16,
                          ),
                        ),
                        TextButton(
                          onPressed: () {
                            Navigator.of(
                              context,
                            ).pushReplacementNamed('/register');
                          },
                          child: const Text(
                            'Register',
                            style: TextStyle(
                              color: AppConfig.goldColor,
                              fontWeight: FontWeight.bold,
                              fontSize: 16,
                            ),
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 24),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
