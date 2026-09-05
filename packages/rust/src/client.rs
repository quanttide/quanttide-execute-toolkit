//! 执行云 API 客户端：封装 docs/specification 定义的端点。
//!
//! 方法语义与规格一致：GET 读取；POST 创建（id 服务端生成，201）；PUT 全量更新；DELETE 删除。

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{Executor, List, Outcome, Task};

/// 系统级 API 网关。
pub const DEFAULT_BASE_URL: &str = "https://api.quanttide.com/qtcloud-execute";

/// API 调用异常：携带 HTTP 状态码与服务端错误信息。
#[derive(Debug)]
pub struct ExecuteApiError {
    pub status_code: u16,
    pub message: String,
}

impl std::fmt::Display for ExecuteApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP {}: {}", self.status_code, self.message)
    }
}

impl std::error::Error for ExecuteApiError {}

/// 执行云 API 客户端。
pub struct ExecuteClient {
    base_url: String,
    http: reqwest::blocking::Client,
}

impl Default for ExecuteClient {
    fn default() -> Self {
        Self::new(DEFAULT_BASE_URL)
    }
}

impl ExecuteClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("构建 HTTP 客户端失败"),
        }
    }

    fn request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, ExecuteApiError> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.request(method, &url);
        if let Some(body) = body {
            req = req.json(body);
        }
        let resp = req.send().map_err(|e| ExecuteApiError {
            status_code: 0,
            message: e.to_string(),
        })?;
        let status = resp.status().as_u16();
        if status >= 400 {
            return Err(ExecuteApiError {
                status_code: status,
                message: resp.text().unwrap_or_default(),
            });
        }
        if status == 204 {
            unreachable!("204 响应不应用此方法解析");
        }
        resp.json::<T>().map_err(|e| ExecuteApiError {
            status_code: status,
            message: e.to_string(),
        })
    }

    fn request_empty<B: Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<(), ExecuteApiError> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.request(method, &url);
        if let Some(body) = body {
            req = req.json(body);
        }
        let resp = req.send().map_err(|e| ExecuteApiError {
            status_code: 0,
            message: e.to_string(),
        })?;
        let status = resp.status().as_u16();
        if status >= 400 {
            return Err(ExecuteApiError {
                status_code: status,
                message: resp.text().unwrap_or_default(),
            });
        }
        Ok(())
    }

    // ---- List ----

    pub fn list_lists(&self) -> Result<Vec<List>, ExecuteApiError> {
        self.request(reqwest::Method::GET, "/lists", None::<&()>)
    }

    pub fn create_list(&self, name: &str) -> Result<List, ExecuteApiError> {
        self.request(
            reqwest::Method::POST,
            "/lists",
            Some(&serde_json::json!({ "name": name })),
        )
    }

    pub fn update_list(&self, list_id: &str, name: &str) -> Result<(), ExecuteApiError> {
        self.request_empty(
            reqwest::Method::PUT,
            &format!("/lists/{list_id}"),
            Some(&serde_json::json!({ "name": name })),
        )
    }

    pub fn delete_list(&self, list_id: &str) -> Result<(), ExecuteApiError> {
        self.request_empty(
            reqwest::Method::DELETE,
            &format!("/lists/{list_id}"),
            None::<&()>,
        )
    }

    // ---- Task ----

    pub fn list_tasks(&self, list_id: &str) -> Result<Vec<Task>, ExecuteApiError> {
        self.request(
            reqwest::Method::GET,
            &format!("/lists/{list_id}/tasks"),
            None::<&()>,
        )
    }

    pub fn create_task(&self, task: &Task) -> Result<Task, ExecuteApiError> {
        self.request(
            reqwest::Method::POST,
            &format!("/lists/{}/tasks", task.list_id),
            Some(task),
        )
    }

    pub fn update_task(&self, task: &Task) -> Result<(), ExecuteApiError> {
        self.request_empty(
            reqwest::Method::PUT,
            &format!("/lists/{}/tasks/{}", task.list_id, task.id),
            Some(task),
        )
    }

    pub fn delete_task(&self, list_id: &str, task_id: &str) -> Result<(), ExecuteApiError> {
        self.request_empty(
            reqwest::Method::DELETE,
            &format!("/lists/{list_id}/tasks/{task_id}"),
            None::<&()>,
        )
    }

    // ---- Outcome ----

    pub fn list_outcomes(
        &self,
        list_id: &str,
        task_id: Option<&str>,
    ) -> Result<Vec<Outcome>, ExecuteApiError> {
        let path = match task_id {
            Some(id) => format!("/lists/{list_id}/outcomes?task_id={id}"),
            None => format!("/lists/{list_id}/outcomes"),
        };
        self.request(reqwest::Method::GET, &path, None::<&()>)
    }

    pub fn create_outcome(&self, outcome: &Outcome) -> Result<Outcome, ExecuteApiError> {
        self.request(
            reqwest::Method::POST,
            &format!("/lists/{}/outcomes", outcome.list_id),
            Some(outcome),
        )
    }

    pub fn update_outcome(&self, outcome: &Outcome) -> Result<(), ExecuteApiError> {
        self.request_empty(
            reqwest::Method::PUT,
            &format!("/lists/{}/outcomes/{}", outcome.list_id, outcome.id),
            Some(outcome),
        )
    }

    pub fn delete_outcome(&self, list_id: &str, outcome_id: &str) -> Result<(), ExecuteApiError> {
        self.request_empty(
            reqwest::Method::DELETE,
            &format!("/lists/{list_id}/outcomes/{outcome_id}"),
            None::<&()>,
        )
    }

    // ---- Executor ----

    pub fn list_executors(&self) -> Result<Vec<Executor>, ExecuteApiError> {
        self.request(reqwest::Method::GET, "/executors", None::<&()>)
    }

    pub fn create_executor(&self, label: &str) -> Result<Executor, ExecuteApiError> {
        self.request(
            reqwest::Method::POST,
            "/executors",
            Some(&serde_json::json!({ "label": label })),
        )
    }

    pub fn update_executor(&self, executor_id: &str, label: &str) -> Result<(), ExecuteApiError> {
        self.request_empty(
            reqwest::Method::PUT,
            &format!("/executors/{executor_id}"),
            Some(&serde_json::json!({ "label": label })),
        )
    }

    pub fn delete_executor(&self, executor_id: &str) -> Result<(), ExecuteApiError> {
        self.request_empty(
            reqwest::Method::DELETE,
            &format!("/executors/{executor_id}"),
            None::<&()>,
        )
    }
}
