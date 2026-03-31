use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::action::Action;
use crate::component::Component;
use crate::event::AppEvent;
use crate::ui::theme::{self, Icons, Theme};

const ORPHAN_TIMEOUT_SECS: u64 = 60;
const MAX_HISTORY: usize = 100;

#[derive(Debug, Clone)]
enum AgentState {
    Running { started: Instant },
    Done,
    Timeout,
}

#[derive(Debug, Clone)]
struct AgentEntry {
    seq: u64,
    agent_id: String,
    agent_type: String,
    state: AgentState,
}

pub struct AgentStatusPanel {
    agents: VecDeque<AgentEntry>,
    running_map: HashMap<String, u64>, // agent_id → seq
    next_seq: u64,
    list_state: ListState,
    hooks_active: bool,
}

impl Default for AgentStatusPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentStatusPanel {
    pub fn new() -> Self {
        Self {
            agents: VecDeque::new(),
            running_map: HashMap::new(),
            next_seq: 0,
            list_state: ListState::default(),
            hooks_active: false,
        }
    }

    pub fn set_hooks_active(&mut self, active: bool) {
        self.hooks_active = active;
    }

    fn find_by_seq(&self, seq: u64) -> Option<usize> {
        self.agents.iter().position(|a| a.seq == seq)
    }

    fn agent_started(&mut self, agent_id: String, agent_type: String) {
        // If same agent_id is already running, mark it as Done first
        if let Some(old_seq) = self.running_map.remove(&agent_id)
            && let Some(idx) = self.find_by_seq(old_seq)
        {
            self.agents[idx].state = AgentState::Done;
        }

        let seq = self.next_seq;
        self.next_seq += 1;

        let entry = AgentEntry {
            seq,
            agent_id: agent_id.clone(),
            agent_type,
            state: AgentState::Running {
                started: Instant::now(),
            },
        };
        self.agents.push_back(entry);
        self.running_map.insert(agent_id, seq);

        // Cap history — pop_front O(1)
        while self.agents.len() > MAX_HISTORY {
            if let Some(removed) = self.agents.pop_front() {
                // Clean up running_map if the removed entry was still tracked
                if let Some(&s) = self.running_map.get(&removed.agent_id)
                    && s == removed.seq
                {
                    self.running_map.remove(&removed.agent_id);
                }
            }
        }
    }

    fn agent_stopped(&mut self, agent_id: &str) {
        if let Some(seq) = self.running_map.remove(agent_id)
            && let Some(idx) = self.find_by_seq(seq)
        {
            self.agents[idx].state = AgentState::Done;
        }
    }

    /// Check for orphan agents (running > 60s without stop).
    pub fn check_timeouts(&mut self) -> bool {
        let mut changed = false;
        let now = Instant::now();
        let mut timed_out_ids = Vec::new();

        for (id, &seq) in &self.running_map {
            if let Some(idx) = self.find_by_seq(seq)
                && let AgentState::Running { started } = self.agents[idx].state
                && now.duration_since(started).as_secs() >= ORPHAN_TIMEOUT_SECS
            {
                timed_out_ids.push(id.clone());
            }
        }

        for id in timed_out_ids {
            if let Some(seq) = self.running_map.remove(&id)
                && let Some(idx) = self.find_by_seq(seq)
            {
                self.agents[idx].state = AgentState::Timeout;
                changed = true;
            }
        }

        changed
    }

    /// Returns true if any running agents exist.
    pub fn has_running_agents(&self) -> bool {
        !self.running_map.is_empty()
    }

    fn running_count(&self) -> usize {
        self.running_map.len()
    }

    pub fn clamp_scroll(&mut self) {
        let len = self.agents.len();
        if let Some(selected) = self.list_state.selected()
            && selected >= len
            && len > 0
        {
            self.list_state.select(Some(len - 1));
        }
    }
}

