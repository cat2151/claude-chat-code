mod app;
mod build;
mod clipboard;
mod config;
mod fs;
mod logger;
mod paths;
mod pipeline;
mod ui;
mod updater;
mod watcher;

#[cfg(test)]
mod tests;

use anyhow::Result;
use app::{AppState, AppStatus};
use build::spawn_cargo_run;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fs::archive::list_archives;
use fs::backup::list_backups;
use fs::ops::{ensure_base_dirs, inspect_src};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, path::PathBuf, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use watcher::{dir_mtime, find_latest_zip, has_changed, list_files};

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Some(msg) = UPDATE_MESSAGE.get() {
        println!("{}", msg);
    }

    result
}

async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    // ── Logger / Config ───────────────────────────────────────────────────────
    let logger = logger::Logger::new()?;

    let cfg = match config::load_or_init() {
        Ok(c)  => { logger.log(&format!("config: {}", paths::config_path().display())); c }
        Err(e) => { logger.log(&format!("config error: {}", e)); config::Config::default_config() }
    };

    let watch_dir      = cfg.resolve_watch_dir();
    let watch_interval = cfg.resolve_watch_interval();
    let work           = paths::work_dir();

    logger.log(&format!("local hash: {}", updater::LOCAL_HASH));
    logger.log(&format!("watch_dir:  {}", watch_dir.display()));
    logger.log(&format!("interval:   {}ms", watch_interval.as_millis()));

    // ── 初期状態 ─────────────────────────────────────────────────────────────
    let state = Arc::new(Mutex::new(AppState::new(logger)));
    {
        let mut st = state.lock().await;
        match ensure_base_dirs(&work) {
            Ok(_)  => st.push_log(format!("work dir 準備完了: {}", work.display())),
            Err(e) => st.push_log(format!("work dir 作成失敗: {}", e)),
        }
        st.watch_dir_label = paths::watch_dir_label(&watch_dir);
        let label = st.watch_dir_label.clone();
        st.push_log(format!("監視対象: {}", label));
        st.push_log(format!("build:    {}", updater::LOCAL_HASH));

        st.file_list     = list_files(&watch_dir);
        st.desktop_mtime = dir_mtime(&watch_dir);
        st.backup_list   = list_backups(&paths::backup_root(&work));
        st.archives_list = list_archives(&paths::archives_dir(&work));

        let stats = inspect_src(&paths::src_dir(&work));
        st.push_log(format!(
            "src 調査: {} ファイル / 最大 {} 行 ({})",
            stats.file_count, stats.max_lines, stats.max_lines_file
        ));
        st.src_stats = Some(stats);

        if let Some(zip) = find_latest_zip(&st.file_list) {
            st.push_log(format!("起動時 ZIP 検出: {} → 確認ダイアログ表示", zip));
            st.startup_zip_dialog = Some(zip);
        }
    }

    // ── update check タスク ───────────────────────────────────────────────────
    {
        let state2 = Arc::clone(&state);
        tokio::spawn(async move {
            if let Some(remote) = updater::fetch_remote_hash().await {
                let mut st = state2.lock().await;
                st.push_log(format!("update check: remote={}", &remote[..8]));
                st.remote_hash = Some(remote);
            }
        });
    }

    // ── 監視タスク ────────────────────────────────────────────────────────────
    {
        let state2     = Arc::clone(&state);
        let watch_dir2 = watch_dir.clone();
        let work2      = work.clone();
        tokio::spawn(async move {
            watch_loop(state2, watch_dir2, work2, watch_interval).await;
        });
    }

    // ── 描画 + キー入力ループ ─────────────────────────────────────────────────
    loop {
        {
            let st = state.lock().await;
            terminal.draw(|f| ui::draw(f, &st))?;
        }

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }

                let dialog_zip = state.lock().await.startup_zip_dialog.clone();

                if let Some(zip_name) = dialog_zip {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            {
                                let mut st = state.lock().await;
                                st.startup_zip_dialog = None;
                                st.push_log(format!("y → ZIP 処理開始: {}", zip_name));
                            }
                            let state2     = Arc::clone(&state);
                            let watch_dir2 = watch_dir.clone();
                            let work2      = work.clone();
                            tokio::spawn(async move {
                                pipeline::process_zip(&state2, &watch_dir2, &work2, &zip_name).await;
                            });
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Enter => {
                            let mut st = state.lock().await;
                            st.startup_zip_dialog = None;
                            st.push_log("N → 起動時 ZIP をスキップ".to_string());
                        }
                        KeyCode::F(5) => restart_cargo_run(&state, &work).await,
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            let log_path = paths::log_file();
                            match clipboard::copy_log_to_clipboard(&log_path) {
                                Ok(_)  => state.lock().await.push_log("ログをクリップボードにコピーしました".to_string()),
                                Err(e) => state.lock().await.push_log(format!("クリップボードコピー失敗: {}", e)),
                            }
                        }
                        KeyCode::F(5) => restart_cargo_run(&state, &work).await,
                        _ => {}
                    }
                }
            }
        }
    }

    // ── update 案内 ───────────────────────────────────────────────────────────
    let remote_hash = state.lock().await.remote_hash.clone();
    if updater::needs_update(remote_hash.as_deref()) {
        if let Some(ref remote) = remote_hash {
            UPDATE_MESSAGE.set(updater::update_message(remote)).ok();
        }
    }

    Ok(())
}

