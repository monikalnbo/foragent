use myagent_types::{AgentError, TaskItem};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub enum WalEvent {
    TaskSnapshot(Vec<TaskItem>),
    FileModified { path: String, summary: String },
    SessionCheckpoint { id: String, timestamp: u64 },
}

pub struct WalStorage {
    file_path: PathBuf,
}

impl WalStorage {
    pub fn new(session_dir: &Path, session_id: &str) -> Result<Self, AgentError> {
        fs::create_dir_all(session_dir)?;
        let file_path = session_dir.join(format!("{}.wal", session_id));
        Ok(Self { file_path })
    }

    pub fn append(&self, event: &WalEvent) -> Result<(), AgentError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        let line = serde_json::to_string(event)?;
        writeln!(file, "{}", line)?;
        file.flush()?;
        Ok(())
    }

    pub fn recover_tasks(&self) -> Result<Option<Vec<TaskItem>>, AgentError> {
        if !self.file_path.exists() {
            return Ok(None);
        }

        let file = fs::File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let mut latest_tasks = None;

        for line in reader.lines() {
            let line = line?;
            if let Ok(event) = serde_json::from_str::<WalEvent>(&line) {
                if let WalEvent::TaskSnapshot(tasks) = event {
                    latest_tasks = Some(tasks);
                }
            }
        }
        Ok(latest_tasks)
    }
}
