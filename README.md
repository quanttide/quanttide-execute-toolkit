# quanttide-execute-toolkit

量潮执行管理标准库 — 执行管理领域共享库与标准库。

## 概述

封装执行管理领域模型（List × Task × Outcome，执行者 Executor 为引用实体）的跨语言 SDK，供各应用与工具复用，保证系统一致性。

- 模型事实源：`docs/specification`（quanttide-specification-of-execution-management）v0.2.0
- 语言包：`packages/go`、`packages/typescript`、`packages/rust`、`packages/dart`、`packages/python`（均含数据模型 + API 客户端）
- 契约测试：`tests/`（Schema + Fixture 为跨语言单一依据）

## 各包状态

| 包 | 状态 |
|---|------|
| `go/` | ✅ 已对齐 |
| `typescript/` | ✅ 已对齐 |
| `rust/` | ✅ 已对齐 |
| `dart/` | ✅ 已对齐 |
| `python/` | ✅ 已对齐 |

## 运行契约测试

```bash
cd packages/go && go test ./... -count=1
cd packages/typescript && npm test
cd packages/rust && cargo test
cd packages/dart && dart test
cd packages/python && uv run pytest
```

## 许可

[CC BY 4.0](LICENSE)
