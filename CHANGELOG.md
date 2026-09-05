# CHANGELOG

## [0.1.0-alpha.1] - 2026-09-06

### Added

- 跨语言契约测试（tests/：executor / list / task / outcome Schema + Fixture，源自 docs/specification v0.2.0 的字段定义）
- `packages/go`：List × Task × Outcome × Executor（package quanttide_execute，契约测试全绿）
- `packages/typescript`：同构 TypeScript SDK + 契约测试（vitest，全绿）
- `packages/rust`：crate quanttide-execute（serde 模型 + 契约测试全绿）
- `packages/python`：quanttide-execute 包（pydantic 模型 + httpx API 客户端，契约测试全绿）
- 各语言包含 ExecuteClient API 客户端：封装规格定义的四组端点（POST 创建 / PUT 更新 / DELETE，id 服务端生成），支持注入 HTTP 客户端便于测试
- GitHub Actions：ci-go / ci-typescript / ci-rust / ci-dart / ci-python（按路径触发，对齐 course/learn-toolkit 惯例）
