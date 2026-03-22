/// config.toml の読み書きに責任を持つ。
/// 初回起動時に初期ファイルを生成し、以降は読み込んで設定を返す。

use crate::paths;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, time::Duration};

// ─── 設定構造体 ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 監視対象ディレクトリ。None のとき Desktop を使う。
    pub watch_dir: Option<String>,

    /// 監視チェック間隔。"500ms" / "1s" / "0.5s" 形式の文字列。
    /// None のとき 500ms をデフォルトとする。
    pub watch_interval: Option<String>,
}

impl Config {
    pub fn default_config() -> Self {
        Self {
            watch_dir: None,
            watch_interval: None,
        }
    }

    /// 実際に監視するディレクトリを解決して返す
    pub fn resolve_watch_dir(&self) -> PathBuf {
        match &self.watch_dir {
            Some(p) => PathBuf::from(p),
            None    => paths::resolve_desktop(),
        }
    }

    /// チェック間隔を Duration で返す。パース失敗時は 500ms にフォールバック。
    pub fn resolve_watch_interval(&self) -> Duration {
        match &self.watch_interval {
            Some(s) => parse_duration(s).unwrap_or(Duration::from_millis(500)),
            None    => Duration::from_millis(500),
        }
    }
}

// ─── 間隔文字列のパース ───────────────────────────────────────────────────────

/// "500ms" / "1s" / "0.5s" / "1500ms" などを Duration に変換する
pub fn parse_duration(s: &str) -> Option<Duration> {
    let s = s.trim();
    if let Some(ms_str) = s.strip_suffix("ms") {
        let ms: u64 = ms_str.trim().parse().ok()?;
        return Some(Duration::from_millis(ms));
    }
    if let Some(s_str) = s.strip_suffix('s') {
        let secs: f64 = s_str.trim().parse().ok()?;
        return Some(Duration::from_millis((secs * 1000.0) as u64));
    }
    None
}

// ─── 初期 config.toml テンプレート ────────────────────────────────────────────

fn initial_toml() -> &'static str {
    r#"# claude-chat-code 設定ファイル
#
# watch_dir: 監視対象ディレクトリのフルパスを指定する。
#   コメントアウト時は Windows Desktop を監視対象とする。
#   例: watch_dir = "C:\Users\<your name>\Desktop"
#
# watch_dir = "C:\Users\<your name>\Desktop"

# watch_interval: 監視チェック間隔。"500ms" / "1s" / "0.5s" などの形式で指定する。
#   コメントアウト時のデフォルトは 500ms。
#
# watch_interval = "500ms"
"#
}

// ─── 公開 API ─────────────────────────────────────────────────────────────────

/// config.toml が存在しなければ初期ファイルを生成し、読み込んで返す
pub fn load_or_init() -> Result<Config> {
    let path = paths::config_path();
    fs::create_dir_all(paths::app_dir())?;

    if !path.exists() {
        fs::write(&path, initial_toml())?;
    }

    let text = fs::read_to_string(&path)?;
    let cfg: Config = toml::from_str(&text)?;
    Ok(cfg)
}
