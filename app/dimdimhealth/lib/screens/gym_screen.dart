import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/gym.dart';
import '../services/api_service.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';
import '../widgets/widgets.dart';

class GymScreen extends StatefulWidget {
  const GymScreen({super.key});

  @override
  State<GymScreen> createState() => _GymScreenState();
}

class _GymScreenState extends State<GymScreen> {
  final ApiService _apiService = ApiService();
  List<GymSession> _sessions = [];
  List<GymExercise> _exercises = [];
  DateTime _selectedDate = DateTime.now();
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
      final sessions = await _apiService.getGymSessions(
        accessToken,
        date: _selectedDate,
      );
      final exercises = await _apiService.getGymExercises(accessToken);

      setState(() {
        _sessions = sessions;
        _exercises = exercises;
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

  Future<void> _selectDate() async {
    final date = await showDatePicker(
      context: context,
      initialDate: _selectedDate,
      firstDate: DateTime(2000),
      lastDate: DateTime.now().add(const Duration(days: 365)),
    );
    if (date != null) {
      setState(() {
        _selectedDate = date;
      });
      await _loadData();
    }
  }

  Future<void> _showAddSessionDialog() async {
    await showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Start New Session'),
        content: Text(
          'Start a new gym session for ${_formatDate(_selectedDate)}?',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () async {
              Navigator.pop(context);
              await _createSession();
            },
            style: ElevatedButton.styleFrom(
              backgroundColor: AppConfig.blueColor,
              foregroundColor: AppConfig.whiteColor,
            ),
            child: const Text('Start'),
          ),
        ],
      ),
    );
  }

  Future<void> _createSession() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.createGymSession(
        accessToken: accessToken,
        date: _selectedDate,
      );
      await _loadData();
      if (mounted) {
        AppSnackBar.showSuccess(context, 'Session started successfully');
      }
    } on ApiException catch (e) {
      if (mounted) {
        AppSnackBar.showError(context, e.message);
      }
    }
  }

  Future<void> _confirmDeleteSession(GymSession session) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Session'),
        content: const Text(
          'Are you sure you want to delete this session and all its sets?',
        ),
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
      await _deleteSession(session.id);
    }
  }

  Future<void> _deleteSession(String id) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.deleteGymSession(accessToken: accessToken, id: id);
      await _loadData();
      if (mounted) {
        AppSnackBar.showSuccess(context, 'Session deleted successfully');
      }
    } on ApiException catch (e) {
      if (mounted) {
        AppSnackBar.showError(context, e.message);
      }
    }
  }

  Future<void> _showAddExerciseDialog() async {
    final nameController = TextEditingController();
    final descriptionController = TextEditingController();
    List<Muscle> selectedPrimaryMuscles = [];
    List<Muscle> selectedSecondaryMuscles = [];

    await showDialog(
      context: context,
      builder: (context) => StatefulBuilder(
        builder: (context, setDialogState) => AlertDialog(
          title: const Text('Add Exercise'),
          content: SingleChildScrollView(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                TextField(
                  controller: nameController,
                  decoration: const InputDecoration(
                    labelText: 'Exercise Name *',
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 16),
                TextField(
                  controller: descriptionController,
                  decoration: const InputDecoration(
                    labelText: 'Description (optional)',
                    border: OutlineInputBorder(),
                  ),
                  maxLines: 2,
                ),
                const SizedBox(height: 16),
                const Text(
                  'Primary Muscles',
                  style: TextStyle(fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 8),
                Wrap(
                  spacing: 4,
                  runSpacing: 4,
                  children: Muscle.values.map((muscle) {
                    final isSelected = selectedPrimaryMuscles.contains(muscle);
                    return FilterChip(
                      label: Text(muscle.displayName),
                      selected: isSelected,
                      onSelected: (selected) {
                        setDialogState(() {
                          if (selected) {
                            selectedPrimaryMuscles.add(muscle);
                            selectedSecondaryMuscles.remove(muscle);
                          } else {
                            selectedPrimaryMuscles.remove(muscle);
                          }
                        });
                      },
                      selectedColor: AppConfig.blueColor.withOpacity(0.3),
                    );
                  }).toList(),
                ),
                const SizedBox(height: 16),
                const Text(
                  'Secondary Muscles',
                  style: TextStyle(fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 8),
                Wrap(
                  spacing: 4,
                  runSpacing: 4,
                  children: Muscle.values.map((muscle) {
                    final isSelected =
                        selectedSecondaryMuscles.contains(muscle);
                    final isPrimary = selectedPrimaryMuscles.contains(muscle);
                    return FilterChip(
                      label: Text(muscle.displayName),
                      selected: isSelected,
                      onSelected: isPrimary
                          ? null
                          : (selected) {
                              setDialogState(() {
                                if (selected) {
                                  selectedSecondaryMuscles.add(muscle);
                                } else {
                                  selectedSecondaryMuscles.remove(muscle);
                                }
                              });
                            },
                      selectedColor: AppConfig.goldColor.withOpacity(0.3),
                    );
                  }).toList(),
                ),
              ],
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Cancel'),
            ),
            ElevatedButton(
              onPressed: () async {
                if (nameController.text.isEmpty) {
                  AppSnackBar.showError(
                    context,
                    'Please enter an exercise name',
                  );
                  return;
                }
                if (selectedPrimaryMuscles.isEmpty) {
                  AppSnackBar.showError(
                    context,
                    'Please select at least one primary muscle',
                  );
                  return;
                }
                Navigator.pop(context);
                await _createExercise(
                  nameController.text,
                  descriptionController.text.isEmpty
                      ? null
                      : descriptionController.text,
                  selectedPrimaryMuscles,
                  selectedSecondaryMuscles,
                );
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

  Future<void> _createExercise(
    String name,
    String? description,
    List<Muscle> primaryMuscles,
    List<Muscle> secondaryMuscles,
  ) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.createGymExercise(
        accessToken: accessToken,
        name: name,
        description: description,
        primaryMuscles: primaryMuscles.map((m) => m.value).toList(),
        secondaryMuscles: secondaryMuscles.map((m) => m.value).toList(),
      );
      await _loadData();
      if (mounted) {
        AppSnackBar.showSuccess(context, 'Exercise added successfully');
      }
    } on ApiException catch (e) {
      if (mounted) {
        AppSnackBar.showError(context, e.message);
      }
    }
  }

  String _formatDate(DateTime date) {
    return '${date.day}/${date.month}/${date.year}';
  }

  @override
  Widget build(BuildContext context) {
    final authProvider = Provider.of<AuthProvider>(context);
    final user = authProvider.user;

    return AppScreenWrapper(
      appBar: AppStandardAppBar(
        title: 'Gym Tracker',
        actions: [
          IconButton(
            icon: const Icon(Icons.fitness_center),
            onPressed: _showAddExerciseDialog,
            tooltip: 'Add Exercise',
          ),
          Padding(
            padding: const EdgeInsets.only(right: 12.0),
            child: UserAvatar(profileImage: user?.profileImage),
          ),
        ],
      ),
      onRefresh: _loadData,
      floatingActionButton: FloatingActionButton(
        onPressed: _showAddSessionDialog,
        backgroundColor: AppConfig.goldColor,
        foregroundColor: AppConfig.blueColor,
        child: const Icon(Icons.add),
      ),
      child: _isLoading
          ? const DataLoadingView()
          : _error != null
              ? DataErrorView(error: _error!, onRetry: _loadData)
              : SingleChildScrollView(
                  physics: const AlwaysScrollableScrollPhysics(),
                  child: Padding(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 12.0,
                      vertical: 16.0,
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        // Date Selector
                        _buildDateSelector(),
                        const SizedBox(height: 16),

                        // Daily Summary
                        _buildDailySummary(),
                        const SizedBox(height: 16),

                        // Sessions List
                        const SectionTitle(title: 'Sessions'),
                        const SizedBox(height: 8),
                        if (_sessions.isEmpty)
                          const EmptyStateView(
                            icon: Icons.fitness_center,
                            title: 'No sessions yet',
                            message:
                                'Tap the + button to start a new gym session',
                          )
                        else
                          ..._buildSessionsList(),

                        const SizedBox(height: 24),

                        // Exercises List
                        const SectionTitle(title: 'Available Exercises'),
                        const SizedBox(height: 8),
                        if (_exercises.isEmpty)
                          const EmptyStateView(
                            icon: Icons.sports_gymnastics,
                            title: 'No exercises yet',
                            message: 'Tap the dumbbell icon to add exercises',
                          )
                        else
                          _buildExercisesList(),
                      ],
                    ),
                  ),
                ),
    );
  }

  Widget _buildDateSelector() {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: InkWell(
        onTap: _selectDate,
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              IconButton(
                icon: const Icon(Icons.chevron_left),
                onPressed: () {
                  setState(() {
                    _selectedDate = _selectedDate.subtract(
                      const Duration(days: 1),
                    );
                  });
                  _loadData();
                },
              ),
              const SizedBox(width: 8),
              const Icon(Icons.calendar_today, color: AppConfig.blueColor),
              const SizedBox(width: 8),
              Text(
                _formatDate(_selectedDate),
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: AppConfig.blueColor,
                ),
              ),
              const SizedBox(width: 8),
              IconButton(
                icon: const Icon(Icons.chevron_right),
                onPressed: () {
                  setState(() {
                    _selectedDate = _selectedDate.add(const Duration(days: 1));
                  });
                  _loadData();
                },
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildDailySummary() {
    return Card(
      elevation: 6,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
      child: Container(
        width: double.infinity,
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [AppConfig.blueColor, AppConfig.blueColor.withOpacity(0.8)],
          ),
          borderRadius: BorderRadius.circular(20),
        ),
        padding: const EdgeInsets.all(20.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Text(
              'Daily Summary - ${_formatDate(_selectedDate)}',
              style: const TextStyle(
                fontSize: 16,
                color: AppConfig.whiteColor,
                fontWeight: FontWeight.w500,
              ),
            ),
            const SizedBox(height: 12),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceAround,
              children: [
                _buildSummaryItem(
                  '${_sessions.length}',
                  'Sessions',
                  Icons.timer,
                ),
                _buildSummaryItem(
                  '${_exercises.length}',
                  'Available',
                  Icons.fitness_center,
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSummaryItem(String value, String label, IconData icon) {
    return Column(
      children: [
        Icon(icon, color: AppConfig.goldColor, size: 24),
        const SizedBox(height: 4),
        Text(
          value,
          style: const TextStyle(
            fontSize: 20,
            fontWeight: FontWeight.bold,
            color: AppConfig.goldColor,
          ),
        ),
        Text(
          label,
          style: TextStyle(
            fontSize: 12,
            color: AppConfig.whiteColor.withOpacity(0.8),
          ),
        ),
      ],
    );
  }

  List<Widget> _buildSessionsList() {
    return _sessions.map((session) {
      return Card(
        elevation: 2,
        margin: const EdgeInsets.only(bottom: 8),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        child: ExpansionTile(
          leading: const CircleAvatar(
            backgroundColor: AppConfig.blueColor,
            child: Icon(Icons.fitness_center, color: AppConfig.goldColor),
          ),
          title: Text(
            'Session ${_formatDate(DateTime.parse(session.date))}',
            style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
          ),
          trailing: PopupMenuButton<String>(
            onSelected: (value) {
              if (value == 'delete') {
                _confirmDeleteSession(session);
              } else if (value == 'add_set') {
                _showAddSetDialog(session);
              }
            },
            itemBuilder: (context) => [
              const PopupMenuItem(
                value: 'add_set',
                child: Row(
                  children: [
                    Icon(Icons.add, color: Colors.green),
                    SizedBox(width: 8),
                    Text('Add Set'),
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
          children: [
            _GymSetsList(
              sessionId: session.id,
              apiService: _apiService,
              exercises: _exercises,
              onRefresh: _loadData,
            ),
          ],
        ),
      );
    }).toList();
  }

  Future<void> _showAddSetDialog(GymSession session) async {
    if (_exercises.isEmpty) {
      AppSnackBar.showError(
        context,
        'No exercises available. Create an exercise first.',
      );
      return;
    }

    GymExercise? selectedExercise = _exercises.first;
    final repsController = TextEditingController(text: '10');
    final weightController = TextEditingController(text: '20');
    final setNumberController = TextEditingController(text: '1');

    await showDialog(
      context: context,
      builder: (context) => StatefulBuilder(
        builder: (context, setDialogState) => AlertDialog(
          title: const Text('Add Set'),
          content: SingleChildScrollView(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Exercise'),
                const SizedBox(height: 8),
                DropdownButtonFormField<GymExercise>(
                  value: selectedExercise,
                  decoration: const InputDecoration(
                    border: OutlineInputBorder(),
                    contentPadding: EdgeInsets.symmetric(
                      horizontal: 12,
                      vertical: 8,
                    ),
                  ),
                  isExpanded: true,
                  items: _exercises.map((exercise) {
                    return DropdownMenuItem(
                      value: exercise,
                      child: Text(
                        exercise.name,
                        overflow: TextOverflow.ellipsis,
                      ),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      setDialogState(() {
                        selectedExercise = value;
                      });
                    }
                  },
                ),
                const SizedBox(height: 16),
                TextField(
                  controller: setNumberController,
                  keyboardType: TextInputType.number,
                  decoration: const InputDecoration(
                    labelText: 'Set Number',
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 16),
                TextField(
                  controller: repsController,
                  keyboardType: TextInputType.number,
                  decoration: const InputDecoration(
                    labelText: 'Repetitions',
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 16),
                TextField(
                  controller: weightController,
                  keyboardType: const TextInputType.numberWithOptions(
                    decimal: true,
                  ),
                  decoration: const InputDecoration(
                    labelText: 'Weight (kg)',
                    border: OutlineInputBorder(),
                  ),
                ),
              ],
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Cancel'),
            ),
            ElevatedButton(
              onPressed: () async {
                final setNumber = int.tryParse(setNumberController.text);
                final reps = int.tryParse(repsController.text);
                final weight = double.tryParse(weightController.text);

                if (setNumber == null || setNumber <= 0) {
                  AppSnackBar.showError(
                    context,
                    'Please enter a valid set number',
                  );
                  return;
                }

                if (reps == null || reps <= 0) {
                  AppSnackBar.showError(
                    context,
                    'Please enter valid repetitions',
                  );
                  return;
                }

                if (weight == null || weight < 0) {
                  AppSnackBar.showError(context, 'Please enter a valid weight');
                  return;
                }

                if (selectedExercise == null) {
                  AppSnackBar.showError(context, 'Please select an exercise');
                  return;
                }

                Navigator.pop(context);
                await _addSet(
                  session.id,
                  selectedExercise!.id,
                  setNumber,
                  reps,
                  weight,
                );
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

  Future<void> _addSet(
    String sessionId,
    String exerciseId,
    int setNumber,
    int reps,
    double weight,
  ) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.createGymSet(
        accessToken: accessToken,
        sessionId: sessionId,
        exerciseId: exerciseId,
        setNumber: setNumber,
        repetitions: reps,
        weightKg: weight,
      );
      await _loadData();
      if (mounted) {
        AppSnackBar.showSuccess(context, 'Set added successfully');
      }
    } on ApiException catch (e) {
      if (mounted) {
        AppSnackBar.showError(context, e.message);
      }
    }
  }

  Widget _buildExercisesList() {
    return Card(
      elevation: 2,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: ListView.separated(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: _exercises.length,
        separatorBuilder: (context, index) => const Divider(height: 1),
        itemBuilder: (context, index) {
          final exercise = _exercises[index];
          return ListTile(
            leading: const CircleAvatar(
              backgroundColor: AppConfig.goldColor,
              child: Icon(Icons.sports_gymnastics, color: AppConfig.blueColor),
            ),
            title: Text(
              exercise.name,
              style: const TextStyle(fontWeight: FontWeight.bold),
            ),
            subtitle: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (exercise.description != null)
                  Text(
                    exercise.description!,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                const SizedBox(height: 4),
                Wrap(
                  spacing: 4,
                  runSpacing: 2,
                  children: [
                    ...exercise.primaryMuscles.map(
                      (m) => Chip(
                        label: Text(
                          m.displayName,
                          style: const TextStyle(fontSize: 10),
                        ),
                        backgroundColor: AppConfig.blueColor.withOpacity(0.2),
                        padding: EdgeInsets.zero,
                        materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                      ),
                    ),
                    ...exercise.secondaryMuscles.map(
                      (m) => Chip(
                        label: Text(
                          m.displayName,
                          style: const TextStyle(fontSize: 10),
                        ),
                        backgroundColor: AppConfig.goldColor.withOpacity(0.2),
                        padding: EdgeInsets.zero,
                        materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                      ),
                    ),
                  ],
                ),
              ],
            ),
            isThreeLine: true,
          );
        },
      ),
    );
  }
}

class _GymSetsList extends StatefulWidget {
  final String sessionId;
  final ApiService apiService;
  final List<GymExercise> exercises;
  final VoidCallback onRefresh;

  const _GymSetsList({
    required this.sessionId,
    required this.apiService,
    required this.exercises,
    required this.onRefresh,
  });

  @override
  State<_GymSetsList> createState() => _GymSetsListState();
}

class _GymSetsListState extends State<_GymSetsList> {
  List<GymSet> _sets = [];
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadSets();
  }

  Future<void> _loadSets() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      final sets = await widget.apiService.getGymSets(
        accessToken,
        widget.sessionId,
      );
      setState(() {
        _sets = sets;
        _isLoading = false;
      });
    } on ApiException catch (e) {
      setState(() {
        _error = e.message;
        _isLoading = false;
      });
    }
  }

  GymExercise? _getExercise(String id) {
    for (final exercise in widget.exercises) {
      if (exercise.id == id) {
        return exercise;
      }
    }
    return null;
  }

  Future<void> _confirmDeleteSet(GymSet gymSet) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Set'),
        content: const Text('Are you sure you want to delete this set?'),
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
      await _deleteSet(gymSet);
    }
  }

  Future<void> _deleteSet(GymSet gymSet) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await widget.apiService.deleteGymSet(
        accessToken: accessToken,
        sessionId: widget.sessionId,
        setId: gymSet.id,
      );
      await _loadSets();
      widget.onRefresh();
      if (mounted) {
        AppSnackBar.showSuccess(context, 'Set deleted successfully');
      }
    } on ApiException catch (e) {
      if (mounted) {
        AppSnackBar.showError(context, e.message);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Padding(
        padding: EdgeInsets.all(16.0),
        child: Center(child: CircularProgressIndicator()),
      );
    }

    if (_error != null) {
      return Padding(
        padding: const EdgeInsets.all(16.0),
        child: Text(_error!, style: const TextStyle(color: AppConfig.redColor)),
      );
    }

    if (_sets.isEmpty) {
      return Padding(
        padding: const EdgeInsets.all(16.0),
        child: Center(
          child: Text(
            'No sets added yet',
            style: TextStyle(color: Colors.grey[600]),
          ),
        ),
      );
    }

    // Group sets by exercise
    final groupedSets = <String, List<GymSet>>{};
    for (final gymSet in _sets) {
      groupedSets.putIfAbsent(gymSet.exerciseId, () => []).add(gymSet);
    }

    return Column(
      children: groupedSets.entries.map((entry) {
        final exercise = _getExercise(entry.key);
        final sets = entry.value;
        sets.sort((a, b) => a.setNumber.compareTo(b.setNumber));

        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Padding(
              padding: const EdgeInsets.symmetric(
                horizontal: 16.0,
                vertical: 8.0,
              ),
              child: Text(
                exercise?.name ?? 'Unknown Exercise',
                style: const TextStyle(
                  fontWeight: FontWeight.bold,
                  fontSize: 14,
                  color: AppConfig.blueColor,
                ),
              ),
            ),
            ...sets.map((gymSet) {
              return ListTile(
                leading: CircleAvatar(
                  backgroundColor: AppConfig.blueColor.withOpacity(0.1),
                  child: Text(
                    '${gymSet.setNumber}',
                    style: const TextStyle(
                      fontWeight: FontWeight.bold,
                      color: AppConfig.blueColor,
                    ),
                  ),
                ),
                title: Text('${gymSet.repetitions} reps'),
                subtitle: Text('${gymSet.weightKgDouble.toStringAsFixed(1)} kg'),
                trailing: IconButton(
                  icon: const Icon(
                    Icons.delete_outline,
                    color: AppConfig.redColor,
                  ),
                  onPressed: () => _confirmDeleteSet(gymSet),
                ),
              );
            }),
          ],
        );
      }).toList(),
    );
  }
}
