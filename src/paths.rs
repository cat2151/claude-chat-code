/// パス解決を一か所に集約する。
/// AppData Local ベースのアプリディレクトリと、work/ 以下の各 sub-dir を管理する。

use std::path::{Path, PathBuf};

// ─── AppData Local ────────────────────────────────────────────────────────────

/// `%LOCALAPPDATA%\claude-chat-code\` を返す
pub fn app_dir() -> PathBuf {
    let local = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| {
            // fallback: %USERPROFILE%\AppData\Local
            let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
            format!(r"{}\AppData\Local", home)
        });
    PathBuf::from(local).join("claude-chat-code")
}

/// `<app_dir>\config.toml`
pub fn config_path() -> PathBuf {
    app_dir().join("config.toml")
}

/// `<app_dir>\logs\`
pub fn logs_dir() -> PathBuf {
    app_dir().join("logs")
}

/// `<app_dir>\logs\log.txt`
pub fn log_file() -> PathBuf {
    logs_dir().join("log.txt")
}

// ─── work ディレクトリ ────────────────────────────────────────────────────────

/// `<app_dir>\work\`  ← これまでの claude-chat-code-work/ に相当する
pub fn work_dir() -> PathBuf {
    app_dir().join("work")
}

/// `<work>\archives\`
pub fn archives_dir(work: &Path) -> PathBuf {
    work.join("archives")
}

/// `<work>\project\`
pub fn project_dir(work: &Path) -> PathBuf {
    work.join("project")
}

/// `<work>\backup\`
pub fn backup_root(work: &Path) -> PathBuf {
    work.join("backup")
}

/// `<work>\project\src\`
pub fn src_dir(work: &Path) -> PathBuf {
    project_dir(work).join("src")
}

// ─── Desktop（監視対象デフォルト） ────────────────────────────────────────────

/// Windows Desktop のパスを解決する。OneDrive Desktop にも対応する。
pub fn resolve_desktop() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());

    let standard = PathBuf::from(&home).join("Desktop");
    if standard.exists() {
        return standard;
    }
    let onedrive = PathBuf::from(&home).join("OneDrive").join("Desktop");
    if onedrive.exists() {
        return onedrive;
    }
    standard
}

// ─── 監視対象ラベル ────────────────────────────────────────────────────────────

/// 監視対象ディレクトリの表示ラベルを返す。
/// Windows Desktop（standard / OneDrive どちらも）なら "Desktop"、それ以外はフルパス。
pub fn watch_dir_label(watch_dir: &std::path::Path) -> String {
    // 判定対象の Desktop 候補を列挙する
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());

    let candidates = [
        std::path::PathBuf::from(&home).join("Desktop"),
        std::path::PathBuf::from(&home).join("OneDrive").join("Desktop"),
    ];

    for candidate in &candidates {
        // 正規化して比較（大文字小文字・末尾スラッシュ等の揺れを吸収する）
        if same_path(watch_dir, candidate) {
            return "Desktop".to_string();
        }
    }

    watch_dir.to_string_lossy().to_string()
}

/// 2つのパスが同じディレクトリを指しているか判定する。
/// canonicalize が失敗した場合は文字列比較にフォールバックする。
fn same_path(a: &std::path::Path, b: &std::path::Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(ca), Ok(cb)) => ca == cb,
        _ => a == b,
    }
}
