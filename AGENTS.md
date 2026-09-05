# AGENTS.md - quanttide-execute-toolkit

## 定位

量潮执行管理工具箱：封装执行管理领域模型（List × Task × Outcome，Executor 引用实体）的跨语言 SDK。

## 模型事实源

**docs/specification（quanttide-specification-of-execution-management）是领域模型单一事实源**，各语言包模型与 JSON 标签必须与其一一对应：

- 实体：List（清单/次级法人，聚合根）× Task（任务）× Outcome（结果），Executor（执行者）为引用实体
- JSON 标签：snake_case，与规范 JSON 示例一致（`list_id` / `owner_id` / `created_at`），不做 case 转换层
- 语义约束随模型携带：id 由服务端生成（uuid）；Task 必含 owner_id / reviewer_id；status 仅 created / in_progress / reviewing / done；priority 仅 urgent / high / medium / low；is_active false 表示归档

## 契约测试

- 根 `tests/` 的 schemas/ + fixtures/ 是跨语言契约的客观依据（变更先行：先改规范 → 同步 Schema/Fixture → 各语言跟进）
- 语言包升级前契约测试必须全绿；未对齐只允许发 alpha 版本
- 对外引用统一命名 `quanttide-execute`（Dart 包 quanttide_execute / TS、Rust、Go import 引用同此）

## 提交规则

1. 本仓库作为子模块挂载于 quanttide-execute 的 `packages/quanttide-execute-toolkit`：本仓库 commit → 回父仓库更新指针 commit → 根仓库 quanttide 同法更新。
2. 提交信息用 Conventional Commits（`feat`/`fix`/`docs`/`chore`）。
