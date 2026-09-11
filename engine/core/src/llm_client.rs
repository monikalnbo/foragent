use crate::config::AgentConfig;
use myagent_types::{AgentError, ChatMessage, Role};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Default)]
pub struct LlmResponse {
    pub content: String,
    pub reasoning_content: Option<String>,
    pub raw_tool_calls: Option<serde_json::Value>,
}

pub struct LlmClient {
    client: reqwest::Client,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
}

impl LlmClient {
    pub fn from_config(config: &AgentConfig) -> Self {
        let p = config
            .find_provider_for_model(&config.default_model)
            .cloned()
            .unwrap_or_else(|| config.providers[0].clone());
        Self {
            client: reqwest::Client::new(),
            base_url: p.base_url,
            api_key: p.resolved_key(),
            model: config.default_model.clone(),
            temperature: 0.6,
        }
    }

    pub async fn chat_completion(&self, messages: &[ChatMessage]) -> Result<LlmResponse, AgentError> {
        let endpoint = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let req_messages = messages.iter().map(|m| OpenAiMessage {
            role: match m.role {
                Role::User => "user".into(),
                Role::Assistant => "assistant".into(),
                Role::System => "system".into(),
                Role::Tool => "tool".into(),
            },
            content: m.content.clone(),
        }).collect();

        let req_body = OpenAiChatRequest {
            model: self.model.clone(),
            messages: req_messages,
            temperature: self.temperature,
        };
        let mut req = self.client.post(&endpoint).json(&req_body);
        if !self.api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_key));
        }
        let resp = req.send().await.map_err(AgentError::Network)?;
        if !resp.status().is_success() {
            let status = resp.status();
            let err = resp.text().await.unwrap_or_default();
            return Err(AgentError::Protocol(format!("API 失败 [{status}]: {err}")));
        }

        let json: serde_json::Value = resp.json().await.map_err(AgentError::Network)?;
        let msg = &json["choices"][0]["message"];
        Ok(LlmResponse {
            content: msg["content"].as_str().unwrap_or_default().to_string(),
            reasoning_content: msg["reasoning_content"].as_str().map(String::from),
            raw_tool_calls: msg.get("tool_calls").cloned(),
        })
    }
}
