import 'dart:convert';
import 'package:http/http.dart' as http;
import '../models/food_item.dart';
import '../models/gym.dart';
import '../models/meal.dart';
import '../models/user.dart';
import '../models/user_info.dart';
import '../models/watch_permission.dart';
import '../models/weight.dart';
import '../utils/app_config.dart';

class ApiException implements Exception {
  final String message;
  final int? statusCode;

  ApiException(this.message, {this.statusCode});

  @override
  String toString() => message;
}

class ApiService {
  final String baseUrl = AppConfig.apiUrl;

  /// Formats a DateTime to YYYY-MM-DD string format for API calls
  String _formatDate(DateTime date) {
    return '${date.year.toString().padLeft(4, '0')}-${date.month.toString().padLeft(2, '0')}-${date.day.toString().padLeft(2, '0')}';
  }

  /// Formats an activity level for API calls with consistent precision
  String _formatActivityLevel(double activityLevel) {
    return activityLevel.toStringAsFixed(3);
  }

  Future<LoginResponse> register({
    required String username,
    required String email,
    required String password,
    String? profileImage,
  }) async {
    final request = RegisterRequest(
      user: RegisterUserData(
        username: username,
        email: email,
        password: password,
        profileImage: profileImage,
      ),
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/users'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return LoginResponse.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 409) {
      throw ApiException(
        'User already exists with this email or username',
        statusCode: 409,
      );
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Registration failed',
        statusCode: response.statusCode,
      );
    }
  }

  Future<LoginResponse> login({
    required String email,
    required String password,
  }) async {
    final request = LoginRequest(
      user: LoginUserData(email: email, password: password),
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/users/login'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return LoginResponse.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Invalid email or password', statusCode: 401);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException('Login failed', statusCode: response.statusCode);
    }
  }

  Future<ForgotPasswordResponse> forgotPassword({required String email}) async {
    final request = ForgotPasswordRequest(email: email);

    final response = await http.post(
      Uri.parse('$baseUrl/api/auth/forgot-password'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return ForgotPasswordResponse.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid email', statusCode: 400);
    } else {
      throw ApiException('Request failed', statusCode: response.statusCode);
    }
  }

  Future<LoginResponse> resetPassword({
    required String token,
    required String newPassword,
  }) async {
    final request = ResetPasswordRequest(
      token: token,
      newPassword: newPassword,
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/auth/reset-password'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return LoginResponse.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else if (response.statusCode == 404) {
      throw ApiException('Invalid or expired reset token', statusCode: 404);
    } else if (response.statusCode == 410) {
      throw ApiException('Reset token has expired', statusCode: 410);
    } else {
      throw ApiException('Request failed', statusCode: response.statusCode);
    }
  }

  Future<LoginResponse> loginAsGuest() async {
    final response = await http.post(
      Uri.parse('$baseUrl/api/users/guest'),
      headers: {'Content-Type': 'application/json'},
    );

    if (response.statusCode == 200) {
      return LoginResponse.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException('Guest login failed', statusCode: response.statusCode);
    }
  }

  Future<User> getCurrentUser(String accessToken) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/user'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final data = jsonDecode(response.body);
      return User.fromJson(data['user']);
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch user data',
        statusCode: response.statusCode,
      );
    }
  }

  Future<String> updateSettings({
    required String accessToken,
    String? username,
    String? email,
    String? profileImage,
    String? currentPassword,
    String? newPassword,
  }) async {
    final body = <String, dynamic>{};

    if (username != null) body['username'] = username;
    if (email != null) body['email'] = email;
    if (profileImage != null) body['profile_image'] = profileImage;

    if (currentPassword != null && newPassword != null) {
      body['passwords'] = {
        'current_password': currentPassword,
        'new_password': newPassword,
      };
    }

    final response = await http.put(
      Uri.parse('$baseUrl/api/settings'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(body),
    );

    if (response.statusCode == 200) {
      final data = jsonDecode(response.body);
      return data['message'] as String;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 409) {
      throw ApiException('Username or email already taken', statusCode: 409);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to update settings',
        statusCode: response.statusCode,
      );
    }
  }

  Future<VerifyEmailResponse> verifyEmail({required String token}) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/auth/verify-email?token=$token'),
      headers: {'Content-Type': 'application/json'},
    );

    if (response.statusCode == 200) {
      return VerifyEmailResponse.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 404) {
      throw ApiException(
        'Invalid or expired verification token',
        statusCode: 404,
      );
    } else if (response.statusCode == 410) {
      throw ApiException('Verification token has expired', statusCode: 410);
    } else {
      throw ApiException(
        'Verification failed',
        statusCode: response.statusCode,
      );
    }
  }

  // Weight API methods
  Future<List<UserWeight>> getWeights(String accessToken) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/user/weights'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      return data.map((json) => UserWeight.fromJson(json)).toList();
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch weights',
        statusCode: response.statusCode,
      );
    }
  }

  Future<UserWeight?> getLastWeight(String accessToken) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/user/weights/last'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      return UserWeight.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 404) {
      return null;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch last weight',
        statusCode: response.statusCode,
      );
    }
  }

  Future<UserWeightInfos?> getWeightInfos(String accessToken) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/user/weights/infos'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final data = jsonDecode(response.body);
      if (data == null) return null;
      return UserWeightInfos.fromJson(data);
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch weight infos',
        statusCode: response.statusCode,
      );
    }
  }

  Future<UserWeight> createWeight({
    required String accessToken,
    required double weightInKg,
    required DateTime recordedAt,
  }) async {
    // Format date as YYYY-MM-DD for NaiveDate
    final dateString =
        '${recordedAt.year.toString().padLeft(4, '0')}-${recordedAt.month.toString().padLeft(2, '0')}-${recordedAt.day.toString().padLeft(2, '0')}';
    final request = CreateWeightRequest(
      weightInKg: weightInKg,
      recordedAt: dateString,
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/user/weights'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return UserWeight.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 409) {
      final error = jsonDecode(response.body);
      throw ApiException(
        error['error'] ?? 'A weight entry already exists for this date',
        statusCode: 409,
      );
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to create weight entry',
        statusCode: response.statusCode,
      );
    }
  }

  Future<UserWeight> updateWeight({
    required String accessToken,
    required String id,
    required double weightInKg,
    required DateTime recordedAt,
  }) async {
    // Format date as YYYY-MM-DD for NaiveDate
    final dateString =
        '${recordedAt.year.toString().padLeft(4, '0')}-${recordedAt.month.toString().padLeft(2, '0')}-${recordedAt.day.toString().padLeft(2, '0')}';
    final request = UpdateWeightRequest(
      weightInKg: weightInKg,
      recordedAt: dateString,
    );

    final response = await http.put(
      Uri.parse('$baseUrl/api/user/weights/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return UserWeight.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to modify this entry', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Weight entry not found', statusCode: 404);
    } else if (response.statusCode == 409) {
      final error = jsonDecode(response.body);
      throw ApiException(
        error['error'] ?? 'A weight entry already exists for this date',
        statusCode: 409,
      );
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to update weight entry',
        statusCode: response.statusCode,
      );
    }
  }

  Future<void> deleteWeight({
    required String accessToken,
    required String id,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl/api/user/weights/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 204) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to delete this entry', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Weight entry not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to delete weight entry',
        statusCode: response.statusCode,
      );
    }
  }

  // Watch Permission API methods

  /// Search for users by username (minimum 3 characters)
  Future<List<UserSearchResult>> searchUsers({
    required String accessToken,
    required String query,
  }) async {
    final response = await http.get(
      Uri.parse(
        '$baseUrl/api/users/search?query=${Uri.encodeComponent(query)}',
      ),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final data = jsonDecode(response.body);
      final searchResponse = SearchUsersResponse.fromJson(data);
      return searchResponse.users;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid query', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to search users',
        statusCode: response.statusCode,
      );
    }
  }

  /// Get list of users that I allow to watch me (watchers)
  Future<List<WatchPermissionUser>> getWatchers(String accessToken) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/watch-permissions/watchers'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final data = jsonDecode(response.body);
      final watchersResponse = WatchersResponse.fromJson(data);
      return watchersResponse.watchers;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch watchers',
        statusCode: response.statusCode,
      );
    }
  }

  /// Get list of users that allow me to watch them (watching)
  Future<List<WatchPermissionUser>> getWatching(String accessToken) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/watch-permissions/watching'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final data = jsonDecode(response.body);
      final watchingResponse = WatchingResponse.fromJson(data);
      return watchingResponse.watching;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch watching users',
        statusCode: response.statusCode,
      );
    }
  }

  /// Grant watch permission to another user (allow them to see my profile)
  Future<void> grantWatchPermission({
    required String accessToken,
    required String userId,
  }) async {
    final request = GrantWatchPermissionRequest(userId: userId);

    final response = await http.post(
      Uri.parse('$baseUrl/api/watch-permissions/grant'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 201) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 404) {
      throw ApiException('User not found', statusCode: 404);
    } else if (response.statusCode == 409) {
      throw ApiException('Permission already granted', statusCode: 409);
    } else {
      throw ApiException(
        'Failed to grant permission',
        statusCode: response.statusCode,
      );
    }
  }

  /// Revoke watch permission from a user
  Future<void> revokeWatchPermission({
    required String accessToken,
    required String userId,
  }) async {
    final request = RevokeWatchPermissionRequest(userId: userId);

    final response = await http.post(
      Uri.parse('$baseUrl/api/watch-permissions/revoke'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 404) {
      throw ApiException('Permission not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to revoke permission',
        statusCode: response.statusCode,
      );
    }
  }

  // User Groups API methods
  Future<List<String>> getUserGroups(String accessToken) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/user-groups/myself'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final data = jsonDecode(response.body);
      return List<String>.from(data['groups'] ?? []);
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch user groups',
        statusCode: response.statusCode,
      );
    }
  }

  Future<void> joinPublicGroup(String accessToken) async {
    final response = await http.post(
      Uri.parse('$baseUrl/api/user-groups/join-public'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to join public group',
        statusCode: response.statusCode,
      );
    }
  }

  Future<void> leavePublicGroup(String accessToken) async {
    final response = await http.post(
      Uri.parse('$baseUrl/api/user-groups/leave-public'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to leave public group',
        statusCode: response.statusCode,
      );
    }
  }

  // Food Item API methods

  /// Get all food items, optionally filtered by name or scan code
  Future<List<FoodItem>> getFoodItems(
    String accessToken, {
    String? name,
    String? scanCode,
  }) async {
    String url = '$baseUrl/api/food-items';
    final queryParams = <String, String>{};
    if (name != null) queryParams['name'] = name;
    if (scanCode != null) queryParams['scan_code'] = scanCode;
    if (queryParams.isNotEmpty) {
      url += '?${Uri(queryParameters: queryParams).query}';
    }

    final response = await http.get(
      Uri.parse(url),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      return data.map((json) => FoodItem.fromJson(json)).toList();
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch food items',
        statusCode: response.statusCode,
      );
    }
  }

  /// Create a new food item
  Future<FoodItem> createFoodItem({
    required String accessToken,
    required String name,
    String? description,
    String? scanCode,
    required int caloriesPer100g,
    required int proteinPer100g,
    required int carbsPer100g,
    required int fatPer100g,
  }) async {
    final request = CreateFoodItemRequest(
      name: name,
      description: description,
      scanCode: scanCode,
      caloriesPer100g: caloriesPer100g,
      proteinPer100g: proteinPer100g,
      carbsPer100g: carbsPer100g,
      fatPer100g: fatPer100g,
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/food-items'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return FoodItem.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to create food item',
        statusCode: response.statusCode,
      );
    }
  }

  /// Update a food item
  Future<FoodItem> updateFoodItem({
    required String accessToken,
    required String id,
    String? name,
    String? description,
    String? scanCode,
    int? caloriesPer100g,
    int? proteinPer100g,
    int? carbsPer100g,
    int? fatPer100g,
  }) async {
    final request = UpdateFoodItemRequest(
      name: name,
      description: description,
      scanCode: scanCode,
      caloriesPer100g: caloriesPer100g,
      proteinPer100g: proteinPer100g,
      carbsPer100g: carbsPer100g,
      fatPer100g: fatPer100g,
    );

    final response = await http.put(
      Uri.parse('$baseUrl/api/food-items/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return FoodItem.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to modify this food item', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Food item not found', statusCode: 404);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to update food item',
        statusCode: response.statusCode,
      );
    }
  }

  /// Delete a food item
  Future<void> deleteFoodItem({
    required String accessToken,
    required String id,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl/api/food-items/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 204) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to delete this food item', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Food item not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to delete food item',
        statusCode: response.statusCode,
      );
    }
  }

  // Meal API methods

  /// Get all meals, optionally filtered by date
  Future<List<Meal>> getMeals(String accessToken, {DateTime? date}) async {
    String url = '$baseUrl/api/meals';
    if (date != null) {
      url += '?date=${_formatDate(date)}';
    }

    final response = await http.get(
      Uri.parse(url),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      return data.map((json) => Meal.fromJson(json)).toList();
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch meals',
        statusCode: response.statusCode,
      );
    }
  }

  /// Create a new meal
  Future<Meal> createMeal({
    required String accessToken,
    required MealType kind,
    required DateTime date,
    String? description,
  }) async {
    final request = CreateMealRequest(
      kind: kind.value,
      date: _formatDate(date),
      description: description,
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/meals'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return Meal.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to create meal',
        statusCode: response.statusCode,
      );
    }
  }

  /// Update a meal
  Future<Meal> updateMeal({
    required String accessToken,
    required String id,
    MealType? kind,
    DateTime? date,
    String? description,
  }) async {
    final Map<String, dynamic> body = {};
    if (kind != null) body['kind'] = kind.value;
    if (date != null) {
      body['date'] = _formatDate(date);
    }
    if (description != null) body['description'] = description;

    final response = await http.put(
      Uri.parse('$baseUrl/api/meals/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(body),
    );

    if (response.statusCode == 200) {
      return Meal.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to modify this meal', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Meal not found', statusCode: 404);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to update meal',
        statusCode: response.statusCode,
      );
    }
  }

  /// Delete a meal
  Future<void> deleteMeal({
    required String accessToken,
    required String id,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl/api/meals/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 204) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to delete this meal', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Meal not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to delete meal',
        statusCode: response.statusCode,
      );
    }
  }

  // Meal Item API methods

  /// Get all items for a meal
  Future<List<MealItem>> getMealItems(
    String accessToken,
    String mealId,
  ) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/meals/$mealId/items'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      return data.map((json) => MealItem.fromJson(json)).toList();
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to view this meal', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Meal not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to fetch meal items',
        statusCode: response.statusCode,
      );
    }
  }

  /// Add an item to a meal
  Future<MealItem> addMealItem({
    required String accessToken,
    required String mealId,
    required String foodItemId,
    required int quantityInGrams,
  }) async {
    final request = AddMealItemRequest(
      foodItemId: foodItemId,
      quantityInGrams: quantityInGrams,
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/meals/$mealId/items'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return MealItem.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to modify this meal', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Meal not found', statusCode: 404);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to add meal item',
        statusCode: response.statusCode,
      );
    }
  }

  /// Update a meal item
  Future<MealItem> updateMealItem({
    required String accessToken,
    required String mealId,
    required String itemId,
    required int quantityInGrams,
  }) async {
    final request = UpdateMealItemRequest(
      quantityInGrams: quantityInGrams,
    );

    final response = await http.put(
      Uri.parse('$baseUrl/api/meals/$mealId/items/$itemId'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return MealItem.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to modify this meal', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Meal or item not found', statusCode: 404);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to update meal item',
        statusCode: response.statusCode,
      );
    }
  }

  /// Delete a meal item
  Future<void> deleteMealItem({
    required String accessToken,
    required String mealId,
    required String itemId,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl/api/meals/$mealId/items/$itemId'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 204) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to modify this meal', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Meal or item not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to delete meal item',
        statusCode: response.statusCode,
      );
    }
  }

  // Gym Exercise API methods

  /// Get all gym exercises, optionally filtered by muscle or name
  Future<List<GymExercise>> getGymExercises(
    String accessToken, {
    String? muscle,
    String? name,
  }) async {
    String url = '$baseUrl/api/gym/exercises';
    final queryParams = <String, String>{};
    if (muscle != null) queryParams['muscle'] = muscle;
    if (name != null) queryParams['name'] = name;
    if (queryParams.isNotEmpty) {
      url += '?${Uri(queryParameters: queryParams).query}';
    }

    final response = await http.get(
      Uri.parse(url),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      return data.map((json) => GymExercise.fromJson(json)).toList();
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch gym exercises',
        statusCode: response.statusCode,
      );
    }
  }

  /// Get a specific gym exercise by ID
  Future<GymExercise> getGymExercise(String accessToken, String id) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/gym/exercises/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      return GymExercise.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 404) {
      throw ApiException('Exercise not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to fetch gym exercise',
        statusCode: response.statusCode,
      );
    }
  }

  /// Create a new gym exercise
  Future<GymExercise> createGymExercise({
    required String accessToken,
    required String name,
    String? description,
    required List<String> primaryMuscles,
    required List<String> secondaryMuscles,
  }) async {
    final request = CreateGymExerciseRequest(
      name: name,
      description: description,
      primaryMuscles: primaryMuscles,
      secondaryMuscles: secondaryMuscles,
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/gym/exercises'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return GymExercise.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to create gym exercise',
        statusCode: response.statusCode,
      );
    }
  }

  /// Delete a gym exercise
  Future<void> deleteGymExercise({
    required String accessToken,
    required String id,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl/api/gym/exercises/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 204) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to delete this exercise', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Exercise not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to delete gym exercise',
        statusCode: response.statusCode,
      );
    }
  }

  // Gym Session API methods

  /// Get all gym sessions, optionally filtered by date
  Future<List<GymSession>> getGymSessions(
    String accessToken, {
    DateTime? date,
  }) async {
    String url = '$baseUrl/api/gym/sessions';
    if (date != null) {
      url += '?date=${_formatDate(date)}';
    }

    final response = await http.get(
      Uri.parse(url),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      return data.map((json) => GymSession.fromJson(json)).toList();
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch gym sessions',
        statusCode: response.statusCode,
      );
    }
  }

  /// Get a specific gym session by ID
  Future<GymSession> getGymSession(String accessToken, String id) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/gym/sessions/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      return GymSession.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to view this session', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Session not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to fetch gym session',
        statusCode: response.statusCode,
      );
    }
  }

  /// Create a new gym session
  Future<GymSession> createGymSession({
    required String accessToken,
    required DateTime date,
  }) async {
    final request = CreateGymSessionRequest(date: _formatDate(date));

    final response = await http.post(
      Uri.parse('$baseUrl/api/gym/sessions'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return GymSession.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to create gym session',
        statusCode: response.statusCode,
      );
    }
  }

  /// Delete a gym session
  Future<void> deleteGymSession({
    required String accessToken,
    required String id,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl/api/gym/sessions/$id'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 204) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to delete this session', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Session not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to delete gym session',
        statusCode: response.statusCode,
      );
    }
  }

  // Gym Set API methods

  /// Get all sets for a gym session
  Future<List<GymSet>> getGymSets(String accessToken, String sessionId) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/gym/sessions/$sessionId/sets'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      return data.map((json) => GymSet.fromJson(json)).toList();
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to view this session', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Session not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to fetch gym sets',
        statusCode: response.statusCode,
      );
    }
  }

  /// Create a new gym set
  Future<GymSet> createGymSet({
    required String accessToken,
    required String sessionId,
    required String exerciseId,
    required int setNumber,
    required int repetitions,
    required double weightKg,
  }) async {
    final request = CreateGymSetRequest(
      exerciseId: exerciseId,
      setNumber: setNumber,
      repetitions: repetitions,
      weightKg: weightKg.toStringAsFixed(2),
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/gym/sessions/$sessionId/sets'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return GymSet.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to modify this session', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Session not found', statusCode: 404);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to create gym set',
        statusCode: response.statusCode,
      );
    }
  }

  /// Delete a gym set
  Future<void> deleteGymSet({
    required String accessToken,
    required String sessionId,
    required String setId,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl/api/gym/sessions/$sessionId/sets/$setId'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 204) {
      return;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 403) {
      throw ApiException('Not allowed to modify this session', statusCode: 403);
    } else if (response.statusCode == 404) {
      throw ApiException('Session or set not found', statusCode: 404);
    } else {
      throw ApiException(
        'Failed to delete gym set',
        statusCode: response.statusCode,
      );
    }
  }

  // User Additional Info API methods

  /// Get user additional info
  Future<UserAdditionalInfo?> getUserInfo(String accessToken) async {
    final response = await http.get(
      Uri.parse('$baseUrl/api/user/info'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
    );

    if (response.statusCode == 200) {
      return UserAdditionalInfo.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 404) {
      return null;
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else {
      throw ApiException(
        'Failed to fetch user info',
        statusCode: response.statusCode,
      );
    }
  }

  /// Create user additional info
  Future<UserAdditionalInfo> createUserInfo({
    required String accessToken,
    required DateTime birthDate,
    required int heightInCm,
    required Gender gender,
    required double activityLevel,
  }) async {
    final request = CreateUserInfoRequest(
      birthDate: _formatDate(birthDate),
      heightInCm: heightInCm,
      gender: gender,
      activityLevel: _formatActivityLevel(activityLevel),
    );

    final response = await http.post(
      Uri.parse('$baseUrl/api/user/info'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(request.toJson()),
    );

    if (response.statusCode == 200) {
      return UserAdditionalInfo.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 409) {
      throw ApiException(
        'User info already exists. Use update instead.',
        statusCode: 409,
      );
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to create user info',
        statusCode: response.statusCode,
      );
    }
  }

  /// Update user additional info
  Future<UserAdditionalInfo> updateUserInfo({
    required String accessToken,
    DateTime? birthDate,
    int? heightInCm,
    Gender? gender,
    double? activityLevel,
  }) async {
    final Map<String, dynamic> body = {};
    if (birthDate != null) body['birth_date'] = _formatDate(birthDate);
    if (heightInCm != null) body['height_in_cm'] = heightInCm;
    if (gender != null) body['gender'] = gender.toJson();
    if (activityLevel != null) {
      body['activity_level'] = _formatActivityLevel(activityLevel);
    }

    final response = await http.put(
      Uri.parse('$baseUrl/api/user/info'),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Token $accessToken',
      },
      body: jsonEncode(body),
    );

    if (response.statusCode == 200) {
      return UserAdditionalInfo.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 401) {
      throw ApiException('Unauthorized', statusCode: 401);
    } else if (response.statusCode == 404) {
      throw ApiException(
        'User info not found. Create it first.',
        statusCode: 404,
      );
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException(
        'Failed to update user info',
        statusCode: response.statusCode,
      );
    }
  }
}
