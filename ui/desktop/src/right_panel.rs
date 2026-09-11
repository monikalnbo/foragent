use crate::diff_panel::DiffPanel;
use crate::task_panel::TaskPanel;
use egui::Ui;
use myagent_runtime::RuntimeCommand;
use myagent_types::TaskItem;
use tokio::sync::mpsc::Sender;

#[derive(PartialEq)]
pub enum RightTab { Tasks, Diff }

pub struct RightWorkspace;

impl RightWorkspace {
    pub fn render(
        ui: &mut Ui,
        tab: &mut RightTab,
        tasks: &[TaskItem],
        diff: &mut DiffPanel,
        cmd_tx: &Sender<RuntimeCommand>,
    ) {
        ui.horizontal(|ui| {
            ui.selectable_value(tab, RightTab::Tasks, " 📋 任务状态 ");
            ui.selectable_value(tab, RightTab::Diff, " 🔍 代码审查 ");
        });
        ui.separator();

        match tab {
            RightTab::Tasks => TaskPanel::render(ui, tasks),
            RightTab::Diff => {
                let tx1 = cmd_tx.clone();
                let tx2 = cmd_tx.clone();
                diff.render(
                    ui,
                    move |id| { let _ = tx1.try_send(RuntimeCommand::ApprovePatch { action_id: id.into() }); },
                    move |id| { let _ = tx2.try_send(RuntimeCommand::RejectPatch { action_id: id.into() }); },
                );
            }
        }
    }
}