impl Component for AgentStatusPanel {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        let len = self.agents.len();
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if len > 0 {
                    let cur = self.list_state.selected().unwrap_or(0);
                    self.list_state.select(Some((cur + 1).min(len - 1)));
                }
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if len > 0 {
                    let cur = self.list_state.selected().unwrap_or(0);
                    self.list_state.select(Some(cur.saturating_sub(1)));
                }
                None
            }
            _ => None,
        }
    }

    fn handle_event(&mut self, event: &AppEvent) {
        match event {
            AppEvent::AgentStarted {
                agent_id,
                agent_type,
            } => {
                self.agent_started(agent_id.clone(), agent_type.clone());
            }
            AppEvent::AgentStopped { agent_id } => {
                self.agent_stopped(agent_id);
            }
            AppEvent::HooksServerStarted { .. } => {
                self.set_hooks_active(true);
            }
            AppEvent::HooksServerFailed { .. } => {
                self.set_hooks_active(false);
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
            .title(theme::panel_title("Agent Status", focused))
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if !self.hooks_active {
            let msg = Paragraph::new(vec![
                Line::from(""),
                Line::from("  에이전트 추적을 위해").dim(),
                Line::from("  hooks 설정이 필요합니다").dim(),
                Line::from(""),
                Line::from("  ? 키로 설정 안내 확인").dim(),
            ]);
            frame.render_widget(msg, inner);
            return;
        }

        if self.agents.is_empty() {
            let msg = Paragraph::new(Line::from("  대기 중...").dim());
            frame.render_widget(msg, inner);
            return;
        }

        let items: Vec<ListItem> = self
            .agents
            .iter()
            .rev() // Most recent first
            .map(|entry| {
                let (icon, style, elapsed_str) = match &entry.state {
                    AgentState::Running { started } => {
                        let secs = Instant::now().duration_since(*started).as_secs();
                        (Icons::active(), Theme::active(), format!("{secs}s"))
                    }
                    AgentState::Done => (Icons::done(), Theme::done(), "done".to_string()),
                    AgentState::Timeout => {
                        (Icons::timeout(), Theme::timeout(), "t/o".to_string())
                    }
                };

                ListItem::new(Line::from(vec![
                    Span::from(format!("  {icon} ")).style(style),
                    Span::from(format!("{:<10}", entry.agent_type)).style(style),
                    Span::from(format!(" {}", elapsed_str)).dim(),
                ]))
            })
            .collect();

        let total = self.agents.len();
        let running = self.running_count();
        let footer = format!("  Total: {total} ({running} running)");

        let list = List::new(items).highlight_style(Theme::highlight());
        frame.render_stateful_widget(list, inner, &mut self.list_state);

        // Footer in the last line of inner
        if inner.height > 1 {
            let footer_area = Rect::new(inner.x, inner.y + inner.height - 1, inner.width, 1);
            frame.render_widget(Paragraph::new(Line::from(footer).dim()), footer_area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_lifecycle() {
        let mut panel = AgentStatusPanel::new();
        panel.set_hooks_active(true);

        panel.agent_started("a1".to_string(), "Explore".to_string());
        assert_eq!(panel.running_count(), 1);
        assert!(panel.has_running_agents());

        panel.agent_stopped("a1");
        assert_eq!(panel.running_count(), 0);
        assert!(!panel.has_running_agents());
    }

    #[test]
    fn test_agent_timeout() {
        let mut panel = AgentStatusPanel::new();
        panel.set_hooks_active(true);

        // Manually create an agent with old start time
        let seq = panel.next_seq;
        panel.next_seq += 1;
        panel.agents.push_back(AgentEntry {
            seq,
            agent_id: "old".to_string(),
            agent_type: "Plan".to_string(),
            state: AgentState::Running {
                started: Instant::now() - std::time::Duration::from_secs(61),
            },
        });
        panel.running_map.insert("old".to_string(), seq);

        let changed = panel.check_timeouts();
        assert!(changed);
        assert_eq!(panel.running_count(), 0);
        assert!(matches!(panel.agents[0].state, AgentState::Timeout));
    }

    #[test]
    fn test_max_history() {
        let mut panel = AgentStatusPanel::new();
        for i in 0..MAX_HISTORY + 5 {
            panel.agent_started(format!("a{i}"), "Explore".to_string());
            panel.agent_stopped(&format!("a{i}"));
        }
        assert_eq!(panel.agents.len(), MAX_HISTORY);
        assert!(panel.running_map.is_empty());
    }

    #[test]
    fn test_duplicate_agent_id() {
        let mut panel = AgentStatusPanel::new();
        panel.set_hooks_active(true);

        panel.agent_started("a1".to_string(), "Explore".to_string());
        assert_eq!(panel.running_count(), 1);

        // Same ID started again — previous should become Done
        panel.agent_started("a1".to_string(), "Plan".to_string());
        assert_eq!(panel.running_count(), 1);
        assert_eq!(panel.agents.len(), 2);
        assert!(matches!(panel.agents[0].state, AgentState::Done));
        assert!(matches!(panel.agents[1].state, AgentState::Running { .. }));
    }

    #[test]
    fn test_hooks_inactive_message() {
        let panel = AgentStatusPanel::new();
        assert!(!panel.hooks_active);
    }

    // ── Render tests ──

    use crate::test_helpers::{buffer_contains_str, render_component};

    #[test]
    fn render_hooks_inactive() {
        let mut panel = AgentStatusPanel::new();
        let terminal = render_component(&mut panel, 50, 12, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "hooks 설정이 필요합니다"));
    }

    #[test]
    fn render_empty_agents() {
        let mut panel = AgentStatusPanel::new();
        panel.set_hooks_active(true);
        let terminal = render_component(&mut panel, 50, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "대기 중..."));
    }

    #[test]
    fn render_with_running_agent() {
        let mut panel = AgentStatusPanel::new();
        panel.set_hooks_active(true);
        panel.agent_started("a1".to_string(), "Explore".to_string());
        let terminal = render_component(&mut panel, 50, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "●"));
        assert!(buffer_contains_str(buf, "Explore"));
    }

    #[test]
    fn render_with_done_agent() {
        let mut panel = AgentStatusPanel::new();
        panel.set_hooks_active(true);
        panel.agent_started("a1".to_string(), "Plan".to_string());
        panel.agent_stopped("a1");
        let terminal = render_component(&mut panel, 50, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "✓"));
        assert!(buffer_contains_str(buf, "done"));
    }

    #[test]
    fn render_with_timeout_agent() {
        let mut panel = AgentStatusPanel::new();
        panel.set_hooks_active(true);
        let seq = panel.next_seq;
        panel.next_seq += 1;
        panel.agents.push_back(AgentEntry {
            seq,
            agent_id: "old".to_string(),
            agent_type: "Plan".to_string(),
            state: AgentState::Running {
                started: Instant::now() - std::time::Duration::from_secs(61),
            },
        });
        panel.running_map.insert("old".to_string(), seq);
        panel.check_timeouts();

        let terminal = render_component(&mut panel, 50, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "⏱"));
        assert!(buffer_contains_str(buf, "t/o"));
    }

    #[test]
    fn render_footer_count() {
        let mut panel = AgentStatusPanel::new();
        panel.set_hooks_active(true);
        panel.agent_started("a1".to_string(), "Explore".to_string());
        panel.agent_started("a2".to_string(), "Plan".to_string());
        panel.agent_stopped("a1");
        let terminal = render_component(&mut panel, 50, 12, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Total: 2 (1 running)"));
    }
}
