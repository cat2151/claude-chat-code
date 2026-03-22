/// 各ペインの描画に責任を持つ。
/// AppState を読み取るだけで状態変更は行わない。

use super::{age::age_label, theme::{styled_block, Mk}};
use crate::app::AppState;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};

// ─── ステータスバー ───────────────────────────────────────────────────────────

pub fn draw_status(f: &mut Frame, app: &AppState, area: Rect) {
    let para = Paragraph::new(Line::from(Span::styled(
        app.status.label(),
        super::theme::status_style(&app.status),
    )))
    .block(styled_block(" Status "));
    f.render_widget(para, area);
}

// ─── 監視ファイル一覧 ─────────────────────────────────────────────────────────

pub fn draw_file_list(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app.file_list.iter().map(|fe| {
        let (name_style, ts_style) = if fe.is_zip() {
            (
                Style::default().fg(Mk::YELLOW).add_modifier(Modifier::BOLD),
                Style::default().fg(Mk::ORANGE),
            )
        } else {
            (
                Style::default().fg(Mk::FG),
                Style::default().fg(Mk::COMMENT),
            )
        };
        let ts = fe.modified.format("%m/%d %H:%M:%S").to_string();
        ListItem::new(Line::from(vec![
            Span::styled(ts, ts_style),
            Span::raw("  "),
            Span::styled(fe.name.clone(), name_style),
        ]))
        .style(Style::default().bg(Mk::BG))
    }).collect();

    let title = format!(" Watch: {} ", app.watch_dir_label);
    f.render_widget(List::new(items).block(styled_block(&title)), area);
}

// ─── バックアップ一覧（age ラベル付き） ──────────────────────────────────────

pub fn draw_backup_list(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app.backup_list.iter().enumerate().map(|(i, name)| {
        let name_style = if i == 0 {
            Style::default().fg(Mk::GREEN).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Mk::FG)
        };
        let age = age_label(name);
        let age_style = Style::default().fg(Mk::COMMENT);

        ListItem::new(Line::from(vec![
            Span::styled(format!("{:>5} ", age), age_style),
            Span::styled(name.clone(), name_style),
        ]))
        .style(Style::default().bg(Mk::BG))
    }).collect();

    f.render_widget(List::new(items).block(styled_block(" Backups ")), area);
}

// ─── アーカイブ一覧（age ラベル付き） ────────────────────────────────────────

pub fn draw_archives_list(f: &mut Frame, app: &AppState, area: Rect) {
    let items: Vec<ListItem> = app.archives_list.iter().enumerate().map(|(i, name)| {
        let name_style = if i == 0 {
            Style::default().fg(Mk::YELLOW).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Mk::FG)
        };
        let age = age_label(name);
        let age_style = Style::default().fg(Mk::COMMENT);

        ListItem::new(Line::from(vec![
            Span::styled(format!("{:>5} ", age), age_style),
            Span::styled(name.clone(), name_style),
        ]))
        .style(Style::default().bg(Mk::BG))
    }).collect();

    f.render_widget(List::new(items).block(styled_block(" Archives ")), area);
}

// ─── src 調査結果 ─────────────────────────────────────────────────────────────

pub fn draw_src_stats(f: &mut Frame, app: &AppState, area: Rect) {
    let text = match &app.src_stats {
        None => Line::from(Span::styled(
            "  src stats: (project/src/ が見つかりません)",
            Style::default().fg(Mk::COMMENT),
        )),
        Some(s) => Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("{} files", s.file_count),
                Style::default().fg(Mk::CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  /  max ", Style::default().fg(Mk::COMMENT)),
            Span::styled(
                format!("{} lines", s.max_lines),
                Style::default().fg(Mk::YELLOW).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (", Style::default().fg(Mk::COMMENT)),
            Span::styled(s.max_lines_file.clone(), Style::default().fg(Mk::FG)),
            Span::styled(")  /  total ", Style::default().fg(Mk::COMMENT)),
            Span::styled(
                format!("{} lines", s.total_lines),
                Style::default().fg(Mk::CYAN),
            ),
            Span::styled(" (", Style::default().fg(Mk::COMMENT)),
            Span::styled(
                format!("{:.1}KB", s.total_kb),
                Style::default().fg(Mk::GREEN),
            ),
            Span::styled(")", Style::default().fg(Mk::COMMENT)),
        ]),
    };
    f.render_widget(
        Paragraph::new(text).block(styled_block(" Src Stats ")),
        area,
    );
}

// ─── ログ ─────────────────────────────────────────────────────────────────────

pub fn draw_log(f: &mut Frame, app: &AppState, area: Rect) {
    let visible = (area.height as usize).saturating_sub(2);
    let items: Vec<ListItem> = app.log
        .iter()
        .rev()
        .take(visible)
        .rev()
        .map(|l| {
            let style = if l.contains("ERROR") {
                Style::default().fg(Mk::RED)
            } else if l.contains("完了") || l.contains("OK") {
                Style::default().fg(Mk::GREEN)
            } else if l.contains("検出") || l.contains("ZIP") {
                Style::default().fg(Mk::YELLOW)
            } else {
                Style::default().fg(Mk::COMMENT)
            };
            ListItem::new(Span::styled(l.clone(), style))
                .style(Style::default().bg(Mk::BG))
        })
        .collect();

    f.render_widget(
        List::new(items).block(styled_block(" Log ")),
        area,
    );
}
