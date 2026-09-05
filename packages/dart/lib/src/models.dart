/// 量潮执行管理领域模型：List × Task × Outcome，执行者 Executor 为引用实体。
///
/// 模型以 docs/specification（quanttide-specification-of-execution-management）
/// 为单一事实源。JSON 字段名与规范示例一一对应（snake_case）。
library;

/// 任务优先级取值。
const priorities = ['urgent', 'high', 'medium', 'low'];

/// 任务状态取值（看板泳道）。
const statuses = ['created', 'in_progress', 'reviewing', 'done'];

/// 执行者：任务的参与主体，负责人与复核人的引用目标。
class Executor {
  final String id;
  final String label;

  Executor({required this.id, required this.label});

  factory Executor.fromJson(Map<String, dynamic> json) =>
      Executor(id: json['id'] as String, label: json['label'] as String);

  Map<String, dynamic> toJson() => {'id': id, 'label': label};
}

/// 清单：执行记忆的归属主体——一个清单就是一个次级法人。
class ExecuteList {
  final String id;
  final String name;
  final List<String> tasks;
  final List<String> outcomes;

  ExecuteList({
    required this.id,
    required this.name,
    this.tasks = const [],
    this.outcomes = const [],
  });

  factory ExecuteList.fromJson(Map<String, dynamic> json) => ExecuteList(
    id: json['id'] as String,
    name: json['name'] as String,
    tasks: (json['tasks'] as List<dynamic>? ?? []).cast<String>(),
    outcomes: (json['outcomes'] as List<dynamic>? ?? []).cast<String>(),
  );

  Map<String, dynamic> toJson() => {
    'id': id,
    'name': name,
    'tasks': tasks,
    'outcomes': outcomes,
  };
}

/// 任务：法人需要记住的一件执行。
class Task {
  final String id;
  final String listId;
  final String title;
  final String description;

  /// urgent / high / medium / low
  final String priority;
  final String ownerId;
  final String reviewerId;
  final String? startedAt;
  final String? dueAt;
  final String? completedAt;

  /// created / in_progress / reviewing / done
  final String status;

  /// false = 已归档：不进看板、不再流转
  final bool isActive;
  final List<String> outcomeIds;
  final String createdAt;
  final String updatedAt;

  Task({
    required this.id,
    required this.listId,
    required this.title,
    required this.description,
    required this.priority,
    required this.ownerId,
    required this.reviewerId,
    this.startedAt,
    this.dueAt,
    this.completedAt,
    required this.status,
    this.isActive = true,
    this.outcomeIds = const [],
    required this.createdAt,
    required this.updatedAt,
  });

  factory Task.fromJson(Map<String, dynamic> json) => Task(
    id: json['id'] as String,
    listId: json['list_id'] as String,
    title: json['title'] as String,
    description: json['description'] as String,
    priority: json['priority'] as String,
    ownerId: json['owner_id'] as String,
    reviewerId: json['reviewer_id'] as String,
    startedAt: json['started_at'] as String?,
    dueAt: json['due_at'] as String?,
    completedAt: json['completed_at'] as String?,
    status: json['status'] as String,
    isActive: json['is_active'] as bool? ?? true,
    outcomeIds: (json['outcome_ids'] as List<dynamic>? ?? []).cast<String>(),
    createdAt: json['created_at'] as String,
    updatedAt: json['updated_at'] as String,
  );

  Map<String, dynamic> toJson() => {
    'id': id,
    'list_id': listId,
    'title': title,
    'description': description,
    'priority': priority,
    'owner_id': ownerId,
    'reviewer_id': reviewerId,
    'started_at': startedAt,
    'due_at': dueAt,
    'completed_at': completedAt,
    'status': status,
    'is_active': isActive,
    'outcome_ids': outcomeIds,
    'created_at': createdAt,
    'updated_at': updatedAt,
  };
}

/// 结果：任务产生的结果，承载一项执行的交付结论/产出。
class Outcome {
  final String id;
  final String listId;
  final List<String> taskIds;
  final String title;
  final String description;

  Outcome({
    required this.id,
    required this.listId,
    this.taskIds = const [],
    required this.title,
    required this.description,
  });

  factory Outcome.fromJson(Map<String, dynamic> json) => Outcome(
    id: json['id'] as String,
    listId: json['list_id'] as String,
    taskIds: (json['task_ids'] as List<dynamic>? ?? []).cast<String>(),
    title: json['title'] as String,
    description: json['description'] as String,
  );

  Map<String, dynamic> toJson() => {
    'id': id,
    'list_id': listId,
    'task_ids': taskIds,
    'title': title,
    'description': description,
  };
}
