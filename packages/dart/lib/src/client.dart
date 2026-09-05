/// 执行云 API 客户端：封装 docs/specification 定义的端点。
///
/// 方法语义与规格一致：GET 读取；POST 创建（id 服务端生成，201）；PUT 全量更新；DELETE 删除。
library;

import 'dart:convert';

import 'package:http/http.dart' as http;

import 'models.dart';

/// API 调用异常：携带 HTTP 状态码与服务端错误信息。
class ExecuteApiException implements Exception {
  const ExecuteApiException(this.statusCode, this.message);

  final int statusCode;
  final String message;

  @override
  String toString() => 'ExecuteApiException($statusCode): $message';
}

/// 执行云 API 客户端。
class ExecuteClient {
  ExecuteClient({required String baseUrl, http.Client? client})
    : baseUrl = baseUrl.replaceAll(RegExp(r'/\$'), ''),
      _client = client ?? http.Client();

  final String baseUrl;
  final http.Client _client;

  Future<dynamic> _request(String method, String path, {Object? body}) async {
    final uri = Uri.parse('$baseUrl$path');
    final streamed = await _client.send(
      http.Request(method, uri)
        ..headers.addAll(
          body != null ? {'Content-Type': 'application/json'} : {},
        )
        ..body = body != null ? jsonEncode(body) : '',
    );
    final response = await http.Response.fromStream(streamed);
    if (response.statusCode >= 400) {
      throw ExecuteApiException(response.statusCode, response.body);
    }
    if (response.statusCode == 204 || response.body.isEmpty) return null;
    return jsonDecode(response.body);
  }

  // ---- List ----

  Future<List<ExecuteList>> listLists() async {
    final data = await _request('GET', '/lists');
    final items = data is Map ? (data['lists'] as List? ?? []) : data as List;
    return items
        .map((item) => ExecuteList.fromJson(item as Map<String, dynamic>))
        .toList();
  }

  Future<ExecuteList> createList(String name) async {
    final data = await _request('POST', '/lists', body: {'name': name});
    return ExecuteList.fromJson(data as Map<String, dynamic>);
  }

  Future<void> updateList(String listId, String name) =>
      _request('PUT', '/lists/$listId', body: {'name': name});

  Future<void> deleteList(String listId) =>
      _request('DELETE', '/lists/$listId');

  // ---- Task ----

  Future<List<Task>> listTasks(String listId) async {
    final data = await _request('GET', '/lists/$listId/tasks');
    final items = data is Map ? (data['tasks'] as List? ?? []) : data as List;
    return items
        .map((item) => Task.fromJson(item as Map<String, dynamic>))
        .toList();
  }

  Future<Task> createTask(Task task) async {
    final json = task.toJson()..remove('id');
    final data = await _request(
      'POST',
      '/lists/${task.listId}/tasks',
      body: json,
    );
    return Task.fromJson(data as Map<String, dynamic>);
  }

  Future<void> updateTask(Task task) => _request(
    'PUT',
    '/lists/${task.listId}/tasks/${task.id}',
    body: task.toJson(),
  );

  Future<void> deleteTask(String listId, String taskId) =>
      _request('DELETE', '/lists/$listId/tasks/$taskId');

  // ---- Outcome ----

  Future<List<Outcome>> listOutcomes(String listId, {String? taskId}) async {
    final path = taskId != null
        ? '/lists/$listId/outcomes?task_id=$taskId'
        : '/lists/$listId/outcomes';
    final data = await _request('GET', path);
    final items = data is Map
        ? (data['outcomes'] as List? ?? [])
        : data as List;
    return items
        .map((item) => Outcome.fromJson(item as Map<String, dynamic>))
        .toList();
  }

  Future<Outcome> createOutcome(Outcome outcome) async {
    final json = outcome.toJson()..remove('id');
    final data = await _request(
      'POST',
      '/lists/${outcome.listId}/outcomes',
      body: json,
    );
    return Outcome.fromJson(data as Map<String, dynamic>);
  }

  Future<void> updateOutcome(Outcome outcome) => _request(
    'PUT',
    '/lists/${outcome.listId}/outcomes/${outcome.id}',
    body: outcome.toJson(),
  );

  Future<void> deleteOutcome(String listId, String outcomeId) =>
      _request('DELETE', '/lists/$listId/outcomes/$outcomeId');

  // ---- Executor ----

  Future<List<Executor>> listExecutors() async {
    final data = await _request('GET', '/executors');
    final items = data is Map
        ? (data['executors'] as List? ?? [])
        : data as List;
    return items
        .map((item) => Executor.fromJson(item as Map<String, dynamic>))
        .toList();
  }

  Future<Executor> createExecutor(String label) async {
    final data = await _request('POST', '/executors', body: {'label': label});
    return Executor.fromJson(data as Map<String, dynamic>);
  }

  Future<void> updateExecutor(String executorId, String label) =>
      _request('PUT', '/executors/$executorId', body: {'label': label});

  Future<void> deleteExecutor(String executorId) =>
      _request('DELETE', '/executors/$executorId');
}
