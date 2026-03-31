use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::ui::theme::key_hint;

/// Render the status bar at the bottom of the screen.
pub fn render(
    frame: &mut Frame,
    area: Rect,
    focus_name: &str,
    phase_stage: &str,
    wide_mode: bool,
) {
    let left_span = Span::from(format!(" [{focus_name}]  {phase_stage}"));

    let mut hints: Vec<Span> = Vec::new();
    hints.extend(key_hint("Tab", "패널"));
    if wide_mode {
        hints.extend(key_hint("C-hjkl", "방향"));
    }
    hints.extend(key_hint("j/k", "스크롤"));
    hints.extend(key_hint("f", "확대"));
    hints.extend(key_hint("Enter", "상세"));
    hints.extend(key_hint("r", "새로고침"));
    hints.extend(key_hint("?", "도움말"));
    hints.extend(key_hint("q", "종료"));

    // Use Span::width() for correct unicode display width
    let right_width: usize = hints.iter().map(|s| s.width()).sum();
    let left_width = left_span.width();
    let total_width = area.width as usize;
    let pad = total_width.saturating_sub(left_width + right_width);

    let mut spans = vec![left_span];
    spans.push(Span::from(" ".repeat(pad)));
    spans.extend(hints);

    let line = Line::from(spans);
    let bar_style = if super::theme::no_color() {
        ratatui::style::Style::default().add_modifier(Modifier::REVERSED)
    } else {
        ratatui::style::Style::new().on_dark_gray().white()
    };
    let widget = Paragraph::new(line).style(bar_style);

    frame.render_widget(widget, area);
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::{buffer_contains_str, render_with};

    #[test]
    fn render_default() {
        let terminal = render_with(160, 1, |frame, area| {
            super::render(frame, area, "Audit Log", "INCEPTION > workspace-detection", false);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "[Audit Log]"));
        assert!(buffer_contains_str(buf, "INCEPTION"));
        assert!(buffer_contains_str(buf, "Tab"));
        assert!(buffer_contains_str(buf, "q"));
    }

    #[test]
    fn render_wide_mode_has_direction_hint() {
        let terminal = render_with(120, 1, |frame, area| {
            super::render(frame, area, "Git", "INCEPTION", true);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "C-hjkl"));
    }

    #[test]
    fn render_narrow_mode_no_direction_hint() {
        let terminal = render_with(80, 1, |frame, area| {
            super::render(frame, area, "Git", "INCEPTION", false);
        });
        let buf = terminal.backend().buffer();
        assert!(!buffer_contains_str(buf, "C-hjkl"));
    }

    #[test]
    fn render_korean_text() {
        let terminal = render_with(160, 1, |frame, area| {
            super::render(frame, area, "Agent", "CONSTRUCTION", false);
        });
        let buf = terminal.backend().buffer();
        // key_hint produces Korean descriptions
        assert!(buffer_contains_str(buf, "패널"));
        assert!(buffer_contains_str(buf, "스크롤"));
    }
}
