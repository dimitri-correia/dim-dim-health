/// Enum representing muscle groups
enum Muscle {
  chest,
  back,
  shoulders,
  biceps,
  triceps,
  forearms,
  quadriceps,
  hamstrings,
  glutes,
  calves,
  abs,
  obliques,
  traps,
  lats,
  lowerBack,
}

extension MuscleExtension on Muscle {
  String get displayName {
    switch (this) {
      case Muscle.chest:
        return 'Chest';
      case Muscle.back:
        return 'Back';
      case Muscle.shoulders:
        return 'Shoulders';
      case Muscle.biceps:
        return 'Biceps';
      case Muscle.triceps:
        return 'Triceps';
      case Muscle.forearms:
        return 'Forearms';
      case Muscle.quadriceps:
        return 'Quadriceps';
      case Muscle.hamstrings:
        return 'Hamstrings';
      case Muscle.glutes:
        return 'Glutes';
      case Muscle.calves:
        return 'Calves';
      case Muscle.abs:
        return 'Abs';
      case Muscle.obliques:
        return 'Obliques';
      case Muscle.traps:
        return 'Traps';
      case Muscle.lats:
        return 'Lats';
      case Muscle.lowerBack:
        return 'Lower Back';
    }
  }

  String get value {
    switch (this) {
      case Muscle.chest:
        return 'chest';
      case Muscle.back:
        return 'back';
      case Muscle.shoulders:
        return 'shoulders';
      case Muscle.biceps:
        return 'biceps';
      case Muscle.triceps:
        return 'triceps';
      case Muscle.forearms:
        return 'forearms';
      case Muscle.quadriceps:
        return 'quadriceps';
      case Muscle.hamstrings:
        return 'hamstrings';
      case Muscle.glutes:
        return 'glutes';
      case Muscle.calves:
        return 'calves';
      case Muscle.abs:
        return 'abs';
      case Muscle.obliques:
        return 'obliques';
      case Muscle.traps:
        return 'traps';
      case Muscle.lats:
        return 'lats';
      case Muscle.lowerBack:
        return 'lower_back';
    }
  }

  static Muscle fromString(String value) {
    switch (value.toLowerCase()) {
      case 'chest':
        return Muscle.chest;
      case 'back':
        return Muscle.back;
      case 'shoulders':
        return Muscle.shoulders;
      case 'biceps':
        return Muscle.biceps;
      case 'triceps':
        return Muscle.triceps;
      case 'forearms':
        return Muscle.forearms;
      case 'quadriceps':
        return Muscle.quadriceps;
      case 'hamstrings':
        return Muscle.hamstrings;
      case 'glutes':
        return Muscle.glutes;
      case 'calves':
        return Muscle.calves;
      case 'abs':
        return Muscle.abs;
      case 'obliques':
        return Muscle.obliques;
      case 'traps':
        return Muscle.traps;
      case 'lats':
        return Muscle.lats;
      case 'lower_back':
        return Muscle.lowerBack;
      default:
        throw ArgumentError('Unknown muscle: $value');
    }
  }
}

class GymExercise {
  final String id;
  final String name;
  final String? description;
  final List<Muscle> primaryMuscles;
  final List<Muscle> secondaryMuscles;

  GymExercise({
    required this.id,
    required this.name,
    this.description,
    required this.primaryMuscles,
    required this.secondaryMuscles,
  });

  factory GymExercise.fromJson(Map<String, dynamic> json) {
    return GymExercise(
      id: json['id'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
      primaryMuscles: (json['primary_muscles'] as List<dynamic>)
          .map((m) => MuscleExtension.fromString(m as String))
          .toList(),
      secondaryMuscles: (json['secondary_muscles'] as List<dynamic>)
          .map((m) => MuscleExtension.fromString(m as String))
          .toList(),
    );
  }

  Map<String, dynamic> toJson() => {
        'id': id,
        'name': name,
        'description': description,
        'primary_muscles': primaryMuscles.map((m) => m.value).toList(),
        'secondary_muscles': secondaryMuscles.map((m) => m.value).toList(),
      };
}

class GymSession {
  final String id;
  final String userId;
  final String date;

  GymSession({
    required this.id,
    required this.userId,
    required this.date,
  });

  factory GymSession.fromJson(Map<String, dynamic> json) {
    return GymSession(
      id: json['id'] as String,
      userId: json['user_id'] as String,
      date: json['date'] as String,
    );
  }

  Map<String, dynamic> toJson() => {
        'id': id,
        'user_id': userId,
        'date': date,
      };
}

class GymSet {
  final String id;
  final String sessionId;
  final String exerciseId;
  final int setNumber;
  final int repetitions;
  final String weightKg;

  GymSet({
    required this.id,
    required this.sessionId,
    required this.exerciseId,
    required this.setNumber,
    required this.repetitions,
    required this.weightKg,
  });

  factory GymSet.fromJson(Map<String, dynamic> json) {
    return GymSet(
      id: json['id'] as String,
      sessionId: json['session_id'] as String,
      exerciseId: json['exercise_id'] as String,
      setNumber: json['set_number'] as int,
      repetitions: json['repetitions'] as int,
      weightKg: json['weight_kg'].toString(),
    );
  }

  Map<String, dynamic> toJson() => {
        'id': id,
        'session_id': sessionId,
        'exercise_id': exerciseId,
        'set_number': setNumber,
        'repetitions': repetitions,
        'weight_kg': weightKg,
      };

  double get weightKgDouble => double.tryParse(weightKg) ?? 0.0;
}

class CreateGymExerciseRequest {
  final String name;
  final String? description;
  final List<String> primaryMuscles;
  final List<String> secondaryMuscles;

  CreateGymExerciseRequest({
    required this.name,
    this.description,
    required this.primaryMuscles,
    required this.secondaryMuscles,
  });

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = {
      'name': name,
      'primary_muscles': primaryMuscles,
      'secondary_muscles': secondaryMuscles,
    };
    if (description != null) {
      data['description'] = description;
    }
    return data;
  }
}

class CreateGymSessionRequest {
  final String date;

  CreateGymSessionRequest({required this.date});

  Map<String, dynamic> toJson() => {'date': date};
}

class CreateGymSetRequest {
  final String exerciseId;
  final int setNumber;
  final int repetitions;
  final String weightKg;

  CreateGymSetRequest({
    required this.exerciseId,
    required this.setNumber,
    required this.repetitions,
    required this.weightKg,
  });

  Map<String, dynamic> toJson() => {
        'exercise_id': exerciseId,
        'set_number': setNumber,
        'repetitions': repetitions,
        'weight_kg': weightKg,
      };
}
