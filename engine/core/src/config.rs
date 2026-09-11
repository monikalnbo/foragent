use myagent_types::AgentError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub models: Vec<String>,
}

impl ProviderConfig {
    pub fn resolved_key(&self) -> String {
        if let Some(env_var) = self.api_key.strip_prefix("ENV:") {
            std::env::var(env_var).unwrap_or_default()
        } else {
            self.api_key.clone()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub default_model: String,
    pub timeout_seconds: u64,
    pub providers: Vec<ProviderConfig>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            default_model: "deepseek-reasoner".to_string(),
            timeout_seconds: 60,
            providers: vec![
                ProviderConfig {
                    name: "deepseek".to_string(),
                    base_url: "https://api.deepseek.com/v1".to_string(),
                    api_key: "ENV:DEEPSEEK_API_KEY".to_string(),
                    models: vec!["deepseek-reasoner".to_string(), "deepseek-chat".to_string()],
                },
                ProviderConfig {
                    name: "ollama".to_string(),
                    base_url: "http://localhost:11434/v1".to_string(),
                    api_key: "ollama".to_string(),
                    models: vec!["qwen2.5-coder:7b".to_string(), "deepseek-r1:14b".to_string()],
                },
            ],
        }
    }
}

impl AgentConfig {
    pub fn load_or_default(path: &Path) -> Self {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(cfg) = toml::from_str(&content) {
                    return cfg;
                }
            }
        }
        Self::default()
    }

    pub fn find_provider_for_model(&self, model: &str) -> Option<&ProviderConfig> {
        self.providers.iter().find(|p| p.models.iter().any(|m| m == model))
    }
}
