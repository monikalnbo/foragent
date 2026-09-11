use myagent_types::AgentError;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SafeFileReader;

impl SafeFileReader {
    pub fn read_file_lines(
        file_path: &Path,
        start_line: Option<usize>,
        end_line: Option<usize>,
    ) -> Result<String, AgentError> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();

        let start = start_line.unwrap_or(1).saturating_sub(1);
        let end = end_line.unwrap_or(lines.len()).min(lines.len());

        if start >= lines.len() {
            return Ok(String::new());
        }

        let slice = &lines[start..end];
        Ok(slice.join("\n"))
    }

    pub fn list_directory(dir_path: &Path) -> Result<Vec<PathBuf>, AgentError> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            entries.push(entry.path());
        }
        Ok(entries)
    }
}
