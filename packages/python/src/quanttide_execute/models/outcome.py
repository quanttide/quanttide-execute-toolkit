from pydantic import BaseModel, ConfigDict


class Outcome(BaseModel):
    """结果：任务产生的结果，承载一项执行的交付结论/产出。"""

    model_config = ConfigDict(frozen=True)

    id: str
    list_id: str
    task_ids: list[str] = []
    title: str
    description: str
