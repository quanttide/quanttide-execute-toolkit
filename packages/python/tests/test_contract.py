import json
from pathlib import Path

import httpx
import pytest

from quanttide_execute import ExecuteClient, Executor, ExecuteList, Outcome, Task

ROOT = Path(__file__).parents[3] / "tests"
FIXTURES = ROOT / "fixtures"
SCHEMAS = ROOT / "schemas"


def _load(name):
    return json.loads((FIXTURES / f"{name}.json").read_text())


def _validate_schema(data: dict, schema: dict) -> list[str]:
    errors = []
    props = schema.get("properties", {})
    for field in schema.get("required", []):
        if field not in data or data[field] is None:
            errors.append(f"missing required field: {field}")
    for field, value in data.items():
        prop = props.get(field, {})
        enum = prop.get("enum")
        if enum and value not in enum:
            errors.append(f"{field}: {value!r} not in {enum}")
    return errors


def test_fixtures_conform_to_schemas():
    for name in ["executor", "list", "task", "outcome"]:
        schema = json.loads((SCHEMAS / f"{name}.json").read_text())
        errors = _validate_schema(_load(name), schema)
        assert errors == [], f"{name}: {errors}"


def _mock_client(handler) -> ExecuteClient:
    return ExecuteClient(base_url="http://test", http=httpx.Client(transport=httpx.MockTransport(handler)))


def test_list_lists():
    def handler(request):
        assert request.url.path == "/lists"
        return httpx.Response(200, json={"lists": [_load("list")]})

    client = _mock_client(handler)
    lists = client.list_lists()
    assert lists[0].name == "量潮数据"


def test_task_round_trip():
    raw = _load("task")
    t = Task.model_validate(raw)
    assert t.status == "in_progress"
    assert t.is_active is True
    assert Task.model_validate(t.model_dump()) == t


def test_create_task_posts_without_id():
    captured = {}

    def handler(request):
        captured["path"] = request.url.path
        captured["body"] = json.loads(request.content)
        return httpx.Response(201, json={**captured["body"], "id": "server-generated"})

    client = _mock_client(handler)
    raw = _load("task")
    raw.pop("id")
    task = Task(id="", **raw)
    created = client.create_task(task)
    assert captured["path"] == f"/lists/{raw['list_id']}/tasks"
    assert "id" not in captured["body"] or captured["body"]["id"] == ""
    assert created.id == "server-generated"


def test_update_task_put():
    def handler(request):
        assert request.method == "PUT"
        assert "/tasks/" in request.url.path
        return httpx.Response(200, json=_load("task"))

    client = _mock_client(handler)
    t = Task.model_validate(_load("task"))
    assert client.update_task(t).id == t.id


def test_error_raises():
    def handler(request):
        return httpx.Response(404, text="任务不存在")

    client = _mock_client(handler)
    with pytest.raises(ExecuteClient.__mro__ and Exception) as exc_info:
        client.delete_list("nope")
    assert "404" in str(exc_info.value)


def test_outcome_and_executor():
    def handler(request):
        return httpx.Response(200, json={"outcomes": [_load("outcome")], "executors": [_load("executor")]})

    client = _mock_client(handler)
    outcomes = client.list_outcomes("5f0c9a1e-3d4b-4e8f-9a2c-1b7d6e5f4a3c")
    assert outcomes[0].title == "口径确认单验收通过"
    executors = client.list_executors()
    assert isinstance(executors[0], Executor)
