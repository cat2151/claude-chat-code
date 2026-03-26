//! project/ のバックアップ操作とバックアップ一覧取得に責任を持つ。

use anyhow::Result;
use std::{fs, path::Path, time::SystemTime};
use walkdir::WalkDir;

/// project/ を dst にコピーする（target/ は除外）
pub fn backup_project(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    if src.exists() {
        copy_dir_ignore_target(src, dst)?;
    }
    Ok(())
}

fn copy_dir_ignore_target(src: &Path, dst: &Path) -> Result<()> {
    for entry in WalkDir::new(src)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| {
            let rel = e.path().strip_prefix(src).unwrap_or(e.path());
            let first = rel.components().next();
            !matches!(
                first,
                Some(std::path::Component::Normal(s)) if s == "target"
            )
        })
        .flatten()
    {
        let rel = entry.path().strip_prefix(src)?;
        let target_path = dst.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target_path)?;
        } else {
            if let Some(p) = target_path.parent() {
                fs::create_dir_all(p)?;
            }
            fs::copy(entry.path(), &target_path)?;
        }
    }
    Ok(())
}

/// backup_root 配下のディレクトリ名を新しい順（mtime 降順）で返す
pub fn list_backups(backup_root: &Path) -> Vec<String> {
    if !backup_root.exists() {
        return Vec::new();
    }
    let mut entries: Vec<(SystemTime, String)> = Vec::new();

    if let Ok(rd) = fs::read_dir(backup_root) {
        for entry in rd.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
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

/// backup ディレクトリ名をタイムスタンプ付きで生成して返す。
/// 例: backup_20250318_153042
pub fn next_backup_name() -> String {
    format!("backup_{}", chrono::Local::now().format("%Y%m%d_%H%M%S"))
}
