class FoodItem {
  final String id;
  final String name;
  final String? description;
  final String? scanCode;
  final int caloriesPer100g;
  final int proteinPer100g;
  final int carbsPer100g;
  final int fatPer100g;
  final String addedBy;
  final String addedAt;

  FoodItem({
    required this.id,
    required this.name,
    this.description,
    this.scanCode,
    required this.caloriesPer100g,
    required this.proteinPer100g,
    required this.carbsPer100g,
    required this.fatPer100g,
    required this.addedBy,
    required this.addedAt,
  });

  factory FoodItem.fromJson(Map<String, dynamic> json) {
    return FoodItem(
      id: json['id'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
      scanCode: json['scan_code'] as String?,
      caloriesPer100g: json['calories_per100g'] as int,
      proteinPer100g: json['protein_per100g'] as int,
      carbsPer100g: json['carbs_per100g'] as int,
      fatPer100g: json['fat_per100g'] as int,
      addedBy: json['added_by'] as String,
      addedAt: json['added_at'] as String,
    );
  }

  Map<String, dynamic> toJson() => {
        'id': id,
        'name': name,
        'description': description,
        'scan_code': scanCode,
        'calories_per100g': caloriesPer100g,
        'protein_per100g': proteinPer100g,
        'carbs_per100g': carbsPer100g,
        'fat_per100g': fatPer100g,
        'added_by': addedBy,
        'added_at': addedAt,
      };

  /// Calculate nutrition values based on quantity in grams
  int caloriesFor(int grams) => (caloriesPer100g * grams / 100).round();
  int proteinFor(int grams) => (proteinPer100g * grams / 100).round();
  int carbsFor(int grams) => (carbsPer100g * grams / 100).round();
  int fatFor(int grams) => (fatPer100g * grams / 100).round();
}

class CreateFoodItemRequest {
  final String name;
  final String? description;
  final String? scanCode;
  final int caloriesPer100g;
  final int proteinPer100g;
  final int carbsPer100g;
  final int fatPer100g;

  CreateFoodItemRequest({
    required this.name,
    this.description,
    this.scanCode,
    required this.caloriesPer100g,
    required this.proteinPer100g,
    required this.carbsPer100g,
    required this.fatPer100g,
  });

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = {
      'name': name,
      'calories_per100g': caloriesPer100g,
      'protein_per100g': proteinPer100g,
      'carbs_per100g': carbsPer100g,
      'fat_per100g': fatPer100g,
    };
    if (description != null) data['description'] = description;
    if (scanCode != null) data['scan_code'] = scanCode;
    return data;
  }
}

class UpdateFoodItemRequest {
  final String? name;
  final String? description;
  final String? scanCode;
  final int? caloriesPer100g;
  final int? proteinPer100g;
  final int? carbsPer100g;
  final int? fatPer100g;

  UpdateFoodItemRequest({
    this.name,
    this.description,
    this.scanCode,
    this.caloriesPer100g,
    this.proteinPer100g,
    this.carbsPer100g,
    this.fatPer100g,
  });

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = {};
    if (name != null) data['name'] = name;
    if (description != null) data['description'] = description;
    if (scanCode != null) data['scan_code'] = scanCode;
    if (caloriesPer100g != null) data['calories_per100g'] = caloriesPer100g;
    if (proteinPer100g != null) data['protein_per100g'] = proteinPer100g;
    if (carbsPer100g != null) data['carbs_per100g'] = carbsPer100g;
    if (fatPer100g != null) data['fat_per100g'] = fatPer100g;
    return data;
  }
}
