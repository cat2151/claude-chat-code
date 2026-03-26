//! デスクトップの変更検知とファイル一覧取得に責任を持つ。
//! OS 呼び出しのみを行い、状態変更や UI には触れない。

use crate::app::FileEntry;
use chrono::Local;
use std::{
    fs,
    path::Path,
    time::SystemTime,
};

/// ディレクトリの mtime を取得する。取得できなければ None を返す。
pub fn dir_mtime(path: &Path) -> Option<SystemTime> {
    fs::metadata(path).ok()?.modified().ok()
}

/// デスクトップ上のファイルを新しい順に列挙する。
pub fn list_files(path: &Path) -> Vec<FileEntry> {
    let mut entries = Vec::new();

    if let Ok(rd) = fs::read_dir(path) {
        for entry in rd.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() {
                    let modified: chrono::DateTime<Local> = meta
                        .modified()
                        .map(|t| t.into())
                        .unwrap_or_else(|_| Local::now());
                    entries.push(FileEntry {
                        name: entry.file_name().to_string_lossy().to_string(),
                        modified,
                    });
                }
            }
        }
    }

    entries.sort_by(|a, b| b.modified.cmp(&a.modified));
    entries
}

/// ファイル一覧の中から最新の ZIP を探す。
/// list_files() の結果は新しい順なので先頭の ZIP が最新となる。
pub fn find_latest_zip(files: &[FileEntry]) -> Option<String> {
    files
        .iter()
        .find(|f| f.is_zip())
        .map(|f| f.name.clone())
}

/// デスクトップの mtime が前回から変化したかを判定する。
pub fn has_changed(prev: Option<SystemTime>, current: Option<SystemTime>) -> bool {
    match (prev, current) {
        (None, Some(_))         => true,
        (Some(p), Some(c))      => c != p,
        _                       => false,
    }
}
