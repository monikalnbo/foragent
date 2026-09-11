use std::path::Path;
use std::process::Command;

pub struct Uninstaller;

impl Uninstaller {
    pub fn run(install_dir: &Path) -> Result<(), String> {
        let key = "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\AgentOS";
        let _ = Command::new("reg").args(["delete", key, "/f"]).output();

        if let Ok(profile) = std::env::var("USERPROFILE") {
            let desktop_sc = Path::new(&profile).join("Desktop").join("AgentOS Studio.lnk");
            let _ = std::fs::remove_file(desktop_sc);
        }

        if let Ok(app_data) = std::env::var("APPDATA") {
            let start_dir = Path::new(&app_data)
                .join("Microsoft\\Windows\\Start Menu\\Programs\\AgentOS");
            let _ = std::fs::remove_dir_all(start_dir);
        }

        let cmd = format!(
            "Start-Sleep -Seconds 1; Remove-Item -Path '{}' -Recurse -Force",
            install_dir.to_string_lossy().replace('\'', "''")
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-WindowStyle", "Hidden", "-Command", &cmd])
            .spawn();

        Ok(())
    }
}
