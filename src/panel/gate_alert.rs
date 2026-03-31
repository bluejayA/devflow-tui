use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
use ratatui::Frame;

use crate::action::Action;
use crate::component::Component;
use crate::event::AppEvent;
use crate::ui::theme::{self, Theme};

/// FR-6: 게이트 알림 패널.
///
/// Stop hook의 last_assistant_message에서 A)/B)/C) 패턴을 감지하여
/// 게이트 대기 상태를 시각적으로 표시한다.
pub struct GateAlertPanel {
    active: bool,
    gate_text: String,
    choices: Vec<String>,
    hooks_active: bool,
    next_steps: Vec<String>,
}

impl Default for GateAlertPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl GateAlertPanel {
    pub fn new() -> Self {
        Self {
            active: false,
            gate_text: String::new(),
            choices: Vec::new(),
            hooks_active: false,
            next_steps: Vec::new(),
        }
    }

    pub fn set_hooks_active(&mut self, active: bool) {
        self.hooks_active = active;
    }

    pub fn set_next_steps(&mut self, steps: Vec<String>) {
        self.next_steps = steps;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    fn clear_gate(&mut self) {
        self.active = false;
        self.gate_text.clear();
        self.choices.clear();
    }

    fn activate_gate(&mut self, text: String, choices: Vec<String>) {
        self.active = true;
        self.gate_text = text;
        self.choices = choices;
    }
}

/// Detect gate pattern in a message.
///
/// Looks for sequential uppercase letter choices (A→B→C...) in Stop hook messages.
/// Supports both multi-line and inline patterns (e.g. "A) Yes B) No" on one line).
/// Returns (gate summary text, list of choices) if found.
pub(crate) fn detect_gate(message: &str) -> Option<(String, Vec<String>)> {
    let mut choices = Vec::new();
    let mut gate_lines = Vec::new();

    for line in message.lines() {
        let trimmed = line.trim();
        let line_choices = extract_choices_from_line(trimmed);
        if line_choices.is_empty() {
            if !trimmed.is_empty() {
                gate_lines.push(trimmed.to_string());
            }
        } else {
            choices.extend(line_choices);
        }
    }

    // Need at least A) and B), and must be sequential starting from A
    if choices.len() >= 2 && validate_sequential(&choices) {
        let summary = gate_lines.last().cloned().unwrap_or_default();
        Some((summary, choices))
    } else {
        None
    }
}

/// Extract all choices from a single line.
/// Handles both "A) text" (single) and "A) text B) text" (inline) patterns.
fn extract_choices_from_line(line: &str) -> Vec<String> {
    let mut choices = Vec::new();
    let mut remaining = line;

    while let Some((choice, rest)) = try_extract_next_choice(remaining) {
        choices.push(choice);
        remaining = rest;
    }

    choices
}

/// Try to extract the next choice from the beginning of a string.
/// Returns (formatted choice, remaining text after this choice).
fn try_extract_next_choice(s: &str) -> Option<(String, &str)> {
    // Find the next "X)" pattern where X is uppercase ASCII
    let bytes = s.as_bytes();
    for i in 0..bytes.len().saturating_sub(1) {
        let ch = bytes[i];
        let next = bytes[i + 1];

        if ch.is_ascii_uppercase() && next == b')' {
            // Check it's at start of string or preceded by whitespace/bold markers
            if i > 0 {
                let prev = bytes[i - 1];
                if prev != b' ' && prev != b'*' && prev != b'\t' {
                    continue;
                }
            }

            let letter = ch as char;

            // Find the text after "X)" up to the next choice or end of string
            let after_paren = &s[i + 2..];
            let text_start = after_paren
                .find(|c: char| c != '*' && c != ' ')
                .unwrap_or(after_paren.len());
            let text_part = &after_paren[text_start..];

            // Look for the next choice pattern to delimit this choice's text
            let end_pos = find_next_choice_start(text_part);
            let choice_text = text_part[..end_pos].trim();
            let rest = &text_part[end_pos..];

            return Some((format!("{letter}) {choice_text}"), rest));
        }
    }
    None
}

/// Find the start position of the next "X)" pattern in text.
fn find_next_choice_start(s: &str) -> usize {
    let bytes = s.as_bytes();
    for i in 0..bytes.len().saturating_sub(1) {
        let ch = bytes[i];
        let next = bytes[i + 1];

        if ch.is_ascii_uppercase() && next == b')' {
            // Must be preceded by whitespace or bold markers (or start of string won't happen here)
            if i > 0 {
                let prev = bytes[i - 1];
                if prev == b' ' || prev == b'*' || prev == b'\t' {
                    // Back up to include the letter
                    return i;
                }
            }
        }
    }
    s.len()
}

/// Validate that choices are sequential starting from 'A'.
fn validate_sequential(choices: &[String]) -> bool {
    choices.iter().enumerate().all(|(i, choice)| {
        let expected = (b'A' + i as u8) as char;
        choice.starts_with(&format!("{expected})"))
    })
}

impl Component for GateAlertPanel {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Esc => {
                self.clear_gate();
                None
            }
            _ => None,
        }
    }

    fn handle_event(&mut self, event: &AppEvent) {
        match event {
            AppEvent::TurnCompleted { last_message } => {
                if let Some((text, choices)) = detect_gate(last_message) {
                    self.activate_gate(text, choices);
                } else {
                    // Non-gate turn → clear any existing gate
                    self.clear_gate();
                }
            }
            AppEvent::ToolUseCompleted { tool_name } if tool_name == "AskUserQuestion" => {
                if !self.active {
                    self.activate_gate(
                        "사용자 입력 대기 중".to_string(),
                        Vec::new(),
                    );
                }
            }
            AppEvent::FlowStateChanged(_) => {
                // Stage change → gate no longer relevant
                self.clear_gate();
            }
            AppEvent::HooksServerStarted { .. } => {
                self.hooks_active = true;
            }
            AppEvent::HooksServerFailed { .. } => {
                self.hooks_active = false;
            }
            AppEvent::SessionSummaryChanged(summary) => {
                self.next_steps = summary.next_steps.clone();
            }
            _ => {}
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
            .title(theme::panel_title("Gate Alert", focused))
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if !self.hooks_active {
            let mut lines = vec![
                Line::from(""),
                Line::from("  게이트 감지를 위해").dim(),
                Line::from("  hooks 설정이 필요합니다").dim(),
            ];
            if !self.next_steps.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from("  Next Steps:").dim());
                for step in &self.next_steps {
                    lines.push(Line::from(format!("    {step}")).dim());
                }
            }
            frame.render_widget(Paragraph::new(lines), inner);
            return;
        }

        if !self.active {
            let msg = Paragraph::new(Line::from("  대기 중...").dim());
            frame.render_widget(msg, inner);
            return;
        }

        // Active gate
        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line::from(vec![
            Span::from("  ▶ GATE ").style(Theme::gate_alert()),
        ]));
        lines.push(Line::from(""));

        if !self.gate_text.is_empty() {
            lines.push(Line::from(format!("  {}", self.gate_text)).bold());
            lines.push(Line::from(""));
        }

        for choice in &self.choices {
            lines.push(Line::from(format!("  {choice}")).style(Theme::gate_alert()));
        }

        if self.choices.is_empty() {
            lines.push(Line::from("  사용자 입력 대기 중").style(Theme::active()));
        }

        let widget = Paragraph::new(lines).wrap(Wrap { trim: false });
        frame.render_widget(widget, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::{FlowState, SessionSummary};
    use crate::test_helpers::{buffer_contains_str, render_component};

    // ── Step 1: detect_gate tests ──

    #[test]
    fn test_detect_gate_ab_pattern() {
        let msg = "Choose one:\nA) First option\nB) Second option";
        let result = detect_gate(msg);
        assert!(result.is_some());
        let (_, choices) = result.unwrap();
        assert_eq!(choices.len(), 2);
        assert!(choices[0].starts_with("A)"));
        assert!(choices[1].starts_with("B)"));
    }

    #[test]
    fn test_detect_gate_abc_pattern() {
        let msg = "A) Option A\nB) Option B\nC) Option C";
        let result = detect_gate(msg);
        assert!(result.is_some());
        let (_, choices) = result.unwrap();
        assert_eq!(choices.len(), 3);
    }

    #[test]
    fn test_detect_gate_no_pattern() {
        let msg = "This is a normal message without any gate choices.";
        assert!(detect_gate(msg).is_none());
    }

    #[test]
    fn test_detect_gate_single_choice_not_gate() {
        let msg = "A) Only one choice is not a gate";
        assert!(detect_gate(msg).is_none());
    }

    #[test]
    fn test_detect_gate_bold_markdown() {
        let msg = "**A)** Bold option\n**B)** Another";
        let result = detect_gate(msg);
        assert!(result.is_some());
        let (_, choices) = result.unwrap();
        assert_eq!(choices.len(), 2);
    }

    // ── Step 2: Event handling tests ──

    #[test]
    fn test_turn_completed_activates_gate() {
        let mut panel = GateAlertPanel::new();
        panel.set_hooks_active(true);
        panel.handle_event(&AppEvent::TurnCompleted {
            last_message: "Choose:\nA) Yes\nB) No".to_string(),
        });
        assert!(panel.is_active());
        assert_eq!(panel.choices.len(), 2);
    }

    #[test]
    fn test_turn_completed_non_gate_clears() {
        let mut panel = GateAlertPanel::new();
        panel.set_hooks_active(true);
        // First activate a gate
        panel.handle_event(&AppEvent::TurnCompleted {
            last_message: "A) Yes\nB) No".to_string(),
        });
        assert!(panel.is_active());
        // Then a non-gate message clears it
        panel.handle_event(&AppEvent::TurnCompleted {
            last_message: "Done, no choices here.".to_string(),
        });
        assert!(!panel.is_active());
    }

    #[test]
    fn test_stage_change_clears_gate() {
        let mut panel = GateAlertPanel::new();
        panel.activate_gate("test".to_string(), vec!["A) x".to_string()]);
        panel.handle_event(&AppEvent::FlowStateChanged(FlowState::default()));
        assert!(!panel.is_active());
    }

    #[test]
    fn test_esc_clears_gate() {
        let mut panel = GateAlertPanel::new();
        panel.activate_gate("test".to_string(), vec!["A) x".to_string(), "B) y".to_string()]);
        panel.handle_key(KeyEvent::from(KeyCode::Esc));
        assert!(!panel.is_active());
    }

    #[test]
    fn test_ask_user_question_activates() {
        let mut panel = GateAlertPanel::new();
        panel.handle_event(&AppEvent::ToolUseCompleted {
            tool_name: "AskUserQuestion".to_string(),
        });
        assert!(panel.is_active());
    }

    #[test]
    fn test_session_summary_updates_next_steps() {
        let mut panel = GateAlertPanel::new();
        let summary = SessionSummary {
            next_steps: vec!["Do something".to_string()],
            ..Default::default()
        };
        panel.handle_event(&AppEvent::SessionSummaryChanged(summary));
        assert_eq!(panel.next_steps.len(), 1);
    }

    // ── Step 3: Render tests ──

    #[test]
    fn render_hooks_inactive() {
        let mut panel = GateAlertPanel::new();
        let terminal = render_component(&mut panel, 50, 12, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "hooks 설정이 필요합니다"));
    }

    #[test]
    fn render_hooks_inactive_with_next_steps() {
        let mut panel = GateAlertPanel::new();
        panel.set_next_steps(vec!["Phase 4 시작".to_string()]);
        let terminal = render_component(&mut panel, 50, 12, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Next Steps:"));
        assert!(buffer_contains_str(buf, "Phase 4"));
    }

    #[test]
    fn render_no_gate() {
        let mut panel = GateAlertPanel::new();
        panel.set_hooks_active(true);
        let terminal = render_component(&mut panel, 50, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "대기 중..."));
    }

    #[test]
    fn render_active_gate() {
        let mut panel = GateAlertPanel::new();
        panel.set_hooks_active(true);
        panel.activate_gate(
            "Choose an option".to_string(),
            vec!["A) First".to_string(), "B) Second".to_string()],
        );
        let terminal = render_component(&mut panel, 50, 12, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "▶"));
        assert!(buffer_contains_str(buf, "GATE"));
        assert!(buffer_contains_str(buf, "A) First"));
        assert!(buffer_contains_str(buf, "B) Second"));
    }

    #[test]
    fn render_generic_gate_ask_user() {
        let mut panel = GateAlertPanel::new();
        panel.set_hooks_active(true);
        panel.activate_gate("사용자 입력 대기 중".to_string(), Vec::new());
        let terminal = render_component(&mut panel, 50, 12, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "▶"));
        assert!(buffer_contains_str(buf, "사용자 입력 대기 중"));
    }

    // ── I1: inline pattern tests ──

    #[test]
    fn test_detect_gate_inline() {
        let msg = "A) Yes B) No";
        let result = detect_gate(msg);
        assert!(result.is_some());
        let (_, choices) = result.unwrap();
        assert_eq!(choices.len(), 2);
        assert!(choices[0].starts_with("A)"));
        assert!(choices[1].starts_with("B)"));
    }

    #[test]
    fn test_detect_gate_inline_three() {
        let msg = "A) Option one B) Option two C) Option three";
        let result = detect_gate(msg);
        assert!(result.is_some());
        let (_, choices) = result.unwrap();
        assert_eq!(choices.len(), 3);
    }

    // ── I2: sequential validation tests ──

    #[test]
    fn test_detect_gate_non_sequential_rejected() {
        // "I) went to store" + "Z) something" should NOT be a gate
        let msg = "I) went to the store\nZ) something else";
        assert!(detect_gate(msg).is_none());
    }

    #[test]
    fn test_detect_gate_must_start_with_a() {
        let msg = "B) Second\nC) Third";
        assert!(detect_gate(msg).is_none());
    }

    #[test]
    fn test_detect_gate_sequential_abc() {
        let msg = "A) First\nB) Second\nC) Third";
        let result = detect_gate(msg);
        assert!(result.is_some());
        let (_, choices) = result.unwrap();
        assert_eq!(choices.len(), 3);
    }
}
