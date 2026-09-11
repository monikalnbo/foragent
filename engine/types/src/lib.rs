use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON 解析错误: {0}")]
    Json(#[from] serde_json::Error),

    #[error("网络请求错误: {0}")]
    Network(String),

    #[error("协议解析错误: {0}")]
    Protocol(String),

    #[error("补丁匹配失败: {0}")]
    PatchMismatch(String),

    #[error("AST 语法错误: {0}")]
    SyntaxError(String),

    #[error("语义保全看门狗拦截: {0}")]
    SemanticViolation(String),

    #[error("任务已被用户取消")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role { User, Assistant, System, Tool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    #[serde(default)]
    pub thought: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Default)]
pub struct AgentStep {
    pub thought: String,
    pub tool_calls: Vec<ToolCall>,
    pub final_content: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItem {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub action_id: String,
    pub file_path: String,
    pub original_code: String,
    pub modified_code: String,
    pub diff_preview: String,
}
