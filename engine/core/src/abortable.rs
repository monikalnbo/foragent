use crate::cancellation::CancellationToken;
use myagent_types::AgentError;
use std::future::Future;

pub struct AbortableGuard;

impl AbortableGuard {
    /// 包装异步网络与计算任务，当令牌取消时在底层立即 Drop Future 掐断 TCP Socket 连接
    pub async fn run<T, F>(token: &CancellationToken, future: F) -> Result<T, AgentError>
    where
        F: Future<Output = Result<T, AgentError>>,
    {
        tokio::pin!(future);

        loop {
            if token.is_cancelled() {
                // 瞬间 Drop future，关闭底层网络套接字，云端即刻停表算费
                return Err(AgentError::Cancelled);
            }

            tokio::select! {
                res = &mut future => return res,
                _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
                    // 定期自旋检查取消信号
                }
            }
        }
    }
}
