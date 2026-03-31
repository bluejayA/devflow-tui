use std::sync::LazyLock;

use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::Span;

static NO_COLOR: LazyLock<bool> = LazyLock::new(|| std::env::var("NO_COLOR").is_ok());

/// Check if NO_COLOR environment variable is set (cached at first call).
pub fn no_color() -> bool {
    *NO_COLOR
}

/// Color tokens for the application.
pub struct Theme;

impl Theme {
    pub fn active() -> Style {
        if no_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::new().yellow().bold()
        }
    }

    pub fn done() -> Style {
        if no_color() {
            Style::default()
        } else {
            Style::new().green()
        }
    }

    pub fn waiting() -> Style {
        if no_color() {
            Style::default().dim()
        } else {
            Style::new().dark_gray()
        }
    }

    pub fn skipped() -> Style {
        if no_color() {
            Style::default().dim()
        } else {
            Style::new().dark_gray()
        }
    }

    pub fn error() -> Style {
        if no_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::new().red()
        }
    }

    pub fn timeout() -> Style {
        if no_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::new().red()
        }
    }

    pub fn focus_border() -> Style {
        if no_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::new().cyan()
        }
    }

    pub fn unfocus_border() -> Style {
        if no_color() {
            Style::default().dim()
        } else {
            Style::new().dark_gray()
        }
    }

    pub fn disabled() -> Style {
        if no_color() {
            Style::default().dim()
        } else {
            Style::new().dark_gray().dim()
        }
    }

    pub fn highlight() -> Style {
        if no_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::new().white().bold()
        }
    }

    pub fn timestamp() -> Style {
        if no_color() {
            Style::default()
        } else {
            Style::new().cyan()
        }
    }

    pub fn gate_alert() -> Style {
        if no_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::new().yellow().bold()
        }
    }

    pub fn staged() -> Style {
        if no_color() { Style::default() } else { Style::new().green() }
    }

    pub fn unstaged() -> Style {
        if no_color() { Style::default() } else { Style::new().yellow() }
    }

    pub fn untracked() -> Style {
        if no_color() { Style::default().dim() } else { Style::new().dark_gray() }
    }

    pub fn conflict() -> Style {
        if no_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::new().red().bold()
        }
    }
}

/// Create a status span with icon and text, respecting NO_COLOR.
pub fn status_span<'a>(icon: &'a str, text: &'a str, style: Style) -> Span<'a> {
    Span::styled(format!("{icon} {text}"), style)
}

/// Panel title formatting based on focus state.
pub fn panel_title(name: &str, focused: bool) -> String {
    if focused {
        format!("[ {name} ]")
    } else {
        format!("  {name}  ")
    }
}

/// Status icons for NO_COLOR compatibility.
pub struct Icons;

impl Icons {
    pub fn active() -> &'static str { "●" }
    pub fn done() -> &'static str { "✓" }
    pub fn waiting() -> &'static str { "○" }
    pub fn skipped() -> &'static str { "–" }
    pub fn error() -> &'static str { "✗" }
    pub fn timeout() -> &'static str { "⏱" }
    pub fn staged() -> &'static str { "S" }
    pub fn unstaged() -> &'static str { "M" }
    pub fn untracked() -> &'static str { "?" }
    pub fn conflict() -> &'static str { "C!" }
}

/// Key hint spans for status bar.
pub fn key_hint<'a>(key: &'a str, desc: &'a str) -> Vec<Span<'a>> {
    if no_color() {
        vec![
            Span::from(format!(" {key} ")).add_modifier(Modifier::BOLD),
            Span::from(format!("{desc} ")),
        ]
    } else {
        vec![
            Span::from(format!(" {key} ")).cyan().bold(),
            Span::from(format!("{desc} ")).dim(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_title_focused() {
        assert_eq!(panel_title("Git Status", true), "[ Git Status ]");
    }

    #[test]
    fn test_panel_title_unfocused() {
        assert_eq!(panel_title("Git Status", false), "  Git Status  ");
    }

    #[test]
    fn test_status_span_format() {
        let span = status_span("✓", "workspace-detection", Theme::done());
        assert!(span.content.contains("✓ workspace-detection"));
    }

    #[test]
    fn test_key_hint_produces_two_spans() {
        let spans = key_hint("Tab", "패널");
        assert_eq!(spans.len(), 2);
    }

    #[test]
    fn render_status_span_in_buffer() {
        use ratatui::text::Line;
        use ratatui::widgets::Paragraph;
        use crate::test_helpers::{buffer_contains_str, render_with};

        let terminal = render_with(40, 1, |frame, area| {
            let span = status_span(Icons::done(), "workspace-detection", Theme::done());
            let widget = Paragraph::new(Line::from(span));
            frame.render_widget(widget, area);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "✓"));
        assert!(buffer_contains_str(buf, "workspace-detection"));
    }

    #[test]
    fn render_key_hint_in_buffer() {
        use ratatui::text::Line;
        use ratatui::widgets::Paragraph;
        use crate::test_helpers::{buffer_contains_str, render_with};

        let terminal = render_with(20, 1, |frame, area| {
            let spans = key_hint("Tab", "패널");
            let widget = Paragraph::new(Line::from(spans));
            frame.render_widget(widget, area);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Tab"));
        assert!(buffer_contains_str(buf, "패널"));
    }
}
