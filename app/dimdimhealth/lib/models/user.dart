import 'package:json_annotation/json_annotation.dart';

part 'user.g.dart';

@JsonSerializable()
class User {
  final String email;
  final String username;
  @JsonKey(name: 'email_verified')
  final bool emailVerified;

  User({
    required this.email,
    required this.username,
    required this.emailVerified,
  });

  factory User.fromJson(Map<String, dynamic> json) => _$UserFromJson(json);
  Map<String, dynamic> toJson() => _$UserToJson(this);
}

@JsonSerializable()
class LoginResponse {
  final User user;
  @JsonKey(name: 'access_token')
  final String accessToken;
  @JsonKey(name: 'refresh_token')
  final String refreshToken;

  LoginResponse({
    required this.user,
    required this.accessToken,
    required this.refreshToken,
  });

  factory LoginResponse.fromJson(Map<String, dynamic> json) =>
      _$LoginResponseFromJson(json);
  Map<String, dynamic> toJson() => _$LoginResponseToJson(this);
}

@JsonSerializable()
class RegisterRequest {
  final RegisterUserData user;

  RegisterRequest({required this.user});

  factory RegisterRequest.fromJson(Map<String, dynamic> json) =>
      _$RegisterRequestFromJson(json);
  Map<String, dynamic> toJson() => _$RegisterRequestToJson(this);
}

@JsonSerializable()
class RegisterUserData {
  final String username;
  final String email;
  final String password;

  RegisterUserData({
    required this.username,
    required this.email,
    required this.password,
  });

  factory RegisterUserData.fromJson(Map<String, dynamic> json) =>
      _$RegisterUserDataFromJson(json);
  Map<String, dynamic> toJson() => _$RegisterUserDataToJson(this);
}

@JsonSerializable()
class LoginRequest {
  final LoginUserData user;

  LoginRequest({required this.user});

  factory LoginRequest.fromJson(Map<String, dynamic> json) =>
      _$LoginRequestFromJson(json);
  Map<String, dynamic> toJson() => _$LoginRequestToJson(this);
}

@JsonSerializable()
class LoginUserData {
  final String email;
  final String password;

  LoginUserData({required this.email, required this.password});

  factory LoginUserData.fromJson(Map<String, dynamic> json) =>
      _$LoginUserDataFromJson(json);
  Map<String, dynamic> toJson() => _$LoginUserDataToJson(this);
}

@JsonSerializable()
class ForgotPasswordRequest {
  final String email;

  ForgotPasswordRequest({required this.email});

  factory ForgotPasswordRequest.fromJson(Map<String, dynamic> json) =>
      _$ForgotPasswordRequestFromJson(json);
  Map<String, dynamic> toJson() => _$ForgotPasswordRequestToJson(this);
}

@JsonSerializable()
class ForgotPasswordResponse {
  final String message;

  ForgotPasswordResponse({required this.message});

  factory ForgotPasswordResponse.fromJson(Map<String, dynamic> json) =>
      _$ForgotPasswordResponseFromJson(json);
  Map<String, dynamic> toJson() => _$ForgotPasswordResponseToJson(this);
}
