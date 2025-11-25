import 'dart:convert';
import 'package:http/http.dart' as http;
import '../models/user.dart';
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
    final request = CreateWeightRequest(
      weightInKg: weightInKg,
      recordedAt: recordedAt.toUtc().toIso8601String(),
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
    final request = UpdateWeightRequest(
      weightInKg: weightInKg,
      recordedAt: recordedAt.toUtc().toIso8601String(),
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
}
