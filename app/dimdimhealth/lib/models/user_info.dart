import 'package:json_annotation/json_annotation.dart';

part 'user_info.g.dart';

double _doubleFromJson(dynamic value) {
  if (value is num) {
    return value.toDouble();
  } else if (value is String) {
    return double.parse(value);
  }
  throw ArgumentError('Cannot convert $value to double');
}

enum Gender {
  male,
  female,
  other;

  String toJson() => name;

  static Gender fromJson(String json) {
    return Gender.values.firstWhere((e) => e.name == json);
  }
}

@JsonSerializable()
class UserAdditionalInfo {
  @JsonKey(name: 'user_id')
  final String userId;
  @JsonKey(name: 'birth_date')
  final String birthDate;
  @JsonKey(name: 'height_in_cm')
  final int heightInCm;
  final Gender gender;
  @JsonKey(name: 'activity_level')
  final double activityLevel;
  @JsonKey(name: 'updated_at')
  final String updatedAt;

  UserAdditionalInfo({
    required this.userId,
    required this.birthDate,
    required this.heightInCm,
    required this.gender,
    required this.activityLevel,
    required this.updatedAt,
  });

  factory UserAdditionalInfo.fromJson(Map<String, dynamic> json) {
    return UserAdditionalInfo(
      userId: json['user_id'] as String,
      birthDate: json['birth_date'] as String,
      heightInCm: json['height_in_cm'] as int,
      gender: Gender.fromJson(json['gender'] as String),
      activityLevel: _doubleFromJson(json['activity_level']),
      updatedAt: json['updated_at'] as String,
    );
  }

  Map<String, dynamic> toJson() => {
        'user_id': userId,
        'birth_date': birthDate,
        'height_in_cm': heightInCm,
        'gender': gender.toJson(),
        'activity_level': activityLevel,
        'updated_at': updatedAt,
      };
}

@JsonSerializable()
class CreateUserInfoRequest {
  @JsonKey(name: 'birth_date')
  final String birthDate;
  @JsonKey(name: 'height_in_cm')
  final int heightInCm;
  final Gender gender;
  @JsonKey(name: 'activity_level')
  final String activityLevel;

  CreateUserInfoRequest({
    required this.birthDate,
    required this.heightInCm,
    required this.gender,
    required this.activityLevel,
  });

  factory CreateUserInfoRequest.fromJson(Map<String, dynamic> json) {
    return CreateUserInfoRequest(
      birthDate: json['birth_date'] as String,
      heightInCm: json['height_in_cm'] as int,
      gender: Gender.fromJson(json['gender'] as String),
      activityLevel: json['activity_level'] as String,
    );
  }

  Map<String, dynamic> toJson() => {
        'birth_date': birthDate,
        'height_in_cm': heightInCm,
        'gender': gender.toJson(),
        'activity_level': activityLevel,
      };
}

@JsonSerializable()
class UpdateUserInfoRequest {
  @JsonKey(name: 'birth_date')
  final String? birthDate;
  @JsonKey(name: 'height_in_cm')
  final int? heightInCm;
  final Gender? gender;
  @JsonKey(name: 'activity_level')
  final String? activityLevel;

  UpdateUserInfoRequest({
    this.birthDate,
    this.heightInCm,
    this.gender,
    this.activityLevel,
  });

  factory UpdateUserInfoRequest.fromJson(Map<String, dynamic> json) {
    return UpdateUserInfoRequest(
      birthDate: json['birth_date'] as String?,
      heightInCm: json['height_in_cm'] as int?,
      gender: json['gender'] != null
          ? Gender.fromJson(json['gender'] as String)
          : null,
      activityLevel: json['activity_level'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> json = {};
    if (birthDate != null) json['birth_date'] = birthDate;
    if (heightInCm != null) json['height_in_cm'] = heightInCm;
    if (gender != null) json['gender'] = gender!.toJson();
    if (activityLevel != null) json['activity_level'] = activityLevel;
    return json;
  }
}
