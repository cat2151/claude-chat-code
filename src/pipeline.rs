/// ZIP 検出後のパイプライン全体を統括する。
/// 個々の操作は fs_ops / build に委譲し、ここでは順序制御とログ出力のみを担う。

use crate::{
    app::{AppState, AppStatus},
    build::spawn_cargo_run,
    fs::archive::{list_archives, move_zip},
    fs::backup::{backup_project, list_backups, next_backup_name},
    fs::extract::extract_zip,
    fs::ops::{clean_src_dir, ensure_base_dirs, inspect_src, touch_src_files},
    paths,
};
use std::{path::Path, sync::Arc, time::Duration};
use tokio::sync::Mutex;

macro_rules! step {
    ($app:expr, $status:expr, $log:expr, $result:expr) => {{
        {
            let mut a = $app.lock().await;
            a.set_status($status);
            a.push_log($log);
        }
        match $result {
            Ok(v) => v,
            Err(e) => {
                let mut a = $app.lock().await;
                let msg = format!("ERROR: {}", e);
                a.push_log(msg.clone());
                a.set_status(AppStatus::Error(msg));
                return;
            }
        }
    }};
}

pub async fn process_zip(
    app: &Arc<Mutex<AppState>>,
    watch_dir: &Path,
    work: &Path,
    zip_name: &str,
) {
    // ── 0. work dirs 確保 ──────────────────────────────────────────────────────
    step!(
        app,
        AppStatus::Watching,
        format!("work dirs 確保: {}", work.display()),
        tokio::task::spawn_blocking({ let w = work.to_path_buf(); move || ensure_base_dirs(&w) })
            .await.unwrap_or_else(|e| Err(anyhow::anyhow!("spawn: {}", e)))
    );

    // ── 1. ZIP 検出通知 ────────────────────────────────────────────────────────
    {
        let mut a = app.lock().await;
        a.push_log(format!("ZIP 検出: {}", zip_name));
        a.set_status(AppStatus::ZipDetected(zip_name.to_string()));
    }
    tokio::time::sleep(Duration::from_millis(300)).await;

    // ── 2. ZIP 移動 ────────────────────────────────────────────────────────────
    let archived_zip_path = step!(
        app,
        AppStatus::Moving,
        format!("ZIP を archives/ に移動: {}", zip_name),
        {
            let d = watch_dir.to_path_buf();
            let w = work.to_path_buf();
            let z = zip_name.to_string();
            tokio::task::spawn_blocking(move || move_zip(&d, &w, &z))
                .await.unwrap_or_else(|e| Err(anyhow::anyhow!("spawn: {}", e)))
        }
    );
    {
        let mut a = app.lock().await;
        a.push_log(format!("ZIP 移動完了: {}", archived_zip_path.file_name().unwrap_or_default().to_string_lossy()));
        a.archives_list = list_archives(&paths::archives_dir(work));
    }

    // ── 3. バックアップ ────────────────────────────────────────────────────────
    let src = paths::project_dir(work);
    let dst = paths::backup_root(work).join(next_backup_name());
    step!(
        app,
        AppStatus::BackingUp,
        format!("project/ をバックアップ → {}", dst.display()),
        {
            let s = src.clone(); let d = dst.clone();
            tokio::task::spawn_blocking(move || backup_project(&s, &d))
                .await.unwrap_or_else(|e| Err(anyhow::anyhow!("spawn: {}", e)))
        }
    );
    {
        let mut a = app.lock().await;
        a.push_log(format!("バックアップ完了: {}", dst.display()));
        // backup_list を更新
        a.backup_list = list_backups(&paths::backup_root(work));
    }

    // ── 4. ZIP 展開（事前に project/src/ を削除して旧ファイルの残留を防ぐ） ────
    let src_to_clean = paths::src_dir(work);
    step!(
        app,
        AppStatus::Extracting,
        "project/src/ をクリア中...".to_string(),
        tokio::task::spawn_blocking(move || clean_src_dir(&src_to_clean))
            .await.unwrap_or_else(|e| Err(anyhow::anyhow!("spawn: {}", e)))
    );
    let project = paths::project_dir(work);
    step!(
        app,
        AppStatus::Extracting,
        format!("ZIP を project/ に展開: {}", archived_zip_path.file_name().unwrap_or_default().to_string_lossy()),
        {
            let zp = archived_zip_path.clone(); let pr = project.clone();
            tokio::task::spawn_blocking(move || extract_zip(&zp, &pr))
                .await.unwrap_or_else(|e| Err(anyhow::anyhow!("spawn: {}", e)))
        }
    );
    app.lock().await.push_log("ZIP 展開完了".to_string());

    // ── 5. touch ──────────────────────────────────────────────────────────────
    let src_dir = paths::src_dir(work);
    let count = step!(
        app,
        AppStatus::Touching,
        "project/src/ を touch 中...".to_string(),
        tokio::task::spawn_blocking(move || touch_src_files(&src_dir))
            .await.unwrap_or_else(|e| Err(anyhow::anyhow!("spawn: {}", e)))
    );
    app.lock().await.push_log(format!("touch 完了: {} ファイル", count));

    // ── 6. build 直前ソース調査 ──────────────────────────────────────────────────
    {
        let src_dir = paths::src_dir(work);
        let stats = tokio::task::spawn_blocking(move || inspect_src(&src_dir))
            .await
            .unwrap_or_else(|_| crate::fs::ops::SrcStats {
                file_count: 0, max_lines: 0, max_lines_file: String::new(),
                total_lines: 0, total_kb: 0.0,
            });
        let mut a = app.lock().await;
        a.push_log(format!(
            "src 調査: {} ファイル / 最大 {} 行 ({})",
            stats.file_count, stats.max_lines, stats.max_lines_file
        ));
        a.src_stats = Some(stats);
    }

    // ── 7. cargo run 直前バックアップ ────────────────────────────────────────
    let src2 = paths::project_dir(work);
    let dst2 = paths::backup_root(work).join(next_backup_name());
    step!(
        app,
        AppStatus::BackingUp,
        format!("cargo run 直前バックアップ → {}", dst2.display()),
        {
            let s = src2.clone(); let d = dst2.clone();
            tokio::task::spawn_blocking(move || backup_project(&s, &d))
                .await.unwrap_or_else(|e| Err(anyhow::anyhow!("spawn: {}", e)))
        }
    );
    {
        let mut a = app.lock().await;
        a.push_log(format!("バックアップ完了: {}", dst2.display()));
        a.backup_list = list_backups(&paths::backup_root(work));
    }

    // ── 8. cargo run ──────────────────────────────────────────────────────────
    let proj_build = paths::project_dir(work);
    let terminal = step!(
        app,
        AppStatus::Building,
        "cargo run を別プロセスで起動中...".to_string(),
        tokio::task::spawn_blocking(move || spawn_cargo_run(&proj_build))
            .await.unwrap_or_else(|e| Err(anyhow::anyhow!("spawn: {}", e)))
    );
    {
        let mut a = app.lock().await;
        a.push_log(format!("cargo run 起動完了 ({} で実行中)", terminal));
        a.set_status(AppStatus::Done("cargo run 起動済み。次の ZIP を待機中".to_string()));
    }

    // ── 9. 監視状態に戻る ─────────────────────────────────────────────────────
    tokio::time::sleep(Duration::from_secs(3)).await;
    {
        let mut a = app.lock().await;
        a.set_status(AppStatus::Watching);
        a.push_log("監視再開".to_string());
    }
}
