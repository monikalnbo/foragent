use crate::job_object::ProcessGuard;
use myagent_types::AgentError;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Debug)]
pub struct CommandOutput {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub struct SandboxExecutor;

impl SandboxExecutor {
    pub async fn run_command(
        work_dir: &Path,
        cmd: &str,
        args: &[&str],
        timeout: Duration,
    ) -> Result<CommandOutput, AgentError> {
        let child = Command::new(cmd)
            .args(args)
            .current_dir(work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let guard = ProcessGuard::new(child);

        // 超时看门狗机制
        let timeout_future = tokio::time::sleep(timeout);
        tokio::pin!(timeout_future);

        let execution = tokio::task::spawn_blocking(move || guard.wait_with_output());

        tokio::select! {
            res = execution => {
                let output = res.map_err(|e| AgentError::Protocol(format!("协程执行失败: {}", e)))??;
                Ok(CommandOutput {
                    success: output.status.success(),
                    exit_code: output.status.code(),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                })
            }
            _ = &mut timeout_future => {
                Err(AgentError::Protocol(format!("沙箱命令执行超时 (超 {} 秒强杀)", timeout.as_secs())))
            }
        }
    }
}
