//! TUI 描画のエントリーポイント。レイアウト分割と各ペインへの委譲のみを担う。

pub mod age;
pub mod dialog;
pub mod layout;
pub mod panes;
pub mod theme;

use crate::app::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
    Frame,
};
use theme::{padded, Mk};

pub fn draw(f: &mut Frame, app: &AppState) {
    let area = f.area();

    let outer = Block::default()
        .title(Span::styled(
            " claude-chat-code ",
            Style::default().fg(Mk::CYAN).add_modifier(Modifier::BOLD),
        ))
        .title_bottom(
            Line::from(vec![
                Span::styled(" q: quit ", Style::default().fg(Mk::COMMENT)),
                Span::styled(" c: copy log ", Style::default().fg(Mk::COMMENT)),
            ])
            .left_aligned(),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Mk::BORDER))
        .style(Style::default().bg(Mk::BG));
    f.render_widget(outer, area);

    let inner = padded(area, 1);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(50),
            Constraint::Length(3),
            Constraint::Min(4),
        ])
        .split(inner);

    panes::draw_status(f, app, rows[0]);
    draw_upper(f, app, rows[1]);
    panes::draw_src_stats(f, app, rows[2]);
    panes::draw_log(f, app, rows[3]);

    if let Some(ref zip_name) = app.startup_zip_dialog {
        dialog::draw_startup_dialog(f, area, zip_name);
    }
}

fn draw_upper(f: &mut Frame, app: &AppState, area: Rect) {
    // Watch ペインのコンテンツ幅は「timestamp + 2 + filename」
    let watch_names: Vec<String> = app
        .file_list
        .iter()
        .map(|fe| format!("{}  {}", fe.modified.format("%m/%d %H:%M:%S"), fe.name))
        .collect();

    let (watch_w, backup_w, archive_w) = layout::upper_widths(
        area.width,
        &watch_names,
        &app.backup_list,
        &app.archives_list,
        &app.watch_dir_label,
    );

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(watch_w),
            Constraint::Length(backup_w),
            Constraint::Min(archive_w),
        ])
        .split(area);

    panes::draw_file_list(f, app, cols[0]);
    panes::draw_backup_list(f, app, cols[1]);
    panes::draw_archives_list(f, app, cols[2]);
}
