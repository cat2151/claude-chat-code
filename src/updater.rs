/// update check に責任を持つ。
/// GitHub API で main の最新 commit hash を取得し、ビルド時の hash と比較する。
///
/// 以下の条件では update check をサイレントにスキップする：
///   - ビルド時 hash が "unknown"（git なし・repository なし）
///   - ネットワークエラー / repository が存在しない（404 等）
///   - レスポンスのパースに失敗
use serde::Deserialize;
use std::process::Command;

const OWNER: &str = "cat2151";
const REPO: &str = "claude-chat-code";
const GIT_URL: &str = "https://github.com/cat2151/claude-chat-code";

/// ビルド時に埋め込まれた commit hash
pub const LOCAL_HASH: &str = env!("BUILD_COMMIT_HASH");

fn install_cmd() -> String {
    format!("cargo install --force --git {GIT_URL}")
}

#[cfg(any(target_os = "windows", test))]
pub(crate) fn start_update_script_args(bat_path: &str) -> Vec<String> {
    vec![
        "/C".to_string(),
        "start".to_string(),
        "".to_string(),
        bat_path.to_string(),
    ]
}

// ─── GitHub API レスポンス（必要なフィールドのみ） ────────────────────────────

#[derive(Deserialize)]
struct CommitResponse {
    sha: String,
}

// ─── 公開 API ─────────────────────────────────────────────────────────────────

/// GitHub の main ブランチの最新 commit hash を取得する。
/// 失敗した場合は None を返す（呼び出し元はスキップとして扱う）。
pub async fn fetch_remote_hash() -> Option<String> {
    // ビルド時 hash が unknown なら repository 未作成とみなしてスキップ
    if LOCAL_HASH == "unknown" {
        return None;
    }

    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/main",
        OWNER, REPO
    );

    let client = reqwest::Client::builder()
        .user_agent(concat!("claude-chat-code/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .ok()?;

    let resp = client.get(&url).send().await.ok()?;

    // 404 など HTTP エラーはスキップ
    if !resp.status().is_success() {
        return None;
    }

    let data: CommitResponse = resp.json().await.ok()?;
    let hash = data.sha.trim().to_string();

    if hash.len() == 40 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(hash)
    } else {
        None
    }
}

/// update があるかを判定する。
/// remote_hash が None（取得失敗）の場合は false を返す。
pub fn needs_update(remote_hash: Option<&str>) -> bool {
    match remote_hash {
        Some(remote) => is_update_available(LOCAL_HASH, remote),
        None => false,
    }
}

pub fn is_update_available(build_hash: &str, remote_hash: &str) -> bool {
    !build_hash.is_empty()
        && build_hash != "unknown"
        && !remote_hash.is_empty()
        && remote_hash != build_hash
}

#[cfg(any(target_os = "windows", test))]
pub fn update_bat_content() -> String {
    format!(
        "@echo off\r\ntimeout /t 3 /nobreak >nul\r\n{cmd}\r\ndel \"%~f0\"\r\n",
        cmd = install_cmd()
    )
}

pub fn run_self_update() -> anyhow::Result<bool> {
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;
        use std::time::{SystemTime, UNIX_EPOCH};

        let pid = std::process::id();
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let bat_path = std::env::temp_dir().join(format!("claude_chat_code_update_{pid}_{ts}.bat"));
        {
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&bat_path)?;
            f.write_all(update_bat_content().as_bytes())?;
        }

        let bat_str = bat_path.to_str().ok_or_else(|| {
            anyhow::anyhow!("temp bat path is not valid UTF-8: {}", bat_path.display())
        })?;
        Command::new("cmd")
            .args(start_update_script_args(bat_str))
            .spawn()?;

        println!("Launching update script: {}", bat_path.display());
        println!("The application will now exit so the file lock is released.");
        Ok(true)
    }

    #[cfg(not(target_os = "windows"))]
    {
        let cmd = install_cmd();
        println!("Running: {cmd}");
        let status = Command::new("cargo")
            .args(["install", "--force", "--git", GIT_URL])
            .status()?;
        if !status.success() {
            anyhow::bail!("cargo install failed with status: {status}");
        }
        Ok(false)
    }
}

/// quit 時に表示する update 案内メッセージを返す
pub fn update_message(remote_hash: &str) -> String {
    format!(
        "\nrepository が update されました。以下のコマンドで update してください：\n\n\
         {repo} update\n\n\
         （内部では `{install_cmd}` を実行します）\n\n\
         local : {local}\n\
         remote: {remote}\n",
        install_cmd = install_cmd(),
        repo = REPO,
        local = &LOCAL_HASH[..8],
        remote = &remote_hash[..8],
    )
}
