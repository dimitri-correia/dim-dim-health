import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/weight.dart';
import '../services/api_service.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';

class WeightScreen extends StatefulWidget {
  const WeightScreen({super.key});

  @override
  State<WeightScreen> createState() => _WeightScreenState();
}

class _WeightScreenState extends State<WeightScreen> {
  final ApiService _apiService = ApiService();
  List<UserWeight> _weights = [];
  UserWeightInfos? _weightInfos;
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadData();
  }

  Future<void> _loadData() async {
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
      final weights = await _apiService.getWeights(accessToken);
      final weightInfos = await _apiService.getWeightInfos(accessToken);

      setState(() {
        _weights = weights;
        _weightInfos = weightInfos;
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

  Future<void> _showAddWeightDialog() async {
    final weightController = TextEditingController();
    DateTime selectedDate = DateTime.now();

    await showDialog(
      context: context,
      builder: (context) => StatefulBuilder(
        builder: (context, setDialogState) => AlertDialog(
          title: const Text('Add Weight Entry'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextField(
                controller: weightController,
                keyboardType:
                    const TextInputType.numberWithOptions(decimal: true),
                decoration: const InputDecoration(
                  labelText: 'Weight (kg)',
                  border: OutlineInputBorder(),
                ),
              ),
              const SizedBox(height: 16),
              ListTile(
                contentPadding: EdgeInsets.zero,
                title: const Text('Date'),
                subtitle: Text(
                  '${selectedDate.day}/${selectedDate.month}/${selectedDate.year}',
                ),
                trailing: const Icon(Icons.calendar_today),
                onTap: () async {
                  final date = await showDatePicker(
                    context: context,
                    initialDate: selectedDate,
                    firstDate: DateTime(2000),
                    lastDate: DateTime.now(),
                  );
                  if (date != null) {
                    setDialogState(() {
                      selectedDate = date;
                    });
                  }
                },
              ),
            ],
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Cancel'),
            ),
            ElevatedButton(
              onPressed: () async {
                final weight = double.tryParse(weightController.text);
                if (weight == null || weight <= 0) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(
                      content: Text('Please enter a valid weight'),
                      backgroundColor: AppConfig.redColor,
                    ),
                  );
                  return;
                }

                Navigator.pop(context);
                await _createWeight(weight, selectedDate);
              },
              style: ElevatedButton.styleFrom(
                backgroundColor: AppConfig.blueColor,
                foregroundColor: AppConfig.whiteColor,
              ),
              child: const Text('Add'),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _createWeight(double weightInKg, DateTime recordedAt) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.createWeight(
        accessToken: accessToken,
        weightInKg: weightInKg,
        recordedAt: recordedAt,
      );
      await _loadData();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Weight entry added successfully'),
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

  Future<void> _showEditWeightDialog(UserWeight weight) async {
    final weightController =
        TextEditingController(text: weight.weightInKg.toString());
    DateTime selectedDate = DateTime.parse(weight.recordedAt);

    await showDialog(
      context: context,
      builder: (context) => StatefulBuilder(
        builder: (context, setDialogState) => AlertDialog(
          title: const Text('Edit Weight Entry'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextField(
                controller: weightController,
                keyboardType:
                    const TextInputType.numberWithOptions(decimal: true),
                decoration: const InputDecoration(
                  labelText: 'Weight (kg)',
                  border: OutlineInputBorder(),
                ),
              ),
              const SizedBox(height: 16),
              ListTile(
                contentPadding: EdgeInsets.zero,
                title: const Text('Date'),
                subtitle: Text(
                  '${selectedDate.day}/${selectedDate.month}/${selectedDate.year}',
                ),
                trailing: const Icon(Icons.calendar_today),
                onTap: () async {
                  final date = await showDatePicker(
                    context: context,
                    initialDate: selectedDate,
                    firstDate: DateTime(2000),
                    lastDate: DateTime.now(),
                  );
                  if (date != null) {
                    setDialogState(() {
                      selectedDate = date;
                    });
                  }
                },
              ),
            ],
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Cancel'),
            ),
            ElevatedButton(
              onPressed: () async {
                final newWeight = double.tryParse(weightController.text);
                if (newWeight == null || newWeight <= 0) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(
                      content: Text('Please enter a valid weight'),
                      backgroundColor: AppConfig.redColor,
                    ),
                  );
                  return;
                }

                Navigator.pop(context);
                await _updateWeight(weight.id, newWeight, selectedDate);
              },
              style: ElevatedButton.styleFrom(
                backgroundColor: AppConfig.blueColor,
                foregroundColor: AppConfig.whiteColor,
              ),
              child: const Text('Update'),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _updateWeight(
      String id, double weightInKg, DateTime recordedAt) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.updateWeight(
        accessToken: accessToken,
        id: id,
        weightInKg: weightInKg,
        recordedAt: recordedAt,
      );
      await _loadData();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Weight entry updated successfully'),
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

  Future<void> _confirmDeleteWeight(UserWeight weight) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Weight Entry'),
        content: const Text('Are you sure you want to delete this entry?'),
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
            child: const Text('Delete'),
          ),
        ],
      ),
    );

    if (confirmed == true) {
      await _deleteWeight(weight.id);
    }
  }

  Future<void> _deleteWeight(String id) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.deleteWeight(
        accessToken: accessToken,
        id: id,
      );
      await _loadData();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Weight entry deleted successfully'),
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

  String _formatDate(String dateString) {
    try {
      final date = DateTime.parse(dateString);
      return '${date.day}/${date.month}/${date.year}';
    } catch (e) {
      return dateString;
    }
  }

  String _formatWeight(double weight) {
    return '${weight.toStringAsFixed(1)} kg';
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Weight Tracker'),
        backgroundColor: AppConfig.blueColor,
        foregroundColor: AppConfig.goldColor,
      ),
      body: Container(
        decoration: BoxDecoration(color: Theme.of(context).colorScheme.primary),
        child: SafeArea(
          child: RefreshIndicator(
            onRefresh: _loadData,
            child: _isLoading
                ? const Center(
                    child: CircularProgressIndicator(
                      color: AppConfig.goldColor,
                    ),
                  )
                : _error != null
                    ? Center(
                        child: Column(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            Icon(
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
                              onPressed: _loadData,
                              child: const Text('Retry'),
                            ),
                          ],
                        ),
                      )
                    : SingleChildScrollView(
                        physics: const AlwaysScrollableScrollPhysics(),
                        padding: const EdgeInsets.all(16.0),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            // Statistics Card
                            if (_weightInfos != null) _buildStatisticsCard(),
                            const SizedBox(height: 16),

                            // Weight History
                            const Text(
                              'Weight History',
                              style: TextStyle(
                                fontSize: 20,
                                fontWeight: FontWeight.bold,
                                color: AppConfig.goldColor,
                              ),
                            ),
                            const SizedBox(height: 8),
                            if (_weights.isEmpty)
                              _buildEmptyState()
                            else
                              ..._buildWeightList(),
                          ],
                        ),
                      ),
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _showAddWeightDialog,
        backgroundColor: AppConfig.goldColor,
        foregroundColor: AppConfig.blueColor,
        child: const Icon(Icons.add),
      ),
    );
  }

  Widget _buildStatisticsCard() {
    final infos = _weightInfos!;
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Statistics',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                color: AppConfig.blueColor,
              ),
            ),
            const Divider(),
            const SizedBox(height: 8),
            _buildStatRow(
              'Total Entries',
              '${infos.numberOfWeightEntries}',
              Icons.format_list_numbered,
            ),
            _buildStatRow(
              'Average Weight',
              _formatWeight(infos.averageWeight),
              Icons.analytics,
            ),
            _buildStatRow(
              'Avg (Last 7 Days)',
              _formatWeight(infos.averageWeightLast7Days),
              Icons.calendar_view_week,
            ),
            _buildStatRow(
              'Avg (Last 30 Days)',
              _formatWeight(infos.averageWeightLast30Days),
              Icons.calendar_month,
            ),
            const Divider(),
            _buildStatRow(
              'Max Weight',
              '${_formatWeight(infos.maxWeight)} (${_formatDate(infos.maxWeightDate)})',
              Icons.arrow_upward,
              color: AppConfig.redColor,
            ),
            _buildStatRow(
              'Min Weight',
              '${_formatWeight(infos.minWeight)} (${_formatDate(infos.minWeightDate)})',
              Icons.arrow_downward,
              color: Colors.green,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildStatRow(String label, String value, IconData icon,
      {Color? color}) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4.0),
      child: Row(
        children: [
          Icon(icon, size: 20, color: color ?? AppConfig.blueColor),
          const SizedBox(width: 8),
          Expanded(
            child: Text(
              label,
              style: const TextStyle(
                fontSize: 14,
                color: Colors.grey,
              ),
            ),
          ),
          Text(
            value,
            style: TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              color: color ?? AppConfig.blueColor,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildEmptyState() {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Padding(
        padding: const EdgeInsets.all(32.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.monitor_weight_outlined,
              size: 64,
              color: Colors.grey[400],
            ),
            const SizedBox(height: 16),
            Text(
              'No weight entries yet',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                color: Colors.grey[600],
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'Tap the + button to add your first weight entry',
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

  List<Widget> _buildWeightList() {
    // Sort weights by recorded_at descending
    final sortedWeights = List<UserWeight>.from(_weights)
      ..sort((a, b) => b.recordedAt.compareTo(a.recordedAt));

    return sortedWeights.map((weight) {
      return Card(
        elevation: 2,
        margin: const EdgeInsets.only(bottom: 8),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        child: ListTile(
          leading: CircleAvatar(
            backgroundColor: AppConfig.blueColor,
            child: const Icon(
              Icons.monitor_weight,
              color: AppConfig.goldColor,
            ),
          ),
          title: Text(
            _formatWeight(weight.weightInKg),
            style: const TextStyle(
              fontWeight: FontWeight.bold,
              fontSize: 16,
            ),
          ),
          subtitle: Text(
            _formatDate(weight.recordedAt),
            style: TextStyle(
              color: Colors.grey[600],
            ),
          ),
          trailing: PopupMenuButton<String>(
            onSelected: (value) {
              if (value == 'edit') {
                _showEditWeightDialog(weight);
              } else if (value == 'delete') {
                _confirmDeleteWeight(weight);
              }
            },
            itemBuilder: (context) => [
              const PopupMenuItem(
                value: 'edit',
                child: Row(
                  children: [
                    Icon(Icons.edit, color: AppConfig.blueColor),
                    SizedBox(width: 8),
                    Text('Edit'),
                  ],
                ),
              ),
              const PopupMenuItem(
                value: 'delete',
                child: Row(
                  children: [
                    Icon(Icons.delete, color: AppConfig.redColor),
                    SizedBox(width: 8),
                    Text('Delete'),
                  ],
                ),
              ),
            ],
          ),
        ),
      );
    }).toList();
  }
}
