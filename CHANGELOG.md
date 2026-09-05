# CHANGELOG

## [0.1.0-alpha.1] - 2026-09-06

### Added

- 跨语言契约测试（tests/：executor / list / task / outcome Schema + Fixture，源自 docs/specification v0.2.0 的字段定义）
- `packages/go`：List × Task × Outcome × Executor（package quanttide_execute，契约测试全绿）
- `packages/typescript`：同构 TypeScript SDK + 契约测试（vitest，全绿）
- `packages/rust`：crate quanttide-execute（serde 模型 + 契约测试全绿）
- `packages/dart`：quanttide_execute 包（契约测试全绿；清单实体类名 ExecuteList，避开 dart:core List）
