//! 方法语义与规格一致：GET 读取；POST 创建（id 服务端生成，201）；PUT 全量更新；DELETE 删除。
//! base_url 必填，由调用方（应用层）提供。
import type { Executor, List, Outcome, Task } from "./models.js";

/** API 调用异常：携带 HTTP 状态码与服务端错误信息。 */
export class ExecuteApiError extends Error {
  constructor(
    public readonly statusCode: number,
    message: string,
  ) {
    super(`HTTP ${statusCode}: ${message}`);
  }
}

export interface ExecuteClientOptions {
  baseUrl: string;
  fetch?: typeof fetch;
}

export class ExecuteClient {
  private readonly baseUrl: string;
  private readonly fetchImpl: typeof fetch;

  constructor({ baseUrl, fetch: fetchImpl = fetch }: ExecuteClientOptions) {
    this.baseUrl = baseUrl.replace(/\/$/, "");
    this.fetchImpl = fetchImpl;
  }

  private async request<T>(method: string, path: string, body?: unknown): Promise<T> {
    const resp = await this.fetchImpl(this.baseUrl + path, {
      method,
      headers: body !== undefined ? { "Content-Type": "application/json" } : undefined,
      body: body !== undefined ? JSON.stringify(body) : undefined,
    });
    if (!resp.ok) {
      throw new ExecuteApiError(resp.status, await resp.text());
    }
    if (resp.status === 204) return undefined as T;
    return (await resp.json()) as T;
  }

  // ---- List ----

  async listLists(): Promise<List[]> {
    const data = await this.request<List[] | { lists: List[] }>("GET", "/lists");
    return Array.isArray(data) ? data : data.lists;
  }

  async createList(name: string): Promise<List> {
    return this.request<List>("POST", "/lists", { name });
  }

  async updateList(listId: string, name: string): Promise<void> {
    await this.request("PUT", `/lists/${listId}`, { name });
  }

  async deleteList(listId: string): Promise<void> {
    await this.request("DELETE", `/lists/${listId}`);
  }

  // ---- Task ----

  async listTasks(listId: string): Promise<Task[]> {
    const data = await this.request<Task[] | { tasks: Task[] }>("GET", `/lists/${listId}/tasks`);
    return Array.isArray(data) ? data : data.tasks;
  }

  async createTask(task: Omit<Task, "id">): Promise<Task> {
    return this.request<Task>("POST", `/lists/${task.list_id}/tasks`, task);
  }

  async updateTask(listId: string, task: Task): Promise<void> {
    await this.request("PUT", `/lists/${listId}/tasks/${task.id}`, task);
  }

  async deleteTask(listId: string, taskId: string): Promise<void> {
    await this.request("DELETE", `/lists/${listId}/tasks/${taskId}`);
  }

  // ---- Outcome ----

  async listOutcomes(listId: string, taskId?: string): Promise<Outcome[]> {
    const path = taskId ? `/lists/${listId}/outcomes?task_id=${taskId}` : `/lists/${listId}/outcomes`;
    const data = await this.request<Outcome[] | { outcomes: Outcome[] }>("GET", path);
    return Array.isArray(data) ? data : data.outcomes;
  }

  async createOutcome(outcome: Omit<Outcome, "id">): Promise<Outcome> {
    return this.request<Outcome>("POST", `/lists/${outcome.list_id}/outcomes`, outcome);
  }

  async updateOutcome(listId: string, outcome: Outcome): Promise<void> {
    await this.request("PUT", `/lists/${listId}/outcomes/${outcome.id}`, outcome);
  }

  async deleteOutcome(listId: string, outcomeId: string): Promise<void> {
    await this.request("DELETE", `/lists/${listId}/outcomes/${outcomeId}`);
  }

  // ---- Executor ----

  async listExecutors(): Promise<Executor[]> {
    const data = await this.request<Executor[] | { executors: Executor[] }>("GET", "/executors");
    return Array.isArray(data) ? data : data.executors;
  }

  async createExecutor(label: string): Promise<Executor> {
    return this.request<Executor>("POST", "/executors", { label });
  }

  async updateExecutor(executorId: string, label: string): Promise<void> {
    await this.request("PUT", `/executors/${executorId}`, { label });
  }

  async deleteExecutor(executorId: string): Promise<void> {
    await this.request("DELETE", `/executors/${executorId}`);
  }
}
