import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/user.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';
import '../widgets/user_avatar.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  Future<void> _handleRefresh() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    await authProvider.refreshUser();
  }

  @override
  Widget build(BuildContext context) {
    final authProvider = Provider.of<AuthProvider>(context);
    final user = authProvider.user;

    return Scaffold(
      appBar: AppBar(
        title: Text('Welcome ${user?.username ?? 'User'}'),
        backgroundColor: AppConfig.blueColor,
        foregroundColor: AppConfig.goldColor,
        leading: Padding(
          padding: const EdgeInsets.all(8.0),
          child: UserAvatar(profileImage: user?.profileImage),
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.logout),
            onPressed: () async {
              await authProvider.logout();
              if (context.mounted) {
                Navigator.of(context).pushReplacementNamed('/login');
              }
            },
            tooltip: 'Logout',
          ),
        ],
      ),
      body: Container(
        decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary),
        child: SafeArea(
          child: RefreshIndicator(
            onRefresh: _handleRefresh,
            child: user?.emailVerified == false
                ? _buildEmailVerificationRequired(context)
                : Padding(
                    padding: const EdgeInsets.all(24.0),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        // Guest Account Banner
                        if (user?.isGuest == true) ...[
                          _buildGuestAccountBanner(context, user!),
                          const SizedBox(height: 16),
                        ],

                        // Quick Actions
                        const Text(
                          'Quick Actions',
                          style: TextStyle(
                            fontSize: 22,
                            fontWeight: FontWeight.bold,
                            color: AppConfig.blueColor,
                          ),
                        ),
                        const SizedBox(height: 16),

                        // Action Cards Grid - Responsive layout
                        Expanded(
                          child: LayoutBuilder(
                            builder: (context, constraints) {
                              // Determine number of columns based on available width
                              // 2 columns for mobile, 4 columns for desktop
                              final crossAxisCount =
                                  constraints.maxWidth >=
                                      AppConfig.desktopBreakpoint
                                  ? 4
                                  : 2;

                              return GridView.count(
                                crossAxisCount: crossAxisCount,
                                mainAxisSpacing: 16,
                                crossAxisSpacing: 16,
                                children: [
                                  _buildActionCard(
                                    context,
                                    icon: Icons.monitor_weight,
                                    title: 'Weight',
                                    subtitle: 'Track your weight',
                                    color: AppConfig.blueColor,
                                    route: '/weight',
                                  ),
                                  _buildActionCard(
                                    context,
                                    icon: Icons.restaurant,
                                    title: 'Meals',
                                    subtitle: 'Log your meals',
                                    color: AppConfig.redColor,
                                  ),
                                  _buildActionCard(
                                    context,
                                    icon: Icons.fitness_center,
                                    title: 'Workouts',
                                    subtitle: 'Plan workouts',
                                    color: AppConfig.goldColor,
                                  ),
                                  _buildActionCard(
                                    context,
                                    icon: Icons.person,
                                    title: 'Profile',
                                    subtitle: 'View profile',
                                    color: Colors.teal,
                                  ),
                                  _buildActionCard(
                                    context,
                                    icon: Icons.people,
                                    title: 'Watchers',
                                    subtitle: 'Manage who can see you',
                                    color: Colors.purple,
                                    route: '/manage-watchers',
                                  ),
                                  _buildActionCard(
                                    context,
                                    icon: Icons.visibility,
                                    title: 'Watching',
                                    subtitle: 'Users who authorized me',
                                    color: Colors.teal,
                                    route: '/watching',
                                  ),
                                  _buildActionCard(
                                    context,
                                    icon: Icons.settings,
                                    title: 'Settings',
                                    subtitle: 'Edit profile',
                                    color: Colors.teal,
                                    route: '/settings',
                                  ),
                                ],
                              );
                            },
                          ),
                        ),
                      ],
                    ),
                  ),
          ),
        ),
      ),
    );
  }

  Widget _buildEmailVerificationRequired(BuildContext context) {
    return SingleChildScrollView(
      physics: const AlwaysScrollableScrollPhysics(),
      child: Center(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Card(
            elevation: 4,
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(16),
            ),
            child: Padding(
              padding: const EdgeInsets.all(32.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(
                    Icons.email_outlined,
                    size: 80,
                    color: AppConfig.blueColor,
                  ),
                  const SizedBox(height: 24),
                  const Text(
                    'Email Verification Required',
                    style: TextStyle(
                      fontSize: 24,
                      fontWeight: FontWeight.bold,
                      color: AppConfig.blueColor,
                    ),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 16),
                  Text(
                    'Please verify your email address before accessing the app features.',
                    style: TextStyle(fontSize: 16, color: Colors.grey[700]),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 24),
                  Text(
                    'Check your inbox for a verification email and click the link to verify your account.',
                    style: TextStyle(fontSize: 14, color: Colors.grey[600]),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 32),
                  const Divider(),
                  const SizedBox(height: 16),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.refresh, size: 20, color: AppConfig.blueColor),
                      const SizedBox(width: 8),
                      Text(
                        'Pull down to refresh after verifying',
                        style: TextStyle(
                          fontSize: 12,
                          color: AppConfig.blueColor,
                          fontWeight: FontWeight.w500,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildActionCard(
    BuildContext context, {
    required IconData icon,
    required String title,
    required String subtitle,
    required Color color,
    String? route,
  }) {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: InkWell(
        onTap: () {
          if (route != null) {
            Navigator.of(context).pushNamed(route);
          } else {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text('$title feature coming soon!'),
                duration: const Duration(seconds: 2),
              ),
            );
          }
        },
        borderRadius: BorderRadius.circular(16),
        child: Container(
          padding: const EdgeInsets.all(16.0),
          decoration: BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: [color.withOpacity(0.8), color],
            ),
            borderRadius: BorderRadius.circular(16),
          ),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(icon, size: 48, color: AppConfig.whiteColor),
              const SizedBox(height: 12),
              Text(
                title,
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: AppConfig.whiteColor,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                subtitle,
                style: TextStyle(
                  fontSize: 12,
                  color: AppConfig.whiteColor.withOpacity(0.9),
                ),
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildGuestAccountBanner(BuildContext context, User user) {
    // Calculate hours remaining until expiration (24 hours from creation)
    final createdAt = DateTime.tryParse(user.createdAt);
    if (createdAt == null) {
      // If we can't parse the date, show a generic message
      return Card(
        elevation: 6,
        color: AppConfig.goldColor.withOpacity(0.95),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: BorderSide(color: AppConfig.redColor, width: 2),
        ),
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Text(
            'Guest Account - Limited Access',
            style: TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: AppConfig.blueColor,
            ),
          ),
        ),
      );
    }

    final expiresAt = createdAt.add(const Duration(hours: 24));
    final now = DateTime.now();
    final hoursRemaining = expiresAt.difference(now).inHours;

    String expirationText;
    if (hoursRemaining <= 0) {
      expirationText = 'This account has expired';
    } else if (hoursRemaining < 1) {
      final minutesRemaining = expiresAt.difference(now).inMinutes;
      expirationText =
          'This guest account will expire in $minutesRemaining ${minutesRemaining == 1 ? 'minute' : 'minutes'}';
    } else {
      expirationText =
          'This guest account will expire in $hoursRemaining ${hoursRemaining == 1 ? 'hour' : 'hours'}';
    }

    return Card(
      elevation: 6,
      color: AppConfig.goldColor.withOpacity(0.95),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(color: AppConfig.redColor, width: 2),
      ),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(Icons.info_outline, color: AppConfig.redColor, size: 24),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    'Guest Account',
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                      color: AppConfig.blueColor,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Text(
              expirationText,
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.w600,
                color: AppConfig.blueColor,
              ),
            ),
            const SizedBox(height: 12),
            const Divider(color: AppConfig.blueColor, thickness: 1),
            const SizedBox(height: 12),
            Text(
              'If you log out, you can reconnect using:',
              style: TextStyle(
                fontSize: 14,
                color: AppConfig.blueColor,
                fontWeight: FontWeight.w500,
              ),
            ),
            const SizedBox(height: 8),
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: AppConfig.whiteColor.withOpacity(0.7),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(Icons.email, size: 16, color: AppConfig.blueColor),
                      const SizedBox(width: 8),
                      Text(
                        'Email: ',
                        style: TextStyle(
                          fontSize: 13,
                          fontWeight: FontWeight.bold,
                          color: AppConfig.blueColor,
                        ),
                      ),
                      Expanded(
                        child: Text(
                          user.email,
                          style: TextStyle(
                            fontSize: 13,
                            color: AppConfig.blueColor,
                          ),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Row(
                    children: [
                      Icon(Icons.lock, size: 16, color: AppConfig.blueColor),
                      const SizedBox(width: 8),
                      Text(
                        'Password: ',
                        style: TextStyle(
                          fontSize: 13,
                          fontWeight: FontWeight.bold,
                          color: AppConfig.blueColor,
                        ),
                      ),
                      Text(
                        'password',
                        style: TextStyle(
                          fontSize: 13,
                          color: AppConfig.blueColor,
                          fontFamily: 'monospace',
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
