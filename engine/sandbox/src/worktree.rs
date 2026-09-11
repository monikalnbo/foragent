use myagent_types::AgentError;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ShadowWorktree {
    pub sandbox_path: PathBuf,
    pub host_path: PathBuf,
}

impl ShadowWorktree {
    pub fn create(session_id: &str, host_path: &Path) -> Result<Self, AgentError> {
        let temp_base = std::env::temp_dir().join("myagent_sandboxes");
        let sandbox_path = temp_base.join(session_id);

        if sandbox_path.exists() {
            let _ = fs::remove_dir_all(&sandbox_path);
        }
        fs::create_dir_all(&sandbox_path)?;

        Ok(Self {
            sandbox_path,
            host_path: host_path.to_path_buf(),
        })
    }

    pub fn copy_file_to_sandbox(&self, rel_path: &Path) -> Result<PathBuf, AgentError> {
        let src = self.host_path.join(rel_path);
        let dst = self.sandbox_path.join(rel_path);

        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        if src.exists() {
            fs::copy(&src, &dst)?;
        }
        Ok(dst)
    }

    pub fn commit_file_to_host(&self, rel_path: &Path) -> Result<(), AgentError> {
        let src = self.sandbox_path.join(rel_path);
        let dst = self.host_path.join(rel_path);

        if src.exists() {
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&src, &dst)?;
        }
        Ok(())
    }

    pub fn destroy(self) -> Result<(), AgentError> {
        if self.sandbox_path.exists() {
            fs::remove_dir_all(&self.sandbox_path)?;
        }
        Ok(())
    }
}
