use egui::{Color32, RichText, ScrollArea, Ui};
use myagent_types::{TaskItem, TaskStatus};

pub struct TaskPanel;

impl TaskPanel {
    pub fn render(ui: &mut Ui, tasks: &[TaskItem]) {
        ui.heading("📋 任务清单");
        ui.separator();

        if tasks.is_empty() {
            ui.label(RichText::new("暂无待办任务").color(Color32::GRAY));
            return;
        }

        ScrollArea::vertical().show(ui, |ui| {
            for task in tasks {
                ui.horizontal(|ui| {
                    let (icon, color) = match &task.status {
                        TaskStatus::Pending => ("☐", Color32::GRAY),
                        TaskStatus::InProgress => ("▶", Color32::from_rgb(99, 102, 241)),
                        TaskStatus::Completed => ("☑", Color32::from_rgb(16, 185, 129)),
                        TaskStatus::Failed(_) => ("☒", Color32::from_rgb(239, 68, 68)),
                    };

                    ui.label(RichText::new(icon).color(color).size(16.0));
                    ui.label(RichText::new(&task.id).strong());
                    ui.label(&task.title);
                });

                if let Some(summary) = &task.summary {
                    ui.indent("task_summary", |ui| {
                        ui.label(RichText::new(summary).color(Color32::LIGHT_GRAY).small());
                    });
                }
                ui.add_space(4.0);
            }
        });
    }
}
