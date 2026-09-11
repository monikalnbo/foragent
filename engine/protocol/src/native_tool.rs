use crate::IReasoningProtocol;
use async_trait::async_trait;
use myagent_types::{AgentError, AgentStep, ChatMessage, Role, ToolCall};

pub struct NativeToolProtocol;

#[async_trait]
impl IReasoningProtocol for NativeToolProtocol {
    fn name(&self) -> &'static str {
        "native-tool"
    }

    fn build_request_body(
        &self,
        messages: &[ChatMessage],
        tools_schema: Option<&serde_json::Value>,
    ) -> serde_json::Value {
        let formatted_msgs: Vec<serde_json::Value> = messages
            .iter()
            .map(|m| {
                serde_json::json!({
                    "role": match m.role {
                        Role::User => "user",
                        Role::Assistant => "assistant",
                        Role::System => "system",
                        Role::Tool => "tool",
                    },
                    "content": m.content
                })
            })
            .collect();

        let mut body = serde_json::json!({
            "messages": formatted_msgs,
            "temperature": 0.2
        });

        if let Some(tools) = tools_schema {
            body["tools"] = tools.clone();
            body["tool_choice"] = serde_json::json!("auto");
        }

        body
    }

    fn parse_response(
        &self,
        raw_text: &str,
        raw_tools: Option<&serde_json::Value>,
    ) -> Result<AgentStep, AgentError> {
        let mut step = AgentStep::default();
        if !raw_text.is_empty() {
            step.final_content = Some(raw_text.to_string());
        }

        if let Some(tools_arr) = raw_tools.and_then(|v| v.as_array()) {
            for item in tools_arr {
                let call_id = item["id"].as_str().unwrap_or_default().to_string();
                let func = &item["function"];
                let name = func["name"].as_str().unwrap_or_default().to_string();
                let args_val = if let Some(arg_str) = func["arguments"].as_str() {
                    serde_json::from_str(arg_str).unwrap_or(serde_json::json!({}))
                } else {
                    func["arguments"].clone()
                };

                step.tool_calls.push(ToolCall {
                    id: call_id,
                    name,
                    arguments: args_val,
                });
            }
        }

        Ok(step)
    }
}
