import 'dart:math' as math;
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:fl_chart/fl_chart.dart';
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

  String _formatShortDate(String dateString) {
    try {
      final date = DateTime.parse(dateString);
      return '${date.day}/${date.month}';
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
        width: double.infinity,
        height: double.infinity,
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
                              onPressed: _loadData,
                              child: const Text('Retry'),
                            ),
                          ],
                        ),
                      )
                    : SingleChildScrollView(
                        physics: const AlwaysScrollableScrollPhysics(),
                        child: Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 12.0, vertical: 16.0),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.stretch,
                            children: [
                              // Current Weight Summary
                              if (_weightInfos != null) _buildCurrentWeightCard(),
                              const SizedBox(height: 16),

                              // Weight Trend Graph
                              if (_weights.isNotEmpty) _buildWeightChartCard(),
                              if (_weights.isNotEmpty) const SizedBox(height: 16),

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
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _showAddWeightDialog,
        backgroundColor: AppConfig.goldColor,
        foregroundColor: AppConfig.blueColor,
        child: const Icon(Icons.add),
      ),
    );
  }

  Widget _buildCurrentWeightCard() {
    final infos = _weightInfos!;
    final latestWeight = infos.last3Weights.isNotEmpty
        ? infos.last3Weights.first
        : null;

    // Calculate weight change (current vs 7-day average)
    double? weightChange;
    if (latestWeight != null && infos.averageWeightLast7Days > 0) {
      weightChange = latestWeight.weightInKg - infos.averageWeightLast7Days;
    }

    return Card(
      elevation: 6,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
      child: Container(
        width: double.infinity,
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              AppConfig.blueColor,
              AppConfig.blueColor.withAlpha(204),
            ],
          ),
          borderRadius: BorderRadius.circular(20),
        ),
        padding: const EdgeInsets.all(20.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            const Text(
              'Current Weight',
              style: TextStyle(
                fontSize: 16,
                color: AppConfig.whiteColor,
                fontWeight: FontWeight.w500,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              latestWeight != null
                  ? _formatWeight(latestWeight.weightInKg)
                  : '--',
              style: const TextStyle(
                fontSize: 42,
                fontWeight: FontWeight.bold,
                color: AppConfig.goldColor,
              ),
            ),
            if (latestWeight != null) ...[
              const SizedBox(height: 4),
              Text(
                _formatDate(latestWeight.recordedAt),
                style: TextStyle(
                  fontSize: 14,
                  color: AppConfig.whiteColor.withAlpha(179),
                ),
              ),
            ],
            if (weightChange != null) ...[
              const SizedBox(height: 12),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                decoration: BoxDecoration(
                  color: weightChange <= 0 ? Colors.green.withAlpha(51) : AppConfig.redColor.withAlpha(51),
                  borderRadius: BorderRadius.circular(20),
                ),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Icon(
                      weightChange <= 0 ? Icons.trending_down : Icons.trending_up,
                      color: weightChange <= 0 ? Colors.green : AppConfig.redColor,
                      size: 18,
                    ),
                    const SizedBox(width: 4),
                    Text(
                      '${weightChange >= 0 ? '+' : ''}${weightChange.toStringAsFixed(1)} kg vs 7-day avg',
                      style: TextStyle(
                        fontSize: 13,
                        fontWeight: FontWeight.w600,
                        color: weightChange <= 0 ? Colors.green : AppConfig.redColor,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildWeightChartCard() {
    // Sort weights by date ascending for the chart
    final sortedWeights = List<UserWeight>.from(_weights)
      ..sort((a, b) => a.recordedAt.compareTo(b.recordedAt));

    // Take last 30 entries for the chart
    final chartWeights = sortedWeights.length > 30
        ? sortedWeights.sublist(sortedWeights.length - 30)
        : sortedWeights;

    if (chartWeights.isEmpty) return const SizedBox.shrink();

    // Create data points
    final spots = <FlSpot>[];
    for (int i = 0; i < chartWeights.length; i++) {
      spots.add(FlSpot(i.toDouble(), chartWeights[i].weightInKg));
    }

    // Calculate min/max for better chart scaling
    final minWeight = chartWeights.map((w) => w.weightInKg).reduce(math.min);
    final maxWeight = chartWeights.map((w) => w.weightInKg).reduce(math.max);
    final padding = (maxWeight - minWeight) * 0.1;
    final chartMinY = (minWeight - padding).clamp(0, double.infinity);
    final chartMaxY = maxWeight + padding;

    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Container(
        width: double.infinity,
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                const Icon(Icons.show_chart, color: AppConfig.blueColor),
                const SizedBox(width: 8),
                const Expanded(
                  child: Text(
                    'Weight Trend',
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                      color: AppConfig.blueColor,
                    ),
                  ),
                ),
                Text(
                  'Last ${chartWeights.length} entries',
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.grey[600],
                  ),
                ),
              ],
            ),
            const SizedBox(height: 20),
            SizedBox(
              height: 200,
              width: double.infinity,
              child: LineChart(
                LineChartData(
                  gridData: FlGridData(
                    show: true,
                    drawVerticalLine: false,
                    horizontalInterval: (chartMaxY - chartMinY) / 4,
                    getDrawingHorizontalLine: (value) {
                      return FlLine(
                        color: Colors.grey.withAlpha(51),
                        strokeWidth: 1,
                      );
                    },
                  ),
                  titlesData: FlTitlesData(
                    show: true,
                    rightTitles: const AxisTitles(
                      sideTitles: SideTitles(showTitles: false),
                    ),
                    topTitles: const AxisTitles(
                      sideTitles: SideTitles(showTitles: false),
                    ),
                    bottomTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        reservedSize: 30,
                        interval: chartWeights.length > 10 
                            ? (chartWeights.length / 5).ceilToDouble() 
                            : 1,
                        getTitlesWidget: (value, meta) {
                          final index = value.toInt();
                          if (index >= 0 && index < chartWeights.length) {
                            return Padding(
                              padding: const EdgeInsets.only(top: 8.0),
                              child: Text(
                                _formatShortDate(chartWeights[index].recordedAt),
                                style: const TextStyle(
                                  color: Colors.grey,
                                  fontSize: 10,
                                ),
                              ),
                            );
                          }
                          return const Text('');
                        },
                      ),
                    ),
                    leftTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        interval: (chartMaxY - chartMinY) / 4,
                        reservedSize: 45,
                        getTitlesWidget: (value, meta) {
                          return Text(
                            '${value.toStringAsFixed(1)}',
                            style: const TextStyle(
                              color: Colors.grey,
                              fontSize: 11,
                            ),
                          );
                        },
                      ),
                    ),
                  ),
                  borderData: FlBorderData(show: false),
                  minX: 0,
                  maxX: (chartWeights.length - 1).toDouble(),
                  minY: chartMinY,
                  maxY: chartMaxY,
                  lineBarsData: [
                    LineChartBarData(
                      spots: spots,
                      isCurved: true,
                      curveSmoothness: 0.3,
                      color: AppConfig.blueColor,
                      barWidth: 3,
                      isStrokeCapRound: true,
                      dotData: FlDotData(
                        show: chartWeights.length <= 15,
                        getDotPainter: (spot, percent, barData, index) {
                          return FlDotCirclePainter(
                            radius: 4,
                            color: AppConfig.goldColor,
                            strokeWidth: 2,
                            strokeColor: AppConfig.blueColor,
                          );
                        },
                      ),
                      belowBarData: BarAreaData(
                        show: true,
                        gradient: LinearGradient(
                          begin: Alignment.topCenter,
                          end: Alignment.bottomCenter,
                          colors: [
                            AppConfig.blueColor.withAlpha(77),
                            AppConfig.blueColor.withAlpha(13),
                          ],
                        ),
                      ),
                    ),
                  ],
                  lineTouchData: LineTouchData(
                    enabled: true,
                    touchTooltipData: LineTouchTooltipData(
                      getTooltipColor: (touchedSpot) => AppConfig.blueColor.withAlpha(230),
                      tooltipRoundedRadius: 8,
                      getTooltipItems: (touchedSpots) {
                        return touchedSpots.map((spot) {
                          final index = spot.x.toInt();
                          if (index >= 0 && index < chartWeights.length) {
                            return LineTooltipItem(
                              '${chartWeights[index].weightInKg.toStringAsFixed(1)} kg\n${_formatDate(chartWeights[index].recordedAt)}',
                              const TextStyle(
                                color: AppConfig.goldColor,
                                fontWeight: FontWeight.bold,
                                fontSize: 12,
                              ),
                            );
                          }
                          return null;
                        }).toList();
                      },
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildStatisticsCard() {
    final infos = _weightInfos!;
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Container(
        width: double.infinity,
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Row(
              children: [
                Icon(Icons.analytics, color: AppConfig.blueColor),
                SizedBox(width: 8),
                Text(
                  'Statistics',
                  style: TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                    color: AppConfig.blueColor,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            
            // Stats Grid
            Row(
              children: [
                Expanded(
                  child: _buildStatTile(
                    'Total Entries',
                    '${infos.numberOfWeightEntries}',
                    Icons.format_list_numbered,
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: _buildStatTile(
                    'Average',
                    _formatWeight(infos.averageWeight),
                    Icons.balance,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                Expanded(
                  child: _buildStatTile(
                    '7-Day Avg',
                    _formatWeight(infos.averageWeightLast7Days),
                    Icons.calendar_view_week,
                    subtitle: '${infos.numberOfWeightEntriesLast7Days} entries',
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: _buildStatTile(
                    '30-Day Avg',
                    _formatWeight(infos.averageWeightLast30Days),
                    Icons.calendar_month,
                    subtitle: '${infos.numberOfWeightEntriesLast30Days} entries',
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 16),
            const Divider(),
            const SizedBox(height: 12),
            
            // Min/Max Row
            Row(
              children: [
                Expanded(
                  child: _buildMinMaxTile(
                    'Highest',
                    _formatWeight(infos.maxWeight),
                    _formatDate(infos.maxWeightDate),
                    Icons.arrow_upward,
                    AppConfig.redColor,
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: _buildMinMaxTile(
                    'Lowest',
                    _formatWeight(infos.minWeight),
                    _formatDate(infos.minWeightDate),
                    Icons.arrow_downward,
                    Colors.green,
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildStatTile(String label, String value, IconData icon, {String? subtitle}) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: AppConfig.blueColor.withAlpha(13),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(icon, size: 16, color: AppConfig.blueColor),
              const SizedBox(width: 6),
              Expanded(
                child: Text(
                  label,
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.grey[600],
                  ),
                  overflow: TextOverflow.ellipsis,
                ),
              ),
            ],
          ),
          const SizedBox(height: 6),
          Text(
            value,
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: AppConfig.blueColor,
            ),
          ),
          if (subtitle != null) ...[
            const SizedBox(height: 2),
            Text(
              subtitle,
              style: TextStyle(
                fontSize: 10,
                color: Colors.grey[500],
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildMinMaxTile(String label, String value, String date, IconData icon, Color color) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: color.withAlpha(26),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color.withAlpha(51)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(icon, size: 18, color: color),
              const SizedBox(width: 6),
              Text(
                label,
                style: TextStyle(
                  fontSize: 12,
                  fontWeight: FontWeight.w600,
                  color: color,
                ),
              ),
            ],
          ),
          const SizedBox(height: 6),
          Text(
            value,
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: color,
            ),
          ),
          const SizedBox(height: 2),
          Text(
            date,
            style: TextStyle(
              fontSize: 11,
              color: Colors.grey[600],
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
      child: Container(
        width: double.infinity,
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
          contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
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
