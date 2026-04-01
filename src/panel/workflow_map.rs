use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
use ratatui::Frame;

use crate::action::Action;
use crate::component::Component;
use crate::event::AppEvent;
use crate::parser::models::{Complexity, FlowState, SessionSummary, StageStatus, WorkMarker};
use crate::ui::theme::{self, Icons, Theme};

/// INCEPTION stage definitions with conditional visibility.
struct StageInfo {
    name: &'static str,
    min_complexity: Option<Complexity>,
}

const INCEPTION_STAGES: &[StageInfo] = &[
    StageInfo { name: "workspace-detection", min_complexity: None },
    StageInfo { name: "complexity-declaration", min_complexity: None },
    StageInfo { name: "requirements-analysis", min_complexity: None },
    StageInfo { name: "user-stories", min_complexity: Some(Complexity::Standard) },
    StageInfo { name: "nfr-requirements", min_complexity: Some(Complexity::Standard) },
    StageInfo { name: "workflow-planning", min_complexity: None },
    StageInfo { name: "application-design", min_complexity: Some(Complexity::Comprehensive) },
    StageInfo { name: "units-generation", min_complexity: Some(Complexity::Comprehensive) },
];

const CONSTRUCTION_STAGES: &[StageInfo] = &[
    StageInfo { name: "functional-design", min_complexity: Some(Complexity::Comprehensive) },
    StageInfo { name: "git-worktree", min_complexity: None },
    StageInfo { name: "code-generation", min_complexity: None },
    StageInfo { name: "build-and-test", min_complexity: None },
    StageInfo { name: "code-review", min_complexity: None },
    StageInfo { name: "finishing-branch", min_complexity: None },
];

pub struct WorkflowMapPanel {
    flow_state: FlowState,
    session_summary: SessionSummary,
    syncing: bool,
    pub scroll_offset: u16,
}

impl Default for WorkflowMapPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowMapPanel {
    pub fn new() -> Self {
        Self {
            flow_state: FlowState::default(),
            session_summary: SessionSummary::default(),
            syncing: false,
            scroll_offset: 0,
        }
    }

    pub fn set_flow_state(&mut self, state: FlowState) {
        self.flow_state = state;
        self.syncing = false;
    }

    pub fn set_session_summary(&mut self, summary: SessionSummary) {
        self.session_summary = summary;
    }

    pub fn set_syncing(&mut self) {
        self.syncing = true;
    }

    fn stage_status(&self, stage_name: &str) -> StageStatus {
        if self.flow_state.stage == stage_name {
            return StageStatus::Active;
        }
        if self.flow_state.completed_stages.iter().any(|s| s.name == stage_name) {
            return StageStatus::Completed;
        }
        if self.flow_state.skipped_stages.iter().any(|s| s.name == stage_name) {
            return StageStatus::Skipped;
        }
        if self.flow_state.deferred_stages.iter().any(|s| s.name == stage_name) {
            return StageStatus::Deferred;
        }
        StageStatus::Waiting
    }

    fn is_visible(&self, info: &StageInfo) -> bool {
        match info.min_complexity {
            None => true,
            Some(Complexity::Standard) => matches!(
                self.flow_state.complexity,
                Complexity::Standard | Complexity::Comprehensive
            ),
            Some(Complexity::Comprehensive) => {
                self.flow_state.complexity == Complexity::Comprehensive
            }
            Some(Complexity::Minimal) => true,
        }
    }

    fn render_stages<'a>(&self, phase_name: &str, stages: &[StageInfo]) -> Vec<Line<'a>> {
        let mut lines = vec![Line::from(phase_name.to_string()).bold()];

        let visible: Vec<&StageInfo> = stages.iter().filter(|s| self.is_visible(s)).collect();

        for (i, info) in visible.iter().enumerate() {
            let status = self.stage_status(info.name);
            let (icon, style) = match status {
                StageStatus::Active => (Icons::active(), Theme::active()),
                StageStatus::Completed => (Icons::done(), Theme::done()),
                StageStatus::Waiting => (Icons::waiting(), Theme::waiting()),
                StageStatus::Skipped => (Icons::skipped(), Theme::skipped()),
                StageStatus::Deferred => (Icons::deferred(), Theme::deferred()),
            };

            let prefix = if i == visible.len() - 1 { "  └── " } else { "  ├── " };
            lines.push(Line::from(vec![
                ratatui::text::Span::from(prefix).dim(),
                theme::status_span(icon, info.name, style),
            ]));
        }

        lines
    }
}

