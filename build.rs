//! Cargo ビルドスクリプト。
//! ビルド時の git commit hash を環境変数としてアプリに埋め込む。
//! git が使えない・repository がない場合は "unknown" を埋め込む（ビルドは通る）。
fn main() {
    let hash = git_commit_hash().unwrap_or_else(|| "unknown".to_string());
    // アプリ側で env!("BUILD_COMMIT_HASH") で参照できる
    println!("cargo:rustc-env=BUILD_COMMIT_HASH={}", hash);
    // build.rs 自体が変わったときだけ再実行（不要な再ビルドを避ける）
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");
}

fn git_commit_hash() -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let hash = String::from_utf8(output.stdout).ok()?;
    let hash = hash.trim().to_string();

    // 40文字の hex でなければ無効とみなす
    if hash.len() == 40 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(hash)
    } else {
        None
    }
}
