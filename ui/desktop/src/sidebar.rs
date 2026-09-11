use egui::{Color32, RichText, ScrollArea, Ui};

pub struct Sidebar {
    pub sessions: Vec<String>,
    pub active_index: usize,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            sessions: vec![
                "重构互斥锁逻辑 (活跃)".to_string(),
                "CMakeLists 现代升级".to_string(),
                "内存泄漏排查与断点自愈".to_string(),
            ],
            active_index: 0,
        }
    }
}

impl Sidebar {
    pub fn render(&mut self, ui: &mut Ui, mut on_new_session: impl FnMut()) {
        ui.heading("📁 会话历史");
        ui.separator();

        if ui.button(RichText::new(" ＋ 新建会话 (New Chat) ").strong().color(Color32::from_rgb(99, 102, 241))).clicked() {
            on_new_session();
        }
        ui.add_space(8.0);

        ScrollArea::vertical().show(ui, |ui| {
            for (idx, name) in self.sessions.iter().enumerate() {
                let is_selected = self.active_index == idx;
                let text = if is_selected {
                    RichText::new(format!("▶ {}", name)).color(Color32::from_rgb(16, 185, 129)).strong()
                } else {
                    RichText::new(format!("• {}", name)).color(Color32::GRAY)
                };

                if ui.selectable_label(is_selected, text).clicked() {
                    self.active_index = idx;
                }
                ui.add_space(2.0);
            }
        });
    }
}