impl Component for WorkflowMapPanel {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                None
            }
            KeyCode::Enter => Some(Action::OpenArtifactModal),
            _ => None,
        }
    }

    fn handle_event(&mut self, event: &AppEvent) {
        match event {
            AppEvent::FlowStateChanged(state) => self.set_flow_state(state.clone()),
            AppEvent::SessionSummaryChanged(summary) => self.set_session_summary(summary.clone()),
            AppEvent::ParseError { .. } => self.set_syncing(),
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
            .title(theme::panel_title("Workflow Map", focused))
            .border_style(border_style);

        let inner = block.inner(area);

        let mut lines: Vec<Line> = Vec::new();

        if self.syncing {
            lines.push(Line::from("  Syncing...").yellow());
            lines.push(Line::from(""));
        }

        // INCEPTION stages
        lines.extend(self.render_stages("  INCEPTION", INCEPTION_STAGES));
        lines.push(Line::from(""));

        // CONSTRUCTION stages
        lines.extend(self.render_stages("  CONSTRUCTION", CONSTRUCTION_STAGES));
        lines.push(Line::from(""));

        // Completed Work
        if !self.session_summary.completed_work.is_empty() {
            lines.push(Line::from("  Completed Work:").dim());
            for item in &self.session_summary.completed_work {
                let (icon, style) = match item.marker {
                    WorkMarker::Done => (Icons::done(), Theme::done()),
                    WorkMarker::InProgress => ("◐", Theme::active()),
                    WorkMarker::Pending => (Icons::waiting(), Theme::waiting()),
                };
                lines.push(Line::from(vec![
                    ratatui::text::Span::from(format!("    {icon} ")).style(style),
                    ratatui::text::Span::from(item.text.as_str()).style(style),
                ]));
            }
            lines.push(Line::from(""));
        }

        // Key Decisions
        if !self.session_summary.key_decisions.is_empty() {
            lines.push(Line::from("  Key Decisions:").dim());
            for decision in &self.session_summary.key_decisions {
                lines.push(Line::from(format!("    • {decision}")).dim());
            }
            lines.push(Line::from(""));
        }

        // Next Steps
        if !self.session_summary.next_steps.is_empty() {
            lines.push(Line::from("  Next:").dim());
            for step in &self.session_summary.next_steps {
                lines.push(Line::from(format!("    {step}")).dim());
            }
        }

        // Active unit info
        if let Some(ref unit) = self.flow_state.active_unit {
            lines.push(Line::from(""));
            lines.push(Line::from(format!("  Active Unit: {unit}")).yellow());
        }

        let text = Text::from(lines);
        let content = Paragraph::new(text)
            .scroll((self.scroll_offset, 0))
            .wrap(Wrap { trim: false });

        frame.render_widget(block, area);
        frame.render_widget(content, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::CompletedStage;

    #[test]
    fn test_stage_status_active() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.stage = "requirements-analysis".to_string();
        assert_eq!(panel.stage_status("requirements-analysis"), StageStatus::Active);
    }

    #[test]
    fn test_stage_status_completed() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.completed_stages.push(CompletedStage {
            name: "workspace-detection".to_string(),
            timestamp: None,
        });
        assert_eq!(panel.stage_status("workspace-detection"), StageStatus::Completed);
    }

    #[test]
    fn test_visibility_minimal() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.complexity = Complexity::Minimal;
        let info = StageInfo {
            name: "user-stories",
            min_complexity: Some(Complexity::Standard),
        };
        assert!(!panel.is_visible(&info));
    }

    #[test]
    fn test_visibility_standard() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.complexity = Complexity::Standard;
        let info = StageInfo {
            name: "user-stories",
            min_complexity: Some(Complexity::Standard),
        };
        assert!(panel.is_visible(&info));
    }

    // ── Render tests ──

    use crate::test_helpers::{buffer_contains_str, render_component};
    use crate::parser::models::WorkItem;

    #[test]
    fn render_default_state() {
        let mut panel = WorkflowMapPanel::new();
        let terminal = render_component(&mut panel, 60, 30, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "INCEPTION"));
        assert!(buffer_contains_str(buf, "CONSTRUCTION"));
        assert!(buffer_contains_str(buf, "○"));
    }

    #[test]
    fn render_active_stage() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.stage = "requirements-analysis".to_string();
        let terminal = render_component(&mut panel, 60, 30, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "●"));
        assert!(buffer_contains_str(buf, "requirements-analysis"));
    }

    #[test]
    fn render_completed_stage() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.completed_stages.push(CompletedStage {
            name: "workspace-detection".to_string(),
            timestamp: None,
        });
        let terminal = render_component(&mut panel, 60, 30, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "✓"));
        assert!(buffer_contains_str(buf, "workspace-detection"));
    }

    #[test]
    fn render_minimal_complexity() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.complexity = Complexity::Minimal;
        let terminal = render_component(&mut panel, 60, 30, true);
        let buf = terminal.backend().buffer();
        // Standard-only stages should be hidden
        assert!(!buffer_contains_str(buf, "user-stories"));
        // Always-visible stages should be present
        assert!(buffer_contains_str(buf, "workspace-detection"));
    }

    #[test]
    fn test_construction_stages_count() {
        assert_eq!(CONSTRUCTION_STAGES.len(), 6);
    }

    #[test]
    fn test_construction_stages_order() {
        let names: Vec<&str> = CONSTRUCTION_STAGES.iter().map(|s| s.name).collect();
        assert_eq!(
            names,
            vec![
                "functional-design",
                "git-worktree",
                "code-generation",
                "build-and-test",
                "code-review",
                "finishing-branch",
            ]
        );
    }

    #[test]
    fn test_new_construction_stages_visible_all_complexities() {
        let new_stages = ["git-worktree", "code-review", "finishing-branch"];
        for complexity in [Complexity::Minimal, Complexity::Standard, Complexity::Comprehensive] {
            let mut panel = WorkflowMapPanel::new();
            panel.flow_state.complexity = complexity;
            for stage_name in &new_stages {
                let info = CONSTRUCTION_STAGES.iter().find(|s| s.name == *stage_name).unwrap();
                assert!(
                    panel.is_visible(info),
                    "{stage_name} should be visible at {complexity:?}"
                );
            }
        }
    }

    #[test]
    fn render_comprehensive_complexity() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.complexity = Complexity::Comprehensive;
        let terminal = render_component(&mut panel, 60, 30, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "application-design"));
        assert!(buffer_contains_str(buf, "units-generation"));
    }

    #[test]
    fn render_new_construction_stages() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.complexity = Complexity::Minimal;
        let terminal = render_component(&mut panel, 60, 30, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "git-worktree"));
        assert!(buffer_contains_str(buf, "code-review"));
        assert!(buffer_contains_str(buf, "finishing-branch"));
    }

    #[test]
    fn render_syncing_state() {
        let mut panel = WorkflowMapPanel::new();
        panel.set_syncing();
        let terminal = render_component(&mut panel, 60, 30, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Syncing..."));
    }

    #[test]
    fn test_stage_status_deferred() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.deferred_stages.push(
            crate::parser::models::DeferredStage {
                name: "user-stories".to_string(),
                reason: Some("Deferred to wave 2".to_string()),
            },
        );
        assert_eq!(panel.stage_status("user-stories"), StageStatus::Deferred);
    }

    #[test]
    fn render_deferred_stage() {
        let mut panel = WorkflowMapPanel::new();
        panel.flow_state.complexity = Complexity::Standard;
        panel.flow_state.deferred_stages.push(
            crate::parser::models::DeferredStage {
                name: "user-stories".to_string(),
                reason: None,
            },
        );
        let terminal = render_component(&mut panel, 60, 30, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "◇"));
        assert!(buffer_contains_str(buf, "user-stories"));
    }

    #[test]
    fn render_completed_work_and_decisions() {
        let mut panel = WorkflowMapPanel::new();
        panel.session_summary.completed_work.push(WorkItem {
            text: "Setup project".to_string(),
            marker: WorkMarker::Done,
        });
        panel.session_summary.key_decisions.push("Use ratatui".to_string());
        let terminal = render_component(&mut panel, 60, 35, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Completed Work:"));
        assert!(buffer_contains_str(buf, "Setup project"));
        assert!(buffer_contains_str(buf, "Key Decisions:"));
        assert!(buffer_contains_str(buf, "Use ratatui"));
    }
}