static UPDATE_MESSAGE: std::sync::OnceLock<String> = std::sync::OnceLock::new();

fn can_restart_cargo_run(state: &AppState) -> bool {
    !state.cargo_run_launching
        && matches!(
            state.status,
            AppStatus::Watching | AppStatus::Done(_) | AppStatus::Error(_)
        )
}

fn prepare_cargo_run_restart(state: &mut AppState) -> bool {
    if !can_restart_cargo_run(state) {
        state.push_log("F5 は現在の処理中は無効です".to_string());
        return false;
    }

    state.cargo_run_launching = true;
    state.push_log("F5 → cargo run を起動します".to_string());
    state.set_status(AppStatus::Building);
    true
}

async fn restart_cargo_run(state: &Arc<Mutex<AppState>>, work: &std::path::Path) {
    {
        let mut st = state.lock().await;
        if !prepare_cargo_run_restart(&mut st) {
            return;
        }
    }

    let state2 = Arc::clone(state);
    let project_dir = paths::project_dir(work);
    tokio::spawn(async move {
        let result = tokio::task::spawn_blocking(move || spawn_cargo_run(&project_dir)).await;
        finish_cargo_run_restart(&state2, result).await;
    });
}

async fn finish_cargo_run_restart(
    state: &Arc<Mutex<AppState>>,
    result: Result<Result<String, anyhow::Error>, tokio::task::JoinError>,
) {
    let mut st = state.lock().await;
    st.cargo_run_launching = false;

    match result {
        Ok(Ok(terminal)) => {
            st.push_log(format!("cargo run 起動完了 ({terminal})"));
            st.set_status(AppStatus::Done("cargo run を起動しました".to_string()));
        }
        Ok(Err(e)) => {
            st.push_log(format!("cargo run 起動失敗: {}", e));
            st.set_status(AppStatus::Error(e.to_string()));
        }
        Err(e) => {
            st.push_log(format!("cargo run 起動タスクエラー: {}", e));
            st.set_status(AppStatus::Error(e.to_string()));
        }
    }
}

// ─── 監視ループ ───────────────────────────────────────────────────────────────

async fn watch_loop(
    state: Arc<Mutex<AppState>>,
    watch_dir: PathBuf,
    work: PathBuf,
    interval: Duration,
) {
    loop {
        tokio::time::sleep(interval).await;

        {
            let st = state.lock().await;
            if st.startup_zip_dialog.is_some() { continue; }
        }

        let prev    = state.lock().await.desktop_mtime;
        let current = dir_mtime(&watch_dir);

        if !has_changed(prev, current) { continue; }

        let files    = list_files(&watch_dir);
        let zip_name = find_latest_zip(&files);

        {
            let mut st = state.lock().await;
            st.desktop_mtime = current;
            st.file_list     = files.clone();
            if let Some(ref z) = zip_name {
                st.push_log(format!("変化検出 → ZIP 発見: {}", z));
            } else {
                st.push_log(format!("変化検出。ファイル数: {}", files.len()));
            }
        }

        if let Some(zip) = zip_name {
            pipeline::process_zip(&state, &watch_dir, &work, &zip).await;
        }
    }
}
