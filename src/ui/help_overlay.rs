use ratatui::layout::Rect;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::ui::layout::centered_rect;
use crate::ui::theme::Theme;

/// Render the help overlay (? key).
pub fn render(frame: &mut Frame, area: Rect) {
    let modal_area = centered_rect(60, 70, area);

    // Clear background
    frame.render_widget(Clear, modal_area);

    let title_style = Theme::highlight();
    let key_style = Theme::focus_border();
    let dim_style = Theme::waiting();

    let help_text = Text::from(vec![
        Line::from("  Keyboard Shortcuts").style(title_style),
        Line::from(""),
        Line::from(vec![
            "  Tab / Shift+Tab  ".into(),
            "패널 간 순환".into(),
        ]).style(key_style),
        Line::from(vec![
            "  Ctrl+h/j/k/l    ".into(),
            "방향 네비게이션 (와이드)".into(),
        ]).style(key_style),
        Line::from(vec![
            "  j / k  ↑ / ↓    ".into(),
            "리스트 스크롤".into(),
        ]).style(key_style),
        Line::from(vec![
            "  Enter            ".into(),
            "상세 보기 / 산출물 모달".into(),
        ]).style(key_style),
        Line::from(vec![
            "  f                ".into(),
            "패널 전체화면 확대".into(),
        ]).style(key_style),
        Line::from(vec![
            "  Esc              ".into(),
            "복귀 / 모달 닫기".into(),
        ]).style(key_style),
        Line::from(vec![
            "  r                ".into(),
            "수동 새로고침".into(),
        ]).style(key_style),
        Line::from(vec![
            "  ?                ".into(),
            "이 도움말 표시/닫기".into(),
        ]).style(key_style),
        Line::from(vec![
            "  q                ".into(),
            "종료".into(),
        ]).style(key_style),
        Line::from(""),
        Line::from("  Press ? or Esc to close").style(dim_style),
    ]);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("  Help  ")
        .border_style(Theme::focus_border());

    let widget = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(widget, modal_area);
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::{buffer_contains_str, render_with};

    #[test]
    fn render_contains_shortcuts() {
        let terminal = render_with(80, 24, |frame, area| {
            super::render(frame, area);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Tab"));
        assert!(buffer_contains_str(buf, "Enter"));
        assert!(buffer_contains_str(buf, "q"));
        assert!(buffer_contains_str(buf, "Esc"));
    }

    #[test]
    fn render_has_border_and_title() {
        let terminal = render_with(80, 24, |frame, area| {
            super::render(frame, area);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Help"));
        assert!(buffer_contains_str(buf, "╭") || buffer_contains_str(buf, "┌"));
    }

    #[test]
    fn render_small_area_no_panic() {
        // Should not panic even on a small terminal
        let _terminal = render_with(40, 12, |frame, area| {
            super::render(frame, area);
        });
    }
}
