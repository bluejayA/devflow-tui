use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use ratatui::Frame;

use crate::action::Action;
use crate::component::Component;
use crate::event::AppEvent;
use crate::parser::models::AuditEntry;
use crate::ui::theme::{self, Theme};

const DEFAULT_BUFFER_CAP: usize = 1000;
const MIN_BUFFER_CAP: usize = 100;
const MAX_BUFFER_CAP: usize = 100_000;

pub struct AuditLogPanel {
    entries: Vec<AuditEntry>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    auto_scroll: bool,
    buffer_cap: usize,
}

impl Default for AuditLogPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditLogPanel {
    pub fn new() -> Self {
        let cap = std::env::var("DEVFLOW_TUI_LOG_BUFFER")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(DEFAULT_BUFFER_CAP);
        Self::new_with_cap(cap)
    }

    pub fn new_with_cap(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            list_state: ListState::default(),
            scrollbar_state: ScrollbarState::default(),
            auto_scroll: true,
            buffer_cap: cap.clamp(MIN_BUFFER_CAP, MAX_BUFFER_CAP),
        }
    }

    pub fn set_entries(&mut self, entries: Vec<AuditEntry>) {
        self.entries = entries;
        self.enforce_cap();
        if self.auto_scroll && !self.entries.is_empty() {
            self.list_state.select(Some(self.entries.len() - 1));
        }
        self.scrollbar_state = ScrollbarState::new(self.entries.len())
            .position(self.list_state.selected().unwrap_or(0));
    }

    pub fn clamp_scroll(&mut self) {
        let len = self.entries.len();
        if let Some(selected) = self.list_state.selected()
            && selected >= len
            && len > 0
        {
            self.list_state.select(Some(len - 1));
        }
    }

    fn enforce_cap(&mut self) {
        if self.entries.len() > self.buffer_cap {
            let excess = self.entries.len() - self.buffer_cap;
            self.entries.drain(..excess);
        }
    }

    fn format_entry(entry: &AuditEntry) -> ListItem<'_> {
        let mut spans = Vec::new();

        if let Some(ref ts) = entry.timestamp {
            spans.push(Span::from(format!("  {ts} ")).style(Theme::timestamp()));
        }

        if let Some(ref stage) = entry.stage {
            spans.push(Span::from(stage.as_str()).bold());
        }

        if let Some(ref choice) = entry.choice {
            spans.push(Span::from(" → ").dim());
            let style = if choice.starts_with('B') || choice.contains("Approve") {
                Theme::done()
            } else if choice.contains("Skip") || choice.contains("skip") {
                Theme::skipped()
            } else {
                Theme::active()
            };
            spans.push(Span::from(choice.as_str()).style(style));
        }

        if spans.is_empty() {
            // Raw/unrecognized line
            spans.push(Span::from(format!("  {}", entry.raw_line)).dim());
        }

        ListItem::new(Line::from(spans))
    }
}

