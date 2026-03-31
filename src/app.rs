use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use tokio::sync::mpsc;

use crate::action::{Action, Direction};
use crate::command::CommandRunner;
use crate::component::Component;
use crate::event::AppEvent;
use crate::panel::agent_status::AgentStatusPanel;
use crate::panel::audit_log::AuditLogPanel;
use crate::panel::gate_alert::GateAlertPanel;
use crate::panel::git_status::GitStatusPanel;
use crate::panel::workflow_map::WorkflowMapPanel;
use crate::parser::models::{FlowState, GitSnapshot};
use crate::service::hook_config::{self, HookConfigStatus};
use crate::ui::layout::{LayoutManager, LayoutMode, PanelAreas};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    WorkflowMap,
    GitStatus,
    AgentStatus,
    AuditLog,
    GateAlert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Expanded,
    HelpOverlay,
}

#[derive(Debug, Clone)]
pub enum HookSetupState {
    Unknown,
    Configured,
    NotConfigured { snippet: String },
    Mismatch { detail: String },
}

pub struct App {
    pub should_quit: bool,
    pub input_mode: InputMode,
    pub focus: FocusPane,

    pub workflow_map: WorkflowMapPanel,
    pub git_status: GitStatusPanel,
    pub agent_status: AgentStatusPanel,
    pub audit_log: AuditLogPanel,
    pub gate_alert: GateAlertPanel,

    pub layout: LayoutManager,
    pub hooks_active: bool,
    pub hooks_port: Option<u16>,
    pub hook_setup: HookSetupState,
    pub token: String,
    pub project_dir: PathBuf,
    pub phase: String,
    pub stage: String,

    command_runner: CommandRunner,
    event_tx: mpsc::Sender<AppEvent>,
}

impl App {
    pub fn new(
        width: u16,
        height: u16,
        event_tx: mpsc::Sender<AppEvent>,
        token: String,
        project_dir: PathBuf,
    ) -> Self {
        Self {
            should_quit: false,
            input_mode: InputMode::Normal,
            focus: FocusPane::WorkflowMap,

            workflow_map: WorkflowMapPanel::new(),
            git_status: GitStatusPanel::new(),
            agent_status: AgentStatusPanel::new(),
            audit_log: AuditLogPanel::new(),
            gate_alert: GateAlertPanel::new(),

            layout: LayoutManager::new(width, height),
            hooks_active: false,
            hooks_port: None,
            hook_setup: HookSetupState::Unknown,
            token,
            project_dir,
            phase: "INCEPTION".to_string(),
            stage: String::new(),

            command_runner: CommandRunner::new(event_tx.clone()),
            event_tx,
        }
    }

    pub fn available_panels(&self) -> Vec<FocusPane> {
        match self.layout.mode() {
            LayoutMode::Wide => vec![
                FocusPane::WorkflowMap,
                FocusPane::GitStatus,
                FocusPane::AgentStatus,
                FocusPane::AuditLog,
                FocusPane::GateAlert,
            ],
            _ => vec![
                FocusPane::WorkflowMap,
                FocusPane::GitStatus,
                FocusPane::AgentStatus,
                FocusPane::AuditLog,
            ],
        }
    }

    pub fn ensure_valid_focus(&mut self) {
        let available = self.available_panels();
        if !available.contains(&self.focus) {
            self.focus = available
                .first()
                .copied()
                .unwrap_or(FocusPane::WorkflowMap);
        }
    }

    pub fn focus_name(&self) -> &str {
        match self.focus {
            FocusPane::WorkflowMap => "Workflow Map",
            FocusPane::GitStatus => "Git Status",
            FocusPane::AgentStatus => "Agent Status",
            FocusPane::AuditLog => "Audit Log",
            FocusPane::GateAlert => "Gate Alert",
        }
    }

    pub fn focused_panel_mut(&mut self) -> &mut dyn Component {
        match self.focus {
            FocusPane::WorkflowMap => &mut self.workflow_map,
            FocusPane::GitStatus => &mut self.git_status,
            FocusPane::AgentStatus => &mut self.agent_status,
            FocusPane::AuditLog => &mut self.audit_log,
            FocusPane::GateAlert => &mut self.gate_alert,
        }
    }

