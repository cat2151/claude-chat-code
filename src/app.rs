/// アプリケーション状態の定義。

use crate::{fs::ops::SrcStats, logger::Logger};
use chrono::{DateTime, Local};
use std::time::SystemTime;

// ─── ステータス ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum AppStatus {
    Watching,
    ZipDetected(String),
    Moving,
    BackingUp,
    Extracting,
    Touching,
    Building,
    Error(String),
    Done(String),
}

impl AppStatus {
    pub fn label(&self) -> String {
        match self {
            AppStatus::Watching         => "👁  監視中 : Claude chat からダウンロードした ZIP を監視して自動ビルドします".to_string(),
            AppStatus::ZipDetected(n)   => format!("📦 ZIP検出: {}", n),
            AppStatus::Moving           => "🚚 移動中".to_string(),
            AppStatus::BackingUp        => "💾 バックアップ中".to_string(),
            AppStatus::Extracting       => "📂 展開中".to_string(),
            AppStatus::Touching         => "✏️  touch中".to_string(),
            AppStatus::Building         => "🔨 ビルド起動".to_string(),
            AppStatus::Error(e)         => format!("❌ ERROR: {}", e),
            AppStatus::Done(msg)        => format!("✅ {}", msg),
        }
    }
}

// ─── ファイルエントリー ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub modified: DateTime<Local>,
}

impl FileEntry {
    pub fn is_zip(&self) -> bool {
        self.name.to_lowercase().ends_with(".zip")
    }
}

// ─── アプリ全体の状態 ─────────────────────────────────────────────────────────

pub struct AppState {
    pub status: AppStatus,
    pub log: Vec<String>,
    pub file_list: Vec<FileEntry>,
    pub desktop_mtime: Option<SystemTime>,
    pub backup_list: Vec<String>,
    pub archives_list: Vec<String>,
    pub logger: Logger,
    /// 起動時 ZIP 確認ダイアログ。Some(zip_name) のとき overlay 表示する。
    pub startup_zip_dialog: Option<String>,
    /// update check で取得したリモート hash（None = 未取得 or 取得失敗）
    pub remote_hash: Option<String>,
    /// 監視対象の表示ラベル（Desktop なら "Desktop"、それ以外はフルパス）
    pub watch_dir_label: String,
    /// build 直前のソースコード調査結果
    pub src_stats: Option<SrcStats>,
}

impl AppState {
    pub fn new(logger: Logger) -> Self {
        Self {
            status: AppStatus::Watching,
            log: Vec::new(),
            file_list: Vec::new(),
            desktop_mtime: None,
            backup_list: Vec::new(),
            archives_list: Vec::new(),
            logger,
            startup_zip_dialog: None,
            remote_hash: None,
            watch_dir_label: String::new(),
            src_stats: None,
        }
    }

    pub fn push_log(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        self.logger.log(&msg);
        let ts = Local::now().format("%H:%M:%S").to_string();
        self.log.push(format!("[{}] {}", ts, msg));
        if self.log.len() > 200 {
            self.log.remove(0);
        }
    }

    pub fn set_status(&mut self, s: AppStatus) {
        self.status = s;
    }

    /// テスト専用コンストラクタ（Logger不要）
    #[cfg(test)]
    pub fn new_for_test() -> Self {
        use crate::logger::Logger;
        Self::new(Logger::new_noop())
    }
}
