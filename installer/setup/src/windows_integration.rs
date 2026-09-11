use std::path::Path;
use std::process::Command;

pub struct WindowsIntegration;

impl WindowsIntegration {
    pub fn create_shortcut(target_exe: &Path, shortcut_path: &Path, desc: &str) {
        let target_str = target_exe.to_string_lossy();
        let sc_str = shortcut_path.to_string_lossy();
        let work_dir = target_exe.parent().unwrap_or(target_exe).to_string_lossy();
        let ps_code = format!(
            "$ws = New-Object -ComObject WScript.Shell; \
             $s = $ws.CreateShortcut('{}'); \
             $s.TargetPath = '{}'; \
             $s.WorkingDirectory = '{}'; \
             $s.Description = '{}'; \
             $s.Save()",
            sc_str.replace('\'', "''"),
            target_str.replace('\'', "''"),
            work_dir.replace('\'', "''"),
            desc.replace('\'', "''")
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-WindowStyle", "Hidden", "-Command", &ps_code])
            .output();
    }

    pub fn register_uninstall(install_dir: &Path, version: &str) {
        let exe_path = install_dir.join("myagent_ui.exe");
        let uninst_path = install_dir.join("uninstall.exe");
        let key = "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\AgentOS";
        
        let _ = Command::new("reg").args(["add", key, "/v", "DisplayName", "/d", "AgentOS Studio", "/f"]).output();
        let _ = Command::new("reg").args(["add", key, "/v", "DisplayVersion", "/d", version, "/f"]).output();
        let _ = Command::new("reg").args(["add", key, "/v", "Publisher", "/d", "AgentOS Team", "/f"]).output();
        let _ = Command::new("reg").args(["add", key, "/v", "InstallLocation", "/d", &install_dir.to_string_lossy(), "/f"]).output();
        let _ = Command::new("reg").args(["add", key, "/v", "DisplayIcon", "/d", &exe_path.to_string_lossy(), "/f"]).output();
        let _ = Command::new("reg").args(["add", key, "/v", "UninstallString", "/d", &format!("\"{}\"", uninst_path.to_string_lossy()), "/f"]).output();
    }

    pub fn setup_shortcuts(install_dir: &Path) {
        let target = install_dir.join("myagent_ui.exe");
        if let Ok(profile) = std::env::var("USERPROFILE") {
            let desktop = Path::new(&profile).join("Desktop").join("AgentOS Studio.lnk");
            Self::create_shortcut(&target, &desktop, "AgentOS Studio (100% 纯 Rust)");
        }
        if let Ok(app_data) = std::env::var("APPDATA") {
            let start_menu = Path::new(&app_data)
                .join("Microsoft\\Windows\\Start Menu\\Programs\\AgentOS");
            let _ = std::fs::create_dir_all(&start_menu);
            let link = start_menu.join("AgentOS Studio.lnk");
            Self::create_shortcut(&target, &link, "AgentOS Studio (100% 纯 Rust)");
        }
    }
}
