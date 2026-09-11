use egui::{Color32, RichText, ScrollArea, Ui};

pub struct DiffPanel {
    pub action_id: String,
    pub file_path: String,
    pub original_lines: Vec<String>,
    pub modified_lines: Vec<String>,
    pub status_msg: Option<String>,
}

impl Default for DiffPanel {
    fn default() -> Self {
        Self {
            action_id: "init".into(),
            file_path: "等待变更输入...".into(),
            original_lines: vec!["// 暂无待审查变更".into()],
            modified_lines: vec!["// 暂无待审查变更".into()],
            status_msg: None,
        }
    }
}

impl DiffPanel {
    pub fn render(&mut self, ui: &mut Ui, mut on_approve: impl FnMut(&str), mut on_reject: impl FnMut(&str)) {
        ui.horizontal(|ui| {
            ui.heading("🔍 代码变更审查");
            ui.label(RichText::new(&self.file_path).monospace().color(Color32::YELLOW));
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(RichText::new(" ✓ 批准写入 ").color(Color32::from_rgb(16, 185, 129))).clicked() {
                    self.status_msg = Some("已核准变更！".into());
                    on_approve(&self.action_id);
                }
                if ui.button(RichText::new(" ✗ 驳回修改 ").color(Color32::from_rgb(239, 68, 68))).clicked() {
                    self.status_msg = Some("已驳回该修改。".into());
                    on_reject(&self.action_id);
                }
            });
        });

        if let Some(msg) = &self.status_msg {
            ui.label(RichText::new(msg).color(Color32::from_rgb(16, 185, 129)).small());
        }
        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(RichText::new("修改前 (原代码)").strong().color(Color32::from_rgb(239, 68, 68)));
                ScrollArea::both().id_source("orig_diff").show(ui, |ui| {
                    for line in &self.original_lines {
                        ui.label(RichText::new(line).color(Color32::from_rgb(248, 113, 113)).monospace());
                    }
                });
            });
            cols[1].group(|ui| {
                ui.label(RichText::new("修改后 (AST 自检通过)").strong().color(Color32::from_rgb(16, 185, 129)));
                ScrollArea::both().id_source("mod_diff").show(ui, |ui| {
                    for line in &self.modified_lines {
                        ui.label(RichText::new(line).color(Color32::from_rgb(74, 222, 128)).monospace());
                    }
                });
            });
        });
    }
}
