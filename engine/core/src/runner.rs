use crate::cancellation::CancellationToken;
use myagent_capability::{SafeFileReader, SafePatchEngine, TaskManager};
use myagent_healing::{AstVerifier, SemanticGuard};
use myagent_types::{AgentError, TaskStatus};
use std::path::Path;

pub struct AgentEngine {
    pub task_mgr: TaskManager,
    pub verifier: AstVerifier,
}

impl AgentEngine {
    pub fn new() -> Result<Self, AgentError> {
        let verifier = AstVerifier::new_rust().map_err(AgentError::SyntaxError)?;
        Ok(Self {
            task_mgr: TaskManager::new(),
            verifier,
        })
    }

    /// 执行单步安全文件 Patch，伴随自动语法校验与语义保护
    pub fn safe_patch_file(
        &mut self,
        file_path: &Path,
        old_block: &str,
        new_block: &str,
    ) -> Result<String, AgentError> {
        let original = SafeFileReader::read_file_lines(file_path, None, None)?;
        let patched = SafePatchEngine::apply_patch(file_path, old_block, new_block)?;

        if let Err(violation) = SemanticGuard::is_safe_patch(&original, &patched) {
            let _ = std::fs::write(file_path, &original);
            return Err(AgentError::SemanticViolation(violation));
        }

        if file_path.extension().map_or(false, |ext| ext == "rs") {
            if let Err(err_msg) = self.verifier.check_syntax(&patched) {
                let _ = std::fs::write(file_path, &original);
                return Err(AgentError::SyntaxError(format!("{err_msg} (已触发安全倒带)")));
            }
        }
        Ok(patched)
    }

    /// 安全写入新文件，并在 Rust 文件上执行 AST 语法自检
    pub fn safe_write_file(&mut self, file_path: &Path, content: &str) -> Result<String, AgentError> {
        SafePatchEngine::write_file(file_path, content)?;
        if file_path.extension().map_or(false, |ext| ext == "rs") {
            if let Err(err_msg) = self.verifier.check_syntax(content) {
                return Err(AgentError::SyntaxError(format!("新文件语法错误: {err_msg}")));
            }
        }
        Ok(format!("成功写入文件: {:?}", file_path))
    }

    /// 标记任务并返回最新清单 Markdown
    pub fn update_task(&mut self, id: &str, status: TaskStatus, summary: Option<String>) -> String {
        self.task_mgr.update_status(id, status, summary);
        self.task_mgr.to_markdown()
    }
}
