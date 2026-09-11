use myagent_types::AgentError;
use std::fs;
use std::path::Path;

pub struct SafePatchEngine;

impl SafePatchEngine {
    pub fn write_file(file_path: &Path, content: &str) -> Result<(), AgentError> {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(file_path, content)?;
        Ok(())
    }

    pub fn apply_patch(file_path: &Path, old_block: &str, new_block: &str) -> Result<String, AgentError> {
        if !file_path.exists() {
            if old_block.trim().is_empty() {
                Self::write_file(file_path, new_block)?;
                return Ok(new_block.to_string());
            }
            return Err(AgentError::PatchMismatch(format!("文件不存在: {:?}", file_path)));
        }
        let raw_bytes = fs::read(file_path)?;
        let is_crlf = raw_bytes.windows(2).any(|w| w == b"\r\n");
        let content = String::from_utf8_lossy(&raw_bytes).to_string();

        let norm_content = content.replace("\r\n", "\n");
        let norm_old = old_block.replace("\r\n", "\n");
        let norm_new = new_block.replace("\r\n", "\n");

        let patched_norm = if let Some(idx) = norm_content.find(&norm_old) {
            let mut res = String::with_capacity(norm_content.len() + norm_new.len());
            res.push_str(&norm_content[..idx]);
            res.push_str(&norm_new);
            res.push_str(&norm_content[idx + norm_old.len()..]);
            res
        } else {
            Self::fuzzy_patch(&norm_content, &norm_old, &norm_new).ok_or_else(|| {
                AgentError::PatchMismatch(format!(
                    "在文件 {:?} 中未定位到待替换的代码块",
                    file_path
                ))
            })?
        };

        let final_content = if is_crlf {
            patched_norm.replace('\n', "\r\n")
        } else {
            patched_norm
        };

        fs::write(file_path, &final_content)?;
        Ok(final_content)
    }

    fn fuzzy_patch(content: &str, old_block: &str, new_block: &str) -> Option<String> {
        let old_trimmed: Vec<&str> = old_block.lines().map(str::trim).filter(|s| !s.is_empty()).collect();
        if old_trimmed.is_empty() {
            return None;
        }

        let content_lines: Vec<&str> = content.lines().collect();
        let target_len = old_trimmed.len();

        for i in 0..=content_lines.len().saturating_sub(target_len) {
            let window: Vec<&str> = content_lines[i..i + target_len]
                .iter()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            if window == old_trimmed {
                let mut res = Vec::new();
                res.extend_from_slice(&content_lines[..i]);
                res.push(new_block);
                res.extend_from_slice(&content_lines[i + target_len..]);
                return Some(res.join("\n"));
            }
        }
        None
    }
}
