use myagent_types::AgentError;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileHashCache {
    cache: HashMap<PathBuf, String>,
}

impl FileHashCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn compute_hash(path: &Path) -> Result<String, AgentError> {
        let bytes = fs::read(path)?;
        let hash = blake3::hash(&bytes);
        Ok(hash.to_hex().to_string())
    }

    pub fn is_changed(&mut self, path: &Path) -> Result<bool, AgentError> {
        let current_hash = Self::compute_hash(path)?;
        if let Some(old_hash) = self.cache.get(path) {
            if old_hash == &current_hash {
                return Ok(false);
            }
        }
        self.cache.insert(path.to_path_buf(), current_hash);
        Ok(true)
    }

    pub fn invalidate(&mut self, path: &Path) {
        self.cache.remove(path);
    }
}
