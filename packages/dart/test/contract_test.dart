import 'dart:convert';
import 'dart:io';

import 'package:test/test.dart';
import 'package:quanttide_execute/quanttide_execute.dart';

Map<String, dynamic> fixture(String name) =>
    jsonDecode(File('../../tests/fixtures/$name.json').readAsStringSync())
        as Map<String, dynamic>;

void main() {
  test('executor fixture', () {
    final e = Executor.fromJson(fixture('executor'));
    expect(e.label, '张三');
    final json = e.toJson();
    expect(Executor.fromJson(json).id, e.id);
  });

  test('list fixture', () {
    final l = ExecuteList.fromJson(fixture('list'));
    expect(l.name, '量潮数据');
    expect(l.tasks, contains('7c9e6679-7425-40de-944b-e07fc1f90ae7'));
    final json = l.toJson();
    expect(ExecuteList.fromJson(json).outcomes, l.outcomes);
  });

  test('task fixture', () {
    final t = Task.fromJson(fixture('task'));
    expect(t.status, 'in_progress');
    expect(t.priority, 'high');
    expect(t.isActive, isTrue);
    expect(t.completedAt, isNull);
    final json = t.toJson();
    expect(Task.fromJson(json).listId, t.listId);
  });

  test('outcome fixture', () {
    final o = Outcome.fromJson(fixture('outcome'));
    expect(o.title, '口径确认单验收通过');
    expect(o.listId, '5f0c9a1e-3d4b-4e8f-9a2c-1b7d6e5f4a3c');
    final json = o.toJson();
    expect(Outcome.fromJson(json).taskIds, o.taskIds);
  });
}
