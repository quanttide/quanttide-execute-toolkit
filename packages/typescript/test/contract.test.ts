import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { ExecuteApiError, ExecuteClient } from "../src/client.js";
import type { Executor, List, Outcome, Task } from "../src/models.js";

function fixture(name: string): any {
  return JSON.parse(readFileSync(`../../tests/fixtures/${name}.json`, "utf-8"));
}

function jsonResponse(status: number, body: unknown): Response {
  return new Response(JSON.stringify(body), { status, headers: { "Content-Type": "application/json" } });
}

describe("契约测试（fixtures 来自根 tests/）", () => {
  it("executor fixture", () => {
    const e = fixture("executor") as Executor;
    expect(e.label).toBe("张三");
    expect(e.id).toBeTruthy();
  });

  it("list fixture", () => {
    const l = fixture("list") as List;
    expect(l.name).toBe("量潮数据");
    expect(l.tasks).toContain("7c9e6679-7425-40de-944b-e07fc1f90ae7");
  });

  it("task fixture", () => {
    const t = fixture("task") as Task;
    expect(t.status).toBe("in_progress");
    expect(t.priority).toBe("high");
    expect(t.is_active).toBe(true);
    expect(t.completed_at).toBeNull();
    expect(t.list_id).toBeTruthy();
    expect(t.owner_id).toBeTruthy();
    expect(t.reviewer_id).toBeTruthy();
  });

  it("outcome fixture", () => {
    const o = fixture("outcome") as Outcome;
    expect(o.title).toBe("口径确认单验收通过");
    expect(o.list_id).toBe("5f0c9a1e-3d4b-4e8f-9a2c-1b7d6e5f4a3c");
    expect(o.task_ids).toContain("7c9e6679-7425-40de-944b-e07fc1f90ae7");
  });

  it("round-trip 值不变", () => {
    for (const name of ["executor", "list", "task", "outcome"]) {
      const raw = fixture(name);
      const json = JSON.parse(JSON.stringify(raw));
      expect(json).toEqual(raw);
    }
  });
});

describe("ExecuteClient", () => {
  it("createTask 走 POST 且不携带 id", async () => {
    const calls: { method: string; url: string; body: any }[] = [];
    const mockFetch = (async (_url: any, init: any = {}) => {
      calls.push({ method: init.method, url: String(_url), body: init.body ? JSON.parse(init.body) : null });
      return jsonResponse(201, { ...calls[0].body, id: "server-generated" });
    }) as unknown as typeof fetch;
    const client = new ExecuteClient({ baseUrl: "http://test", fetch: mockFetch });
    const raw = { ...fixture("task") } as Partial<Task>;
    delete raw.id;
    const created = await client.createTask(raw as Task);
    expect(calls[0].method).toBe("POST");
    expect(calls[0].url).toBe("http://test/lists/5f0c9a1e-3d4b-4e8f-9a2c-1b7d6e5f4a3c/tasks");
    expect(created.id).toBe("server-generated");
  });

  it("updateTask 走 PUT，deleteList 走 DELETE", async () => {
    const calls: { method: string; url: string }[] = [];
    const mockFetch = (async (_url: any, init: any = {}) => {
      calls.push({ method: init.method, url: String(_url) });
      return jsonResponse(200, fixture("task"));
    }) as unknown as typeof fetch;
    const client = new ExecuteClient({ baseUrl: "http://test", fetch: mockFetch });
    const t = fixture("task") as Task;
    await client.updateTask(t.list_id, t);
    await client.deleteList(t.list_id);
    expect(calls[0].method).toBe("PUT");
    expect(calls[1].method).toBe("DELETE");
  });

  it("非 2xx 抛 ExecuteApiError", async () => {
    const mockFetch = (async () => jsonResponse(404, "任务不存在")) as unknown as typeof fetch;
    const client = new ExecuteClient({ baseUrl: "http://test", fetch: mockFetch });
    await expect(client.deleteList("nope")).rejects.toThrow(ExecuteApiError);
  });

  it("listLists 解析 lists 包裹", async () => {
    const mockFetch = (async () => jsonResponse(200, { lists: [fixture("list")] })) as unknown as typeof fetch;
    const client = new ExecuteClient({ baseUrl: "http://test", fetch: mockFetch });
    const lists = await client.listLists();
    expect(lists[0].name).toBe("量潮数据");
  });
});
