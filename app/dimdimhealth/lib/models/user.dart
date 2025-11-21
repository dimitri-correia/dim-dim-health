import 'user_profile_image.dart';

class User {
  final String email;
  final String username;
  final bool emailVerified;
  final UserProfileImage profileImage;

  User({
    required this.email,
    required this.username,
    required this.emailVerified,
    required this.profileImage,
  });

  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      email: json['email'] as String,
      username: json['username'] as String,
      emailVerified: json['email_verified'] as bool,
      profileImage: UserProfileImageExtension.fromString(
        json['profile_image'] as String? ?? 'avatar1',
      ),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'email': email,
      'username': username,
      'email_verified': emailVerified,
      'profile_image': profileImage.toJson(),
    };
  }
}

class LoginResponse {
  final User user;
  final String accessToken;
  final String refreshToken;

  LoginResponse({
    required this.user,
    required this.accessToken,
    required this.refreshToken,
  });

  factory LoginResponse.fromJson(Map<String, dynamic> json) {
    return LoginResponse(
      user: User.fromJson(json['user'] as Map<String, dynamic>),
      accessToken: json['access_token'] as String,
      refreshToken: json['refresh_token'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'user': user.toJson(),
      'access_token': accessToken,
      'refresh_token': refreshToken,
    };
  }
}

class RegisterRequest {
  final RegisterUserData user;

  RegisterRequest({required this.user});

  factory RegisterRequest.fromJson(Map<String, dynamic> json) {
    return RegisterRequest(
      user: RegisterUserData.fromJson(json['user'] as Map<String, dynamic>),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'user': user.toJson(),
    };
  }
}

class RegisterUserData {
  final String username;
  final String email;
  final String password;
  final String? profileImage;

  RegisterUserData({
    required this.username,
    required this.email,
    required this.password,
    this.profileImage,
  });

  factory RegisterUserData.fromJson(Map<String, dynamic> json) {
    return RegisterUserData(
      username: json['username'] as String,
      email: json['email'] as String,
      password: json['password'] as String,
      profileImage: json['profile_image'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    final data = {
      'username': username,
      'email': email,
      'password': password,
    };
    if (profileImage != null) {
      data['profile_image'] = profileImage!;
    }
    return data;
  }
}

class LoginRequest {
  final LoginUserData user;

  LoginRequest({required this.user});

  factory LoginRequest.fromJson(Map<String, dynamic> json) {
    return LoginRequest(
      user: LoginUserData.fromJson(json['user'] as Map<String, dynamic>),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'user': user.toJson(),
    };
  }
}

class LoginUserData {
  final String email;
  final String password;

  LoginUserData({required this.email, required this.password});

  factory LoginUserData.fromJson(Map<String, dynamic> json) {
    return LoginUserData(
      email: json['email'] as String,
      password: json['password'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'email': email,
      'password': password,
    };
  }
}

class ForgotPasswordRequest {
  final String email;

  ForgotPasswordRequest({required this.email});

  factory ForgotPasswordRequest.fromJson(Map<String, dynamic> json) {
    return ForgotPasswordRequest(
      email: json['email'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'email': email,
    };
  }
}

class ForgotPasswordResponse {
  final String message;

  ForgotPasswordResponse({required this.message});

  factory ForgotPasswordResponse.fromJson(Map<String, dynamic> json) {
    return ForgotPasswordResponse(
      message: json['message'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'message': message,
    };
  }
}

class UpdateSettingsRequest {
  final String? username;
  final String? email;
  final String? profileImage;
  final PasswordChange? passwords;

  UpdateSettingsRequest({
    this.username,
    this.email,
    this.profileImage,
    this.passwords,
  });

  Map<String, dynamic> toJson() {
    final data = <String, dynamic>{};
    if (username != null) data['username'] = username;
    if (email != null) data['email'] = email;
    if (profileImage != null) data['profile_image'] = profileImage;
    if (passwords != null) data['passwords'] = passwords!.toJson();
    return data;
  }
}

class PasswordChange {
  final String currentPassword;
  final String newPassword;

  PasswordChange({
    required this.currentPassword,
    required this.newPassword,
  });

  Map<String, dynamic> toJson() {
    return {
      'current_password': currentPassword,
      'new_password': newPassword,
    };
  }
}

class UpdateSettingsResponse {
  final String message;

  UpdateSettingsResponse({required this.message});

  factory UpdateSettingsResponse.fromJson(Map<String, dynamic> json) {
    return UpdateSettingsResponse(
      message: json['message'] as String,
    );
  }
}
