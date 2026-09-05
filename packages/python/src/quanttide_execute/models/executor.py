from pydantic import BaseModel, ConfigDict


class Executor(BaseModel):
    """执行者：任务的参与主体，负责人与复核人的引用目标。"""

    model_config = ConfigDict(frozen=True)

    id: str
    label: str