    // ── Key handling (Step 2) ──

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.input_mode {
            InputMode::HelpOverlay => {
                if matches!(key.code, KeyCode::Esc | KeyCode::Char('?')) {
                    self.input_mode = InputMode::Normal;
                }
                true
            }
            InputMode::Expanded => {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('f') => {
                        self.input_mode = InputMode::Normal;
                        true
                    }
                    _ => {
                        // Delegate to focused panel in expanded mode
                        if let Some(action) = self.focused_panel_mut().handle_key(key) {
                            self.execute_action(action);
                        }
                        true
                    }
                }
            }
            InputMode::Normal => self.handle_normal_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyEvent) -> bool {
        // Global keys
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
                return true;
            }
            KeyCode::Char('?') => {
                self.input_mode = InputMode::HelpOverlay;
                return true;
            }
            KeyCode::Char('f') => {
                self.input_mode = InputMode::Expanded;
                return true;
            }
            KeyCode::Tab => {
                self.execute_action(Action::FocusNextPanel);
                return true;
            }
            KeyCode::BackTab => {
                self.execute_action(Action::FocusPrevPanel);
                return true;
            }
            KeyCode::Char('r') => {
                self.command_runner.execute(Action::Refresh);
                return true;
            }
            KeyCode::Char('c') => {
                self.copy_hooks_snippet();
                return true;
            }
            _ => {}
        }

        // Ctrl+hjkl direction navigation
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            let dir = match key.code {
                KeyCode::Char('h') => Some(Direction::Left),
                KeyCode::Char('j') => Some(Direction::Down),
                KeyCode::Char('k') => Some(Direction::Up),
                KeyCode::Char('l') => Some(Direction::Right),
                _ => None,
            };
            if let Some(d) = dir {
                self.execute_action(Action::FocusDirection(d));
                return true;
            }
        }

        // Delegate to focused panel
        if let Some(action) = self.focused_panel_mut().handle_key(key) {
            self.execute_action(action);
            return true;
        }

        false
    }

    pub fn execute_action(&mut self, action: Action) {
        match action {
            Action::FocusNextPanel => {
                let panels = self.available_panels();
                if let Some(idx) = panels.iter().position(|p| *p == self.focus) {
                    self.focus = panels[(idx + 1) % panels.len()];
                }
            }
            Action::FocusPrevPanel => {
                let panels = self.available_panels();
                if let Some(idx) = panels.iter().position(|p| *p == self.focus) {
                    self.focus = panels[(idx + panels.len() - 1) % panels.len()];
                }
            }
            Action::FocusDirection(dir) => {
                self.focus_direction(dir);
            }
            Action::ExpandPanel => {
                self.input_mode = InputMode::Expanded;
            }
            Action::CollapsePanel => {
                self.input_mode = InputMode::Normal;
            }
            Action::Quit => {
                self.should_quit = true;
            }
            action if action.is_async() => {
                self.command_runner.execute(action);
            }
            _ => {
                // Sync panel actions (ScrollUp/Down, Select, etc.) are already
                // handled by the panel's handle_key
            }
        }
    }

    fn focus_direction(&mut self, dir: Direction) {
        // Standard/Compact: Left/Right act like Tab/Shift+Tab
        // Wide: 2x3 grid navigation (to be expanded in Unit 12)
        match dir {
            Direction::Right => self.execute_action(Action::FocusNextPanel),
            Direction::Left => self.execute_action(Action::FocusPrevPanel),
            _ => {} // Up/Down: noop for Standard/Compact
        }
    }

    // ── Event handling (Step 3) ──

    pub fn handle_event(&mut self, event: AppEvent) {
        // Broadcast to all panels
        self.workflow_map.handle_event(&event);
        self.git_status.handle_event(&event);
        self.agent_status.handle_event(&event);
        self.audit_log.handle_event(&event);
        self.gate_alert.handle_event(&event);

        // App-level handling
        match &event {
            AppEvent::HooksServerStarted { port } => {
                self.hooks_active = true;
                self.hooks_port = Some(*port);
                self.check_hooks_config();
            }
            AppEvent::HooksServerFailed { .. } => {
                self.hooks_active = false;
                self.hook_setup = HookSetupState::Unknown;
            }
            AppEvent::FlowStateChanged(state) => {
                self.phase = state.phase.to_string();
                self.stage = state.stage.clone();
            }
            _ => {}
        }
    }

    pub fn handle_flow_state(&mut self, state: FlowState) {
        self.phase = state.phase.to_string();
        self.stage = state.stage.clone();
        self.workflow_map.set_flow_state(state);
    }

    pub fn handle_git_snapshot(&mut self, snapshot: GitSnapshot) {
        self.git_status.set_snapshot(snapshot);
    }

    pub fn on_tick(&mut self) -> bool {
        self.agent_status.check_timeouts()
    }

    pub fn on_resize(&mut self, w: u16, h: u16) {
        self.layout.on_resize(w, h);
        self.ensure_valid_focus();
        self.workflow_map.scroll_offset = 0;
        self.git_status.clamp_scroll();
        self.agent_status.clamp_scroll();
        self.audit_log.clamp_scroll();
    }

    pub fn check_hooks_config(&mut self) {
        let port = match self.hooks_port {
            Some(p) => p,
            None => return,
        };
        let status = hook_config::check_hooks_config(&self.project_dir, port, &self.token);
        self.hook_setup = match status {
            HookConfigStatus::Configured => HookSetupState::Configured,
            HookConfigStatus::NotConfigured => HookSetupState::NotConfigured {
                snippet: hook_config::generate_hooks_snippet(port, &self.token),
            },
            HookConfigStatus::EndpointMismatch { configured_url } => {
                HookSetupState::Mismatch {
                    detail: configured_url,
                }
            }
        };
    }

    pub fn copy_hooks_snippet(&mut self) {
        let snippet = match &self.hook_setup {
            HookSetupState::NotConfigured { snippet } => Some(snippet.clone()),
            HookSetupState::Mismatch { .. } => {
                // Mismatch: generate fresh snippet with correct port/token
                self.hooks_port
                    .map(|port| hook_config::generate_hooks_snippet(port, &self.token))
            }
            _ => None,
        };
        if let Some(text) = snippet {
            self.command_runner
                .execute(Action::CopyToClipboard(text));
        }
    }

    // ── Rendering (Step 4) ──

    pub fn render(&mut self, frame: &mut Frame) {
        let areas = self.layout.areas(frame.area());
        let wide_mode = self.layout.mode() == LayoutMode::Wide;

        // Header
        crate::ui::header::render(
            frame,
            areas.header,
            &self.phase,
            self.hooks_active,
            self.hooks_port,
        );

        // Status bar
        let phase_stage = if self.stage.is_empty() {
            self.phase.clone()
        } else {
            format!("{} > {}", self.phase, self.stage)
        };
        crate::ui::status_bar::render(
            frame,
            areas.status_bar,
            self.focus_name(),
            &phase_stage,
            wide_mode,
        );

        // Body
        let mut body = areas.body;

        // Hooks setup banner
        if matches!(
            self.hook_setup,
            HookSetupState::NotConfigured { .. } | HookSetupState::Mismatch { .. }
        ) && body.height > 1
        {
            let banner_area = ratatui::layout::Rect::new(body.x, body.y, body.width, 1);
            let banner_msg = match &self.hook_setup {
                HookSetupState::NotConfigured { .. } => {
                    " ⚠ Hooks 미설정 — c 키로 설정 JSON 복사"
                }
                HookSetupState::Mismatch { .. } => {
                    " ⚠ Hooks 설정 불일치 — c 키로 새 설정 복사"
                }
                _ => "",
            };
            let banner = ratatui::widgets::Paragraph::new(banner_msg)
                .style(crate::ui::theme::Theme::gate_alert());
            frame.render_widget(banner, banner_area);
            body = ratatui::layout::Rect::new(body.x, body.y + 1, body.width, body.height - 1);
        }

        // Main content based on input mode
        match self.input_mode {
            InputMode::HelpOverlay => {
                self.render_panels(frame, body);
                crate::ui::help_overlay::render(frame, frame.area());
            }
            InputMode::Expanded => {
                self.render_focused_panel(frame, body);
            }
            InputMode::Normal => {
                self.render_panels(frame, body);
            }
        }
    }

    fn render_panels(&mut self, frame: &mut Frame, body: ratatui::layout::Rect) {
        let panel_areas = self.layout.panel_areas(body);

        match panel_areas {
            PanelAreas::TooSmall { message } => {
                let msg = ratatui::widgets::Paragraph::new(
                    "터미널이 너무 작습니다 (최소 80x24)",
                )
                .style(crate::ui::theme::Theme::error());
                frame.render_widget(msg, message);
            }
            PanelAreas::Compact { panel } => {
                self.render_focused_panel(frame, panel);
            }
            PanelAreas::Standard {
                workflow_map,
                git_status,
                agent_status,
                audit_log,
            } => {
                let focus = self.focus;
                self.workflow_map
                    .render(frame, workflow_map, focus == FocusPane::WorkflowMap);
                self.git_status
                    .render(frame, git_status, focus == FocusPane::GitStatus);
                self.agent_status
                    .render(frame, agent_status, focus == FocusPane::AgentStatus);
                self.audit_log
                    .render(frame, audit_log, focus == FocusPane::AuditLog);
            }
            PanelAreas::Wide {
                workflow_map,
                git_status,
                artifacts,
                agent_status,
                audit_log,
                gate_alert,
            } => {
                let focus = self.focus;
                self.workflow_map
                    .render(frame, workflow_map, focus == FocusPane::WorkflowMap);
                self.git_status
                    .render(frame, git_status, focus == FocusPane::GitStatus);
                self.agent_status
                    .render(frame, agent_status, focus == FocusPane::AgentStatus);
                self.audit_log
                    .render(frame, audit_log, focus == FocusPane::AuditLog);

                // Artifacts placeholder (deferred to v1.1)
                let placeholder = ratatui::widgets::Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(crate::ui::theme::panel_title("Artifacts", false))
                    .border_style(crate::ui::theme::Theme::unfocus_border());
                frame.render_widget(placeholder, artifacts);

                // Gate Alert panel
                self.gate_alert
                    .render(frame, gate_alert, focus == FocusPane::GateAlert);
            }
        }
    }

    fn render_focused_panel(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
        match self.focus {
            FocusPane::WorkflowMap => self.workflow_map.render(frame, area, true),
            FocusPane::GitStatus => self.git_status.render(frame, area, true),
            FocusPane::AgentStatus => self.agent_status.render(frame, area, true),
            FocusPane::AuditLog => self.audit_log.render(frame, area, true),
            FocusPane::GateAlert => self.gate_alert.render(frame, area, true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{buffer_contains_str, render_with};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn make_app(width: u16, height: u16) -> App {
        let (tx, _rx) = mpsc::channel(16);
        App::new(width, height, tx, "test-token".to_string(), PathBuf::from("/tmp/test"))
    }

    // ── Step 1: App struct tests ──

    #[test]
    fn test_available_panels_compact() {
        let app = make_app(80, 24);
        let panels = app.available_panels();
        assert_eq!(panels.len(), 4);
        assert_eq!(panels[0], FocusPane::WorkflowMap);
    }

    #[test]
    fn test_available_panels_standard() {
        let app = make_app(120, 30);
        let panels = app.available_panels();
        assert_eq!(panels.len(), 4);
    }

    #[test]
    fn test_ensure_valid_focus() {
        let mut app = make_app(80, 24);
        // Focus is valid, should not change
        app.focus = FocusPane::AuditLog;
        app.ensure_valid_focus();
        assert_eq!(app.focus, FocusPane::AuditLog);
    }

    #[test]
    fn test_focus_name() {
        let app = make_app(80, 24);
        assert_eq!(app.focus_name(), "Workflow Map");
    }

    // ── Step 2: Key handling tests ──

    #[test]
    fn test_handle_key_quit() {
        let mut app = make_app(80, 24);
        app.handle_key(KeyEvent::from(KeyCode::Char('q')));
        assert!(app.should_quit);
    }

    #[test]
    fn test_handle_key_tab_focus() {
        let mut app = make_app(80, 24);
        assert_eq!(app.focus, FocusPane::WorkflowMap);
        app.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(app.focus, FocusPane::GitStatus);
        app.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(app.focus, FocusPane::AgentStatus);
        app.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(app.focus, FocusPane::AuditLog);
        app.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(app.focus, FocusPane::WorkflowMap); // wraps around
    }

    #[test]
    fn test_handle_key_backtab_focus() {
        let mut app = make_app(80, 24);
        app.handle_key(KeyEvent::from(KeyCode::BackTab));
        assert_eq!(app.focus, FocusPane::AuditLog); // wraps backward
    }

    #[test]
    fn test_handle_key_help_overlay() {
        let mut app = make_app(80, 24);
        app.handle_key(KeyEvent::from(KeyCode::Char('?')));
        assert_eq!(app.input_mode, InputMode::HelpOverlay);
    }

    #[test]
    fn test_help_overlay_esc_returns_normal() {
        let mut app = make_app(80, 24);
        app.input_mode = InputMode::HelpOverlay;
        app.handle_key(KeyEvent::from(KeyCode::Esc));
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn test_help_overlay_question_returns_normal() {
        let mut app = make_app(80, 24);
        app.input_mode = InputMode::HelpOverlay;
        app.handle_key(KeyEvent::from(KeyCode::Char('?')));
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn test_handle_key_expand() {
        let mut app = make_app(80, 24);
        app.handle_key(KeyEvent::from(KeyCode::Char('f')));
        assert_eq!(app.input_mode, InputMode::Expanded);
    }

    #[test]
    fn test_expanded_esc_returns_normal() {
        let mut app = make_app(80, 24);
        app.input_mode = InputMode::Expanded;
        app.handle_key(KeyEvent::from(KeyCode::Esc));
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn test_expanded_f_returns_normal() {
        let mut app = make_app(80, 24);
        app.input_mode = InputMode::Expanded;
        app.handle_key(KeyEvent::from(KeyCode::Char('f')));
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn test_focus_direction_standard() {
        let mut app = make_app(120, 30);
        assert_eq!(app.focus, FocusPane::WorkflowMap);
        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL);
        app.handle_key(key);
        assert_eq!(app.focus, FocusPane::GitStatus); // Right = next
    }

    #[test]
    fn test_handle_key_c_noop_when_configured() {
        let mut app = make_app(80, 24);
        app.hook_setup = HookSetupState::Configured;
        // c key should not trigger copy when hooks are configured
        let result = app.handle_key(KeyEvent::from(KeyCode::Char('c')));
        assert!(result); // key was consumed, but no copy action
    }

    // ── Step 3: Event handling tests ──

    #[test]
    fn test_handle_event_hooks_started() {
        let mut app = make_app(80, 24);
        app.handle_event(AppEvent::HooksServerStarted { port: 9100 });
        assert!(app.hooks_active);
        assert_eq!(app.hooks_port, Some(9100));
    }

    #[test]
    fn test_handle_event_hooks_failed() {
        let mut app = make_app(80, 24);
        app.hooks_active = true;
        app.handle_event(AppEvent::HooksServerFailed {
            reason: "bind error".to_string(),
        });
        assert!(!app.hooks_active);
    }

    #[test]
    fn test_handle_flow_state() {
        let mut app = make_app(80, 24);
        let state = FlowState {
            stage: "requirements-analysis".to_string(),
            ..Default::default()
        };
        app.handle_flow_state(state);
        assert_eq!(app.phase, "INCEPTION");
        assert_eq!(app.stage, "requirements-analysis");
    }

    #[test]
    fn test_on_tick_returns_false_when_no_change() {
        let mut app = make_app(80, 24);
        assert!(!app.on_tick());
    }

    #[test]
    fn test_on_resize_revalidates_focus() {
        let mut app = make_app(200, 50);
        app.focus = FocusPane::WorkflowMap;
        app.on_resize(80, 24);
        // Focus should still be valid after resize
        assert!(app.available_panels().contains(&app.focus));
    }

    #[test]
    fn test_handle_event_broadcast() {
        let mut app = make_app(80, 24);
        app.agent_status.set_hooks_active(true);
        // AgentStarted should reach agent_status panel
        app.handle_event(AppEvent::AgentStarted {
            agent_id: "a1".to_string(),
            agent_type: "Explore".to_string(),
        });
        assert!(app.agent_status.has_running_agents());
    }

    #[test]
    fn test_check_hooks_not_configured() {
        let mut app = make_app(80, 24);
        app.hooks_port = Some(9100);
        // project_dir /tmp/test has no .claude/settings.json
        app.check_hooks_config();
        assert!(matches!(app.hook_setup, HookSetupState::NotConfigured { .. }));
    }

    #[test]
    fn test_check_hooks_configured() {
        let mut app = make_app(80, 24);
        // Without hooks_port set, check_hooks_config should bail early
        app.check_hooks_config();
        assert!(matches!(app.hook_setup, HookSetupState::Unknown));
    }

    #[tokio::test]
    async fn test_handle_key_c_copies_snippet() {
        let mut app = make_app(80, 24);
        app.hooks_port = Some(9100);
        app.hook_setup = HookSetupState::NotConfigured {
            snippet: r#"{"hooks": {}}"#.to_string(),
        };
        // c key should trigger copy (via command_runner.execute which needs tokio runtime)
        let result = app.handle_key(KeyEvent::from(KeyCode::Char('c')));
        assert!(result); // key was consumed
    }

    // ── Step 4: Render tests ──

    #[test]
    fn render_compact_mode() {
        let mut app = make_app(80, 24);
        let terminal = render_with(80, 24, |frame, _area| {
            app.render(frame);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "devflow-tui"));
        assert!(buffer_contains_str(buf, "Workflow Map"));
    }

    #[test]
    fn render_standard_mode() {
        let mut app = make_app(120, 30);
        let terminal = render_with(120, 30, |frame, _area| {
            app.render(frame);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "devflow-tui"));
        // All 4 panels should have their titles
        assert!(buffer_contains_str(buf, "Workflow Map"));
        assert!(buffer_contains_str(buf, "Git Status"));
        assert!(buffer_contains_str(buf, "Agent Status"));
        assert!(buffer_contains_str(buf, "Audit Log"));
    }

    #[test]
    fn render_too_small() {
        let mut app = make_app(60, 15);
        let terminal = render_with(60, 15, |frame, _area| {
            app.render(frame);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "터미널이 너무 작습니다"));
    }

    #[test]
    fn render_help_overlay() {
        let mut app = make_app(80, 24);
        app.input_mode = InputMode::HelpOverlay;
        let terminal = render_with(80, 24, |frame, _area| {
            app.render(frame);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Help"));
        assert!(buffer_contains_str(buf, "Tab"));
    }

    #[test]
    fn render_expanded_mode() {
        let mut app = make_app(120, 30);
        app.input_mode = InputMode::Expanded;
        let terminal = render_with(120, 30, |frame, _area| {
            app.render(frame);
        });
        let buf = terminal.backend().buffer();
        // Should show focused panel title
        assert!(buffer_contains_str(buf, "Workflow Map"));
    }

    #[test]
    fn render_hooks_not_configured_banner() {
        let mut app = make_app(120, 30);
        app.hook_setup = HookSetupState::NotConfigured {
            snippet: "{}".to_string(),
        };
        let terminal = render_with(120, 30, |frame, _area| {
            app.render(frame);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Hooks 미설정"));
    }

    #[test]
    fn render_hooks_configured_no_banner() {
        let mut app = make_app(120, 30);
        app.hook_setup = HookSetupState::Configured;
        let terminal = render_with(120, 30, |frame, _area| {
            app.render(frame);
        });
        let buf = terminal.backend().buffer();
        assert!(!buffer_contains_str(buf, "Hooks 미설정"));
    }

    // ── Unit 12: GateAlert integration tests ──

    #[test]
    fn test_available_panels_wide_includes_gate() {
        let app = make_app(200, 50);
        let panels = app.available_panels();
        assert_eq!(panels.len(), 5);
        assert!(panels.contains(&FocusPane::GateAlert));
    }

    #[test]
    fn test_available_panels_standard_excludes_gate() {
        let app = make_app(120, 30);
        let panels = app.available_panels();
        assert_eq!(panels.len(), 4);
        assert!(!panels.contains(&FocusPane::GateAlert));
    }

    #[test]
    fn test_gate_alert_event_broadcast() {
        let mut app = make_app(200, 50);
        app.gate_alert.set_hooks_active(true);
        app.handle_event(AppEvent::TurnCompleted {
            last_message: "A) Yes\nB) No".to_string(),
        });
        assert!(app.gate_alert.is_active());
    }

    #[test]
    fn render_wide_mode_gate_alert() {
        let mut app = make_app(200, 50);
        let terminal = render_with(200, 50, |frame, _area| {
            app.render(frame);
        });
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Gate Alert"));
    }
}
