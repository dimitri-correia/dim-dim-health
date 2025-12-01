import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:email_validator/email_validator.dart';
import '../models/user_info.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';
import '../widgets/widgets.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({super.key});

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  final _formKey = GlobalKey<FormState>();
  final _usernameController = TextEditingController();
  final _emailController = TextEditingController();
  final _currentPasswordController = TextEditingController();
  final _newPasswordController = TextEditingController();
  final _confirmPasswordController = TextEditingController();

  // User additional info controllers
  final _heightController = TextEditingController();
  DateTime? _selectedBirthDate;
  Gender? _selectedGender;
  double _selectedActivityLevel = 1.2;

  bool _isSubmitting = false;
  bool _isUserInfoSubmitting = false;
  String? _selectedAvatar;

  // Available avatar options matching the backend enum
  static const List<String> _avatarOptions = [
    'avatar1',
    'avatar2',
    'avatar3',
    'avatar4',
    'avatar5',
  ];

  // Activity level options
  static const Map<double, String> _activityLevels = {
    1.2: 'Sedentary (little or no exercise)',
    1.375: 'Lightly active (1-3 days/week)',
    1.55: 'Moderately active (3-5 days/week)',
    1.725: 'Very active (6-7 days/week)',
    1.9: 'Extra active (very hard exercise)',
  };

  @override
  void initState() {
    super.initState();
    _initializeFields();
    _fetchUserGroups();
    _fetchUserInfo();
  }

  void _initializeFields() {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final user = authProvider.user;
    if (user != null) {
      _usernameController.text = user.username;
      _emailController.text = user.email;
      _selectedAvatar = user.profileImage;
    }
  }

  void _initializeUserInfoFields(AuthProvider authProvider) {
    final userInfo = authProvider.userInfo;
    if (userInfo != null) {
      _heightController.text = userInfo.heightInCm.toString();
      _selectedBirthDate = DateTime.parse(userInfo.birthDate);
      _selectedGender = userInfo.gender;
      _selectedActivityLevel = userInfo.activityLevel;
    }
  }

  Future<void> _fetchUserGroups() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    await authProvider.fetchUserGroups();
  }

  Future<void> _fetchUserInfo() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    await authProvider.fetchUserInfo();
    if (mounted) {
      _initializeUserInfoFields(authProvider);
      setState(() {});
    }
  }

  @override
  void dispose() {
    _usernameController.dispose();
    _emailController.dispose();
    _currentPasswordController.dispose();
    _newPasswordController.dispose();
    _confirmPasswordController.dispose();
    _heightController.dispose();
    super.dispose();
  }

  bool _hasChanges(AuthProvider authProvider) {
    final user = authProvider.user;
    if (user == null) return false;

    final usernameChanged = _usernameController.text.trim() != user.username;
    final emailChanged = _emailController.text.trim() != user.email;
    final avatarChanged =
        _selectedAvatar != null && _selectedAvatar != user.profileImage;
    final passwordChanged =
        _currentPasswordController.text.isNotEmpty &&
        _newPasswordController.text.isNotEmpty;

    return usernameChanged || emailChanged || avatarChanged || passwordChanged;
  }

  Future<void> _handleSave() async {
    if (!_formKey.currentState!.validate()) {
      return;
    }

    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final user = authProvider.user;
    if (user == null) return;

    setState(() {
      _isSubmitting = true;
    });

    String? username;
    String? email;
    String? profileImage;
    String? currentPassword;
    String? newPassword;

    // Only include changed fields
    if (_usernameController.text.trim() != user.username) {
      username = _usernameController.text.trim();
    }
    if (_emailController.text.trim() != user.email) {
      email = _emailController.text.trim();
    }
    if (_selectedAvatar != null && _selectedAvatar != user.profileImage) {
      profileImage = _selectedAvatar;
    }
    if (_currentPasswordController.text.isNotEmpty &&
        _newPasswordController.text.isNotEmpty) {
      currentPassword = _currentPasswordController.text;
      newPassword = _newPasswordController.text;
    }

    final result = await authProvider.updateSettings(
      username: username,
      email: email,
      profileImage: profileImage,
      currentPassword: currentPassword,
      newPassword: newPassword,
    );

    if (mounted) {
      setState(() {
        _isSubmitting = false;
      });

      if (result != null) {
        // Clear password fields after successful update
        _currentPasswordController.clear();
        _newPasswordController.clear();
        _confirmPasswordController.clear();

        AppSnackBar.showSuccess(context, result);
      } else {
        AppSnackBar.showError(context, authProvider.error ?? 'Update failed');
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final authProvider = Provider.of<AuthProvider>(context);
    final user = authProvider.user;

    return AppScreenWrapper(
      appBar: const AppStandardAppBar(title: 'Settings'),
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Form(
          key: _formKey,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Avatar Selection Section
              const SectionTitle(title: 'Profile Avatar'),
              const SizedBox(height: 16),
              _buildAvatarSelector(),
              const SizedBox(height: 32),

              // Account Information Section
              const SectionTitle(title: 'Account Information'),
              const SizedBox(height: 16),

              // Username Field
              AuthTextField(
                controller: _usernameController,
                labelText: 'Username',
                prefixIcon: Icons.person,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Username is required';
                  }
                  if (value.length < 3) {
                    return 'Username must be at least 3 characters';
                  }
                  if (value.length > 20) {
                    return 'Username must be at most 20 characters';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),

              // Email Field
              AuthTextField(
                controller: _emailController,
                labelText: 'Email',
                prefixIcon: Icons.email,
                keyboardType: TextInputType.emailAddress,
                helperText: 'Email changes require verification',
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Email is required';
                  }
                  if (!EmailValidator.validate(value)) {
                    return 'Please enter a valid email';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 32),

              // Change Password Section
              const SectionTitle(title: 'Change Password'),
              const SizedBox(height: 8),
              Text(
                'Leave empty to keep current password',
                style: TextStyle(
                  fontSize: 12,
                  color: AppConfig.whiteColor.withOpacity(0.8),
                ),
              ),
              const SizedBox(height: 16),

              // Current Password Field
              PasswordTextField(
                controller: _currentPasswordController,
                labelText: 'Current Password',
                prefixIcon: Icons.lock_outline,
                validator: (value) {
                  if (_newPasswordController.text.isNotEmpty &&
                      (value == null || value.isEmpty)) {
                    return 'Current password is required to change password';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),

              // New Password Field
              PasswordTextField(
                controller: _newPasswordController,
                labelText: 'New Password',
                validator: (value) {
                  if (_currentPasswordController.text.isNotEmpty &&
                      (value == null || value.isEmpty)) {
                    return 'New password is required';
                  }
                  if (value != null &&
                      value.isNotEmpty &&
                      value.length < 8) {
                    return 'Password must be at least 8 characters';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),

              // Confirm New Password Field
              PasswordTextField(
                controller: _confirmPasswordController,
                labelText: 'Confirm New Password',
                prefixIcon: Icons.lock_outline,
                validator: (value) {
                  if (_newPasswordController.text.isNotEmpty &&
                      value != _newPasswordController.text) {
                    return 'Passwords do not match';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 32),

              // Additional Info Section
              const SectionTitle(title: 'Health Information'),
              const SizedBox(height: 8),
              Text(
                'Update your personal health information for better tracking',
                style: TextStyle(
                  fontSize: 12,
                  color: AppConfig.whiteColor.withOpacity(0.8),
                ),
              ),
              const SizedBox(height: 16),
              _buildUserInfoCard(authProvider),
              const SizedBox(height: 32),

              // Public Group Section
              const SectionTitle(title: 'Public Group'),
              const SizedBox(height: 8),
              Text(
                'Join the public group to share your progress with others',
                style: TextStyle(
                  fontSize: 12,
                  color: AppConfig.whiteColor.withOpacity(0.8),
                ),
              ),
              const SizedBox(height: 16),
              _buildPublicGroupCard(authProvider),
              const SizedBox(height: 32),

              // Save Button
              PrimaryButton(
                text: 'Save Changes',
                onPressed: _hasChanges(authProvider) ? _handleSave : null,
                isLoading: _isSubmitting,
              ),
              const SizedBox(height: 16),

              // Account Info
              if (user != null) ...[
                const SizedBox(height: 16),
                Center(
                  child: Text(
                    'Account created: ${_formatDate(user.createdAt)}',
                    style: TextStyle(
                      fontSize: 12,
                      color: AppConfig.whiteColor.withOpacity(0.7),
                    ),
                  ),
                ),
                if (!user.emailVerified) ...[
                  const SizedBox(height: 8),
                  Center(
                    child: Container(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 12,
                        vertical: 6,
                      ),
                      decoration: BoxDecoration(
                        color: AppConfig.redColor.withOpacity(0.8),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: const Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Icon(
                            Icons.warning,
                            size: 16,
                            color: AppConfig.whiteColor,
                          ),
                          SizedBox(width: 8),
                          Text(
                            'Email not verified',
                            style: TextStyle(
                              fontSize: 12,
                              color: AppConfig.whiteColor,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ],
              ],
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildAvatarSelector() {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Text(
              'Select your avatar',
              style: TextStyle(fontSize: 14, color: Colors.grey.shade600),
            ),
            const SizedBox(height: 16),
            Wrap(
              spacing: 12,
              runSpacing: 12,
              alignment: WrapAlignment.center,
              children: _avatarOptions.map((avatar) {
                final isSelected = _selectedAvatar == avatar;
                return GestureDetector(
                  onTap: () {
                    setState(() {
                      _selectedAvatar = avatar;
                    });
                  },
                  child: Container(
                    width: 60,
                    height: 60,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(
                        color: isSelected
                            ? AppConfig.blueColor
                            : Colors.grey.shade300,
                        width: isSelected ? 3 : 1,
                      ),
                      boxShadow: isSelected
                          ? [
                              BoxShadow(
                                color: AppConfig.blueColor.withOpacity(0.3),
                                blurRadius: 8,
                                spreadRadius: 2,
                              ),
                            ]
                          : null,
                    ),
                    child: ClipOval(
                      child: Image.asset(
                        'avatars/${avatar.toLowerCase()}.png',
                        fit: BoxFit.cover,
                        errorBuilder: (context, error, stackTrace) {
                          return Container(
                            color: Colors.grey.shade300,
                            child: const Icon(Icons.person, size: 30),
                          );
                        },
                      ),
                    ),
                  ),
                );
              }).toList(),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildPublicGroupCard(AuthProvider authProvider) {
    final isInPublicGroup = authProvider.isInPublicGroup;
    final user = authProvider.user;
    final isGuest = user?.isGuest ?? false;
    final isEmailVerified = user?.emailVerified ?? false;
    final canJoinGroup = !isGuest && isEmailVerified;

    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Row(
              children: [
                Icon(
                  isInPublicGroup ? Icons.group : Icons.group_outlined,
                  color: isInPublicGroup ? Colors.green : Colors.grey,
                  size: 32,
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        isInPublicGroup
                            ? 'Member of Public Group'
                            : 'Not in Public Group',
                        style: TextStyle(
                          fontSize: 16,
                          fontWeight: FontWeight.bold,
                          color: isInPublicGroup
                              ? Colors.green.shade700
                              : Colors.grey.shade700,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        isInPublicGroup
                            ? 'Your progress is visible to other members'
                            : 'Join to share your progress',
                        style: TextStyle(
                          fontSize: 12,
                          color: Colors.grey.shade600,
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            if (!canJoinGroup) ...[
              Container(
                padding: const EdgeInsets.all(8),
                decoration: BoxDecoration(
                  color: Colors.orange.shade50,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Row(
                  children: [
                    Icon(
                      Icons.info_outline,
                      color: Colors.orange.shade700,
                      size: 20,
                    ),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        isGuest
                            ? 'Guest accounts cannot join groups'
                            : 'Verify your email to join groups',
                        style: TextStyle(
                          fontSize: 12,
                          color: Colors.orange.shade700,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 12),
            ],
            SizedBox(
              width: double.infinity,
              child: ElevatedButton.icon(
                onPressed: (!canJoinGroup || authProvider.isLoading)
                    ? null
                    : () async {
                        final bool success;
                        if (isInPublicGroup) {
                          success = await authProvider.leavePublicGroup();
                        } else {
                          success = await authProvider.joinPublicGroup();
                        }

                        if (mounted) {
                          ScaffoldMessenger.of(context).showSnackBar(
                            SnackBar(
                              content: Text(
                                success
                                    ? (isInPublicGroup
                                          ? 'Left public group'
                                          : 'Joined public group')
                                    : (authProvider.error ??
                                          'Operation failed'),
                              ),
                              backgroundColor: success
                                  ? Colors.green
                                  : Colors.red,
                            ),
                          );
                        }
                      },
                icon: authProvider.isLoading
                    ? const SizedBox(
                        width: 16,
                        height: 16,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      )
                    : Icon(isInPublicGroup ? Icons.logout : Icons.login),
                label: Text(
                  isInPublicGroup ? 'Leave Public Group' : 'Join Public Group',
                ),
                style: ElevatedButton.styleFrom(
                  backgroundColor: isInPublicGroup
                      ? Colors.red.shade400
                      : Colors.green.shade400,
                  foregroundColor: Colors.white,
                  disabledBackgroundColor: Colors.grey.shade300,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildUserInfoCard(AuthProvider authProvider) {
    final userInfo = authProvider.userInfo;
    final hasInfo = userInfo != null;
    final user = authProvider.user;
    final isGuest = user?.isGuest ?? false;
    final isEmailVerified = user?.emailVerified ?? false;
    final canEdit = !isGuest && isEmailVerified;

    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (!canEdit) ...[
              Container(
                padding: const EdgeInsets.all(8),
                decoration: BoxDecoration(
                  color: Colors.orange.shade50,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Row(
                  children: [
                    Icon(
                      Icons.info_outline,
                      color: Colors.orange.shade700,
                      size: 20,
                    ),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        isGuest
                            ? 'Guest accounts cannot set health info'
                            : 'Verify your email to set health info',
                        style: TextStyle(
                          fontSize: 12,
                          color: Colors.orange.shade700,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 12),
            ],
            // Birth Date
            Text(
              'Birth Date',
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.bold,
                color: Colors.grey.shade700,
              ),
            ),
            const SizedBox(height: 8),
            InkWell(
              onTap: canEdit ? () => _selectBirthDate(context) : null,
              child: Container(
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 16,
                ),
                decoration: BoxDecoration(
                  border: Border.all(color: Colors.grey.shade300),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      _selectedBirthDate != null
                          ? _formatDate(_selectedBirthDate!.toIso8601String())
                          : 'Select birth date',
                      style: TextStyle(
                        color: _selectedBirthDate != null
                            ? Colors.black87
                            : Colors.grey,
                      ),
                    ),
                    Icon(Icons.calendar_today, color: Colors.grey.shade600),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            // Height
            Text(
              'Height (cm)',
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.bold,
                color: Colors.grey.shade700,
              ),
            ),
            const SizedBox(height: 8),
            TextField(
              controller: _heightController,
              enabled: canEdit,
              keyboardType: TextInputType.number,
              decoration: InputDecoration(
                hintText: 'Enter height in cm',
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(8),
                ),
                prefixIcon: const Icon(Icons.height),
              ),
            ),
            const SizedBox(height: 16),
            // Gender
            Text(
              'Gender',
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.bold,
                color: Colors.grey.shade700,
              ),
            ),
            const SizedBox(height: 8),
            DropdownButtonFormField<Gender>(
              value: _selectedGender,
              decoration: InputDecoration(
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(8),
                ),
                prefixIcon: const Icon(Icons.person),
              ),
              items: Gender.values.map((gender) {
                return DropdownMenuItem<Gender>(
                  value: gender,
                  child: Text(gender.name[0].toUpperCase() +
                      gender.name.substring(1)),
                );
              }).toList(),
              onChanged: canEdit
                  ? (Gender? value) {
                      setState(() {
                        _selectedGender = value;
                      });
                    }
                  : null,
            ),
            const SizedBox(height: 16),
            // Activity Level
            Text(
              'Activity Level',
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.bold,
                color: Colors.grey.shade700,
              ),
            ),
            const SizedBox(height: 8),
            DropdownButtonFormField<double>(
              value: _selectedActivityLevel,
              isExpanded: true,
              decoration: InputDecoration(
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(8),
                ),
                prefixIcon: const Icon(Icons.fitness_center),
              ),
              items: _activityLevels.entries.map((entry) {
                return DropdownMenuItem<double>(
                  value: entry.key,
                  child: Text(
                    entry.value,
                    overflow: TextOverflow.ellipsis,
                  ),
                );
              }).toList(),
              onChanged: canEdit
                  ? (double? value) {
                      setState(() {
                        _selectedActivityLevel = value ?? 1.2;
                      });
                    }
                  : null,
            ),
            const SizedBox(height: 20),
            // Save User Info Button
            SizedBox(
              width: double.infinity,
              child: ElevatedButton.icon(
                onPressed: (!canEdit || _isUserInfoSubmitting)
                    ? null
                    : _handleSaveUserInfo,
                icon: _isUserInfoSubmitting
                    ? const SizedBox(
                        width: 16,
                        height: 16,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      )
                    : Icon(hasInfo ? Icons.update : Icons.add),
                label: Text(hasInfo ? 'Update Health Info' : 'Save Health Info'),
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppConfig.blueColor,
                  foregroundColor: Colors.white,
                  disabledBackgroundColor: Colors.grey.shade300,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _selectBirthDate(BuildContext context) async {
    final DateTime? picked = await showDatePicker(
      context: context,
      initialDate: _selectedBirthDate ?? DateTime.now().subtract(
        const Duration(days: 365 * 25),
      ),
      firstDate: DateTime(1900),
      lastDate: DateTime.now(),
    );
    if (picked != null && picked != _selectedBirthDate) {
      setState(() {
        _selectedBirthDate = picked;
      });
    }
  }

  Future<void> _handleSaveUserInfo() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final hasInfo = authProvider.userInfo != null;

    // Validate inputs
    final height = int.tryParse(_heightController.text);
    if (_selectedBirthDate == null) {
      AppSnackBar.showError(context, 'Please select birth date');
      return;
    }
    if (height == null || height < 50 || height > 300) {
      AppSnackBar.showError(context, 'Please enter a valid height (50-300 cm)');
      return;
    }
    if (_selectedGender == null) {
      AppSnackBar.showError(context, 'Please select gender');
      return;
    }

    setState(() {
      _isUserInfoSubmitting = true;
    });

    bool success;
    if (hasInfo) {
      success = await authProvider.updateUserInfo(
        birthDate: _selectedBirthDate,
        heightInCm: height,
        gender: _selectedGender,
        activityLevel: _selectedActivityLevel,
      );
    } else {
      success = await authProvider.createUserInfo(
        birthDate: _selectedBirthDate!,
        heightInCm: height,
        gender: _selectedGender!,
        activityLevel: _selectedActivityLevel,
      );
    }

    if (mounted) {
      setState(() {
        _isUserInfoSubmitting = false;
      });

      if (success) {
        AppSnackBar.showSuccess(
          context,
          hasInfo ? 'Health info updated' : 'Health info saved',
        );
      } else {
        AppSnackBar.showError(
          context,
          authProvider.error ?? 'Failed to save health info',
        );
      }
    }
  }

  String _formatDate(String dateStr) {
    try {
      final date = DateTime.parse(dateStr);
      final day = date.day.toString().padLeft(2, '0');
      final month = date.month.toString().padLeft(2, '0');
      final year = date.year.toString();
      return '$day/$month/$year';
    } catch (e) {
      return 'Unknown date';
    }
  }
}
