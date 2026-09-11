mod app;
mod extractor;
mod uninstaller;
mod windows_integration;

use app::InstallerApp;
use eframe::NativeOptions;
use extractor::Extractor;
use uninstaller::Uninstaller;
use windows_integration::WindowsIntegration;

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let dest = Extractor::default_install_dir();

    if args.iter().any(|a| a == "--uninstall") {
        let _ = Uninstaller::run(&dest);
        return Ok(());
    }

    if args.iter().any(|a| a == "--silent" || a == "-y" || a == "/S") {
        if let Ok(_) = Extractor::extract_all(&dest, |_, _| {}) {
            WindowsIntegration::setup_shortcuts(&dest);
            WindowsIntegration::register_uninstall(&dest, "0.1.0");
        }
        return Ok(());
    }

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([540.0, 320.0])
            .with_title("AgentOS Studio 安装向导 (100% 纯 Rust)"),
        ..Default::default()
    };

    eframe::run_native(
        "AgentOS Installer",
        options,
        Box::new(|_| Ok(Box::new(InstallerApp::default()))),
    )
}
