use egui::{Color32, RichText, ScrollArea, Ui};
use myagent_types::{ChatMessage, Role};

pub struct ChatPanel {
    pub messages: Vec<ChatMessage>,
    pub input_text: String,
}

impl Default for ChatPanel {
    fn default() -> Self {
        Self {
            messages: vec![ChatMessage {
                role: Role::Assistant,
                content: "AgentOS 纯 Rust 原生引擎已挂载。输入需求我将自主探索工程并进行修改自愈。".into(),
                thought: Some("内部核心引擎就绪，DeepSeek-R1 思考流已开启。".into()),
            }],
            input_text: String::new(),
        }
    }
}

impl ChatPanel {
    pub fn append_thought(&mut self, delta: &str) {
        if let Some(last) = self.messages.last_mut() {
            last.thought.get_or_insert_with(String::new).push_str(delta);
        }
    }

    pub fn append_content(&mut self, delta: &str) {
        if let Some(last) = self.messages.last_mut() {
            if last.role == Role::Assistant {
                last.content.push_str(delta);
                return;
            }
        }
        self.messages.push(ChatMessage { role: Role::Assistant, content: delta.into(), thought: None });
    }

    pub fn append_system(&mut self, msg: &str) {
        self.messages.push(ChatMessage { role: Role::System, content: msg.into(), thought: None });
    }

    pub fn render(&mut self, ui: &mut Ui, mut on_send: impl FnMut(&str)) {
        ui.vertical(|ui| {
            ScrollArea::vertical().auto_shrink([false; 2]).max_height(ui.available_height() - 50.0).show(ui, |ui| {
                for msg in &self.messages {
                    ui.group(|ui| {
                        let (label, col) = match msg.role {
                            Role::User => ("你", Color32::from_rgb(99, 102, 241)),
                            Role::Assistant => ("AgentOS", Color32::from_rgb(16, 185, 129)),
                            Role::System => ("系统", Color32::GRAY),
                            Role::Tool => ("工具", Color32::from_rgb(245, 158, 11)),
                        };
                        ui.label(RichText::new(label).color(col).strong());
                        if let Some(t) = &msg.thought {
                            ui.collapsing("🧠 思考推理流", |ui| {
                                ui.label(RichText::new(t).color(Color32::from_rgb(199, 210, 254)).monospace());
                            });
                        }
                        ui.label(&msg.content);
                    });
                    ui.add_space(4.0);
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                let edit = ui.text_edit_singleline(&mut self.input_text);
                let btn = ui.button(RichText::new(" 发送 (Enter) ").strong()).clicked();
                if (btn || (edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) && !self.input_text.trim().is_empty() {
                    let prompt = std::mem::take(&mut self.input_text);
                    self.messages.push(ChatMessage { role: Role::User, content: prompt.clone(), thought: None });
                    on_send(&prompt);
                }
            });
        });
    }
}
