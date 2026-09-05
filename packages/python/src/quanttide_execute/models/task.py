from pydantic import BaseModel, ConfigDict


class Task(BaseModel):
    """任务：法人需要记住的一件执行。"""

    model_config = ConfigDict(frozen=True)

    id: str
    list_id: str
    title: str
    description: str
    priority: str  # urgent / high / medium / low
    owner_id: str
    reviewer_id: str
    started_at: str | None = None
    due_at: str | None = None
    completed_at: str | None = None
    status: str  # created / in_progress / reviewing / done
    is_active: bool = True
    outcome_ids: list[str] = []
    created_at: str
    updated_at: str


class Outcome(BaseModel):
    """结果：任务产生的结果，承载一项执行的交付结论/产出。"""

    model_config = ConfigDict(frozen=True)

    id: str
    list_id: str
    task_ids: list[str] = []
    title: str
    description: str
