/// update check に責任を持つ。
/// GitHub API で main の最新 commit hash を取得し、ビルド時の hash と比較する。
///
/// 以下の条件では update check をサイレントにスキップする：
///   - ビルド時 hash が "unknown"（git なし・repository なし）
///   - ネットワークエラー / repository が存在しない（404 等）
///   - レスポンスのパースに失敗

use serde::Deserialize;

const OWNER: &str = "cat2151";
const REPO:  &str = "claude-chat-code";

/// ビルド時に埋め込まれた commit hash
pub const LOCAL_HASH: &str = env!("BUILD_COMMIT_HASH");

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
        Some(remote) => LOCAL_HASH != "unknown" && LOCAL_HASH != remote,
        None         => false,
    }
}

/// quit 時に表示する update 案内メッセージを返す
pub fn update_message(remote_hash: &str) -> String {
    format!(
        "\nrepository が update されました。以下のコマンドで update してください：\n\n\
         cargo install --force --git https://github.com/{owner}/{repo}\n\n\
         local : {local}\n\
         remote: {remote}\n",
        owner  = OWNER,
        repo   = REPO,
        local  = &LOCAL_HASH[..8],
        remote = &remote_hash[..8],
    )
}
