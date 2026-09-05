//! 量潮执行管理领域模型：List × Task × Outcome，执行者 Executor 为引用实体。
//!
//! 模型以 docs/specification（quanttide-specification-of-execution-management）
//! 为单一事实源。JSON 字段名与规范示例一一对应（snake_case），Rust 字段名
//! 天然 snake_case，无需 rename。

pub mod client;
pub use client::{ExecuteApiError, ExecuteClient, DEFAULT_BASE_URL};

use serde::{Deserialize, Serialize};

/// 任务优先级：urgent / high / medium / low。
pub const PRIORITIES: [&str; 4] = ["urgent", "high", "medium", "low"];

/// 任务状态（看板泳道）：created / in_progress / reviewing / done。
pub const STATUSES: [&str; 4] = ["created", "in_progress", "reviewing", "done"];

/// 执行者：任务的参与主体，负责人与复核人的引用目标。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Executor {
    pub id: String,
    pub label: String,
}

/// 清单：执行记忆的归属主体——一个清单就是一个次级法人。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub tasks: Vec<String>,
    #[serde(default)]
    pub outcomes: Vec<String>,
}

/// 任务：法人需要记住的一件执行。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub list_id: String,
    pub title: String,
    pub description: String,
    /// "urgent" | "high" | "medium" | "low"
    pub priority: String,
    pub owner_id: String,
    pub reviewer_id: String,
    #[serde(default)]
    pub started_at: Option<String>,
    #[serde(default)]
    pub due_at: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    /// "created" | "in_progress" | "reviewing" | "done"
    pub status: String,
    /// False = 已归档：不进看板、不再流转
    pub is_active: bool,
    #[serde(default)]
    pub outcome_ids: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 结果：任务产生的结果，承载一项执行的交付结论/产出。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    pub id: String,
    pub list_id: String,
    #[serde(default)]
    pub task_ids: Vec<String>,
    pub title: String,
    pub description: String,
}
