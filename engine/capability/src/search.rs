use ignore::WalkBuilder;
use myagent_types::AgentError;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SearchMatch {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
}

pub struct FastSearcher;

impl FastSearcher {
    pub fn grep(root: &Path, pattern: &str, max_results: usize) -> Result<Vec<SearchMatch>, AgentError> {
        let mut results = Vec::new();
        let walker = WalkBuilder::new(root).hidden(false).git_ignore(true).build();

        for result in walker {
            let entry = match result {
                Ok(e) => e,
                Err(_) => continue,
            };

            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                let path = entry.path();
                if let Ok(content) = fs::read_to_string(path) {
                    for (idx, line) in content.lines().enumerate() {
                        if line.contains(pattern) {
                            results.push(SearchMatch {
                                file_path: path.to_path_buf(),
                                line_number: idx + 1,
                                line_content: line.trim().to_string(),
                            });
                            if results.len() >= max_results {
                                return Ok(results);
                            }
                        }
                    }
                }
            }
        }
        Ok(results)
    }
}
