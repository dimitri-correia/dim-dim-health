import 'dart:convert';
import 'package:http/http.dart' as http;
import '../models/user.dart';
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
  }) async {
    final request = RegisterRequest(
      user: RegisterUserData(
        username: username,
        email: email,
        password: password,
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
      throw ApiException('User already exists with this email or username',
          statusCode: 409);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException('Registration failed', statusCode: response.statusCode);
    }
  }

  Future<LoginResponse> login({
    required String email,
    required String password,
  }) async {
    final request = LoginRequest(
      user: LoginUserData(
        email: email,
        password: password,
      ),
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

  Future<ForgotPasswordResponse> forgotPassword({
    required String email,
  }) async {
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

  Future<ResetPasswordResponse> resetPassword({
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
      return ResetPasswordResponse.fromJson(jsonDecode(response.body));
    } else if (response.statusCode == 404) {
      throw ApiException('Invalid or expired reset token', statusCode: 404);
    } else if (response.statusCode == 410) {
      throw ApiException('Reset token has expired', statusCode: 410);
    } else if (response.statusCode == 400) {
      final error = jsonDecode(response.body);
      throw ApiException(error['error'] ?? 'Invalid data', statusCode: 400);
    } else {
      throw ApiException('Password reset failed', statusCode: response.statusCode);
    }
  }
}
