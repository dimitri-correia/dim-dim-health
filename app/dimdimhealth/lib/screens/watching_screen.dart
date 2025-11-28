import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/watch_permission.dart';
import '../services/api_service.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';
import '../widgets/widgets.dart';

class WatchingScreen extends StatefulWidget {
  const WatchingScreen({super.key});

  @override
  State<WatchingScreen> createState() => _WatchingScreenState();
}

class _WatchingScreenState extends State<WatchingScreen> {
  final ApiService _apiService = ApiService();
  
  List<WatchPermissionUser> _watching = [];
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadWatching();
  }

  Future<void> _loadWatching() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) {
      setState(() {
        _error = 'Not authenticated';
        _isLoading = false;
      });
      return;
    }

    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final watching = await _apiService.getWatching(accessToken);
      setState(() {
        _watching = watching;
        _isLoading = false;
      });
    } on ApiException catch (e) {
      setState(() {
        _error = e.message;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _error = 'Failed to load data. Please try again.';
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return AppScreenWrapper(
      appBar: const AppStandardAppBar(title: 'Users I Can Watch'),
      onRefresh: _loadWatching,
      child: _isLoading
          ? const DataLoadingView()
          : _error != null
              ? DataErrorView(error: _error!, onRetry: _loadWatching)
              : _buildContent(),
    );
  }

  Widget _buildContent() {
    return SingleChildScrollView(
      physics: const AlwaysScrollableScrollPhysics(),
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Description
          const InfoCard(
            icon: Icons.info_outline,
            title: 'About Watching',
            description: 'These are users who have authorized you to view their profile and data. You can access their information because they granted you permission.',
          ),
          const SizedBox(height: 24),

          // Users I Can Watch Section
          const SectionTitle(title: 'Users Who Authorized Me'),
          const SizedBox(height: 8),
          if (_watching.isEmpty)
            const EmptyStateView(
              icon: Icons.visibility_off,
              title: 'No authorized users yet',
              message: 'When other users authorize you to view their profile, they will appear here',
            )
          else
            ..._watching.map((user) => _buildUserCard(user)),
        ],
      ),
    );
  }

  Widget _buildUserCard(WatchPermissionUser user) {
    return Card(
      elevation: 2,
      margin: const EdgeInsets.only(bottom: 8),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: ListTile(
        leading: CircleAvatar(
          backgroundColor: AppConfig.blueColor,
          child: Text(
            user.username.isNotEmpty ? user.username[0].toUpperCase() : '?',
            style: const TextStyle(
              color: AppConfig.goldColor,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
        title: Text(
          user.username,
          style: const TextStyle(
            fontWeight: FontWeight.bold,
            fontSize: 16,
          ),
        ),
        subtitle: const Text('Has authorized you to view their profile'),
        trailing: Icon(
          Icons.visibility,
          color: Colors.green[600],
        ),
      ),
    );
  }
}
