use crate::runner::AgentEngine;
use myagent_capability::{FastSearcher, SafeFileReader};
use myagent_sandbox::SandboxExecutor;
use myagent_types::{AgentError, TaskStatus, ToolCall};
use std::path::Path;
use std::time::Duration;

pub struct ToolDispatcher;

impl ToolDispatcher {
    pub async fn dispatch(engine: &mut AgentEngine, call: &ToolCall) -> Result<String, AgentError> {
        match call.name.as_str() {
            "fs_patch" => {
                let file = call.arguments["file_path"].as_str().ok_or_else(|| AgentError::Protocol("缺少 file_path".into()))?;
                let old = call.arguments["old_block"].as_str().unwrap_or_default();
                let new = call.arguments["new_block"].as_str().ok_or_else(|| AgentError::Protocol("缺少 new_block".into()))?;
                engine.safe_patch_file(Path::new(file), old, new)
            }
            "fs_write" => {
                let file = call.arguments["file_path"].as_str().ok_or_else(|| AgentError::Protocol("缺少 file_path".into()))?;
                let content = call.arguments["content"].as_str().ok_or_else(|| AgentError::Protocol("缺少 content".into()))?;
                engine.safe_write_file(Path::new(file), content)
            }
            "fs_read" => {
                let file = call.arguments["file_path"].as_str().ok_or_else(|| AgentError::Protocol("缺少 file_path".into()))?;
                let start = call.arguments["start_line"].as_u64().map(|v| v as usize);
                let end = call.arguments["end_line"].as_u64().map(|v| v as usize);
                SafeFileReader::read_file_lines(Path::new(file), start, end)
            }
            "list_dir" => {
                let dir = call.arguments["dir_path"].as_str().unwrap_or(".");
                let entries = SafeFileReader::list_directory(Path::new(dir))?;
                let list: Vec<String> = entries.into_iter().map(|p| p.display().to_string()).collect();
                Ok(list.join("\n"))
            }
            "grep_search" => {
                let pat = call.arguments["pattern"].as_str().ok_or_else(|| AgentError::Protocol("缺少 pattern".into()))?;
                let root = call.arguments["root"].as_str().unwrap_or(".");
                let matches = FastSearcher::grep(Path::new(root), pat, 20)?;
                let mut out = String::new();
                for m in matches {
                    out.push_str(&format!("{}:{} {}\n", m.file_path.display(), m.line_number, m.line_content));
                }
                Ok(out)
            }
            "execute_command" => {
                let cmd = call.arguments["cmd"].as_str().ok_or_else(|| AgentError::Protocol("缺少 cmd".into()))?;
                let empty_vec = vec![];
                let args_val = call.arguments["args"].as_array().unwrap_or(&empty_vec);
                let args: Vec<&str> = args_val.iter().filter_map(|v| v.as_str()).collect();
                let res = SandboxExecutor::run_command(Path::new("."), cmd, &args, Duration::from_secs(30)).await?;
                Ok(format!("退出码: {:?}\n[标准输出]:\n{}\n[错误输出]:\n{}", res.exit_code, res.stdout, res.stderr))
            }
            "update_task" => {
                let id = call.arguments["id"].as_str().unwrap_or_default();
                let summary = call.arguments["summary"].as_str().map(String::from);
                Ok(engine.update_task(id, TaskStatus::Completed, summary))
            }
            _ => Err(AgentError::Protocol(format!("未识别的工具: {}", call.name))),
        }
    }
}
