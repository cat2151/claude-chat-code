//! 起動時 ZIP 確認ダイアログの描画に責任を持つ。
//! overlay として最前面に描画する。

use super::theme::{padded, Mk};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn draw_startup_dialog(f: &mut Frame, screen: Rect, zip_name: &str) {
    let w = 62u16.min(screen.width.saturating_sub(4));
    let h = 7u16;
    let x = screen.x + screen.width.saturating_sub(w) / 2;
    let y = screen.y + screen.height.saturating_sub(h) / 2;
    let area = Rect { x, y, width: w, height: h };

    f.render_widget(Clear, area);

    let block = Block::default()
        .title(Span::styled(
            " 起動時 ZIP を検出 ",
            Style::default().fg(Mk::YELLOW).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Mk::ORANGE))
        .style(Style::default().bg(Mk::DIALOG_BG));
    f.render_widget(block, area);

    let inner = padded(area, 1);
    let text = vec![
        Line::from(vec![
            Span::raw("  "),
            Span::styled(zip_name, Style::default().fg(Mk::YELLOW).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  このZIPを処理対象にしますか？",
            Style::default().fg(Mk::FG),
        )),
        Line::from(vec![
            Span::styled("  [y] ", Style::default().fg(Mk::GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("処理する   ", Style::default().fg(Mk::FG)),
            Span::styled("[N] ", Style::default().fg(Mk::COMMENT).add_modifier(Modifier::BOLD)),
            Span::styled("スキップ", Style::default().fg(Mk::COMMENT)),
        ]),
    ];
    f.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Left)
            .style(Style::default().bg(Mk::DIALOG_BG)),
        inner,
    );
}
