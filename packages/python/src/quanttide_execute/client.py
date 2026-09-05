"""执行云 API 客户端：封装 docs/specification 定义的端点。

方法语义与规格一致：GET 读取；POST 创建（id 服务端生成，201）；PUT 全量更新；
DELETE 删除。非 2xx 抛 ExecuteApiError。
"""

from typing import Any

import httpx

from .models.executor import Executor
from .models.list import ExecuteList
from .models.outcome import Outcome
from .models.task import Task

DEFAULT_BASE_URL = "https://api.quanttide.com/qtcloud-execute"


class ExecuteApiError(Exception):
    """API 调用异常：携带 HTTP 状态码与服务端错误信息。"""

    def __init__(self, status_code: int, message: str):
        self.status_code = status_code
        self.message = message
        super().__init__(f"HTTP {status_code}: {message}")


class ExecuteClient:
    """执行云 API 客户端。"""

    def __init__(self, base_url: str = DEFAULT_BASE_URL, http: httpx.Client | None = None):
        self.base_url = base_url.rstrip("/")
        self._http = http or httpx.Client(timeout=10)

    def _request(self, method: str, path: str, json: Any = None) -> Any:
        resp = self._http.request(method, f"{self.base_url}{path}", json=json)
        if resp.is_error:
            raise ExecuteApiError(resp.status_code, resp.text)
        if resp.status_code == 204 or not resp.content:
            return None
        return resp.json()

    # ---- List ----

    def list_lists(self) -> list[ExecuteList]:
        data = self._request("GET", "/lists")
        return [ExecuteList.model_validate(item) for item in data.get("lists", data)]

    def create_list(self, name: str) -> ExecuteList:
        body = self._request("POST", "/lists", {"name": name})
        return ExecuteList.model_validate(body)

    def update_list(self, list_id: str, name: str) -> ExecuteList:
        body = self._request("PUT", f"/lists/{list_id}", {"name": name})
        return ExecuteList.model_validate(body)

    def delete_list(self, list_id: str) -> None:
        self._request("DELETE", f"/lists/{list_id}")

    # ---- Task ----

    def list_tasks(self, list_id: str) -> list[Task]:
        data = self._request("GET", f"/lists/{list_id}/tasks")
        return [Task.model_validate(item) for item in data.get("tasks", data)]

    def create_task(self, task: Task) -> Task:
        body = self._request("POST", f"/lists/{task.list_id}/tasks", task.model_dump(exclude={"id"}))
        return Task.model_validate(body)

    def update_task(self, task: Task) -> Task:
        body = self._request("PUT", f"/lists/{task.list_id}/tasks/{task.id}", task.model_dump())
        return Task.model_validate(body)

    def delete_task(self, list_id: str, task_id: str) -> None:
        self._request("DELETE", f"/lists/{list_id}/tasks/{task_id}")

    # ---- Outcome ----

    def list_outcomes(self, list_id: str, task_id: str | None = None) -> list[Outcome]:
        path = f"/lists/{list_id}/outcomes"
        if task_id:
            path += f"?task_id={task_id}"
        data = self._request("GET", path)
        return [Outcome.model_validate(item) for item in data.get("outcomes", data)]

    def create_outcome(self, outcome: Outcome) -> Outcome:
        body = self._request("POST", f"/lists/{outcome.list_id}/outcomes", outcome.model_dump(exclude={"id"}))
        return Outcome.model_validate(body)

    def update_outcome(self, outcome: Outcome) -> Outcome:
        body = self._request("PUT", f"/lists/{outcome.list_id}/outcomes/{outcome.id}", outcome.model_dump())
        return Outcome.model_validate(body)

    def delete_outcome(self, list_id: str, outcome_id: str) -> None:
        self._request("DELETE", f"/lists/{list_id}/outcomes/{outcome_id}")

    # ---- Executor ----

    def list_executors(self) -> list[Executor]:
        data = self._request("GET", "/executors")
        return [Executor.model_validate(item) for item in data.get("executors", data)]

    def create_executor(self, label: str) -> Executor:
        body = self._request("POST", "/executors", {"label": label})
        return Executor.model_validate(body)

    def update_executor(self, executor_id: str, label: str) -> Executor:
        body = self._request("PUT", f"/executors/{executor_id}", {"label": label})
        return Executor.model_validate(body)

    def delete_executor(self, executor_id: str) -> None:
        self._request("DELETE", f"/executors/{executor_id}")
