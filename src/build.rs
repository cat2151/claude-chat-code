/// cargo run の起動に責任を持つ。
/// TUI とは独立した別プロセスで非同期実行する。
/// wt → pwsh → cmd の順でフォールバックして新ウィンドウを開く。
/// wt / pwsh はフォント設定が引き継がれるため cmd より表示品質が高い。

use anyhow::Result;
use std::{path::Path, process::Stdio};

/// project_dir を cwd として `cargo run` を別プロセスで起動する。
/// 戻り値は実際に使用したターミナル名。TUI プロセスをブロックしない。
pub fn spawn_cargo_run(project_dir: &Path) -> Result<String> {
    if try_wt(project_dir).is_ok() {
        return Ok("wt".to_string());
    }
    if try_pwsh(project_dir).is_ok() {
        return Ok("pwsh".to_string());
    }
    try_cmd(project_dir)?;
    Ok("cmd".to_string())
}

/// Windows Terminal で起動する
fn try_wt(project_dir: &Path) -> Result<()> {
    which("wt")?;
    std::process::Command::new("wt")
        .args(["new-tab", "--", "cargo", "run"])
        .current_dir(project_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

/// PowerShell で起動する
fn try_pwsh(project_dir: &Path) -> Result<()> {
    which("pwsh")?;
    std::process::Command::new("pwsh")
        .args(["-NoExit", "-Command", "cargo run"])
        .current_dir(project_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

/// cmd で起動する（最終フォールバック）
fn try_cmd(project_dir: &Path) -> Result<()> {
    std::process::Command::new("cmd")
        .args(["/c", "start", "cmd", "/k", "cargo run"])
        .current_dir(project_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

/// コマンドが PATH 上に存在するか確認する
fn which(cmd: &str) -> Result<()> {
    let status = std::process::Command::new("where")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("{} not found", cmd))
    }
}
