use egui::{ComboBox, Ui};
use myagent_core::config::AgentConfig;
use myagent_runtime::RuntimeCommand;
use tokio::sync::mpsc::Sender;

pub struct HeaderBar;

impl HeaderBar {
    pub fn render(ui: &mut Ui, config: &mut AgentConfig, cmd_tx: &Sender<RuntimeCommand>) {
        ui.horizontal(|ui| {
            ui.heading("⚡ AgentOS Studio (Pure Rust)");
            ui.separator();
            let mut sel = config.default_model.clone();
            ComboBox::from_label("当前模型").selected_text(&sel).show_ui(ui, |ui| {
                for p in &config.providers {
                    for m in &p.models {
                        if ui.selectable_value(&mut sel, m.clone(), m).clicked() {
                            config.default_model = sel.clone();
                            let _ = cmd_tx.try_send(RuntimeCommand::SwitchModel(sel.clone()));
                        }
                    }
                }
            });
        });
    }
}
