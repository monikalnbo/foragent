use crate::IReasoningProtocol;
use async_trait::async_trait;
use myagent_types::{AgentError, AgentStep, ChatMessage};
use std::sync::Arc;

pub struct CascadeProtocol {
    protocols: Vec<Arc<dyn IReasoningProtocol>>,
}

impl CascadeProtocol {
    pub fn new(protocols: Vec<Arc<dyn IReasoningProtocol>>) -> Self {
        Self { protocols }
    }

    pub fn parse_with_fallback(
        &self,
        raw_text: &str,
        raw_tools: Option<&serde_json::Value>,
    ) -> Result<AgentStep, AgentError> {
        let mut last_err = AgentError::Protocol("无可用模型协议适配器".to_string());

        for proto in &self.protocols {
            match proto.parse_response(raw_text, raw_tools) {
                Ok(step) => return Ok(step),
                Err(err) => {
                    last_err = err;
                }
            }
        }
        Err(last_err)
    }
}

#[async_trait]
impl IReasoningProtocol for CascadeProtocol {
    fn name(&self) -> &'static str {
        "cascade-fallback"
    }

    fn build_request_body(
        &self,
        messages: &[ChatMessage],
        tools_schema: Option<&serde_json::Value>,
    ) -> serde_json::Value {
        if let Some(first) = self.protocols.first() {
            first.build_request_body(messages, tools_schema)
        } else {
            serde_json::json!({})
        }
    }

    fn parse_response(
        &self,
        raw_text: &str,
        raw_tools: Option<&serde_json::Value>,
    ) -> Result<AgentStep, AgentError> {
        self.parse_with_fallback(raw_text, raw_tools)
    }
}
