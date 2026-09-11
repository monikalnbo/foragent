use crate::chat_panel::ChatPanel;
use crate::diff_panel::DiffPanel;
use crate::header::HeaderBar;
use crate::right_panel::{RightTab, RightWorkspace};
use crate::sidebar::Sidebar;
use egui::{CentralPanel, Context, SidePanel, TopBottomPanel};
use myagent_core::config::AgentConfig;
use myagent_runtime::{RuntimeCommand, RuntimeEvent, RuntimeService};
use myagent_types::TaskItem;
use std::path::Path;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct AgentApp {
    config: AgentConfig,
    cmd_tx: Sender<RuntimeCommand>,
    event_rx: Receiver<RuntimeEvent>,
    sidebar: Sidebar,
    chat_panel: ChatPanel,
    diff_panel: DiffPanel,
    tasks: Vec<TaskItem>,
    right_tab: RightTab,
}

impl AgentApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = AgentConfig::load_or_default(Path::new("myagent.toml"));
        let (cmd_tx, cmd_rx) = channel(32);
        let (event_tx, event_rx) = channel(32);
        tokio::spawn(async move {
            if let Ok(mut s) = RuntimeService::new(cmd_rx, event_tx) { s.run_loop().await; }
        });
        Self {
            config, cmd_tx, event_rx,
            sidebar: Sidebar::default(), chat_panel: ChatPanel::default(),
            diff_panel: DiffPanel::default(), tasks: vec![], right_tab: RightTab::Tasks,
        }
    }
}

impl eframe::App for AgentApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        while let Ok(ev) = self.event_rx.try_recv() {
            match ev {
                RuntimeEvent::TasksUpdated(t) => self.tasks = t,
                RuntimeEvent::ThinkingDelta(d) => self.chat_panel.append_thought(&d),
                RuntimeEvent::MessageDelta(d) => self.chat_panel.append_content(&d),
                RuntimeEvent::DiffReady(req) => {
                    self.diff_panel.action_id = req.action_id;
                    self.diff_panel.file_path = req.file_path;
                    self.diff_panel.original_lines = req.original_code.lines().map(String::from).collect();
                    self.diff_panel.modified_lines = req.modified_code.lines().map(String::from).collect();
                    self.right_tab = RightTab::Diff;
                }
                RuntimeEvent::Error(err) => self.chat_panel.append_system(&format!("⚠️ 错误: {err}")),
                _ => {}
            }
        }

        TopBottomPanel::top("header").show(ctx, |ui| HeaderBar::render(ui, &mut self.config, &self.cmd_tx));
        SidePanel::left("left_sidebar").min_width(180.0).show(ctx, |ui| self.sidebar.render(ui, || {}));
        SidePanel::right("right_workspace").min_width(420.0).show(ctx, |ui| {
            RightWorkspace::render(ui, &mut self.right_tab, &self.tasks, &mut self.diff_panel, &self.cmd_tx);
        });

        CentralPanel::default().show(ctx, |ui| {
            let tx = self.cmd_tx.clone();
            self.chat_panel.render(ui, move |p| { let _ = tx.try_send(RuntimeCommand::UserPrompt(p.into())); });
        });
    }
}
