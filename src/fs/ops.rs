//! ファイルシステム共通操作に責任を持つ。
//! ディレクトリ準備 / touch / ソースコード調査を担当する。

use anyhow::Result;
use std::{fs, path::Path};
use walkdir::WalkDir;

// ─── ディレクトリ準備 ─────────────────────────────────────────────────────────

pub fn ensure_base_dirs(base: &Path) -> Result<()> {
    for sub in &["archives", "project", "backup"] {
        fs::create_dir_all(base.join(sub))?;
    }
    Ok(())
}

// ─── touch ────────────────────────────────────────────────────────────────────

pub fn touch_src_files(src_dir: &Path) -> Result<usize> {
    if !src_dir.exists() {
        return Ok(0);
    }
    let now = filetime::FileTime::now();
    let mut count = 0usize;

    for entry in WalkDir::new(src_dir).into_iter().flatten() {
        if entry.file_type().is_file() {
            filetime::set_file_times(entry.path(), now, now)?;
            count += 1;
        }
    }
    Ok(count)
}

// ─── ソースコード調査 ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SrcStats {
    pub file_count: usize,
    pub max_lines: usize,
    pub max_lines_file: String,
    pub total_lines: usize,
    pub total_kb: f64,
}

pub fn inspect_src(src_dir: &Path) -> SrcStats {
    let mut file_count = 0usize;
    let mut max_lines = 0usize;
    let mut max_lines_file = String::new();
    let mut total_lines = 0usize;
    let mut total_bytes = 0u64;

    if !src_dir.exists() {
        return SrcStats {
            file_count,
            max_lines,
            max_lines_file,
            total_lines,
            total_kb: 0.0,
        };
    }

    for entry in WalkDir::new(src_dir).into_iter().flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }

        file_count += 1;

        if let Ok(content) = fs::read_to_string(path) {
            let lines = content.lines().count();
            total_lines += lines;
            total_bytes += content.len() as u64;

            if lines > max_lines {
                max_lines = lines;
                max_lines_file = path
                    .strip_prefix(src_dir)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
            }
        }
    }

    SrcStats {
        file_count,
        max_lines,
        max_lines_file,
        total_lines,
        total_kb: total_bytes as f64 / 1024.0,
    }
}

// ─── src/ クリーン ────────────────────────────────────────────────────────────

/// project/src/ を丸ごと削除する。
/// 存在しない場合は何もしない（エラーにならない）。
pub fn clean_src_dir(src_dir: &Path) -> Result<()> {
    if src_dir.exists() {
        fs::remove_dir_all(src_dir)?;
    }
    Ok(())
}
