use myagent_runtime::{RuntimeCommand, RuntimeEvent, RuntimeService};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 AgentOS 无头独立运行时 (Headless Runtime Daemon) 已启动...");

    let (cmd_tx, cmd_rx) = mpsc::channel::<RuntimeCommand>(32);
    let (event_tx, mut event_rx) = mpsc::channel::<RuntimeEvent>(32);

    let mut service = RuntimeService::new(cmd_rx, event_tx)?;

    // 启动独立事件监听协程
    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            println!("[Runtime Event] {:?}", event);
        }
    });

    // 模拟接收前端或外部指令
    cmd_tx.send(RuntimeCommand::UserPrompt("初始化代码库自检".to_string())).await?;

    service.run_loop().await;
    Ok(())
}
