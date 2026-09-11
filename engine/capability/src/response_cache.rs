use myagent_types::{AgentError, AgentStep};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct ResponseCache {
    memory_cache: HashMap<String, AgentStep>,
    disk_dir: PathBuf,
}

impl ResponseCache {
    pub fn new(cache_dir: &Path) -> Result<Self, AgentError> {
        fs::create_dir_all(cache_dir)?;
        Ok(Self {
            memory_cache: HashMap::new(),
            disk_dir: cache_dir.to_path_buf(),
        })
    }

    pub fn make_key(model: &str, prompt: &str) -> String {
        let input = format!("{}:{}", model, prompt);
        blake3::hash(input.as_bytes()).to_hex().to_string()
    }

    pub fn get(&self, key: &str) -> Option<AgentStep> {
        if let Some(step) = self.memory_cache.get(key) {
            return Some(step.clone());
        }

        let disk_file = self.disk_dir.join(format!("{}.json", key));
        if disk_file.exists() {
            if let Ok(content) = fs::read_to_string(&disk_file) {
                if let Ok(thought) = serde_json::from_str::<String>(&content) {
                    return Some(AgentStep {
                        thought,
                        tool_calls: Vec::new(),
                        final_content: None,
                    });
                }
            }
        }
        None
    }

    pub fn insert(&mut self, key: String, step: AgentStep) -> Result<(), AgentError> {
        let disk_file = self.disk_dir.join(format!("{}.json", key));
        let payload = serde_json::to_string(&step.thought)?;
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(disk_file)?;
        file.write_all(payload.as_bytes())?;

        self.memory_cache.insert(key, step);
        Ok(())
    }
}
