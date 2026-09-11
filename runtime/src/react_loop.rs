use crate::protocol::RuntimeEvent;
use myagent_core::cancellation::CancellationToken;
use myagent_core::config::AgentConfig;
use myagent_core::llm_client::LlmClient;
use myagent_core::prompt::SystemPrompt;
use myagent_core::tool_dispatcher::ToolDispatcher;
use myagent_core::AgentEngine;
use myagent_protocol::{IReasoningProtocol, ThinkingR1Protocol};
use myagent_types::{ApprovalRequest, ChatMessage, Role};
use tokio::sync::mpsc::Sender;

pub struct ReActExecutor;

impl ReActExecutor {
    pub async fn run_react(
        engine: &mut AgentEngine,
        config: &AgentConfig,
        messages: &mut Vec<ChatMessage>,
        event_tx: &Sender<RuntimeEvent>,
        cancel: &CancellationToken,
    ) {
        let protocol = ThinkingR1Protocol;
        let sys_prompt = SystemPrompt::build(None);
        if messages.first().map_or(true, |m| m.role != Role::System) {
            messages.insert(0, ChatMessage { role: Role::System, content: sys_prompt, thought: None });
        }

        for _ in 0..15 {
            if cancel.is_cancelled() { break; }
            let client = LlmClient::from_config(config);
            let resp = match client.chat_completion(messages).await {
                Ok(r) => r,
                Err(e) => {
                    let _ = event_tx.send(RuntimeEvent::Error(format!("大模型故障: {e}"))).await;
                    break;
                }
            };

            if let Some(r) = &resp.reasoning_content {
                let _ = event_tx.send(RuntimeEvent::ThinkingDelta(r.clone())).await;
            }

            let step = protocol.parse_response(&resp.content, resp.raw_tool_calls.as_ref()).unwrap_or_default();
            if !step.thought.is_empty() {
                let _ = event_tx.send(RuntimeEvent::ThinkingDelta(step.thought.clone())).await;
            }
            if let Some(ans) = &step.final_content {
                let _ = event_tx.send(RuntimeEvent::MessageDelta(ans.clone())).await;
            }

            if step.tool_calls.is_empty() {
                messages.push(ChatMessage { role: Role::Assistant, content: resp.content, thought: Some(step.thought) });
                break;
            }

            messages.push(ChatMessage { role: Role::Assistant, content: resp.content, thought: Some(step.thought) });
            for call in &step.tool_calls {
                if call.name == "fs_patch" {
                    let path = call.arguments["file_path"].as_str().unwrap_or_default();
                    let _ = event_tx.send(RuntimeEvent::DiffReady(ApprovalRequest {
                        action_id: call.id.clone(),
                        file_path: path.to_string(),
                        original_code: call.arguments["old_block"].as_str().unwrap_or("").into(),
                        modified_code: call.arguments["new_block"].as_str().unwrap_or("").into(),
                        diff_preview: String::new(),
                    })).await;
                }

                match ToolDispatcher::dispatch(engine, call).await {
                    Ok(out) => {
                        messages.push(ChatMessage { role: Role::Tool, content: format!("[工具 {} 结果]:\n{out}", call.name), thought: None });
                    }
                    Err(e) => {
                        messages.push(ChatMessage { role: Role::Tool, content: format!("[工具 {} 错误]:\n{e}", call.name), thought: None });
                    }
                }
            }
            let _ = event_tx.send(RuntimeEvent::TasksUpdated(engine.task_mgr.get_tasks().to_vec())).await;
        }
        let _ = event_tx.send(RuntimeEvent::Done { summary: "自主闭环任务完成".into() }).await;
    }
}
