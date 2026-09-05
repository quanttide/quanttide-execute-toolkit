from typing import Any

from pydantic import BaseModel, ConfigDict, field_validator


class ExecuteList(BaseModel):
    """清单：执行记忆的归属主体——一个清单就是一个次级法人。

    Python 类名 ExecuteList 避开内置 list；JSON 字段名不变。
    """

    model_config = ConfigDict(frozen=True)

    id: str
    name: str
    tasks: list[str] = []
    outcomes: list[str] = []

    @field_validator("tasks", "outcomes", mode="before")
    @classmethod
    def _coerce_ids(cls, v: Any) -> list[str]:
        if v is None:
            return []
        if isinstance(v, list):
            return [str(item) for item in v]
        return []
