package quanttide_execute

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
)

// APIError API 调用异常：携带 HTTP 状态码与服务端错误信息。
type APIError struct {
	StatusCode int
	Message    string
}

func (e *APIError) Error() string { return fmt.Sprintf("HTTP %d: %s", e.StatusCode, e.Message) }

// Client 执行云 API 客户端：封装 docs/specification 定义的端点。
//
// 方法语义与规格一致：GET 读取；POST 创建（id 服务端生成，201）；PUT 全量更新；DELETE 删除。
type Client struct {
	BaseURL string
	HTTP    *http.Client
}

// NewClient 创建客户端（baseURL 必填，由调用方/应用层提供）。
func NewClient(baseURL string) *Client {
	if baseURL == "" {
		panic("baseURL 必填：工程标准不在工具库内定义 API 地址")
	}
	return &Client{BaseURL: strings.TrimRight(baseURL, "/"), HTTP: http.DefaultClient}
}

func (c *Client) do(method, path string, body any, out any) error {
	var reader io.Reader
	if body != nil {
		raw, err := json.Marshal(body)
		if err != nil {
			return err
		}
		reader = bytes.NewReader(raw)
	}
	req, err := http.NewRequest(method, c.BaseURL+path, reader)
	if err != nil {
		return err
	}
	if body != nil {
		req.Header.Set("Content-Type", "application/json")
	}
	resp, err := c.HTTP.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 400 {
		raw, _ := io.ReadAll(resp.Body)
		return &APIError{StatusCode: resp.StatusCode, Message: string(raw)}
	}
	if out != nil && resp.StatusCode != http.StatusNoContent {
		return json.NewDecoder(resp.Body).Decode(out)
	}
	return nil
}

// ---- List ----

// ListLists 列出全部清单（含各自任务）。
func (c *Client) ListLists() ([]List, error) {
	var lists []List
	err := c.do(http.MethodGet, "/lists", nil, &lists)
	return lists, err
}

// CreateList 创建清单（id 由服务端生成）。
func (c *Client) CreateList(name string) (*List, error) {
	var l List
	if err := c.do(http.MethodPost, "/lists", map[string]any{"name": name}, &l); err != nil {
		return nil, err
	}
	return &l, nil
}

// UpdateList 全量更新清单。
func (c *Client) UpdateList(listID, name string) error {
	return c.do(http.MethodPut, "/lists/"+listID, map[string]any{"name": name}, nil)
}

// DeleteList 删除清单（名下无任务与结果方可删）。
func (c *Client) DeleteList(listID string) error {
	return c.do(http.MethodDelete, "/lists/"+listID, nil, nil)
}

// ---- Task ----

// ListTasks 列出某清单的任务。
func (c *Client) ListTasks(listID string) ([]Task, error) {
	var tasks []Task
	err := c.do(http.MethodGet, "/lists/"+listID+"/tasks", nil, &tasks)
	return tasks, err
}

// CreateTask 创建任务（id 由服务端生成）。
func (c *Client) CreateTask(listID string, task *Task) (*Task, error) {
	var created Task
	if err := c.do(http.MethodPost, "/lists/"+listID+"/tasks", task, &created); err != nil {
		return nil, err
	}
	return &created, nil
}

// UpdateTask 全量更新任务。
func (c *Client) UpdateTask(listID string, task *Task) error {
	return c.do(http.MethodPut, "/lists/"+listID+"/tasks/"+task.ID, task, nil)
}

// DeleteTask 删除任务。
func (c *Client) DeleteTask(listID, taskID string) error {
	return c.do(http.MethodDelete, "/lists/"+listID+"/tasks/"+taskID, nil, nil)
}

// ---- Outcome ----

// ListOutcomes 列出某清单的结果（taskID 非空时按任务过滤）。
func (c *Client) ListOutcomes(listID, taskID string) ([]Outcome, error) {
	path := "/lists/" + listID + "/outcomes"
	if taskID != "" {
		path += "?task_id=" + taskID
	}
	var outcomes []Outcome
	err := c.do(http.MethodGet, path, nil, &outcomes)
	return outcomes, err
}

// CreateOutcome 创建结果（id 由服务端生成）。
func (c *Client) CreateOutcome(listID string, outcome *Outcome) (*Outcome, error) {
	var created Outcome
	if err := c.do(http.MethodPost, "/lists/"+listID+"/outcomes", outcome, &created); err != nil {
		return nil, err
	}
	return &created, nil
}

// UpdateOutcome 全量更新结果。
func (c *Client) UpdateOutcome(listID string, outcome *Outcome) error {
	return c.do(http.MethodPut, "/lists/"+listID+"/outcomes/"+outcome.ID, outcome, nil)
}

// DeleteOutcome 删除结果。
func (c *Client) DeleteOutcome(listID, outcomeID string) error {
	return c.do(http.MethodDelete, "/lists/"+listID+"/outcomes/"+outcomeID, nil, nil)
}

// ---- Executor ----

// ListExecutors 列出执行者。
func (c *Client) ListExecutors() ([]Executor, error) {
	var executors []Executor
	err := c.do(http.MethodGet, "/executors", nil, &executors)
	return executors, err
}

// CreateExecutor 创建执行者（id 由服务端生成）。
func (c *Client) CreateExecutor(label string) (*Executor, error) {
	var e Executor
	if err := c.do(http.MethodPost, "/executors", map[string]any{"label": label}, &e); err != nil {
		return nil, err
	}
	return &e, nil
}

// UpdateExecutor 全量更新执行者。
func (c *Client) UpdateExecutor(executorID, label string) error {
	return c.do(http.MethodPut, "/executors/"+executorID, map[string]any{"label": label}, nil)
}

// DeleteExecutor 删除执行者（未被任何任务引用方可删）。
func (c *Client) DeleteExecutor(executorID string) error {
	return c.do(http.MethodDelete, "/executors/"+executorID, nil, nil)
}
