/// クリップボード操作に責任を持つ。
/// log.txt の内容を Windows クリップボードにコピーする。

use anyhow::Result;
use std::fs;

/// log.txt の全内容をクリップボードにコピーする
pub fn copy_log_to_clipboard(log_path: &std::path::Path) -> Result<()> {
    let content = fs::read_to_string(log_path)?;
    let mut ctx = arboard::Clipboard::new()?;
    ctx.set_text(content)?;
    Ok(())
}
