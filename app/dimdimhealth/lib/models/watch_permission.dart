import 'package:json_annotation/json_annotation.dart';

part 'watch_permission.g.dart';

/// User search result from the API
@JsonSerializable()
class UserSearchResult {
  final String id;
  final String username;

  UserSearchResult({required this.id, required this.username});

  factory UserSearchResult.fromJson(Map<String, dynamic> json) =>
      _$UserSearchResultFromJson(json);
  Map<String, dynamic> toJson() => _$UserSearchResultToJson(this);
}

/// Response for user search
@JsonSerializable()
class SearchUsersResponse {
  final List<UserSearchResult> users;

  SearchUsersResponse({required this.users});

  factory SearchUsersResponse.fromJson(Map<String, dynamic> json) =>
      _$SearchUsersResponseFromJson(json);
  Map<String, dynamic> toJson() => _$SearchUsersResponseToJson(this);
}

/// User with watch permission
@JsonSerializable()
class WatchPermissionUser {
  @JsonKey(name: 'user_id')
  final String userId;
  final String username;

  WatchPermissionUser({required this.userId, required this.username});

  factory WatchPermissionUser.fromJson(Map<String, dynamic> json) =>
      _$WatchPermissionUserFromJson(json);
  Map<String, dynamic> toJson() => _$WatchPermissionUserToJson(this);
}

/// Response for getting watchers (users I allow to watch me)
@JsonSerializable()
class WatchersResponse {
  final List<WatchPermissionUser> watchers;

  WatchersResponse({required this.watchers});

  factory WatchersResponse.fromJson(Map<String, dynamic> json) =>
      _$WatchersResponseFromJson(json);
  Map<String, dynamic> toJson() => _$WatchersResponseToJson(this);
}

/// Response for getting watching (users that allow me to watch them)
@JsonSerializable()
class WatchingResponse {
  final List<WatchPermissionUser> watching;

  WatchingResponse({required this.watching});

  factory WatchingResponse.fromJson(Map<String, dynamic> json) =>
      _$WatchingResponseFromJson(json);
  Map<String, dynamic> toJson() => _$WatchingResponseToJson(this);
}

/// Request to grant watch permission
@JsonSerializable()
class GrantWatchPermissionRequest {
  @JsonKey(name: 'user_id')
  final String userId;

  GrantWatchPermissionRequest({required this.userId});

  factory GrantWatchPermissionRequest.fromJson(Map<String, dynamic> json) =>
      _$GrantWatchPermissionRequestFromJson(json);
  Map<String, dynamic> toJson() => _$GrantWatchPermissionRequestToJson(this);
}

/// Request to revoke watch permission
@JsonSerializable()
class RevokeWatchPermissionRequest {
  @JsonKey(name: 'user_id')
  final String userId;

  RevokeWatchPermissionRequest({required this.userId});

  factory RevokeWatchPermissionRequest.fromJson(Map<String, dynamic> json) =>
      _$RevokeWatchPermissionRequestFromJson(json);
  Map<String, dynamic> toJson() => _$RevokeWatchPermissionRequestToJson(this);
}
