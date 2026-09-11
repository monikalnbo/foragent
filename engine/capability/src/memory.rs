use myagent_types::AgentError;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub id: String,
    pub category: String, // "rule" | "preference" | "solution"
    pub content: String,
    pub tags: Vec<String>,
}

pub struct PersistentMemory {
    file_path: PathBuf,
    items: Vec<MemoryItem>,
}

impl PersistentMemory {
    pub fn load_or_create(storage_dir: &Path) -> Result<Self, AgentError> {
        fs::create_dir_all(storage_dir)?;
        let file_path = storage_dir.join("memory.json");
        let mut items = Vec::new();

        if file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            if let Ok(loaded) = serde_json::from_str::<Vec<MemoryItem>>(&content) {
                items = loaded;
            }
        }

        Ok(Self { file_path, items })
    }

    pub fn add_item(&mut self, category: &str, content: &str, tags: Vec<&str>) -> Result<(), AgentError> {
        let item = MemoryItem {
            id: format!("mem_{}", self.items.len() + 1),
            category: category.to_string(),
            content: content.to_string(),
            tags: tags.into_iter().map(String::from).collect(),
        };
        self.items.push(item);
        self.save()
    }

    pub fn query_relevant(&self, keyword: &str) -> Vec<&MemoryItem> {
        self.items
            .iter()
            .filter(|m| m.content.contains(keyword) || m.tags.iter().any(|t| t.contains(keyword)))
            .collect()
    }

    pub fn to_prompt_context(&self) -> String {
        if self.items.is_empty() {
            return String::new();
        }
        let mut text = String::from("\n【长期持久记忆与用户规则】\n");
        for m in &self.items {
            text.push_str(&format!("- [{}] {}\n", m.category, m.content));
        }
        text
    }

    fn save(&self) -> Result<(), AgentError> {
        let json = serde_json::to_string_pretty(&self.items)?;
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&self.file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