impl Component for AuditLogPanel {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        let len = self.entries.len();
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if len > 0 {
                    let cur = self.list_state.selected().unwrap_or(0);
                    let next = (cur + 1).min(len - 1);
                    self.list_state.select(Some(next));
                    self.auto_scroll = next == len - 1;
                    self.scrollbar_state = self.scrollbar_state.position(next);
                }
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if len > 0 {
                    let cur = self.list_state.selected().unwrap_or(0);
                    let next = cur.saturating_sub(1);
                    self.list_state.select(Some(next));
                    self.auto_scroll = false;
                    self.scrollbar_state = self.scrollbar_state.position(next);
                }
                None
            }
            KeyCode::Char('G') => {
                // Jump to bottom, re-enable auto scroll
                if len > 0 {
                    self.list_state.select(Some(len - 1));
                    self.auto_scroll = true;
                    self.scrollbar_state = self.scrollbar_state.position(len - 1);
                }
                None
            }
            _ => None,
        }
    }

    fn handle_event(&mut self, event: &AppEvent) {
        if let AppEvent::AuditLogAppended(entries) = event {
            self.set_entries(entries.clone());
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
            .title(theme::panel_title("Audit Log", focused))
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.entries.is_empty() {
            let msg = ratatui::widgets::Paragraph::new(
                Line::from("  감사 로그 대기 중...").dim(),
            );
            frame.render_widget(msg, inner);
            return;
        }

        let items: Vec<ListItem> = self.entries.iter().map(Self::format_entry).collect();

        let list = List::new(items).highlight_style(Theme::highlight());
        frame.render_stateful_widget(list, inner, &mut self.list_state);

        // Scrollbar
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(scrollbar, area, &mut self.scrollbar_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entries(n: usize) -> Vec<AuditEntry> {
        (0..n)
            .map(|i| AuditEntry {
                timestamp: Some(format!("2026-03-30T14:{i:02}")),
                stage: Some(format!("stage-{i}")),
                choice: Some("B (Approve)".to_string()),
                raw_line: String::new(),
            })
            .collect()
    }

    #[test]
    fn test_buffer_cap() {
        let mut panel = AuditLogPanel::new_with_cap(200);
        assert_eq!(panel.buffer_cap, 200);
        panel.set_entries(make_entries(300));
        assert_eq!(panel.entries.len(), 200);
    }

    #[test]
    fn test_buffer_cap_clamp_lower() {
        let panel = AuditLogPanel::new_with_cap(5);
        assert_eq!(panel.buffer_cap, MIN_BUFFER_CAP);
    }

    #[test]
    fn test_buffer_cap_clamp_upper() {
        let panel = AuditLogPanel::new_with_cap(usize::MAX);
        assert_eq!(panel.buffer_cap, MAX_BUFFER_CAP);
    }

    #[test]
    fn test_auto_scroll() {
        let mut panel = AuditLogPanel::new();
        panel.set_entries(make_entries(5));
        assert_eq!(panel.list_state.selected(), Some(4)); // Last item
        assert!(panel.auto_scroll);
    }

    #[test]
    fn test_manual_scroll_disables_auto() {
        let mut panel = AuditLogPanel::new();
        panel.set_entries(make_entries(10));

        // Scroll up
        panel.handle_key(KeyEvent::from(KeyCode::Char('k')));
        assert!(!panel.auto_scroll);
    }

    #[test]
    fn test_jump_to_bottom_re_enables_auto() {
        let mut panel = AuditLogPanel::new();
        panel.set_entries(make_entries(10));

        panel.handle_key(KeyEvent::from(KeyCode::Char('k')));
        assert!(!panel.auto_scroll);

        panel.handle_key(KeyEvent::from(KeyCode::Char('G')));
        assert!(panel.auto_scroll);
        assert_eq!(panel.list_state.selected(), Some(9));
    }

    #[test]
    fn test_format_entry_with_choice() {
        let entry = AuditEntry {
            timestamp: Some("14:20".to_string()),
            stage: Some("requirements".to_string()),
            choice: Some("B (Approve)".to_string()),
            raw_line: String::new(),
        };
        let _item = AuditLogPanel::format_entry(&entry);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_format_entry_raw() {
        let entry = AuditEntry {
            timestamp: None,
            stage: None,
            choice: None,
            raw_line: "some unrecognized line".to_string(),
        };
        let _item = AuditLogPanel::format_entry(&entry);
    }

    // ── Render tests ──

    use crate::test_helpers::{buffer_contains_str, render_component};

    #[test]
    fn render_empty_shows_waiting() {
        let mut panel = AuditLogPanel::new();
        let terminal = render_component(&mut panel, 50, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "감사 로그 대기 중..."));
    }

    #[test]
    fn render_with_entries() {
        let mut panel = AuditLogPanel::new();
        panel.set_entries(make_entries(3));
        let terminal = render_component(&mut panel, 60, 12, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "stage-0"));
        assert!(buffer_contains_str(buf, "Approve"));
    }

    #[test]
    fn render_focused_border() {
        let mut panel = AuditLogPanel::new();
        let terminal = render_component(&mut panel, 50, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "[ Audit Log ]"));
    }

    #[test]
    fn render_unfocused_border() {
        let mut panel = AuditLogPanel::new();
        let terminal = render_component(&mut panel, 50, 10, false);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "  Audit Log  "));
    }

    #[test]
    fn render_scrollbar_present() {
        let mut panel = AuditLogPanel::new();
        panel.set_entries(make_entries(20));
        let terminal = render_component(&mut panel, 50, 10, true);
        let buf = terminal.backend().buffer();
        // Scrollbar uses block elements like "█" or "▐"
        assert!(
            buffer_contains_str(buf, "█")
                || buffer_contains_str(buf, "▐")
                || buffer_contains_str(buf, "│")
                || buffer_contains_str(buf, "▮")
        );
    }

    #[test]
    fn render_raw_entry() {
        let mut panel = AuditLogPanel::new();
        panel.set_entries(vec![AuditEntry {
            timestamp: None,
            stage: None,
            choice: None,
            raw_line: "some raw log text".to_string(),
        }]);
        let terminal = render_component(&mut panel, 60, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "some raw log text"));
    }
}
