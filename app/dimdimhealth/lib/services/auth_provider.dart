import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import '../models/user.dart';
import '../services/api_service.dart';

class AuthProvider with ChangeNotifier {
  final ApiService _apiService = ApiService();
  final FlutterSecureStorage _storage = const FlutterSecureStorage();

  User? _user;
  String? _accessToken;
  String? _refreshToken;
  bool _isLoading = false;
  String? _error;

  User? get user => _user;
  String? get accessToken => _accessToken;
  bool get isAuthenticated => _user != null && _accessToken != null;
  bool get isLoading => _isLoading;
  String? get error => _error;

  Future<void> loadSavedAuth() async {
    try {
      _accessToken = await _storage.read(key: 'access_token');
      _refreshToken = await _storage.read(key: 'refresh_token');
      final userJson = await _storage.read(key: 'user');
      if (userJson != null && _accessToken != null) {
        // We have stored credentials, consider user authenticated
        // In a real app, you'd want to validate the token
        notifyListeners();
      }
    } catch (e) {
      // Ignore errors loading saved auth
    }
  }

  Future<bool> register({
    required String username,
    required String email,
    required String password,
    String? profileImage,
  }) async {
    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      final response = await _apiService.register(
        username: username,
        email: email,
        password: password,
        profileImage: profileImage,
      );

      _user = response.user;
      _accessToken = response.accessToken;
      _refreshToken = response.refreshToken;

      await _saveAuth();

      _isLoading = false;
      notifyListeners();
      return true;
    } on ApiException catch (e) {
      _error = e.message;
      _isLoading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Network error. Please check your connection.';
      _isLoading = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> login({required String email, required String password}) async {
    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      final response = await _apiService.login(
        email: email,
        password: password,
      );

      _user = response.user;
      _accessToken = response.accessToken;
      _refreshToken = response.refreshToken;

      await _saveAuth();

      _isLoading = false;
      notifyListeners();
      return true;
    } on ApiException catch (e) {
      _error = e.message;
      _isLoading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Network error. Please check your connection.';
      _isLoading = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> forgotPassword(String email) async {
    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      await _apiService.forgotPassword(email: email);
      _isLoading = false;
      notifyListeners();
      return true;
    } on ApiException catch (e) {
      _error = e.message;
      _isLoading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Network error. Please check your connection.';
      _isLoading = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> resetPassword({
    required String token,
    required String newPassword,
  }) async {
    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      final response = await _apiService.resetPassword(
        token: token,
        newPassword: newPassword,
      );

      // Auto-login after successful password reset
      _user = response.user;
      _accessToken = response.accessToken;
      _refreshToken = response.refreshToken;

      await _saveAuth();

      _isLoading = false;
      notifyListeners();
      return true;
    } on ApiException catch (e) {
      _error = e.message;
      _isLoading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Network error. Please check your connection.';
      _isLoading = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> loginAsGuest() async {
    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      final response = await _apiService.loginAsGuest();

      _user = response.user;
      _accessToken = response.accessToken;
      _refreshToken = response.refreshToken;

      await _saveAuth();

      _isLoading = false;
      notifyListeners();
      return true;
    } on ApiException catch (e) {
      _error = e.message;
      _isLoading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Network error. Please check your connection.';
      _isLoading = false;
      notifyListeners();
      return false;
    }
  }

  Future<void> logout() async {
    _user = null;
    _accessToken = null;
    _refreshToken = null;
    await _storage.deleteAll();
    notifyListeners();
  }

  Future<bool> refreshUser() async {
    if (_accessToken == null) {
      return false;
    }

    try {
      final updatedUser = await _apiService.getCurrentUser(_accessToken!);
      _user = updatedUser;

      // Save updated user to storage
      await _storage.write(key: 'user', value: updatedUser.toJson().toString());

      notifyListeners();
      return true;
    } catch (e) {
      // If refresh fails, don't update anything
      return false;
    }
  }

  Future<String?> updateSettings({
    String? username,
    String? email,
    String? profileImage,
    String? currentPassword,
    String? newPassword,
  }) async {
    if (_accessToken == null) {
      return 'Not authenticated';
    }

    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      final message = await _apiService.updateSettings(
        accessToken: _accessToken!,
        username: username,
        email: email,
        profileImage: profileImage,
        currentPassword: currentPassword,
        newPassword: newPassword,
      );

      // Refresh user data after successful update
      await refreshUser();

      _isLoading = false;
      notifyListeners();
      return message;
    } on ApiException catch (e) {
      _error = e.message;
      _isLoading = false;
      notifyListeners();
      return null;
    } catch (e) {
      _error = 'Network error. Please check your connection.';
      _isLoading = false;
      notifyListeners();
      return null;
    }
  }

  Future<void> _saveAuth() async {
    if (_accessToken != null) {
      await _storage.write(key: 'access_token', value: _accessToken);
    }
    if (_refreshToken != null) {
      await _storage.write(key: 'refresh_token', value: _refreshToken);
    }
  }

  Future<bool> verifyEmail({required String token}) async {
    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      await _apiService.verifyEmail(token: token);
      _isLoading = false;
      notifyListeners();
      return true;
    } on ApiException catch (e) {
      _error = e.message;
      _isLoading = false;
      notifyListeners();
      return false;
    } catch (e) {
      _error = 'Network error. Please check your connection.';
      _isLoading = false;
      notifyListeners();
      return false;
    }
  }

  void clearError() {
    _error = null;
    notifyListeners();
  }
}
