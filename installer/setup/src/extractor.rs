use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub static PAYLOAD: &[u8] = include_bytes!("../payload.zip");

pub struct Extractor;

impl Extractor {
    pub fn default_install_dir() -> PathBuf {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            PathBuf::from(local_app_data).join("Programs").join("AgentOS")
        } else {
            PathBuf::from("C:\\AgentOS")
        }
    }

    pub fn extract_all<F>(dest: &Path, mut on_progress: F) -> Result<usize, String>
    where
        F: FnMut(&str, f32),
    {
        if PAYLOAD.len() < 22 {
            return Err("安装包数据不完整".into());
        }
        let reader = Cursor::new(PAYLOAD);
        let mut archive = ZipArchive::new(reader).map_err(|e| format!("解压负载读取失败: {e}"))?;
        let total = archive.len();
        fs::create_dir_all(dest).map_err(|e| format!("创建安装目录失败: {e}"))?;

        for i in 0..total {
            let mut file = archive.by_index(i).map_err(|e| format!("读取文件错误: {e}"))?;
            let name = file.mangled_name();
            let outpath = dest.join(&name);

            let progress = (i as f32) / (total as f32).max(1.0);
            on_progress(file.name(), progress);

            if file.is_dir() {
                fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).map_err(|e| e.to_string())?;
                    }
                }
                let mut outfile = File::create(&outpath).map_err(|e| format!("无法写入 {}: {e}", outpath.display()))?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                outfile.write_all(&buffer).map_err(|e| e.to_string())?;
            }
        }
        on_progress("完成解压", 1.0);
        Ok(total)
    }
}
