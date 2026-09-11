use crate::IReasoningProtocol;
use async_trait::async_trait;
use myagent_types::{AgentError, AgentStep, ChatMessage, Role, ToolCall};

pub struct XmlTagProtocol;

#[async_trait]
impl IReasoningProtocol for XmlTagProtocol {
    fn name(&self) -> &'static str { "xml-tag" }

    fn build_request_body(&self, msgs: &[ChatMessage], _t: Option<&serde_json::Value>) -> serde_json::Value {
        let f: Vec<_> = msgs.iter().map(|m| serde_json::json!({
            "role": match m.role {
                Role::User => "user", Role::Assistant => "assistant",
                Role::System => "system", Role::Tool => "tool",
            },
            "content": m.content
        })).collect();
        serde_json::json!({ "messages": f, "temperature": 0.2 })
    }

    fn parse_response(&self, raw: &str, _raw_tools: Option<&serde_json::Value>) -> Result<AgentStep, AgentError> {
        let mut step = AgentStep::default();
        let (mut cursor, mut text_clean) = (0, String::new());

        while let Some(start_idx) = raw[cursor..].find("<action ") {
            let abs_start = cursor + start_idx;
            text_clean.push_str(&raw[cursor..abs_start]);
            if let Some(tag_end) = raw[abs_start..].find('>') {
                let header = &raw[abs_start..abs_start + tag_end];
                if let Some(close_idx) = raw[abs_start + tag_end..].find("</action>") {
                    let c_start = abs_start + tag_end + 1;
                    let c_end = abs_start + tag_end + close_idx;
                    let payload = raw[c_start..c_end].trim();
                    let name = header.find("name=\"").map(|p| {
                        raw[abs_start + p + 6..].split('"').next().unwrap_or("unknown")
                    }).unwrap_or("unknown");
                    let args = serde_json::from_str(payload).unwrap_or_else(|_| serde_json::json!({ "raw": payload }));
                    step.tool_calls.push(ToolCall {
                        id: format!("xml_call_{}", step.tool_calls.len() + 1),
                        name: name.into(),
                        arguments: args,
                    });
                    cursor = c_end + 9;
                    continue;
                }
            }
            break;
        }
        text_clean.push_str(&raw[cursor..]);
        step.final_content = Some(text_clean.trim().into());
        Ok(step)
    }
}
