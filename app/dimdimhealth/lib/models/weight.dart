import 'package:json_annotation/json_annotation.dart';

part 'weight.g.dart';

@JsonSerializable()
class UserWeight {
  final String id;
  @JsonKey(name: 'user_id')
  final String userId;
  @JsonKey(name: 'weight_in_kg')
  final double weightInKg;
  @JsonKey(name: 'recorded_at')
  final String recordedAt;
  @JsonKey(name: 'created_at')
  final String createdAt;

  UserWeight({
    required this.id,
    required this.userId,
    required this.weightInKg,
    required this.recordedAt,
    required this.createdAt,
  });

  factory UserWeight.fromJson(Map<String, dynamic> json) =>
      _$UserWeightFromJson(json);
  Map<String, dynamic> toJson() => _$UserWeightToJson(this);
}

@JsonSerializable()
class UserWeightInfos {
  @JsonKey(name: 'last_3_weights')
  final List<UserWeight> last3Weights;
  @JsonKey(name: 'average_weight')
  final double averageWeight;
  @JsonKey(name: 'number_of_weight_entries')
  final int numberOfWeightEntries;
  @JsonKey(name: 'average_weight_last_7_days')
  final double averageWeightLast7Days;
  @JsonKey(name: 'number_of_weight_entries_last_7_days')
  final int numberOfWeightEntriesLast7Days;
  @JsonKey(name: 'average_weight_last_30_days')
  final double averageWeightLast30Days;
  @JsonKey(name: 'number_of_weight_entries_last_30_days')
  final int numberOfWeightEntriesLast30Days;
  @JsonKey(name: 'max_weight')
  final double maxWeight;
  @JsonKey(name: 'max_weight_date')
  final String maxWeightDate;
  @JsonKey(name: 'min_weight')
  final double minWeight;
  @JsonKey(name: 'min_weight_date')
  final String minWeightDate;

  UserWeightInfos({
    required this.last3Weights,
    required this.averageWeight,
    required this.numberOfWeightEntries,
    required this.averageWeightLast7Days,
    required this.numberOfWeightEntriesLast7Days,
    required this.averageWeightLast30Days,
    required this.numberOfWeightEntriesLast30Days,
    required this.maxWeight,
    required this.maxWeightDate,
    required this.minWeight,
    required this.minWeightDate,
  });

  factory UserWeightInfos.fromJson(Map<String, dynamic> json) =>
      _$UserWeightInfosFromJson(json);
  Map<String, dynamic> toJson() => _$UserWeightInfosToJson(this);
}

@JsonSerializable()
class CreateWeightRequest {
  @JsonKey(name: 'weight_in_kg')
  final double weightInKg;
  @JsonKey(name: 'recorded_at')
  final String recordedAt;

  CreateWeightRequest({
    required this.weightInKg,
    required this.recordedAt,
  });

  factory CreateWeightRequest.fromJson(Map<String, dynamic> json) =>
      _$CreateWeightRequestFromJson(json);
  Map<String, dynamic> toJson() => _$CreateWeightRequestToJson(this);
}

@JsonSerializable()
class UpdateWeightRequest {
  @JsonKey(name: 'weight_in_kg')
  final double weightInKg;
  @JsonKey(name: 'recorded_at')
  final String recordedAt;

  UpdateWeightRequest({
    required this.weightInKg,
    required this.recordedAt,
  });

  factory UpdateWeightRequest.fromJson(Map<String, dynamic> json) =>
      _$UpdateWeightRequestFromJson(json);
  Map<String, dynamic> toJson() => _$UpdateWeightRequestToJson(this);
}
