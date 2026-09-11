use crate::IReasoningProtocol;
use async_trait::async_trait;
use myagent_types::{AgentError, AgentStep, ChatMessage, Role, ToolCall};

pub struct ThinkingR1Protocol;

#[async_trait]
impl IReasoningProtocol for ThinkingR1Protocol {
    fn name(&self) -> &'static str {
        "deepseek-r1"
    }

    fn build_request_body(&self, messages: &[ChatMessage], _tools: Option<&serde_json::Value>) -> serde_json::Value {
        let formatted: Vec<_> = messages.iter().map(|m| serde_json::json!({
            "role": match m.role {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
                Role::Tool => "tool",
            },
            "content": m.content
        })).collect();
        serde_json::json!({ "messages": formatted, "temperature": 0.6 })
    }

    fn parse_response(&self, raw_text: &str, raw_tools: Option<&serde_json::Value>) -> Result<AgentStep, AgentError> {
        let mut step = AgentStep::default();
        let (thought, rest) = if let (Some(s), Some(e)) = (raw_text.find("<think>"), raw_text.find("</think>")) {
            (raw_text[s + 7..e].trim().to_string(), raw_text[e + 8..].trim())
        } else {
            (String::new(), raw_text.trim())
        };
        step.thought = thought;

        let mut cursor = 0;
        let mut text_clean = String::new();
        while let Some(start_idx) = rest[cursor..].find("<action ") {
            let abs_start = cursor + start_idx;
            text_clean.push_str(&rest[cursor..abs_start]);
            if let Some(tag_end) = rest[abs_start..].find('>') {
                let header = &rest[abs_start..abs_start + tag_end];
                if let Some(close_idx) = rest[abs_start + tag_end..].find("</action>") {
                    let c_start = abs_start + tag_end + 1;
                    let c_end = abs_start + tag_end + close_idx;
                    let payload = rest[c_start..c_end].trim();
                    let name = header.find("name=\"").map(|p| {
                        rest[abs_start + p + 6..].split('"').next().unwrap_or("unknown")
                    }).unwrap_or("unknown");
                    let args = serde_json::from_str(payload).unwrap_or_else(|_| serde_json::json!({ "raw": payload }));
                    step.tool_calls.push(ToolCall {
                        id: format!("call_{}", step.tool_calls.len() + 1),
                        name: name.to_string(),
                        arguments: args,
                    });
                    cursor = c_end + 9;
                    continue;
                }
            }
            break;
        }
        text_clean.push_str(&rest[cursor..]);
        step.final_content = Some(text_clean.trim().to_string());

        if let Some(arr) = raw_tools.and_then(|v| v.as_array()) {
            for item in arr {
                let f = &item["function"];
                let args = f["arguments"].as_str().and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default();
                step.tool_calls.push(ToolCall {
                    id: item["id"].as_str().unwrap_or_default().to_string(),
                    name: f["name"].as_str().unwrap_or_default().to_string(),
                    arguments: args,
                });
            }
        }
        Ok(step)
    }
}
