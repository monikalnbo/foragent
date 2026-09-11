use myagent_types::{ApprovalRequest, TaskItem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeCommand {
    UserPrompt(String),
    ApprovePatch { action_id: String },
    RejectPatch { action_id: String },
    SwitchModel(String),
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeEvent {
    ThinkingDelta(String),
    TasksUpdated(Vec<TaskItem>),
    DiffReady(ApprovalRequest),
    SelfHealingReport { level: String, message: String },
    MessageDelta(String),
    Done { summary: String },
    Error(String),
}
