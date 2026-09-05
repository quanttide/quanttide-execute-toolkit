import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Executor, List, Outcome, Task } from "../src/models.js";

function fixture(name: string): any {
  return JSON.parse(readFileSync(`../../tests/fixtures/${name}.json`, "utf-8"));
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
