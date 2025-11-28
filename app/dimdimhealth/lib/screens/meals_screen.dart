import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/food_item.dart';
import '../models/meal.dart';
import '../services/api_service.dart';
import '../services/auth_provider.dart';
import '../utils/app_config.dart';
import '../widgets/user_avatar.dart';

class MealsScreen extends StatefulWidget {
  const MealsScreen({super.key});

  @override
  State<MealsScreen> createState() => _MealsScreenState();
}

class _MealsScreenState extends State<MealsScreen> {
  final ApiService _apiService = ApiService();
  List<Meal> _meals = [];
  List<FoodItem> _foodItems = [];
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
      final meals = await _apiService.getMeals(accessToken, date: _selectedDate);
      final foodItems = await _apiService.getFoodItems(accessToken);

      setState(() {
        _meals = meals;
        _foodItems = foodItems;
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

  Future<void> _showAddMealDialog() async {
    MealType selectedType = MealType.breakfast;
    final descriptionController = TextEditingController();

    await showDialog(
      context: context,
      builder: (context) => StatefulBuilder(
        builder: (context, setDialogState) => AlertDialog(
          title: const Text('Add Meal'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Meal Type'),
              const SizedBox(height: 8),
              DropdownButtonFormField<MealType>(
                value: selectedType,
                decoration: const InputDecoration(
                  border: OutlineInputBorder(),
                  contentPadding:
                      EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                ),
                items: MealType.values.map((type) {
                  return DropdownMenuItem(
                    value: type,
                    child: Text(type.displayName),
                  );
                }).toList(),
                onChanged: (value) {
                  if (value != null) {
                    setDialogState(() {
                      selectedType = value;
                    });
                  }
                },
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
            ],
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Cancel'),
            ),
            ElevatedButton(
              onPressed: () async {
                Navigator.pop(context);
                await _createMeal(
                  selectedType,
                  descriptionController.text.isEmpty
                      ? null
                      : descriptionController.text,
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

  Future<void> _createMeal(MealType kind, String? description) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.createMeal(
        accessToken: accessToken,
        kind: kind,
        date: _selectedDate,
        description: description,
      );
      await _loadData();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Meal added successfully'),
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

  Future<void> _showEditMealDialog(Meal meal) async {
    MealType selectedType = meal.mealType;
    final descriptionController = TextEditingController(
      text: meal.description ?? '',
    );

    await showDialog(
      context: context,
      builder: (context) => StatefulBuilder(
        builder: (context, setDialogState) => AlertDialog(
          title: const Text('Edit Meal'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Meal Type'),
              const SizedBox(height: 8),
              DropdownButtonFormField<MealType>(
                value: selectedType,
                decoration: const InputDecoration(
                  border: OutlineInputBorder(),
                  contentPadding:
                      EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                ),
                items: MealType.values.map((type) {
                  return DropdownMenuItem(
                    value: type,
                    child: Text(type.displayName),
                  );
                }).toList(),
                onChanged: (value) {
                  if (value != null) {
                    setDialogState(() {
                      selectedType = value;
                    });
                  }
                },
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
            ],
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Cancel'),
            ),
            ElevatedButton(
              onPressed: () async {
                Navigator.pop(context);
                await _updateMeal(
                  meal.id,
                  selectedType,
                  descriptionController.text.isEmpty
                      ? null
                      : descriptionController.text,
                );
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

  Future<void> _updateMeal(
    String id,
    MealType kind,
    String? description,
  ) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.updateMeal(
        accessToken: accessToken,
        id: id,
        kind: kind,
        description: description,
      );
      await _loadData();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Meal updated successfully'),
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

  Future<void> _confirmDeleteMeal(Meal meal) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Meal'),
        content: const Text(
          'Are you sure you want to delete this meal and all its items?',
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
      await _deleteMeal(meal.id);
    }
  }

  Future<void> _deleteMeal(String id) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.deleteMeal(accessToken: accessToken, id: id);
      await _loadData();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Meal deleted successfully'),
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

  String _formatDate(DateTime date) {
    return '${date.day}/${date.month}/${date.year}';
  }

  IconData _getMealIcon(MealType type) {
    switch (type) {
      case MealType.breakfast:
        return Icons.free_breakfast;
      case MealType.lunch:
        return Icons.lunch_dining;
      case MealType.snack:
        return Icons.cookie;
      case MealType.dinner:
        return Icons.dinner_dining;
    }
  }

  Color _getMealColor(MealType type) {
    switch (type) {
      case MealType.breakfast:
        return Colors.orange;
      case MealType.lunch:
        return AppConfig.blueColor;
      case MealType.snack:
        return Colors.purple;
      case MealType.dinner:
        return Colors.teal;
    }
  }

  @override
  Widget build(BuildContext context) {
    final authProvider = Provider.of<AuthProvider>(context);
    final user = authProvider.user;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Meal Tracker'),
        backgroundColor: AppConfig.blueColor,
        foregroundColor: AppConfig.goldColor,
        actions: [
          Padding(
            padding: const EdgeInsets.only(right: 12.0),
            child: UserAvatar(profileImage: user?.profileImage),
          ),
        ],
      ),
      body: Container(
        width: double.infinity,
        height: double.infinity,
        decoration:
            BoxDecoration(color: Theme.of(context).colorScheme.primary),
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

                              // Meals List
                              const Text(
                                'Meals',
                                style: TextStyle(
                                  fontSize: 20,
                                  fontWeight: FontWeight.bold,
                                  color: AppConfig.goldColor,
                                ),
                              ),
                              const SizedBox(height: 8),
                              if (_meals.isEmpty)
                                _buildEmptyState()
                              else
                                ..._buildMealsList(),
                            ],
                          ),
                        ),
                      ),
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _showAddMealDialog,
        backgroundColor: AppConfig.goldColor,
        foregroundColor: AppConfig.blueColor,
        child: const Icon(Icons.add),
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
                    _selectedDate =
                        _selectedDate.subtract(const Duration(days: 1));
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
    // Calculate total nutrition for the day - this would require loading meal items
    // For now, show a placeholder summary
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
                  '${_meals.length}',
                  'Meals',
                  Icons.restaurant,
                ),
                _buildSummaryItem(
                  _countMealsByType(MealType.breakfast).toString(),
                  'Breakfast',
                  Icons.free_breakfast,
                ),
                _buildSummaryItem(
                  _countMealsByType(MealType.lunch).toString(),
                  'Lunch',
                  Icons.lunch_dining,
                ),
                _buildSummaryItem(
                  _countMealsByType(MealType.dinner).toString(),
                  'Dinner',
                  Icons.dinner_dining,
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  int _countMealsByType(MealType type) {
    return _meals.where((meal) => meal.mealType == type).length;
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
              Icons.restaurant_menu,
              size: 64,
              color: Colors.grey[400],
            ),
            const SizedBox(height: 16),
            Text(
              'No meals recorded',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                color: Colors.grey[600],
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'Tap the + button to add your first meal for this day',
              style: TextStyle(fontSize: 14, color: Colors.grey[500]),
              textAlign: TextAlign.center,
            ),
          ],
        ),
      ),
    );
  }

  List<Widget> _buildMealsList() {
    // Group meals by type and sort them in order: breakfast, lunch, snack, dinner
    final mealOrder = [
      MealType.breakfast,
      MealType.lunch,
      MealType.snack,
      MealType.dinner,
    ];

    final sortedMeals = List<Meal>.from(_meals);
    sortedMeals.sort((a, b) {
      final aIndex = mealOrder.indexOf(a.mealType);
      final bIndex = mealOrder.indexOf(b.mealType);
      return aIndex.compareTo(bIndex);
    });

    return sortedMeals.map((meal) {
      final mealColor = _getMealColor(meal.mealType);
      return Card(
        elevation: 2,
        margin: const EdgeInsets.only(bottom: 8),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        child: ExpansionTile(
          leading: CircleAvatar(
            backgroundColor: mealColor,
            child: Icon(
              _getMealIcon(meal.mealType),
              color: AppConfig.whiteColor,
            ),
          ),
          title: Text(
            meal.mealType.displayName,
            style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
          ),
          subtitle: meal.description != null
              ? Text(
                  meal.description!,
                  style: TextStyle(color: Colors.grey[600]),
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                )
              : null,
          trailing: PopupMenuButton<String>(
            onSelected: (value) {
              if (value == 'edit') {
                _showEditMealDialog(meal);
              } else if (value == 'delete') {
                _confirmDeleteMeal(meal);
              } else if (value == 'add_item') {
                _showAddMealItemDialog(meal);
              }
            },
            itemBuilder: (context) => [
              const PopupMenuItem(
                value: 'add_item',
                child: Row(
                  children: [
                    Icon(Icons.add, color: Colors.green),
                    SizedBox(width: 8),
                    Text('Add Item'),
                  ],
                ),
              ),
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
          children: [
            _MealItemsList(
              mealId: meal.id,
              apiService: _apiService,
              foodItems: _foodItems,
              onRefresh: _loadData,
            ),
          ],
        ),
      );
    }).toList();
  }

  Future<void> _showAddMealItemDialog(Meal meal) async {
    if (_foodItems.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('No food items available. Create a food item first.'),
          backgroundColor: AppConfig.redColor,
        ),
      );
      return;
    }

    FoodItem? selectedFoodItem = _foodItems.first;
    final quantityController = TextEditingController(text: '100');

    await showDialog(
      context: context,
      builder: (context) => StatefulBuilder(
        builder: (context, setDialogState) => AlertDialog(
          title: const Text('Add Food Item'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Food Item'),
              const SizedBox(height: 8),
              DropdownButtonFormField<FoodItem>(
                value: selectedFoodItem,
                decoration: const InputDecoration(
                  border: OutlineInputBorder(),
                  contentPadding:
                      EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                ),
                isExpanded: true,
                items: _foodItems.map((item) {
                  return DropdownMenuItem(
                    value: item,
                    child: Text(
                      item.name,
                      overflow: TextOverflow.ellipsis,
                    ),
                  );
                }).toList(),
                onChanged: (value) {
                  if (value != null) {
                    setDialogState(() {
                      selectedFoodItem = value;
                    });
                  }
                },
              ),
              const SizedBox(height: 16),
              TextField(
                controller: quantityController,
                keyboardType: TextInputType.number,
                decoration: const InputDecoration(
                  labelText: 'Quantity (grams)',
                  border: OutlineInputBorder(),
                ),
              ),
              if (selectedFoodItem != null) ...[
                const SizedBox(height: 16),
                _buildNutritionPreview(
                  selectedFoodItem!,
                  int.tryParse(quantityController.text) ?? 100,
                ),
              ],
            ],
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Cancel'),
            ),
            ElevatedButton(
              onPressed: () async {
                final quantity = int.tryParse(quantityController.text);
                if (quantity == null || quantity <= 0) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(
                      content: Text('Please enter a valid quantity'),
                      backgroundColor: AppConfig.redColor,
                    ),
                  );
                  return;
                }

                Navigator.pop(context);
                await _addMealItem(
                  meal.id,
                  selectedFoodItem!.id,
                  quantity,
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

  Widget _buildNutritionPreview(FoodItem foodItem, int grams) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: Colors.grey[100],
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Nutrition for ${grams}g:',
            style: const TextStyle(fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 8),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              _buildNutritionValue(
                'Calories',
                '${foodItem.caloriesFor(grams)}',
              ),
              _buildNutritionValue(
                'Protein',
                '${foodItem.proteinFor(grams)}g',
              ),
              _buildNutritionValue(
                'Carbs',
                '${foodItem.carbsFor(grams)}g',
              ),
              _buildNutritionValue(
                'Fat',
                '${foodItem.fatFor(grams)}g',
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildNutritionValue(String label, String value) {
    return Column(
      children: [
        Text(
          value,
          style: const TextStyle(
            fontWeight: FontWeight.bold,
            color: AppConfig.blueColor,
          ),
        ),
        Text(
          label,
          style: TextStyle(fontSize: 10, color: Colors.grey[600]),
        ),
      ],
    );
  }

  Future<void> _addMealItem(
    String mealId,
    String foodItemId,
    int quantity,
  ) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await _apiService.addMealItem(
        accessToken: accessToken,
        mealId: mealId,
        foodItemId: foodItemId,
        quantityInGrams: quantity,
      );
      await _loadData();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Item added successfully'),
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
}

class _MealItemsList extends StatefulWidget {
  final String mealId;
  final ApiService apiService;
  final List<FoodItem> foodItems;
  final VoidCallback onRefresh;

  const _MealItemsList({
    required this.mealId,
    required this.apiService,
    required this.foodItems,
    required this.onRefresh,
  });

  @override
  State<_MealItemsList> createState() => _MealItemsListState();
}

class _MealItemsListState extends State<_MealItemsList> {
  List<MealItem> _items = [];
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadItems();
  }

  Future<void> _loadItems() async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      final items = await widget.apiService.getMealItems(
        accessToken,
        widget.mealId,
      );
      setState(() {
        _items = items;
        _isLoading = false;
      });
    } on ApiException catch (e) {
      setState(() {
        _error = e.message;
        _isLoading = false;
      });
    }
  }

  FoodItem? _getFoodItem(String id) {
    try {
      return widget.foodItems.firstWhere((item) => item.id == id);
    } catch (e) {
      return null;
    }
  }

  Future<void> _confirmDeleteItem(MealItem item) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Item'),
        content: const Text('Are you sure you want to delete this item?'),
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
      await _deleteItem(item);
    }
  }

  Future<void> _deleteItem(MealItem item) async {
    final authProvider = Provider.of<AuthProvider>(context, listen: false);
    final accessToken = authProvider.accessToken;

    if (accessToken == null) return;

    try {
      await widget.apiService.deleteMealItem(
        accessToken: accessToken,
        mealId: widget.mealId,
        itemId: item.id,
      );
      await _loadItems();
      widget.onRefresh();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Item deleted successfully'),
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

    if (_items.isEmpty) {
      return Padding(
        padding: const EdgeInsets.all(16.0),
        child: Center(
          child: Text(
            'No items added yet',
            style: TextStyle(color: Colors.grey[600]),
          ),
        ),
      );
    }

    return Column(
      children: _items.map((item) {
        final foodItem = _getFoodItem(item.foodItemId);
        return ListTile(
          leading: const Icon(Icons.fastfood, color: AppConfig.blueColor),
          title: Text(foodItem?.name ?? 'Unknown Food Item'),
          subtitle: Text('${item.quantityInGrams}g'),
          trailing: IconButton(
            icon: const Icon(Icons.delete_outline, color: AppConfig.redColor),
            onPressed: () => _confirmDeleteItem(item),
          ),
        );
      }).toList(),
    );
  }
}
