use crate::protocol::{RuntimeCommand, RuntimeEvent};
use crate::react_loop::ReActExecutor;
use myagent_core::cancellation::CancellationToken;
use myagent_core::config::AgentConfig;
use myagent_core::AgentEngine;
use myagent_types::{ChatMessage, Role, TaskStatus};
use std::path::Path;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct RuntimeService {
    engine: AgentEngine,
    config: AgentConfig,
    cmd_rx: Receiver<RuntimeCommand>,
    event_tx: Sender<RuntimeEvent>,
    messages: Vec<ChatMessage>,
    cancel_token: CancellationToken,
}

impl RuntimeService {
    pub fn new(cmd_rx: Receiver<RuntimeCommand>, event_tx: Sender<RuntimeEvent>) -> Result<Self, String> {
        let engine = AgentEngine::new().map_err(|e| e.to_string())?;
        let config = AgentConfig::load_or_default(Path::new("myagent.toml"));
        Ok(Self {
            engine,
            config,
            cmd_rx,
            event_tx,
            messages: Vec::new(),
            cancel_token: CancellationToken::new(),
        })
    }

    pub async fn run_loop(&mut self) {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                RuntimeCommand::UserPrompt(prompt) => {
                    self.cancel_token = CancellationToken::new();
                    self.messages.push(ChatMessage { role: Role::User, content: prompt, thought: None });
                    let _ = self.event_tx.send(RuntimeEvent::ThinkingDelta("正在进行深度自主推演与工程行动...\n".into())).await;
                    ReActExecutor::run_react(
                        &mut self.engine,
                        &self.config,
                        &mut self.messages,
                        &self.event_tx,
                        &self.cancel_token,
                    ).await;
                }
                RuntimeCommand::ApprovePatch { action_id } => {
                    self.engine.task_mgr.update_status(&action_id, TaskStatus::Completed, Some("代码变更已核准".into()));
                    let _ = self.event_tx.send(RuntimeEvent::TasksUpdated(self.engine.task_mgr.get_tasks().to_vec())).await;
                }
                RuntimeCommand::RejectPatch { action_id } => {
                    self.engine.task_mgr.update_status(&action_id, TaskStatus::Failed("已驳回".into()), None);
                    let _ = self.event_tx.send(RuntimeEvent::TasksUpdated(self.engine.task_mgr.get_tasks().to_vec())).await;
                }
                RuntimeCommand::SwitchModel(m) => {
                    self.config.default_model = m.clone();
                    let _ = self.event_tx.send(RuntimeEvent::ThinkingDelta(format!("已切换模型: {m}\n"))).await;
                }
                RuntimeCommand::Cancel => {
                    self.cancel_token.cancel();
                    let _ = self.event_tx.send(RuntimeEvent::Done { summary: "已取消当前任务".into() }).await;
                }
            }
        }
    }
}
