/// ZIP の archives/ への移動とアーカイブ一覧取得に責任を持つ。

use anyhow::Result;
use std::{fs, path::Path, time::SystemTime};

/// デスクトップの zip を archives/ に移動する。
/// 常にタイムスタンプサフィックスを付ける。例: foo_20250318_153042.zip
/// 戻り値は実際の移動先フルパス。
pub fn move_zip(desktop: &Path, base: &Path, zip_name: &str) -> Result<std::path::PathBuf> {
    let src = desktop.join(zip_name);
    let dst = archives_dest(base, zip_name);
    fs::rename(&src, &dst)?;
    Ok(dst)
}

/// archives/ への移動先パスを決定する（常にタイムスタンプサフィックス付き）
fn archives_dest(base: &Path, zip_name: &str) -> std::path::PathBuf {
    let path = std::path::Path::new(zip_name);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let ext  = path.extension().unwrap_or_default().to_string_lossy();
    let ts   = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let new_name = if ext.is_empty() {
        format!("{}_{}", stem, ts)
    } else {
        format!("{}_{}.{}", stem, ts, ext)
    };
    base.join("archives").join(new_name)
}

/// archives/ 配下のファイル名を新しい順（mtime 降順）で返す
pub fn list_archives(archives_dir: &Path) -> Vec<String> {
    if !archives_dir.exists() {
        return Vec::new();
    }
    let mut entries: Vec<(SystemTime, String)> = Vec::new();

    if let Ok(rd) = fs::read_dir(archives_dir) {
        for entry in rd.flatten() {
            if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                let name  = entry.file_name().to_string_lossy().to_string();
                let mtime = entry.metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or(SystemTime::UNIX_EPOCH);
                entries.push((mtime, name));
            }
        }
    }

    entries.sort_by(|a, b| b.0.cmp(&a.0));
    entries.into_iter().map(|(_, n)| n).collect()
}
