import 'dart:async';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/watch_permission.dart';
import '../services/api_service.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';

class ManageWatchersScreen extends StatefulWidget {
  const ManageWatchersScreen({super.key});

  @override
  State<ManageWatchersScreen> createState() => _ManageWatchersScreenState();
}

class _ManageWatchersScreenState extends State<ManageWatchersScreen> {
  final ApiService _apiService = ApiService();
  final TextEditingController _searchController = TextEditingController();
  
  List<WatchPermissionUser> _watchers = [];
  List<UserSearchResult> _searchResults = [];
  bool _isLoading = true;
  bool _isSearching = false;
  String? _error;
  Timer? _debounceTimer;

  @override
  void initState() {
    super.initState();
    _loadWatchers();
  }

  @override
  void dispose() {
    _searchController.dispose();
    _debounceTimer?.cancel();
    super.dispose();
  }

  Future<void> _loadWatchers() async {
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
      final watchers = await _apiService.getWatchers(accessToken);
      setState(() {
        _watchers = watchers;
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

  void _onSearchChanged(String query) {
    _debounceTimer?.cancel();
    
    if (query.length < 3) {
      setState(() {
        _searchResults = [];
        _isSearching = false;
      });
      return;
    }

    _debounceTimer = Timer(const Duration(milliseconds: 300), () {
      _searchUsers(query);
    });
  }

  Future<void> _searchUsers(String query) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    setState(() {
      _isSearching = true;
    });

    try {
      final results = await _apiService.searchUsers(
        accessToken: accessToken,
        query: query,
      );
      
      // Filter out users who are already watchers
      final watcherIds = _watchers.map((w) => w.userId).toSet();
      final filteredResults = results.where((r) => !watcherIds.contains(r.id)).toList();
      
      setState(() {
        _searchResults = filteredResults;
        _isSearching = false;
      });
    } on ApiException catch (e) {
      setState(() {
        _isSearching = false;
      });
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(e.message),
            backgroundColor: AppConfig.redColor,
          ),
        );
      }
    } catch (e) {
      setState(() {
        _isSearching = false;
      });
    }
  }

  Future<void> _grantPermission(UserSearchResult user) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.grantWatchPermission(
        accessToken: accessToken,
        userId: user.id,
      );
      
      // Clear search and reload watchers
      _searchController.clear();
      setState(() {
        _searchResults = [];
      });
      await _loadWatchers();
      
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('${user.username} can now see your profile'),
            backgroundColor: Colors.green,
          ),
        );
      }
    } on ApiException catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(e.message),
            backgroundColor: AppConfig.redColor,
          ),
        );
      }
    }
  }

  Future<void> _revokePermission(WatchPermissionUser watcher) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Revoke Access'),
        content: Text("Are you sure you want to revoke ${watcher.username}'s access to your profile?"),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () => Navigator.pop(context, true),
            style: ElevatedButton.styleFrom(
              backgroundColor: AppConfig.redColor,
              foregroundColor: AppConfig.whiteColor,
            ),
            child: const Text('Revoke'),
          ),
        ],
      ),
    );

    if (confirmed != true) return;

    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.revokeWatchPermission(
        accessToken: accessToken,
        userId: watcher.userId,
      );
      
      await _loadWatchers();
      
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text("${watcher.username}'s access has been revoked"),
            backgroundColor: Colors.green,
          ),
        );
      }
    } on ApiException catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(e.message),
            backgroundColor: AppConfig.redColor,
          ),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Manage Watchers'),
        backgroundColor: AppConfig.blueColor,
        foregroundColor: AppConfig.goldColor,
      ),
      body: Container(
        decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary),
        child: SafeArea(
          child: RefreshIndicator(
            onRefresh: _loadWatchers,
            child: _isLoading
                ? const Center(
                    child: CircularProgressIndicator(
                      color: AppConfig.goldColor,
                    ),
                  )
                : _error != null
                    ? _buildErrorState()
                    : _buildContent(),
          ),
        ),
      ),
    );
  }

  Widget _buildErrorState() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(
            Icons.error_outline,
            size: 64,
            color: AppConfig.redColor,
          ),
          const SizedBox(height: 16),
          Text(
            _error!,
            style: const TextStyle(
              color: AppConfig.whiteColor,
              fontSize: 16,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: _loadWatchers,
            child: const Text('Retry'),
          ),
        ],
      ),
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
          Card(
            elevation: 4,
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Row(
                    children: [
                      Icon(Icons.info_outline, color: AppConfig.blueColor),
                      SizedBox(width: 8),
                      Text(
                        'About Watchers',
                        style: TextStyle(
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                          color: AppConfig.blueColor,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Text(
                    'Watchers are users you authorize to view your profile and data. Search for users below and add them as watchers.',
                    style: TextStyle(
                      fontSize: 14,
                      color: Colors.grey[700],
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),

          // Search Section
          const Text(
            'Add New Watcher',
            style: TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
              color: AppConfig.goldColor,
            ),
          ),
          const SizedBox(height: 8),
          Card(
            elevation: 4,
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                children: [
                  TextField(
                    controller: _searchController,
                    onChanged: _onSearchChanged,
                    decoration: InputDecoration(
                      labelText: 'Search users (min 3 characters)',
                      prefixIcon: const Icon(Icons.search),
                      suffixIcon: _isSearching
                          ? const SizedBox(
                              width: 20,
                              height: 20,
                              child: Padding(
                                padding: EdgeInsets.all(12.0),
                                child: CircularProgressIndicator(strokeWidth: 2),
                              ),
                            )
                          : _searchController.text.isNotEmpty
                              ? IconButton(
                                  icon: const Icon(Icons.clear),
                                  onPressed: () {
                                    _searchController.clear();
                                    setState(() {
                                      _searchResults = [];
                                    });
                                  },
                                )
                              : null,
                      border: const OutlineInputBorder(),
                    ),
                  ),
                  if (_searchResults.isNotEmpty) ...[
                    const SizedBox(height: 12),
                    const Divider(),
                    ..._searchResults.map((user) => ListTile(
                      leading: CircleAvatar(
                        backgroundColor: AppConfig.blueColor,
                        child: Text(
                          user.username.isNotEmpty ? user.username[0].toUpperCase() : '?',
                          style: const TextStyle(color: AppConfig.whiteColor),
                        ),
                      ),
                      title: Text(user.username),
                      trailing: IconButton(
                        icon: const Icon(Icons.person_add, color: Colors.green),
                        onPressed: () => _grantPermission(user),
                      ),
                    )),
                  ],
                  if (_searchController.text.length >= 3 && _searchResults.isEmpty && !_isSearching) ...[
                    const SizedBox(height: 12),
                    Text(
                      'No users found',
                      style: TextStyle(color: Colors.grey[600]),
                    ),
                  ],
                ],
              ),
            ),
          ),
          const SizedBox(height: 24),

          // Current Watchers Section
          const Text(
            'Current Watchers',
            style: TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
              color: AppConfig.goldColor,
            ),
          ),
          const SizedBox(height: 8),
          if (_watchers.isEmpty)
            _buildEmptyWatchersState()
          else
            ..._watchers.map((watcher) => _buildWatcherCard(watcher)),
        ],
      ),
    );
  }

  Widget _buildEmptyWatchersState() {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(32.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.people_outline,
              size: 64,
              color: Colors.grey[400],
            ),
            const SizedBox(height: 16),
            Text(
              'No watchers yet',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                color: Colors.grey[600],
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'Search for users above to add them as watchers',
              style: TextStyle(
                fontSize: 14,
                color: Colors.grey[500],
              ),
              textAlign: TextAlign.center,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildWatcherCard(WatchPermissionUser watcher) {
    return Card(
      elevation: 2,
      margin: const EdgeInsets.only(bottom: 8),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: ListTile(
        leading: CircleAvatar(
          backgroundColor: AppConfig.blueColor,
          child: Text(
            watcher.username.isNotEmpty ? watcher.username[0].toUpperCase() : '?',
            style: const TextStyle(
              color: AppConfig.goldColor,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
        title: Text(
          watcher.username,
          style: const TextStyle(
            fontWeight: FontWeight.bold,
            fontSize: 16,
          ),
        ),
        subtitle: const Text('Can view your profile'),
        trailing: IconButton(
          icon: const Icon(Icons.remove_circle, color: AppConfig.redColor),
          onPressed: () => _revokePermission(watcher),
          tooltip: 'Revoke access',
        ),
      ),
    );
  }
}
