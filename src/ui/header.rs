use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::ui::theme::Theme;

/// Render the header bar at the top of the screen.
pub fn render(
    frame: &mut Frame,
    area: Rect,
    phase: &str,
    hooks_active: bool,
    hooks_port: Option<u16>,
) {
    let app_name = Span::from(" devflow-tui ").style(Theme::highlight());

    let phase_span = Span::from(format!(" Phase: {phase} ")).style(Theme::focus_border());

    let hooks_span = if hooks_active {
        let port_str = hooks_port
            .map(|p| format!(" {p}"))
            .unwrap_or_default();
        Span::from(format!(" Hooks: ●{port_str} ")).style(Theme::done())
    } else {
        Span::from(" Hooks: ○ ").style(Theme::error())
    };

    // Calculate padding — use Span::width() for correct unicode display width
    let left_len = app_name.width();
    let separator_len = 2; // "──"
    let right_len = phase_span.width() + hooks_span.width();
    let total = area.width as usize;
    let pad = total.saturating_sub(left_len + right_len + separator_len);

    let line = Line::from(vec![
        app_name,
        Span::from("─".repeat(pad)),
        phase_span,
        Span::from("──"),
        hooks_span,
    ]);

    let widget = Paragraph::new(line).style(ratatui::style::Style::new().add_modifier(Modifier::DIM));
    frame.render_widget(widget, area);
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::{buffer_contains_str, render_with};

    #[test]
    fn render_default_state() {
        let terminal = render_with(80, 1, |frame, area| {
            super::render(frame, area, "INCEPTION", false, None);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "devflow-tui"));
        assert!(buffer_contains_str(buf, "Phase:"));
        assert!(buffer_contains_str(buf, "Hooks: ○"));
    }

    #[test]
    fn render_hooks_active_with_port() {
        let terminal = render_with(80, 1, |frame, area| {
            super::render(frame, area, "CONSTRUCTION", true, Some(9464));
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Hooks: ●"));
        assert!(buffer_contains_str(buf, "9464"));
    }

    #[test]
    fn render_hooks_active_no_port() {
        let terminal = render_with(80, 1, |frame, area| {
            super::render(frame, area, "INCEPTION", true, None);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Hooks: ●"));
        assert!(!buffer_contains_str(buf, "9464"));
    }

    #[test]
    fn render_long_phase_no_panic() {
        let terminal = render_with(80, 1, |frame, area| {
            super::render(frame, area, "VERY_LONG_PHASE_NAME_TESTING", false, None);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "devflow-tui"));
    }

    #[test]
    fn render_narrow_terminal() {
        // width=40 should not panic
        let terminal = render_with(40, 1, |frame, area| {
            super::render(frame, area, "INCEPTION", true, Some(8080));
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "devflow-tui"));
    }
}
