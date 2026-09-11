mod app;
mod chat_panel;
mod diff_panel;
mod header;
mod right_panel;
mod sidebar;
mod task_panel;

use app::AgentApp;
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1150.0, 720.0])
            .with_title("AgentOS Studio (100% Pure Rust)"),
        ..Default::default()
    };

    eframe::run_native(
        "AgentOS Studio",
        options,
        Box::new(|cc| Ok(Box::new(AgentApp::new(cc)))),
    )
}
