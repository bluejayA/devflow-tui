use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

use crate::action::Action;
use crate::component::Component;
use crate::event::AppEvent;
use crate::parser::models::TokenSnapshot;
use crate::ui::theme::{self, Theme};

pub struct TokenUsagePanel {
    snapshot: TokenSnapshot,
    scroll_offset: usize,
}

impl Default for TokenUsagePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenUsagePanel {
    pub fn new() -> Self {
        Self {
            snapshot: TokenSnapshot::default(),
            scroll_offset: 0,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: TokenSnapshot) {
        self.snapshot = snapshot;
    }

    fn format_tokens(n: u64) -> String {
        if n >= 1_000_000 {
            format!("{:.1}M", n as f64 / 1_000_000.0)
        } else if n >= 1_000 {
            format!("{:.1}K", n as f64 / 1_000.0)
        } else {
            n.to_string()
        }
    }

    fn format_rate(rate: f64) -> String {
        if !rate.is_finite() || rate <= 0.0 {
            return "—".to_string();
        }
        Self::format_tokens(rate as u64)
    }

    pub fn clamp_scroll(&mut self) {
        let max = self.build_lines().len().saturating_sub(1);
        if self.scroll_offset > max {
            self.scroll_offset = max;
        }
    }

    fn build_lines(&self) -> Vec<Line<'_>> {
        let snap = &self.snapshot;
        let mut lines = Vec::new();

        if snap.turn_count == 0 {
            lines.push(Line::from("  JSONL 대기 중...").dim());
            return lines;
        }

        // Cumulative input tokens
        let input_display = Self::format_tokens(snap.total_input_display());
        lines.push(Line::from(vec![
            Span::from("  Input   ").style(Theme::token_label()),
            Span::from(input_display).style(Theme::token_normal()),
        ]));

        // Breakdown: actual + cache_creation + cache_read
        let actual = Self::format_tokens(snap.cumulative.input_tokens);
        let cache_create = Self::format_tokens(snap.cumulative.cache_creation_input_tokens);
        let cache_read = Self::format_tokens(snap.cumulative.cache_read_input_tokens);
        lines.push(Line::from(vec![
            Span::from("          ").style(Theme::token_label()),
            Span::from(format!("{actual} + {cache_create}c + {cache_read}r")).dim(),
        ]));

        // Output tokens
        let output = Self::format_tokens(snap.cumulative.output_tokens);
        lines.push(Line::from(vec![
            Span::from("  Output  ").style(Theme::token_label()),
            Span::from(output).style(Theme::token_normal()),
        ]));

        // Total billable
        let billable = Self::format_tokens(snap.total_billable());
        lines.push(Line::from(vec![
            Span::from("  Total   ").style(Theme::token_label()),
            Span::from(billable).style(Theme::highlight()),
        ]));

        lines.push(Line::from(""));

        // Burn rate
        let rate = Self::format_rate(snap.burn_rate_per_min);
        lines.push(Line::from(vec![
            Span::from("  Rate    ").style(Theme::token_label()),
            Span::from(format!("{rate}/min")).style(Theme::token_normal()),
        ]));

        // Turn count
        lines.push(Line::from(vec![
            Span::from("  Turns   ").style(Theme::token_label()),
            Span::from(snap.turn_count.to_string()).style(Theme::token_normal()),
        ]));

        lines
    }
}

