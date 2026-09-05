// 量潮执行管理领域模型：List × Task × Outcome，执行者 Executor 为引用实体。
// 模型以 docs/specification（quanttide-specification-of-execution-management）
// 为单一事实源。字段名与规范 JSON 示例一一对应（snake_case）。

export const PRIORITIES = ["urgent", "high", "medium", "low"] as const;
export type Priority = (typeof PRIORITIES)[number];

export const STATUSES = ["created", "in_progress", "reviewing", "done"] as const;
export type Status = (typeof STATUSES)[number];

/** 执行者：任务的参与主体，负责人与复核人的引用目标。 */
export interface Executor {
  id: string;
  label: string;
}

/** 清单：执行记忆的归属主体——一个清单就是一个次级法人。 */
export interface List {
  id: string;
  name: string;
  tasks?: string[];
  outcomes?: string[];
}

/** 任务：法人需要记住的一件执行。 */
export interface Task {
  id: string;
  list_id: string;
  title: string;
  description: string;
  priority: Priority;
  owner_id: string;
  reviewer_id: string;
  started_at?: string | null;
  due_at?: string | null;
  completed_at?: string | null;
  status: Status;
  /** false = 已归档：不进看板、不再流转 */
  is_active: boolean;
  outcome_ids?: string[];
  created_at: string;
  updated_at: string;
}

/** 结果：任务产生的结果，承载一项执行的交付结论/产出。 */
export interface Outcome {
  id: string;
  list_id: string;
  task_ids?: string[];
  title: string;
  description: string;
}
