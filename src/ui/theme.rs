/// Monokai カラーパレットと共有ウィジェットヘルパー。
/// 色定義・スタイル・ブロック生成の責任を持つ。

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
};

pub struct Mk;
impl Mk {
    pub const BG:        Color = Color::Rgb(39, 40, 34);
    pub const FG:        Color = Color::Rgb(248, 248, 242);
    pub const COMMENT:   Color = Color::Rgb(117, 113, 94);
    pub const RED:       Color = Color::Rgb(249, 38, 114);
    pub const ORANGE:    Color = Color::Rgb(253, 151, 31);
    pub const YELLOW:    Color = Color::Rgb(230, 219, 116);
    pub const GREEN:     Color = Color::Rgb(166, 226, 46);
    pub const CYAN:      Color = Color::Rgb(102, 217, 239);
    pub const PURPLE:    Color = Color::Rgb(174, 129, 255);
    pub const BORDER:    Color = Color::Rgb(73, 72, 62);
    pub const DIALOG_BG: Color = Color::Rgb(50, 50, 44);
}

/// Monokai テーマ統一ブロック
pub fn styled_block(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(title, Style::default().fg(Mk::CYAN)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Mk::BORDER))
        .style(Style::default().bg(Mk::BG))
}

/// 外側の Rect から n px 内側の Rect を返す
pub fn padded(r: Rect, n: u16) -> Rect {
    Rect {
        x: r.x + n,
        y: r.y + n,
        width:  r.width.saturating_sub(n * 2),
        height: r.height.saturating_sub(n * 2),
    }
}

/// AppStatus に対応する Style を返す
pub fn status_style(s: &crate::app::AppStatus) -> Style {
    use crate::app::AppStatus::*;
    match s {
        Watching         => Style::default().fg(Mk::CYAN),
        ZipDetected(_)   => Style::default().fg(Mk::YELLOW).add_modifier(Modifier::BOLD),
        Moving | BackingUp | Extracting | Touching
                         => Style::default().fg(Mk::PURPLE),
        Building         => Style::default().fg(Mk::ORANGE).add_modifier(Modifier::BOLD),
        Error(_)         => Style::default().fg(Mk::RED).add_modifier(Modifier::BOLD),
        Done(_)          => Style::default().fg(Mk::GREEN),
    }
}