impl Component for TokenUsagePanel {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
                self.clamp_scroll();
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                None
            }
            _ => None,
        }
    }

    fn handle_event(&mut self, event: &AppEvent) {
        if let AppEvent::TokenUsageUpdated(snapshot) = event {
            self.snapshot = snapshot.clone();
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let border_style = if focused {
            Theme::focus_border()
        } else {
            Theme::unfocus_border()
        };

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(theme::panel_title("Token Usage", focused))
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let lines = self.build_lines();
        let paragraph = Paragraph::new(lines).scroll((self.scroll_offset as u16, 0));
        frame.render_widget(paragraph, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::TokenRecord;
    use crate::test_helpers::{buffer_contains_str, render_component};

    fn make_snapshot(input: u64, output: u64, cache_create: u64, cache_read: u64, turns: u32) -> TokenSnapshot {
        TokenSnapshot {
            cumulative: TokenRecord {
                input_tokens: input,
                output_tokens: output,
                cache_creation_input_tokens: cache_create,
                cache_read_input_tokens: cache_read,
            },
            turn_count: turns,
            burn_rate_per_min: 0.0,
            recent_turns: Vec::new(),
        }
    }

    #[test]
    fn render_empty_shows_waiting() {
        let mut panel = TokenUsagePanel::new();
        let terminal = render_component(&mut panel, 40, 12, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "JSONL"));
    }

    #[test]
    fn render_with_data_shows_tokens() {
        let mut panel = TokenUsagePanel::new();
        panel.set_snapshot(make_snapshot(100_000, 50_000, 200_000, 300_000, 5));

        let terminal = render_component(&mut panel, 45, 14, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Input"));
        assert!(buffer_contains_str(buf, "Output"));
        assert!(buffer_contains_str(buf, "Total"));
        assert!(buffer_contains_str(buf, "Turns"));
    }

    #[test]
    fn render_title_focused() {
        let mut panel = TokenUsagePanel::new();
        let terminal = render_component(&mut panel, 40, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "[ Token Usage ]"));
    }

    #[test]
    fn render_title_unfocused() {
        let mut panel = TokenUsagePanel::new();
        let terminal = render_component(&mut panel, 40, 10, false);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Token Usage"));
    }

    #[test]
    fn test_format_tokens_units() {
        assert_eq!(TokenUsagePanel::format_tokens(0), "0");
        assert_eq!(TokenUsagePanel::format_tokens(500), "500");
        assert_eq!(TokenUsagePanel::format_tokens(999), "999");
        assert_eq!(TokenUsagePanel::format_tokens(1_000), "1.0K");
        assert_eq!(TokenUsagePanel::format_tokens(1_500), "1.5K");
        assert_eq!(TokenUsagePanel::format_tokens(999_999), "1000.0K");
        assert_eq!(TokenUsagePanel::format_tokens(1_000_000), "1.0M");
        assert_eq!(TokenUsagePanel::format_tokens(1_500_000), "1.5M");
    }

    #[test]
    fn test_format_rate_edge_cases() {
        assert_eq!(TokenUsagePanel::format_rate(0.0), "—");
        assert_eq!(TokenUsagePanel::format_rate(-1.0), "—");
        assert_eq!(TokenUsagePanel::format_rate(f64::NAN), "—");
        assert_eq!(TokenUsagePanel::format_rate(f64::INFINITY), "—");
        assert_eq!(TokenUsagePanel::format_rate(1200.5), "1.2K");
    }

    #[test]
    fn test_handle_event_updates_snapshot() {
        let mut panel = TokenUsagePanel::new();
        let snapshot = make_snapshot(100, 50, 0, 0, 1);
        panel.handle_event(&AppEvent::TokenUsageUpdated(snapshot));
        assert_eq!(panel.snapshot.cumulative.input_tokens, 100);
        assert_eq!(panel.snapshot.turn_count, 1);
    }

    #[test]
    fn test_handle_event_ignores_other_events() {
        let mut panel = TokenUsagePanel::new();
        panel.handle_event(&AppEvent::GitPollError {
            error: "test".to_string(),
        });
        assert_eq!(panel.snapshot.turn_count, 0);
    }

    #[test]
    fn render_burn_rate_shown() {
        let mut panel = TokenUsagePanel::new();
        let mut snap = make_snapshot(500, 100, 0, 0, 3);
        snap.burn_rate_per_min = 1200.0;
        panel.set_snapshot(snap);

        let terminal = render_component(&mut panel, 45, 14, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Rate"));
        assert!(buffer_contains_str(buf, "/min"));
    }

    #[test]
    fn test_scroll_keys() {
        let mut panel = TokenUsagePanel::new();
        // Need data for scroll to have effect (empty panel has 1 line max)
        panel.set_snapshot(make_snapshot(100_000, 50_000, 200_000, 300_000, 5));
        assert_eq!(panel.scroll_offset, 0);

        panel.handle_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(panel.scroll_offset, 1);

        panel.handle_key(KeyEvent::from(KeyCode::Char('k')));
        assert_eq!(panel.scroll_offset, 0);

        // Can't go below 0
        panel.handle_key(KeyEvent::from(KeyCode::Char('k')));
        assert_eq!(panel.scroll_offset, 0);
    }
}
