/// Enum representing the type of meal
enum MealType {
  breakfast,
  lunch,
  snack,
  dinner,
}

extension MealTypeExtension on MealType {
  String get displayName {
    switch (this) {
      case MealType.breakfast:
        return 'Breakfast';
      case MealType.lunch:
        return 'Lunch';
      case MealType.snack:
        return 'Snack';
      case MealType.dinner:
        return 'Dinner';
    }
  }

  String get value {
    switch (this) {
      case MealType.breakfast:
        return 'breakfast';
      case MealType.lunch:
        return 'lunch';
      case MealType.snack:
        return 'snack';
      case MealType.dinner:
        return 'dinner';
    }
  }

  static MealType fromString(String value) {
    switch (value.toLowerCase()) {
      case 'breakfast':
        return MealType.breakfast;
      case 'lunch':
        return MealType.lunch;
      case 'snack':
        return MealType.snack;
      case 'dinner':
        return MealType.dinner;
      default:
        throw ArgumentError('Unknown meal type: $value');
    }
  }
}

class Meal {
  final String id;
  final String userId;
  final String kind;
  final String date;
  final String? description;
  final String createdAt;
  final String updatedAt;

  Meal({
    required this.id,
    required this.userId,
    required this.kind,
    required this.date,
    this.description,
    required this.createdAt,
    required this.updatedAt,
  });

  factory Meal.fromJson(Map<String, dynamic> json) {
    return Meal(
      id: json['id'] as String,
      userId: json['user_id'] as String,
      kind: json['kind'] as String,
      date: json['date'] as String,
      description: json['description'] as String?,
      createdAt: json['created_at'] as String,
      updatedAt: json['updated_at'] as String,
    );
  }

  Map<String, dynamic> toJson() => {
        'id': id,
        'user_id': userId,
        'kind': kind,
        'date': date,
        'description': description,
        'created_at': createdAt,
        'updated_at': updatedAt,
      };

  MealType get mealType => MealTypeExtension.fromString(kind);
}

class MealItem {
  final String id;
  final String mealId;
  final String foodItemId;
  final int quantityInGrams;

  MealItem({
    required this.id,
    required this.mealId,
    required this.foodItemId,
    required this.quantityInGrams,
  });

  factory MealItem.fromJson(Map<String, dynamic> json) {
    return MealItem(
      id: json['id'] as String,
      mealId: json['meal_id'] as String,
      foodItemId: json['food_item_id'] as String,
      quantityInGrams: json['quantity_in_grams'] as int,
    );
  }

  Map<String, dynamic> toJson() => {
        'id': id,
        'meal_id': mealId,
        'food_item_id': foodItemId,
        'quantity_in_grams': quantityInGrams,
      };
}

class CreateMealRequest {
  final String kind;
  final String date;
  final String? description;

  CreateMealRequest({
    required this.kind,
    required this.date,
    this.description,
  });

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = {
      'kind': kind,
      'date': date,
    };
    if (description != null) {
      data['description'] = description;
    }
    return data;
  }
}

class UpdateMealRequest {
  final String? kind;
  final String? date;
  final String? description;

  UpdateMealRequest({
    this.kind,
    this.date,
    this.description,
  });

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = {};
    if (kind != null) data['kind'] = kind;
    if (date != null) data['date'] = date;
    if (description != null) data['description'] = description;
    return data;
  }
}

class AddMealItemRequest {
  final String foodItemId;
  final int quantityInGrams;

  AddMealItemRequest({
    required this.foodItemId,
    required this.quantityInGrams,
  });

  Map<String, dynamic> toJson() => {
        'food_item_id': foodItemId,
        'quantity_in_grams': quantityInGrams,
      };
}

class UpdateMealItemRequest {
  final int quantityInGrams;

  UpdateMealItemRequest({
    required this.quantityInGrams,
  });

  Map<String, dynamic> toJson() => {
        'quantity_in_grams': quantityInGrams,
      };
}
