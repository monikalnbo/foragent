pub mod cascade;
pub mod native_tool;
pub mod thinking_r1;
pub mod xml_tag;

use async_trait::async_trait;
use myagent_types::{AgentError, AgentStep, ChatMessage};

pub use cascade::CascadeProtocol;
pub use native_tool::NativeToolProtocol;
pub use thinking_r1::ThinkingR1Protocol;
pub use xml_tag::XmlTagProtocol;

#[async_trait]
pub trait IReasoningProtocol: Send + Sync {
    fn name(&self) -> &'static str;

    fn build_request_body(
        &self,
        messages: &[ChatMessage],
        tools_schema: Option<&serde_json::Value>,
    ) -> serde_json::Value;

    fn parse_response(
        &self,
        raw_text: &str,
        raw_tools: Option<&serde_json::Value>,
    ) -> Result<AgentStep, AgentError>;
}
