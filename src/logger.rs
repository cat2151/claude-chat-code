//! ファイルログに責任を持つ。
//! 起動時に log.txt を新規作成し、以降は追記する。
//! TUI が panic しても log.txt は残る。

use crate::paths;
use anyhow::Result;
use chrono::Local;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Logger {
    inner: Arc<Mutex<std::fs::File>>,
}

impl Logger {
    /// logs/ ディレクトリを作り log.txt を起動ごとに新規作成する
    pub fn new() -> Result<Self> {
        fs::create_dir_all(paths::logs_dir())?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(paths::log_file())?;
        let logger = Self { inner: Arc::new(Mutex::new(file)) };
        logger.write_raw("=== claude-chat-code started ===")?;
        Ok(logger)
    }

    /// タイムスタンプ付きで 1 行書き込む
    pub fn log(&self, msg: &str) {
        let ts = Local::now().format("%H:%M:%S").to_string();
        let _ = self.write_raw(&format!("[{}] {}", ts, msg));
    }

    fn write_raw(&self, line: &str) -> Result<()> {
        let mut f = self.inner.lock().unwrap();
        writeln!(f, "{}", line)?;
        // sync_all で OS バッファまで強制書き出し（Windows 対応）
        f.sync_all()?;
        Ok(())
    }
    /// テスト専用：`log.txt` には書かず、一時ファイルに書き込むロガー
    #[cfg(test)]
    pub fn new_noop() -> Self {
        let file = tempfile::tempfile().expect("tempfile");
        Self { inner: std::sync::Arc::new(std::sync::Mutex::new(file)) }
    }
}
