use crate::extractor::Extractor;
use crate::windows_integration::WindowsIntegration;
use egui::{Color32, ProgressBar, RichText};
use std::path::Path;

pub struct InstallerApp {
    pub dest_path: String,
    pub status: String,
    pub progress: f32,
    pub installed: bool,
    pub error: Option<String>,
}

impl Default for InstallerApp {
    fn default() -> Self {
        Self {
            dest_path: Extractor::default_install_dir().to_string_lossy().into(),
            status: "就绪，准备部署 100% 纯 Rust 智能体系统".into(),
            progress: 0.0,
            installed: false,
            error: None,
        }
    }
}

impl InstallerApp {
    pub fn do_install(&mut self) {
        let path = Path::new(&self.dest_path);
        let mut err = None;
        match Extractor::extract_all(path, |name, p| {
            self.status = format!("正在解压: {name}");
            self.progress = p;
        }) {
            Ok(_) => {
                WindowsIntegration::setup_shortcuts(path);
                WindowsIntegration::register_uninstall(path, "0.1.0");
                self.progress = 1.0;
                self.status = "部署完毕！快捷方式已生成。".into();
                self.installed = true;
            }
            Err(e) => err = Some(e),
        }
        self.error = err;
    }
}

impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("⚡ AgentOS Studio (Windows 原生安装器)").strong());
            ui.add_space(8.0);
            ui.label(RichText::new("安装目标目录:").color(Color32::GRAY));
            ui.text_edit_singleline(&mut self.dest_path);
            ui.add_space(10.0);

            if let Some(e) = &self.error {
                ui.label(RichText::new(format!("❌ 错误: {e}")).color(Color32::from_rgb(239, 68, 68)));
            } else {
                ui.label(&self.status);
            }
            ui.add(ProgressBar::new(self.progress).show_percentage());
            ui.add_space(15.0);

            ui.horizontal(|ui| {
                if !self.installed {
                    if ui.button(RichText::new(" 🚀 立即一键安装 ").strong().size(16.0)).clicked() {
                        self.do_install();
                    }
                } else {
                    if ui.button(RichText::new(" ⚡ 启动 AgentOS Studio ").color(Color32::from_rgb(16, 185, 129)).strong()).clicked() {
                        let exe = Path::new(&self.dest_path).join("myagent_ui.exe");
                        let _ = std::process::Command::new(exe).spawn();
                        std::process::exit(0);
                    }
                    if ui.button(" 完成退出 ").clicked() {
                        std::process::exit(0);
                    }
                }
            });
        });
    }
}
