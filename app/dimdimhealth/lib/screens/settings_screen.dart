import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:email_validator/email_validator.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';
import '../models/user_profile_image.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({super.key});

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  final _usernameController = TextEditingController();
  final _emailController = TextEditingController();
  final _currentPasswordController = TextEditingController();
  final _newPasswordController = TextEditingController();
  final _confirmPasswordController = TextEditingController();
  
  bool _obscureCurrentPassword = true;
  bool _obscureNewPassword = true;
  bool _obscureConfirmPassword = true;
  
  UserProfileImage? _selectedAvatar;
  bool _isLoading = false;

  @override
  void initState() {
    super.initState();
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final user = authProvider.user;
    if (user != null) {
      _usernameController.text = user.username;
      _emailController.text = user.email;
      _selectedAvatar = user.profileImage;
    }
  }

  @override
  void dispose() {
    _usernameController.dispose();
    _emailController.dispose();
    _currentPasswordController.dispose();
    _newPasswordController.dispose();
    _confirmPasswordController.dispose();
    super.dispose();
  }

  Future<void> _handleUpdateSettings() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final user = authProvider.user;
    if (user == null) return;

    setState(() {
      _isLoading = true;
    });

    String? username;
    String? email;
    String? profileImage;
    String? currentPassword;
    String? newPassword;

    // Check what has changed
    if (_usernameController.text.trim() != user.username) {
      username = _usernameController.text.trim();
    }

    if (_emailController.text.trim() != user.email) {
      if (!EmailValidator.validate(_emailController.text.trim())) {
        _showError('Please enter a valid email address');
        setState(() {
          _isLoading = false;
        });
        return;
      }
      email = _emailController.text.trim();
    }

    if (_selectedAvatar != null && _selectedAvatar != user.profileImage) {
      profileImage = _selectedAvatar!.toJson();
    }

    // Handle password change
    if (_currentPasswordController.text.isNotEmpty ||
        _newPasswordController.text.isNotEmpty) {
      if (_currentPasswordController.text.isEmpty) {
        _showError('Please enter your current password');
        setState(() {
          _isLoading = false;
        });
        return;
      }
      
      if (_newPasswordController.text.isEmpty) {
        _showError('Please enter a new password');
        setState(() {
          _isLoading = false;
        });
        return;
      }

      if (_newPasswordController.text.length < 6) {
        _showError('New password must be at least 6 characters');
        setState(() {
          _isLoading = false;
        });
        return;
      }

      if (_newPasswordController.text != _confirmPasswordController.text) {
        _showError('New passwords do not match');
        setState(() {
          _isLoading = false;
        });
        return;
      }

      currentPassword = _currentPasswordController.text;
      newPassword = _newPasswordController.text;
    }

    // Check if anything changed
    if (username == null &&
        email == null &&
        profileImage == null &&
        currentPassword == null) {
      _showError('No changes to save');
      setState(() {
        _isLoading = false;
      });
      return;
    }

    final message = await authProvider.updateSettings(
      username: username,
      email: email,
      profileImage: profileImage,
      currentPassword: currentPassword,
      newPassword: newPassword,
    );

    setState(() {
      _isLoading = false;
    });

    if (mounted) {
      if (message != null) {
        // Clear password fields after successful update
        _currentPasswordController.clear();
        _newPasswordController.clear();
        _confirmPasswordController.clear();
        
        _showSuccess(message);
        
        // Update the form with new user data
        final updatedUser = authProvider.user;
        if (updatedUser != null) {
          _usernameController.text = updatedUser.username;
          _emailController.text = updatedUser.email;
          _selectedAvatar = updatedUser.profileImage;
        }
      } else {
        _showError(authProvider.error ?? 'Failed to update settings');
      }
    }
  }

  void _showError(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: Colors.red,
      ),
    );
  }

  void _showSuccess(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: Colors.green,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final authProvider = Provider.of<AuthProvider>(context);
    final user = authProvider.user;

    if (user == null) {
      return Scaffold(
        appBar: AppBar(
          title: const Text('Settings'),
          backgroundColor: AppConfig.blueColor,
          foregroundColor: AppConfig.goldColor,
        ),
        body: const Center(
          child: Text('Please log in to access settings'),
        ),
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Settings'),
        backgroundColor: AppConfig.blueColor,
        foregroundColor: AppConfig.goldColor,
      ),
      body: Container(
        decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary),
        child: SafeArea(
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(24.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Profile Section
                Center(
                  child: Column(
                    children: [
                      if (_selectedAvatar != null)
                        _selectedAvatar!.buildAvatar(size: 80),
                      const SizedBox(height: 16),
                      Text(
                        user.username,
                        style: const TextStyle(
                          fontSize: 24,
                          fontWeight: FontWeight.bold,
                          color: AppConfig.goldColor,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        user.email,
                        style: const TextStyle(
                          fontSize: 14,
                          color: AppConfig.whiteColor,
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 32),

                // Username Field
                const Text(
                  'Username',
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: AppConfig.whiteColor,
                  ),
                ),
                const SizedBox(height: 8),
                TextFormField(
                  controller: _usernameController,
                  decoration: InputDecoration(
                    prefixIcon: const Icon(Icons.person),
                    filled: true,
                    fillColor: AppConfig.whiteColor,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),
                const SizedBox(height: 24),

                // Email Field
                const Text(
                  'Email',
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: AppConfig.whiteColor,
                  ),
                ),
                const SizedBox(height: 8),
                TextFormField(
                  controller: _emailController,
                  keyboardType: TextInputType.emailAddress,
                  decoration: InputDecoration(
                    prefixIcon: const Icon(Icons.email),
                    filled: true,
                    fillColor: AppConfig.whiteColor,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),
                const SizedBox(height: 8),
                const Text(
                  'Note: Your login email will remain the same until you verify the new email.',
                  style: TextStyle(
                    fontSize: 12,
                    color: AppConfig.goldColor,
                    fontStyle: FontStyle.italic,
                  ),
                ),
                const SizedBox(height: 24),

                // Avatar Selection
                const Text(
                  'Choose Your Avatar',
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: AppConfig.whiteColor,
                  ),
                ),
                const SizedBox(height: 16),
                Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: AppConfig.whiteColor,
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                    children: UserProfileImage.values.map((avatar) {
                      final isSelected = _selectedAvatar == avatar;
                      return GestureDetector(
                        onTap: () {
                          setState(() {
                            _selectedAvatar = avatar;
                          });
                        },
                        child: Container(
                          decoration: BoxDecoration(
                            shape: BoxShape.circle,
                            border: Border.all(
                              color: isSelected
                                  ? AppConfig.goldColor
                                  : Colors.transparent,
                              width: 3,
                            ),
                          ),
                          child: avatar.buildAvatar(size: 50),
                        ),
                      );
                    }).toList(),
                  ),
                ),
                const SizedBox(height: 32),

                // Password Change Section
                const Text(
                  'Change Password',
                  style: TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                    color: AppConfig.goldColor,
                  ),
                ),
                const SizedBox(height: 16),

                // Current Password
                TextFormField(
                  controller: _currentPasswordController,
                  obscureText: _obscureCurrentPassword,
                  decoration: InputDecoration(
                    labelText: 'Current Password',
                    prefixIcon: const Icon(Icons.lock),
                    suffixIcon: IconButton(
                      icon: Icon(
                        _obscureCurrentPassword
                            ? Icons.visibility_off
                            : Icons.visibility,
                      ),
                      onPressed: () {
                        setState(() {
                          _obscureCurrentPassword = !_obscureCurrentPassword;
                        });
                      },
                    ),
                    filled: true,
                    fillColor: AppConfig.whiteColor,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),
                const SizedBox(height: 16),

                // New Password
                TextFormField(
                  controller: _newPasswordController,
                  obscureText: _obscureNewPassword,
                  decoration: InputDecoration(
                    labelText: 'New Password',
                    prefixIcon: const Icon(Icons.lock_outline),
                    suffixIcon: IconButton(
                      icon: Icon(
                        _obscureNewPassword
                            ? Icons.visibility_off
                            : Icons.visibility,
                      ),
                      onPressed: () {
                        setState(() {
                          _obscureNewPassword = !_obscureNewPassword;
                        });
                      },
                    ),
                    filled: true,
                    fillColor: AppConfig.whiteColor,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),
                const SizedBox(height: 16),

                // Confirm New Password
                TextFormField(
                  controller: _confirmPasswordController,
                  obscureText: _obscureConfirmPassword,
                  decoration: InputDecoration(
                    labelText: 'Confirm New Password',
                    prefixIcon: const Icon(Icons.lock_outline),
                    suffixIcon: IconButton(
                      icon: Icon(
                        _obscureConfirmPassword
                            ? Icons.visibility_off
                            : Icons.visibility,
                      ),
                      onPressed: () {
                        setState(() {
                          _obscureConfirmPassword = !_obscureConfirmPassword;
                        });
                      },
                    ),
                    filled: true,
                    fillColor: AppConfig.whiteColor,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),
                const SizedBox(height: 32),

                // Save Button
                SizedBox(
                  width: double.infinity,
                  height: 50,
                  child: ElevatedButton(
                    onPressed: _isLoading ? null : _handleUpdateSettings,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppConfig.goldColor,
                      foregroundColor: AppConfig.blueColor,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                    ),
                    child: _isLoading
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
                            'Save Changes',
                            style: TextStyle(
                              fontSize: 18,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                  ),
                ),
                const SizedBox(height: 24),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
