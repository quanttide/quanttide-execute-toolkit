// Package quanttide_execute 量潮执行管理领域模型：List × Task × Outcome，执行者 Executor 为引用实体。
//
// 模型以 docs/specification（quanttide-specification-of-execution-management）
// 为单一事实源。JSON 标签与规范示例一一对应（snake_case）。
package quanttide_execute

// 任务优先级与状态取值（规格定义）。
const (
	PriorityUrgent = "urgent"
	PriorityHigh   = "high"
	PriorityMedium = "medium"
	PriorityLow    = "low"

	StatusCreated    = "created"
	StatusInProgress = "in_progress"
	StatusReviewing  = "reviewing"
	StatusDone       = "done"
)

// Executor 执行者：任务的参与主体，负责人与复核人的引用目标。
type Executor struct {
	ID    string `json:"id"`
	Label string `json:"label"`
}

// List 清单：执行记忆的归属主体——一个清单就是一个次级法人。
type List struct {
	ID       string   `json:"id"`
	Name     string   `json:"name"`
	Tasks    []string `json:"tasks,omitempty"`
	Outcomes []string `json:"outcomes,omitempty"`
}

// Task 任务：法人需要记住的一件执行。
type Task struct {
	ID          string   `json:"id"`
	ListID      string   `json:"list_id"`
	Title       string   `json:"title"`
	Description string   `json:"description"`
	Priority    string   `json:"priority"` // urgent / high / medium / low
	OwnerID     string   `json:"owner_id"`
	ReviewerID  string   `json:"reviewer_id"`
	StartedAt   *string  `json:"started_at"`
	DueAt       *string  `json:"due_at"`
	CompletedAt *string  `json:"completed_at"`
	Status      string   `json:"status"` // created / in_progress / reviewing / done
	IsActive    bool     `json:"is_active"`
	OutcomeIDs  []string `json:"outcome_ids,omitempty"`
	CreatedAt   string   `json:"created_at"`
	UpdatedAt   string   `json:"updated_at"`
}

// Outcome 结果：任务产生的结果，承载一项执行的交付结论/产出。
type Outcome struct {
	ID          string   `json:"id"`
	ListID      string   `json:"list_id"`
	TaskIDs     []string `json:"task_ids,omitempty"`
	Title       string   `json:"title"`
	Description string   `json:"description"`
}
