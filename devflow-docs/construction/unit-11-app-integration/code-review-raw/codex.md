OpenAI Codex v0.117.0 (research preview)
--------
workdir: /Users/jay.ahn/projects/backend/devflow-tui
model: gpt-5.3-codex
provider: openai
approval: never
sandbox: workspace-write [workdir, /tmp, $TMPDIR, /Users/jay.ahn/.codex/memories]
reasoning effort: high
reasoning summaries: none
session id: 019d4159-0052-7562-88a4-588c0c0e8424
--------
user
You are a code quality reviewer for Rust. Review this ratatui TUI app code (Unit 11: App Integration). Focus on: code quality, naming, structure, error handling, idiomatic Rust, unnecessary complexity, DRY violations. Output markdown with Critical/High/Medium/Low severity. Do NOT modify any files - only output your review.
codex
I’ll review the current Rust `ratatui` app implementation for Unit 11 integration and produce a severity-ranked code quality audit only. First I’m locating the exact source files and then I’ll inspect structure, naming, error handling, and idiomatic patterns end-to-end.
exec
/bin/zsh -lc 'rg -n "Unit 11|unit 11|integration|App Integration|ratatui" -S .' in /Users/jay.ahn/projects/backend/devflow-tui
exec
/bin/zsh -lc 'pwd && rg --files' in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
/Users/jay.ahn/projects/backend/devflow-tui
src/event_loop.rs
src/app.rs
src/port/mod.rs
src/service/mod.rs
src/service/token.rs
src/service/sanitizer.rs
src/service/hook_config.rs
src/demo.rs
src/main.rs
src/component.rs
src/test_helpers.rs
src/parser/mod.rs
src/parser/audit_log.rs
src/parser/models.rs
src/parser/devflow_state.rs
src/event.rs
src/adapter/hooks_server.rs
src/adapter/git_poller.rs
src/adapter/file_watcher.rs
src/adapter/mod.rs
src/adapter/handle.rs
src/lib.rs
src/command.rs
src/config.rs
src/error.rs
src/panel/agent_status.rs
src/panel/git_status.rs
src/panel/workflow_map.rs
src/panel/mod.rs
src/panel/audit_log.rs
src/action.rs
src/ui/mod.rs
src/ui/help_overlay.rs
src/ui/status_bar.rs
src/ui/theme.rs
src/ui/header.rs
src/ui/layout.rs
clippy.toml
Cargo.lock
devflow-docs/session-summary.md
devflow-docs/construction/unit-03-file-watcher/code-plan.md
devflow-docs/construction/unit-04-git-adapter/code-plan.md
devflow-docs/construction/unit-11-app-integration/code-review-raw/gemini.md
devflow-docs/construction/unit-11-app-integration/code-review-raw/codex.md
devflow-docs/construction/unit-11-app-integration/code-plan.md
devflow-docs/construction/unit-02-parser/code-plan.md
devflow-docs/construction/unit-01-foundation/code-plan.md
devflow-docs/construction/unit-05-hooks-server/code-plan.md
devflow-docs/devflow-state.md
devflow-docs/audit.md
Cargo.toml
devflow-docs/inception/requirements.md
devflow-docs/inception/application-design.md
devflow-docs/inception/nfr-requirements.md
devflow-docs/inception/workflow-plan.md
devflow-docs/inception/workspace.md
devflow-docs/inception/units.md
devflow-docs/inception/user-stories.md

 succeeded in 0ms:
./src/event_loop.rs:6:use ratatui::Terminal;
./src/event_loop.rs:7:use ratatui::backend::CrosstermBackend;
./src/app.rs:4:use ratatui::Frame;
./src/app.rs:382:            let banner_area = ratatui::layout::Rect::new(body.x, body.y, body.width, 1);
./src/app.rs:392:            let banner = ratatui::widgets::Paragraph::new(banner_msg)
./src/app.rs:395:            body = ratatui::layout::Rect::new(body.x, body.y + 1, body.width, body.height - 1);
./src/app.rs:413:    fn render_panels(&mut self, frame: &mut Frame, body: ratatui::layout::Rect) {
./src/app.rs:418:                let msg = ratatui::widgets::Paragraph::new(
./src/app.rs:462:                let placeholder = ratatui::widgets::Block::bordered()
./src/app.rs:463:                    .border_type(ratatui::widgets::BorderType::Rounded)
./src/app.rs:471:    fn render_focused_panel(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
./src/demo.rs:31:        active_unit: Some("Unit 11".to_string()),
./src/demo.rs:54:            "ratatui 0.30 + crossterm 0.29".to_string(),
./src/demo.rs:56:        next_steps: vec!["Phase 4: App Integration".to_string()],
./src/test_helpers.rs:3:use ratatui::backend::TestBackend;
./src/test_helpers.rs:4:use ratatui::buffer::Buffer;
./src/test_helpers.rs:5:use ratatui::layout::Rect;
./src/test_helpers.rs:6:use ratatui::Terminal;
./src/test_helpers.rs:33:    F: FnOnce(&mut ratatui::Frame, Rect),
./src/main.rs:7:use ratatui::{Terminal, backend::CrosstermBackend};
./src/panel/agent_status.rs:5:use ratatui::layout::Rect;
./src/panel/agent_status.rs:6:use ratatui::style::Stylize;
./src/panel/agent_status.rs:7:use ratatui::text::{Line, Span};
./src/panel/agent_status.rs:8:use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph};
./src/panel/agent_status.rs:9:use ratatui::Frame;
./src/ui/layout.rs:1:use ratatui::layout::{Constraint, Layout, Rect};
./src/ui/layout.rs:273:            let block = ratatui::widgets::Block::bordered();
./src/ui/layout.rs:289:            let block = ratatui::widgets::Block::bordered()
./src/ui/layout.rs:291:            frame.render_widget(ratatui::widgets::Clear, center);
./src/ui/theme.rs:3:use ratatui::style::{Modifier, Style, Stylize};
./src/ui/theme.rs:4:use ratatui::text::Span;
./src/ui/theme.rs:207:        use ratatui::text::Line;
./src/ui/theme.rs:208:        use ratatui::widgets::Paragraph;
./src/ui/theme.rs:223:        use ratatui::text::Line;
./src/ui/theme.rs:224:        use ratatui::widgets::Paragraph;
./src/ui/help_overlay.rs:1:use ratatui::layout::Rect;
./src/ui/help_overlay.rs:2:use ratatui::text::{Line, Text};
./src/ui/help_overlay.rs:3:use ratatui::widgets::{Block, BorderType, Clear, Paragraph, Wrap};
./src/ui/help_overlay.rs:4:use ratatui::Frame;
./src/component.rs:2:use ratatui::{Frame, layout::Rect};
./src/ui/status_bar.rs:1:use ratatui::layout::Rect;
./src/ui/status_bar.rs:2:use ratatui::style::Modifier;
./src/ui/status_bar.rs:3:use ratatui::text::{Line, Span};
./src/ui/status_bar.rs:4:use ratatui::widgets::Paragraph;
./src/ui/status_bar.rs:5:use ratatui::Frame;
./src/ui/status_bar.rs:43:        ratatui::style::Style::default().add_modifier(Modifier::REVERSED)
./src/ui/status_bar.rs:45:        ratatui::style::Style::new().on_dark_gray().white()
./src/ui/header.rs:1:use ratatui::layout::Rect;
./src/ui/header.rs:2:use ratatui::style::Modifier;
./src/ui/header.rs:3:use ratatui::text::{Line, Span};
./src/ui/header.rs:4:use ratatui::widgets::Paragraph;
./src/ui/header.rs:5:use ratatui::Frame;
./src/ui/header.rs:45:    let widget = Paragraph::new(line).style(ratatui::style::Style::new().add_modifier(Modifier::DIM));
./src/panel/git_status.rs:2:use ratatui::layout::{Constraint, Layout, Rect};
./src/panel/git_status.rs:3:use ratatui::style::Stylize;
./src/panel/git_status.rs:4:use ratatui::text::{Line, Span};
./src/panel/git_status.rs:5:use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph};
./src/panel/git_status.rs:6:use ratatui::Frame;
./src/panel/audit_log.rs:2:use ratatui::layout::Rect;
./src/panel/audit_log.rs:3:use ratatui::style::Stylize;
./src/panel/audit_log.rs:4:use ratatui::text::{Line, Span};
./src/panel/audit_log.rs:5:use ratatui::widgets::{
./src/panel/audit_log.rs:8:use ratatui::Frame;
./src/panel/audit_log.rs:171:            let msg = ratatui::widgets::Paragraph::new(
./devflow-docs/session-summary.md:8:- Stage: code-generation (Phase 4 Unit 11 구현 완료)
./devflow-docs/session-summary.md:13:- 2026-03-30 workspace-detection B -- Greenfield, Rust+ratatui 기술스택
./devflow-docs/session-summary.md:20:- 2026-03-30 application-design B -- v3 (council + ratatui 리뷰, Implementation Notes 포함)
./devflow-docs/session-summary.md:25:- [x] workspace-detection -- Greenfield, Rust+ratatui+tokio+axum
./devflow-docs/session-summary.md:38:- [~] Phase 4: Unit 11 (App 통합) -- 182 tests, 리뷰 대기
./devflow-docs/session-summary.md:42:- Phase 4: Unit 11 (App 통합 + 이벤트 루프 + 레이아웃 + 네비게이션)
./devflow-docs/session-summary.md:63:  - Unit 11 code-plan에 명시 필수
./devflow-docs/devflow-state.md:7:code-generation (Phase 4 Unit 11 구현 완료, 리뷰 대기)
./devflow-docs/devflow-state.md:22:- application-design (v3, council + ratatui 리뷰 반영)
./src/adapter/file_watcher.rs:192:// notify crate behavior which is tested via manual/integration testing.
./src/panel/workflow_map.rs:2:use ratatui::layout::Rect;
./src/panel/workflow_map.rs:3:use ratatui::style::Stylize;
./src/panel/workflow_map.rs:4:use ratatui::text::{Line, Text};
./src/panel/workflow_map.rs:5:use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
./src/panel/workflow_map.rs:6:use ratatui::Frame;
./src/panel/workflow_map.rs:116:                ratatui::text::Span::from(prefix).dim(),
./src/panel/workflow_map.rs:189:                    ratatui::text::Span::from(format!("    {icon} ")).style(style),
./src/panel/workflow_map.rs:190:                    ratatui::text::Span::from(item.text.as_str()).style(style),
./src/panel/workflow_map.rs:349:        panel.session_summary.key_decisions.push("Use ratatui".to_string());
./src/panel/workflow_map.rs:355:        assert!(buffer_contains_str(buf, "Use ratatui"));
./Cargo.lock:397: "ratatui",
./Cargo.lock:1424:name = "ratatui"
./Cargo.lock:1430: "ratatui-core",
./Cargo.lock:1431: "ratatui-crossterm",
./Cargo.lock:1432: "ratatui-macros",
./Cargo.lock:1433: "ratatui-termwiz",
./Cargo.lock:1434: "ratatui-widgets",
./Cargo.lock:1438:name = "ratatui-core"
./Cargo.lock:1458:name = "ratatui-crossterm"
./Cargo.lock:1466: "ratatui-core",
./Cargo.lock:1470:name = "ratatui-macros"
./Cargo.lock:1475: "ratatui-core",
./Cargo.lock:1476: "ratatui-widgets",
./Cargo.lock:1480:name = "ratatui-termwiz"
./Cargo.lock:1485: "ratatui-core",
./Cargo.lock:1490:name = "ratatui-widgets"
./Cargo.lock:1501: "ratatui-core",
./Cargo.toml:7:ratatui = "0.30"
./devflow-docs/audit.md:4:[2026-03-31T00:00:00+09:00] code-plan: Unit 11 (App 통합) — B (승인, code-plan 작성 진행)
./devflow-docs/audit.md:6:[2026-03-31T00:00:00+09:00] code-plan: Unit 11 — B (승인, 코드 생성 진행)
./devflow-docs/inception/units.md:28:   └── Unit 11: App 통합 + 레이아웃 + 네비게이션
./devflow-docs/inception/units.md:47:- `main.rs` — 터미널 셋업/복원 + panic hook + tracing 초기화 (이벤트 루프는 Unit 11)
./devflow-docs/inception/units.md:206:### Phase 4: 통합 (Unit 11)
./devflow-docs/inception/units.md:208:#### Unit 11: App 통합 + 이벤트 루프 + 레이아웃 + 네비게이션
./devflow-docs/inception/units.md:251:**의존성:** Unit 11 (App 통합)
./devflow-docs/inception/units.md:272:| 12 | [P1] 산출물 + 게이트 | 5 | Unit 11 | — |
./devflow-docs/inception/units.md:280:Phase 4: Unit 11                             (통합)        → 3자 리뷰 → Fix
./devflow-docs/inception/workspace.md:13:| TUI 프레임워크 | ratatui 0.30 + crossterm 0.29 |
./devflow-docs/inception/nfr-requirements.md:9:- ratatui immediate-mode 렌더링 모델 적용:
./devflow-docs/inception/nfr-requirements.md:10:  - 매 tick마다 `terminal.draw()` 호출, ratatui 내부 backend diff가 변경된 셀만 업데이트
./devflow-docs/inception/nfr-requirements.md:137:- **v2**: agent-council 리뷰 반영 — ratatui immediate-mode 렌더링 모델, SLO p95 측정 조건, burst write/single-save 구분, CPU NFR 추가, JoinHandle supervisor, 대체 포트 탐색, 축소 터미널 대응, stable token, NO_COLOR, 테스트 harness, backpressure 정책
./devflow-docs/inception/workflow-plan.md:43:Unit 11: [P1] 산출물 미리보기 + 게이트 알림
./devflow-docs/inception/application-design.md:755:### 3.1 ratatui 위젯 매핑
./devflow-docs/inception/application-design.md:759:| Workflow Map | `Paragraph` + `Line`/`Span` 조합 | 없음 (정적 렌더링) | `├──`, `└──` 접두어 + `Span` 스타일로 트리 구현. ratatui에 내장 Tree 없음 |
./devflow-docs/inception/application-design.md:774:use ratatui::style::Stylize;
./devflow-docs/inception/application-design.md:894:상태바의 단축키 힌트는 nexttui/ratatui 관용 패턴:
./devflow-docs/inception/application-design.md:915:- **v3**: ratatui 0.30 리뷰 반영 — Part 3 Implementation Notes 추가 (위젯 매핑, Stylize/theme.rs, Rounded 테두리, Clear 모달, Fill constraint, Scrollbar, tick 250ms, 리사이즈 재보정)
./devflow-docs/inception/requirements.md:168:| TUI 프레임워크 | ratatui 0.30 + crossterm 0.29 |
./devflow-docs/construction/unit-11-app-integration/code-review-raw/codex.md:13:You are a code quality reviewer for Rust. Review this ratatui TUI app code (Unit 11: App Integration). Focus on: code quality, naming, structure, error handling, idiomatic Rust, unnecessary complexity, DRY violations. Output markdown with Critical/High/Medium/Low severity. Do NOT modify any files - only output your review.
./devflow-docs/construction/unit-11-app-integration/code-review-raw/codex.md:15:I’ll review the current Rust `ratatui` app implementation for Unit 11 integration and produce a severity-ranked code quality audit only. First I’m locating the exact source files and then I’ll inspect structure, naming, error handling, and idiomatic patterns end-to-end.
./devflow-docs/construction/unit-11-app-integration/code-review-raw/gemini.md:2:I have analyzed the provided code for Unit 11 (App Integration) of `devflow-tui`. To provide a comprehensive security and architectural review, I will first examine the underlying implementations of the token service and the hooks server, as these are critical areas for security and resource management.
./devflow-docs/construction/unit-11-app-integration/code-review-raw/gemini.md:17:This is a security and edge-case review for the **Unit 11: App Integration** phase of `devflow-tui`.
./devflow-docs/construction/unit-11-app-integration/code-review-raw/gemini.md:19:The review covers the integration of the HTTP hooks server, file watcher, git poller, and the main event loop.
./devflow-docs/construction/unit-11-app-integration/code-plan.md:1:# Code Plan: Unit 11 — App 통합 + 이벤트 루프 + 레이아웃 + 네비게이션
./devflow-docs/construction/unit-01-foundation/code-plan.md:76:  - main 함수: config 로드 → tracing 초기화 → 터미널 셋업 → (이벤트 루프는 Unit 11) → cleanup

codex
I’ve identified the Unit 11 surface area; next I’m reading `app`, `event_loop`, and startup wiring to find concrete quality issues with line-level references. After that I’ll scan adapters/panels for DRY and complexity hotspots.
exec
/bin/zsh -lc "nl -ba src/app.rs | sed -n '1,260p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::path::PathBuf;
     2	
     3	use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
     4	use ratatui::Frame;
     5	use tokio::sync::mpsc;
     6	
     7	use crate::action::{Action, Direction};
     8	use crate::command::CommandRunner;
     9	use crate::component::Component;
    10	use crate::event::AppEvent;
    11	use crate::panel::agent_status::AgentStatusPanel;
    12	use crate::panel::audit_log::AuditLogPanel;
    13	use crate::panel::git_status::GitStatusPanel;
    14	use crate::panel::workflow_map::WorkflowMapPanel;
    15	use crate::parser::models::{FlowState, GitSnapshot};
    16	use crate::service::hook_config::{self, HookConfigStatus};
    17	use crate::ui::layout::{LayoutManager, LayoutMode, PanelAreas};
    18	
    19	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
    20	pub enum FocusPane {
    21	    WorkflowMap,
    22	    GitStatus,
    23	    AgentStatus,
    24	    AuditLog,
    25	}
    26	
    27	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
    28	pub enum InputMode {
    29	    Normal,
    30	    Expanded,
    31	    HelpOverlay,
    32	}
    33	
    34	#[derive(Debug, Clone)]
    35	pub enum HookSetupState {
    36	    Unknown,
    37	    Configured,
    38	    NotConfigured { snippet: String },
    39	    Mismatch { detail: String },
    40	}
    41	
    42	pub struct App {
    43	    pub should_quit: bool,
    44	    pub input_mode: InputMode,
    45	    pub focus: FocusPane,
    46	
    47	    pub workflow_map: WorkflowMapPanel,
    48	    pub git_status: GitStatusPanel,
    49	    pub agent_status: AgentStatusPanel,
    50	    pub audit_log: AuditLogPanel,
    51	
    52	    pub layout: LayoutManager,
    53	    pub hooks_active: bool,
    54	    pub hooks_port: Option<u16>,
    55	    pub hook_setup: HookSetupState,
    56	    pub token: String,
    57	    pub project_dir: PathBuf,
    58	    pub phase: String,
    59	    pub stage: String,
    60	
    61	    command_runner: CommandRunner,
    62	    event_tx: mpsc::Sender<AppEvent>,
    63	}
    64	
    65	impl App {
    66	    pub fn new(
    67	        width: u16,
    68	        height: u16,
    69	        event_tx: mpsc::Sender<AppEvent>,
    70	        token: String,
    71	        project_dir: PathBuf,
    72	    ) -> Self {
    73	        Self {
    74	            should_quit: false,
    75	            input_mode: InputMode::Normal,
    76	            focus: FocusPane::WorkflowMap,
    77	
    78	            workflow_map: WorkflowMapPanel::new(),
    79	            git_status: GitStatusPanel::new(),
    80	            agent_status: AgentStatusPanel::new(),
    81	            audit_log: AuditLogPanel::new(),
    82	
    83	            layout: LayoutManager::new(width, height),
    84	            hooks_active: false,
    85	            hooks_port: None,
    86	            hook_setup: HookSetupState::Unknown,
    87	            token,
    88	            project_dir,
    89	            phase: "INCEPTION".to_string(),
    90	            stage: String::new(),
    91	
    92	            command_runner: CommandRunner::new(event_tx.clone()),
    93	            event_tx,
    94	        }
    95	    }
    96	
    97	    pub fn available_panels(&self) -> Vec<FocusPane> {
    98	        // All layout modes show the same 4 panels for now (P1 adds more in Wide)
    99	        vec![
   100	            FocusPane::WorkflowMap,
   101	            FocusPane::GitStatus,
   102	            FocusPane::AgentStatus,
   103	            FocusPane::AuditLog,
   104	        ]
   105	    }
   106	
   107	    pub fn ensure_valid_focus(&mut self) {
   108	        let available = self.available_panels();
   109	        if !available.contains(&self.focus) {
   110	            self.focus = available[0];
   111	        }
   112	    }
   113	
   114	    pub fn focus_name(&self) -> &str {
   115	        match self.focus {
   116	            FocusPane::WorkflowMap => "Workflow Map",
   117	            FocusPane::GitStatus => "Git Status",
   118	            FocusPane::AgentStatus => "Agent Status",
   119	            FocusPane::AuditLog => "Audit Log",
   120	        }
   121	    }
   122	
   123	    pub fn focused_panel_mut(&mut self) -> &mut dyn Component {
   124	        match self.focus {
   125	            FocusPane::WorkflowMap => &mut self.workflow_map,
   126	            FocusPane::GitStatus => &mut self.git_status,
   127	            FocusPane::AgentStatus => &mut self.agent_status,
   128	            FocusPane::AuditLog => &mut self.audit_log,
   129	        }
   130	    }
   131	
   132	    // ── Key handling (Step 2) ──
   133	
   134	    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
   135	        match self.input_mode {
   136	            InputMode::HelpOverlay => {
   137	                if matches!(key.code, KeyCode::Esc | KeyCode::Char('?')) {
   138	                    self.input_mode = InputMode::Normal;
   139	                }
   140	                true
   141	            }
   142	            InputMode::Expanded => {
   143	                match key.code {
   144	                    KeyCode::Esc | KeyCode::Char('f') => {
   145	                        self.input_mode = InputMode::Normal;
   146	                        true
   147	                    }
   148	                    _ => {
   149	                        // Delegate to focused panel in expanded mode
   150	                        if let Some(action) = self.focused_panel_mut().handle_key(key) {
   151	                            self.execute_action(action);
   152	                        }
   153	                        true
   154	                    }
   155	                }
   156	            }
   157	            InputMode::Normal => self.handle_normal_key(key),
   158	        }
   159	    }
   160	
   161	    fn handle_normal_key(&mut self, key: KeyEvent) -> bool {
   162	        // Global keys
   163	        match key.code {
   164	            KeyCode::Char('q') => {
   165	                self.should_quit = true;
   166	                return true;
   167	            }
   168	            KeyCode::Char('?') => {
   169	                self.input_mode = InputMode::HelpOverlay;
   170	                return true;
   171	            }
   172	            KeyCode::Char('f') => {
   173	                self.input_mode = InputMode::Expanded;
   174	                return true;
   175	            }
   176	            KeyCode::Tab => {
   177	                self.execute_action(Action::FocusNextPanel);
   178	                return true;
   179	            }
   180	            KeyCode::BackTab => {
   181	                self.execute_action(Action::FocusPrevPanel);
   182	                return true;
   183	            }
   184	            KeyCode::Char('r') => {
   185	                self.command_runner.execute(Action::Refresh);
   186	                return true;
   187	            }
   188	            KeyCode::Char('c') => {
   189	                self.copy_hooks_snippet();
   190	                return true;
   191	            }
   192	            _ => {}
   193	        }
   194	
   195	        // Ctrl+hjkl direction navigation
   196	        if key.modifiers.contains(KeyModifiers::CONTROL) {
   197	            let dir = match key.code {
   198	                KeyCode::Char('h') => Some(Direction::Left),
   199	                KeyCode::Char('j') => Some(Direction::Down),
   200	                KeyCode::Char('k') => Some(Direction::Up),
   201	                KeyCode::Char('l') => Some(Direction::Right),
   202	                _ => None,
   203	            };
   204	            if let Some(d) = dir {
   205	                self.execute_action(Action::FocusDirection(d));
   206	                return true;
   207	            }
   208	        }
   209	
   210	        // Delegate to focused panel
   211	        if let Some(action) = self.focused_panel_mut().handle_key(key) {
   212	            self.execute_action(action);
   213	            return true;
   214	        }
   215	
   216	        false
   217	    }
   218	
   219	    pub fn execute_action(&mut self, action: Action) {
   220	        match action {
   221	            Action::FocusNextPanel => {
   222	                let panels = self.available_panels();
   223	                if let Some(idx) = panels.iter().position(|p| *p == self.focus) {
   224	                    self.focus = panels[(idx + 1) % panels.len()];
   225	                }
   226	            }
   227	            Action::FocusPrevPanel => {
   228	                let panels = self.available_panels();
   229	                if let Some(idx) = panels.iter().position(|p| *p == self.focus) {
   230	                    self.focus = panels[(idx + panels.len() - 1) % panels.len()];
   231	                }
   232	            }
   233	            Action::FocusDirection(dir) => {
   234	                self.focus_direction(dir);
   235	            }
   236	            Action::ExpandPanel => {
   237	                self.input_mode = InputMode::Expanded;
   238	            }
   239	            Action::CollapsePanel => {
   240	                self.input_mode = InputMode::Normal;
   241	            }
   242	            Action::Quit => {
   243	                self.should_quit = true;
   244	            }
   245	            action if action.is_async() => {
   246	                self.command_runner.execute(action);
   247	            }
   248	            _ => {
   249	                // Sync panel actions (ScrollUp/Down, Select, etc.) are already
   250	                // handled by the panel's handle_key
   251	            }
   252	        }
   253	    }
   254	
   255	    fn focus_direction(&mut self, dir: Direction) {
   256	        // Standard/Compact: Left/Right act like Tab/Shift+Tab
   257	        // Wide: 2x3 grid navigation (to be expanded in Unit 12)
   258	        match dir {
   259	            Direction::Right => self.execute_action(Action::FocusNextPanel),
   260	            Direction::Left => self.execute_action(Action::FocusPrevPanel),

exec
/bin/zsh -lc "nl -ba src/app.rs | sed -n '261,620p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
   261	            _ => {} // Up/Down: noop for Standard/Compact
   262	        }
   263	    }
   264	
   265	    // ── Event handling (Step 3) ──
   266	
   267	    pub fn handle_event(&mut self, event: AppEvent) {
   268	        // Broadcast to all panels
   269	        self.workflow_map.handle_event(&event);
   270	        self.git_status.handle_event(&event);
   271	        self.agent_status.handle_event(&event);
   272	        self.audit_log.handle_event(&event);
   273	
   274	        // App-level handling
   275	        match &event {
   276	            AppEvent::HooksServerStarted { port } => {
   277	                self.hooks_active = true;
   278	                self.hooks_port = Some(*port);
   279	                self.check_hooks_config();
   280	            }
   281	            AppEvent::HooksServerFailed { .. } => {
   282	                self.hooks_active = false;
   283	                self.hook_setup = HookSetupState::Unknown;
   284	            }
   285	            AppEvent::FlowStateChanged(state) => {
   286	                self.phase = state.phase.to_string();
   287	                self.stage = state.stage.clone();
   288	            }
   289	            _ => {}
   290	        }
   291	    }
   292	
   293	    pub fn handle_flow_state(&mut self, state: FlowState) {
   294	        self.phase = state.phase.to_string();
   295	        self.stage = state.stage.clone();
   296	        self.workflow_map.set_flow_state(state);
   297	    }
   298	
   299	    pub fn handle_git_snapshot(&mut self, snapshot: GitSnapshot) {
   300	        self.git_status.set_snapshot(snapshot);
   301	    }
   302	
   303	    pub fn on_tick(&mut self) -> bool {
   304	        self.agent_status.check_timeouts()
   305	    }
   306	
   307	    pub fn on_resize(&mut self, w: u16, h: u16) {
   308	        self.layout.on_resize(w, h);
   309	        self.ensure_valid_focus();
   310	        self.workflow_map.scroll_offset = 0;
   311	        self.git_status.clamp_scroll();
   312	        self.agent_status.clamp_scroll();
   313	        self.audit_log.clamp_scroll();
   314	    }
   315	
   316	    pub fn check_hooks_config(&mut self) {
   317	        let port = match self.hooks_port {
   318	            Some(p) => p,
   319	            None => return,
   320	        };
   321	        let status = hook_config::check_hooks_config(&self.project_dir, port, &self.token);
   322	        self.hook_setup = match status {
   323	            HookConfigStatus::Configured => HookSetupState::Configured,
   324	            HookConfigStatus::NotConfigured => HookSetupState::NotConfigured {
   325	                snippet: hook_config::generate_hooks_snippet(port, &self.token),
   326	            },
   327	            HookConfigStatus::EndpointMismatch { configured_url } => {
   328	                HookSetupState::Mismatch {
   329	                    detail: configured_url,
   330	                }
   331	            }
   332	        };
   333	    }
   334	
   335	    pub fn copy_hooks_snippet(&mut self) {
   336	        if let HookSetupState::NotConfigured { ref snippet }
   337	        | HookSetupState::Mismatch { detail: ref snippet } = self.hook_setup
   338	        {
   339	            self.command_runner
   340	                .execute(Action::CopyToClipboard(snippet.clone()));
   341	        }
   342	    }
   343	
   344	    // ── Rendering (Step 4) ──
   345	
   346	    pub fn render(&mut self, frame: &mut Frame) {
   347	        let areas = self.layout.areas(frame.area());
   348	        let wide_mode = self.layout.mode() == LayoutMode::Wide;
   349	
   350	        // Header
   351	        crate::ui::header::render(
   352	            frame,
   353	            areas.header,
   354	            &self.phase,
   355	            self.hooks_active,
   356	            self.hooks_port,
   357	        );
   358	
   359	        // Status bar
   360	        let phase_stage = if self.stage.is_empty() {
   361	            self.phase.clone()
   362	        } else {
   363	            format!("{} > {}", self.phase, self.stage)
   364	        };
   365	        crate::ui::status_bar::render(
   366	            frame,
   367	            areas.status_bar,
   368	            self.focus_name(),
   369	            &phase_stage,
   370	            wide_mode,
   371	        );
   372	
   373	        // Body
   374	        let mut body = areas.body;
   375	
   376	        // Hooks setup banner
   377	        if matches!(
   378	            self.hook_setup,
   379	            HookSetupState::NotConfigured { .. } | HookSetupState::Mismatch { .. }
   380	        ) && body.height > 1
   381	        {
   382	            let banner_area = ratatui::layout::Rect::new(body.x, body.y, body.width, 1);
   383	            let banner_msg = match &self.hook_setup {
   384	                HookSetupState::NotConfigured { .. } => {
   385	                    " ⚠ Hooks 미설정 — c 키로 설정 JSON 복사"
   386	                }
   387	                HookSetupState::Mismatch { .. } => {
   388	                    " ⚠ Hooks 설정 불일치 — c 키로 새 설정 복사"
   389	                }
   390	                _ => "",
   391	            };
   392	            let banner = ratatui::widgets::Paragraph::new(banner_msg)
   393	                .style(crate::ui::theme::Theme::gate_alert());
   394	            frame.render_widget(banner, banner_area);
   395	            body = ratatui::layout::Rect::new(body.x, body.y + 1, body.width, body.height - 1);
   396	        }
   397	
   398	        // Main content based on input mode
   399	        match self.input_mode {
   400	            InputMode::HelpOverlay => {
   401	                self.render_panels(frame, body);
   402	                crate::ui::help_overlay::render(frame, frame.area());
   403	            }
   404	            InputMode::Expanded => {
   405	                self.render_focused_panel(frame, body);
   406	            }
   407	            InputMode::Normal => {
   408	                self.render_panels(frame, body);
   409	            }
   410	        }
   411	    }
   412	
   413	    fn render_panels(&mut self, frame: &mut Frame, body: ratatui::layout::Rect) {
   414	        let panel_areas = self.layout.panel_areas(body);
   415	
   416	        match panel_areas {
   417	            PanelAreas::TooSmall { message } => {
   418	                let msg = ratatui::widgets::Paragraph::new(
   419	                    "터미널이 너무 작습니다 (최소 80x24)",
   420	                )
   421	                .style(crate::ui::theme::Theme::error());
   422	                frame.render_widget(msg, message);
   423	            }
   424	            PanelAreas::Compact { panel } => {
   425	                self.render_focused_panel(frame, panel);
   426	            }
   427	            PanelAreas::Standard {
   428	                workflow_map,
   429	                git_status,
   430	                agent_status,
   431	                audit_log,
   432	            } => {
   433	                let focus = self.focus;
   434	                self.workflow_map
   435	                    .render(frame, workflow_map, focus == FocusPane::WorkflowMap);
   436	                self.git_status
   437	                    .render(frame, git_status, focus == FocusPane::GitStatus);
   438	                self.agent_status
   439	                    .render(frame, agent_status, focus == FocusPane::AgentStatus);
   440	                self.audit_log
   441	                    .render(frame, audit_log, focus == FocusPane::AuditLog);
   442	            }
   443	            PanelAreas::Wide {
   444	                workflow_map,
   445	                git_status,
   446	                artifacts,
   447	                agent_status,
   448	                audit_log,
   449	                gate_alert,
   450	            } => {
   451	                let focus = self.focus;
   452	                self.workflow_map
   453	                    .render(frame, workflow_map, focus == FocusPane::WorkflowMap);
   454	                self.git_status
   455	                    .render(frame, git_status, focus == FocusPane::GitStatus);
   456	                self.agent_status
   457	                    .render(frame, agent_status, focus == FocusPane::AgentStatus);
   458	                self.audit_log
   459	                    .render(frame, audit_log, focus == FocusPane::AuditLog);
   460	
   461	                // P1 placeholder blocks
   462	                let placeholder = ratatui::widgets::Block::bordered()
   463	                    .border_type(ratatui::widgets::BorderType::Rounded)
   464	                    .border_style(crate::ui::theme::Theme::unfocus_border());
   465	                frame.render_widget(placeholder.clone().title("  Artifacts  "), artifacts);
   466	                frame.render_widget(placeholder.title("  Gate Alert  "), gate_alert);
   467	            }
   468	        }
   469	    }
   470	
   471	    fn render_focused_panel(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
   472	        match self.focus {
   473	            FocusPane::WorkflowMap => self.workflow_map.render(frame, area, true),
   474	            FocusPane::GitStatus => self.git_status.render(frame, area, true),
   475	            FocusPane::AgentStatus => self.agent_status.render(frame, area, true),
   476	            FocusPane::AuditLog => self.audit_log.render(frame, area, true),
   477	        }
   478	    }
   479	}
   480	
   481	#[cfg(test)]
   482	mod tests {
   483	    use super::*;
   484	    use crate::test_helpers::{buffer_contains_str, render_with};
   485	    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
   486	
   487	    fn make_app(width: u16, height: u16) -> App {
   488	        let (tx, _rx) = mpsc::channel(16);
   489	        App::new(width, height, tx, "test-token".to_string(), PathBuf::from("/tmp/test"))
   490	    }
   491	
   492	    // ── Step 1: App struct tests ──
   493	
   494	    #[test]
   495	    fn test_available_panels_compact() {
   496	        let app = make_app(80, 24);
   497	        let panels = app.available_panels();
   498	        assert_eq!(panels.len(), 4);
   499	        assert_eq!(panels[0], FocusPane::WorkflowMap);
   500	    }
   501	
   502	    #[test]
   503	    fn test_available_panels_standard() {
   504	        let app = make_app(120, 30);
   505	        let panels = app.available_panels();
   506	        assert_eq!(panels.len(), 4);
   507	    }
   508	
   509	    #[test]
   510	    fn test_ensure_valid_focus() {
   511	        let mut app = make_app(80, 24);
   512	        // Focus is valid, should not change
   513	        app.focus = FocusPane::AuditLog;
   514	        app.ensure_valid_focus();
   515	        assert_eq!(app.focus, FocusPane::AuditLog);
   516	    }
   517	
   518	    #[test]
   519	    fn test_focus_name() {
   520	        let app = make_app(80, 24);
   521	        assert_eq!(app.focus_name(), "Workflow Map");
   522	    }
   523	
   524	    // ── Step 2: Key handling tests ──
   525	
   526	    #[test]
   527	    fn test_handle_key_quit() {
   528	        let mut app = make_app(80, 24);
   529	        app.handle_key(KeyEvent::from(KeyCode::Char('q')));
   530	        assert!(app.should_quit);
   531	    }
   532	
   533	    #[test]
   534	    fn test_handle_key_tab_focus() {
   535	        let mut app = make_app(80, 24);
   536	        assert_eq!(app.focus, FocusPane::WorkflowMap);
   537	        app.handle_key(KeyEvent::from(KeyCode::Tab));
   538	        assert_eq!(app.focus, FocusPane::GitStatus);
   539	        app.handle_key(KeyEvent::from(KeyCode::Tab));
   540	        assert_eq!(app.focus, FocusPane::AgentStatus);
   541	        app.handle_key(KeyEvent::from(KeyCode::Tab));
   542	        assert_eq!(app.focus, FocusPane::AuditLog);
   543	        app.handle_key(KeyEvent::from(KeyCode::Tab));
   544	        assert_eq!(app.focus, FocusPane::WorkflowMap); // wraps around
   545	    }
   546	
   547	    #[test]
   548	    fn test_handle_key_backtab_focus() {
   549	        let mut app = make_app(80, 24);
   550	        app.handle_key(KeyEvent::from(KeyCode::BackTab));
   551	        assert_eq!(app.focus, FocusPane::AuditLog); // wraps backward
   552	    }
   553	
   554	    #[test]
   555	    fn test_handle_key_help_overlay() {
   556	        let mut app = make_app(80, 24);
   557	        app.handle_key(KeyEvent::from(KeyCode::Char('?')));
   558	        assert_eq!(app.input_mode, InputMode::HelpOverlay);
   559	    }
   560	
   561	    #[test]
   562	    fn test_help_overlay_esc_returns_normal() {
   563	        let mut app = make_app(80, 24);
   564	        app.input_mode = InputMode::HelpOverlay;
   565	        app.handle_key(KeyEvent::from(KeyCode::Esc));
   566	        assert_eq!(app.input_mode, InputMode::Normal);
   567	    }
   568	
   569	    #[test]
   570	    fn test_help_overlay_question_returns_normal() {
   571	        let mut app = make_app(80, 24);
   572	        app.input_mode = InputMode::HelpOverlay;
   573	        app.handle_key(KeyEvent::from(KeyCode::Char('?')));
   574	        assert_eq!(app.input_mode, InputMode::Normal);
   575	    }
   576	
   577	    #[test]
   578	    fn test_handle_key_expand() {
   579	        let mut app = make_app(80, 24);
   580	        app.handle_key(KeyEvent::from(KeyCode::Char('f')));
   581	        assert_eq!(app.input_mode, InputMode::Expanded);
   582	    }
   583	
   584	    #[test]
   585	    fn test_expanded_esc_returns_normal() {
   586	        let mut app = make_app(80, 24);
   587	        app.input_mode = InputMode::Expanded;
   588	        app.handle_key(KeyEvent::from(KeyCode::Esc));
   589	        assert_eq!(app.input_mode, InputMode::Normal);
   590	    }
   591	
   592	    #[test]
   593	    fn test_expanded_f_returns_normal() {
   594	        let mut app = make_app(80, 24);
   595	        app.input_mode = InputMode::Expanded;
   596	        app.handle_key(KeyEvent::from(KeyCode::Char('f')));
   597	        assert_eq!(app.input_mode, InputMode::Normal);
   598	    }
   599	
   600	    #[test]
   601	    fn test_focus_direction_standard() {
   602	        let mut app = make_app(120, 30);
   603	        assert_eq!(app.focus, FocusPane::WorkflowMap);
   604	        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL);
   605	        app.handle_key(key);
   606	        assert_eq!(app.focus, FocusPane::GitStatus); // Right = next
   607	    }
   608	
   609	    #[test]
   610	    fn test_handle_key_c_noop_when_configured() {
   611	        let mut app = make_app(80, 24);
   612	        app.hook_setup = HookSetupState::Configured;
   613	        // c key should not trigger copy when hooks are configured
   614	        let result = app.handle_key(KeyEvent::from(KeyCode::Char('c')));
   615	        assert!(result); // key was consumed, but no copy action
   616	    }
   617	
   618	    // ── Step 3: Event handling tests ──
   619	
   620	    #[test]

exec
/bin/zsh -lc "nl -ba src/event_loop.rs | sed -n '1,320p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::io;
     2	use std::time::Duration;
     3	
     4	use crossterm::event::{Event, EventStream, KeyEventKind};
     5	use futures::StreamExt;
     6	use ratatui::Terminal;
     7	use ratatui::backend::CrosstermBackend;
     8	use tokio::sync::{mpsc, watch};
     9	
    10	use crate::adapter::handle::AdapterHandle;
    11	use crate::app::App;
    12	use crate::error::Result;
    13	use crate::event::AppEvent;
    14	use crate::parser::models::{FlowState, GitSnapshot};
    15	
    16	const TICK_RATE_MS: u64 = 250;
    17	
    18	/// Run the main event loop.
    19	///
    20	/// Integrates keyboard input, adapter events, and periodic ticks
    21	/// into a single tokio::select! loop with conditional rendering.
    22	pub async fn run_event_loop(
    23	    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    24	    app: &mut App,
    25	    mut event_rx: mpsc::Receiver<AppEvent>,
    26	    mut flow_state_rx: watch::Receiver<FlowState>,
    27	    mut git_snapshot_rx: watch::Receiver<GitSnapshot>,
    28	    adapter_handles: Vec<AdapterHandle>,
    29	) -> Result<()> {
    30	    let mut key_events = EventStream::new();
    31	    let mut tick = tokio::time::interval(Duration::from_millis(TICK_RATE_MS));
    32	    let mut needs_render = true;
    33	
    34	    loop {
    35	        tokio::select! {
    36	            // Branch 1: Keyboard + resize events
    37	            maybe_event = key_events.next() => {
    38	                if let Some(Ok(event)) = maybe_event {
    39	                    match event {
    40	                        Event::Key(key) if key.kind == KeyEventKind::Press => {
    41	                            needs_render = app.handle_key(key);
    42	                        }
    43	                        Event::Resize(w, h) => {
    44	                            app.on_resize(w, h);
    45	                            needs_render = true;
    46	                        }
    47	                        _ => {}
    48	                    }
    49	                }
    50	            }
    51	
    52	            // Branch 2: Periodic tick (250ms)
    53	            _ = tick.tick() => {
    54	                if app.on_tick() {
    55	                    needs_render = true;
    56	                }
    57	            }
    58	
    59	            // Branch 3: Flow state changes (watch channel)
    60	            Ok(()) = flow_state_rx.changed() => {
    61	                let state = flow_state_rx.borrow_and_update().clone();
    62	                app.handle_flow_state(state);
    63	                needs_render = true;
    64	            }
    65	
    66	            // Branch 4: Git snapshot changes (watch channel)
    67	            Ok(()) = git_snapshot_rx.changed() => {
    68	                let snapshot = git_snapshot_rx.borrow_and_update().clone();
    69	                app.handle_git_snapshot(snapshot);
    70	                needs_render = true;
    71	            }
    72	
    73	            // Branch 5: Discrete events (mpsc channel)
    74	            event = event_rx.recv() => {
    75	                match event {
    76	                    Some(ev) => {
    77	                        app.handle_event(ev);
    78	                        needs_render = true;
    79	                    }
    80	                    None => {
    81	                        // All senders dropped — shut down
    82	                        app.should_quit = true;
    83	                    }
    84	                }
    85	            }
    86	        }
    87	
    88	        // Adapter supervisor: detect crashed adapters
    89	        for handle in &adapter_handles {
    90	            if handle.is_finished() {
    91	                tracing::warn!("Adapter '{}' finished unexpectedly", handle.name());
    92	            }
    93	        }
    94	
    95	        // Conditional render
    96	        if needs_render {
    97	            terminal.draw(|frame| app.render(frame))?;
    98	            needs_render = false;
    99	        }
   100	
   101	        if app.should_quit {
   102	            break;
   103	        }
   104	    }
   105	
   106	    // Graceful shutdown
   107	    for handle in adapter_handles {
   108	        handle.shutdown().await;
   109	    }
   110	
   111	    Ok(())
   112	}

exec
/bin/zsh -lc "nl -ba src/app.rs | sed -n '620,980p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
   620	    #[test]
   621	    fn test_handle_event_hooks_started() {
   622	        let mut app = make_app(80, 24);
   623	        app.handle_event(AppEvent::HooksServerStarted { port: 9100 });
   624	        assert!(app.hooks_active);
   625	        assert_eq!(app.hooks_port, Some(9100));
   626	    }
   627	
   628	    #[test]
   629	    fn test_handle_event_hooks_failed() {
   630	        let mut app = make_app(80, 24);
   631	        app.hooks_active = true;
   632	        app.handle_event(AppEvent::HooksServerFailed {
   633	            reason: "bind error".to_string(),
   634	        });
   635	        assert!(!app.hooks_active);
   636	    }
   637	
   638	    #[test]
   639	    fn test_handle_flow_state() {
   640	        let mut app = make_app(80, 24);
   641	        let state = FlowState {
   642	            stage: "requirements-analysis".to_string(),
   643	            ..Default::default()
   644	        };
   645	        app.handle_flow_state(state);
   646	        assert_eq!(app.phase, "INCEPTION");
   647	        assert_eq!(app.stage, "requirements-analysis");
   648	    }
   649	
   650	    #[test]
   651	    fn test_on_tick_returns_false_when_no_change() {
   652	        let mut app = make_app(80, 24);
   653	        assert!(!app.on_tick());
   654	    }
   655	
   656	    #[test]
   657	    fn test_on_resize_revalidates_focus() {
   658	        let mut app = make_app(200, 50);
   659	        app.focus = FocusPane::WorkflowMap;
   660	        app.on_resize(80, 24);
   661	        // Focus should still be valid after resize
   662	        assert!(app.available_panels().contains(&app.focus));
   663	    }
   664	
   665	    #[test]
   666	    fn test_handle_event_broadcast() {
   667	        let mut app = make_app(80, 24);
   668	        app.agent_status.set_hooks_active(true);
   669	        // AgentStarted should reach agent_status panel
   670	        app.handle_event(AppEvent::AgentStarted {
   671	            agent_id: "a1".to_string(),
   672	            agent_type: "Explore".to_string(),
   673	        });
   674	        assert!(app.agent_status.has_running_agents());
   675	    }
   676	
   677	    #[test]
   678	    fn test_check_hooks_not_configured() {
   679	        let mut app = make_app(80, 24);
   680	        app.hooks_port = Some(9100);
   681	        // project_dir /tmp/test has no .claude/settings.json
   682	        app.check_hooks_config();
   683	        assert!(matches!(app.hook_setup, HookSetupState::NotConfigured { .. }));
   684	    }
   685	
   686	    // ── Step 4: Render tests ──
   687	
   688	    #[test]
   689	    fn render_compact_mode() {
   690	        let mut app = make_app(80, 24);
   691	        let terminal = render_with(80, 24, |frame, _area| {
   692	            app.render(frame);
   693	        });
   694	        let buf = terminal.backend().buffer();
   695	        assert!(buffer_contains_str(buf, "devflow-tui"));
   696	        assert!(buffer_contains_str(buf, "Workflow Map"));
   697	    }
   698	
   699	    #[test]
   700	    fn render_standard_mode() {
   701	        let mut app = make_app(120, 30);
   702	        let terminal = render_with(120, 30, |frame, _area| {
   703	            app.render(frame);
   704	        });
   705	        let buf = terminal.backend().buffer();
   706	        assert!(buffer_contains_str(buf, "devflow-tui"));
   707	        // All 4 panels should have their titles
   708	        assert!(buffer_contains_str(buf, "Workflow Map"));
   709	        assert!(buffer_contains_str(buf, "Git Status"));
   710	        assert!(buffer_contains_str(buf, "Agent Status"));
   711	        assert!(buffer_contains_str(buf, "Audit Log"));
   712	    }
   713	
   714	    #[test]
   715	    fn render_too_small() {
   716	        let mut app = make_app(60, 15);
   717	        let terminal = render_with(60, 15, |frame, _area| {
   718	            app.render(frame);
   719	        });
   720	        let buf = terminal.backend().buffer();
   721	        assert!(buffer_contains_str(buf, "터미널이 너무 작습니다"));
   722	    }
   723	
   724	    #[test]
   725	    fn render_help_overlay() {
   726	        let mut app = make_app(80, 24);
   727	        app.input_mode = InputMode::HelpOverlay;
   728	        let terminal = render_with(80, 24, |frame, _area| {
   729	            app.render(frame);
   730	        });
   731	        let buf = terminal.backend().buffer();
   732	        assert!(buffer_contains_str(buf, "Help"));
   733	        assert!(buffer_contains_str(buf, "Tab"));
   734	    }
   735	
   736	    #[test]
   737	    fn render_expanded_mode() {
   738	        let mut app = make_app(120, 30);
   739	        app.input_mode = InputMode::Expanded;
   740	        let terminal = render_with(120, 30, |frame, _area| {
   741	            app.render(frame);
   742	        });
   743	        let buf = terminal.backend().buffer();
   744	        // Should show focused panel title
   745	        assert!(buffer_contains_str(buf, "Workflow Map"));
   746	    }
   747	
   748	    #[test]
   749	    fn render_hooks_not_configured_banner() {
   750	        let mut app = make_app(120, 30);
   751	        app.hook_setup = HookSetupState::NotConfigured {
   752	            snippet: "{}".to_string(),
   753	        };
   754	        let terminal = render_with(120, 30, |frame, _area| {
   755	            app.render(frame);
   756	        });
   757	        let buf = terminal.backend().buffer();
   758	        assert!(buffer_contains_str(buf, "Hooks 미설정"));
   759	    }
   760	
   761	    #[test]
   762	    fn render_hooks_configured_no_banner() {
   763	        let mut app = make_app(120, 30);
   764	        app.hook_setup = HookSetupState::Configured;
   765	        let terminal = render_with(120, 30, |frame, _area| {
   766	            app.render(frame);
   767	        });
   768	        let buf = terminal.backend().buffer();
   769	        assert!(!buffer_contains_str(buf, "Hooks 미설정"));
   770	    }
   771	}

codex
I’ve finished the core app/event-loop pass and found several concrete hotspots; I’m now validating them against startup and adapter code to avoid false positives and rank severity correctly.
exec
/bin/zsh -lc "nl -ba src/main.rs | sed -n '1,320p'" in /Users/jay.ahn/projects/backend/devflow-tui
exec
/bin/zsh -lc "nl -ba src/adapter/handle.rs | sed -n '1,320p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::future::Future;
     2	use std::time::Duration;
     3	
     4	use tokio::task::JoinHandle;
     5	use tokio_util::sync::CancellationToken;
     6	
     7	use crate::error::Result;
     8	
     9	/// RAII handle for a long-running background adapter task.
    10	///
    11	/// On drop, cancels the token and aborts the task if still running.
    12	/// For graceful shutdown, call `shutdown()` explicitly before drop.
    13	pub struct AdapterHandle {
    14	    cancel: CancellationToken,
    15	    join: Option<JoinHandle<Result<()>>>,
    16	    name: &'static str,
    17	}
    18	
    19	impl Drop for AdapterHandle {
    20	    fn drop(&mut self) {
    21	        self.cancel.cancel();
    22	        if let Some(join) = self.join.take()
    23	            && !join.is_finished()
    24	        {
    25	            tracing::warn!("adapter '{}': dropped without shutdown, aborting task", self.name);
    26	            join.abort();
    27	        }
    28	    }
    29	}
    30	
    31	impl AdapterHandle {
    32	    /// Spawn a background adapter task.
    33	    ///
    34	    /// The closure receives a CancellationToken that it should poll for graceful shutdown.
    35	    pub fn spawn<F, Fut>(name: &'static str, f: F) -> Self
    36	    where
    37	        F: FnOnce(CancellationToken) -> Fut,
    38	        Fut: Future<Output = Result<()>> + Send + 'static,
    39	    {
    40	        let cancel = CancellationToken::new();
    41	        let token = cancel.clone();
    42	        let join = tokio::spawn(f(token));
    43	
    44	        Self {
    45	            cancel,
    46	            join: Some(join),
    47	            name,
    48	        }
    49	    }
    50	
    51	    /// Graceful shutdown: cancel the token and wait up to 3 seconds.
    52	    /// On timeout, aborts the task.
    53	    pub async fn shutdown(mut self) {
    54	        self.cancel.cancel();
    55	        if let Some(join) = self.join.take() {
    56	            let timeout = Duration::from_secs(3);
    57	            match tokio::time::timeout(timeout, join).await {
    58	                Ok(Ok(Ok(()))) => {
    59	                    tracing::info!("adapter '{}' shut down cleanly", self.name);
    60	                }
    61	                Ok(Ok(Err(e))) => {
    62	                    tracing::warn!("adapter '{}' exited with error: {e}", self.name);
    63	                }
    64	                Ok(Err(join_err)) => {
    65	                    tracing::error!("adapter '{}' panicked: {join_err}", self.name);
    66	                }
    67	                Err(_) => {
    68	                    tracing::warn!(
    69	                        "adapter '{}' shutdown timed out after {timeout:?}, aborting",
    70	                        self.name
    71	                    );
    72	                    // Abort is handled by Drop since join is now None
    73	                }
    74	            }
    75	        }
    76	    }
    77	
    78	    /// Check if the background task has finished (panic or completion).
    79	    pub fn is_finished(&self) -> bool {
    80	        self.join.as_ref().is_some_and(|j| j.is_finished())
    81	    }
    82	
    83	    /// Get the adapter name.
    84	    pub fn name(&self) -> &'static str {
    85	        self.name
    86	    }
    87	}
    88	
    89	#[cfg(test)]
    90	mod tests {
    91	    use super::*;
    92	
    93	    #[tokio::test]
    94	    async fn test_adapter_handle_spawn_and_shutdown() {
    95	        let handle = AdapterHandle::spawn("test", |cancel| async move {
    96	            cancel.cancelled().await;
    97	            Ok(())
    98	        });
    99	
   100	        assert!(!handle.is_finished());
   101	        handle.shutdown().await;
   102	    }
   103	
   104	    #[tokio::test]
   105	    async fn test_adapter_handle_immediate_completion() {
   106	        let handle = AdapterHandle::spawn("test-fast", |_cancel| async move { Ok(()) });
   107	
   108	        tokio::time::sleep(Duration::from_millis(10)).await;
   109	        assert!(handle.is_finished());
   110	        handle.shutdown().await;
   111	    }
   112	
   113	    #[tokio::test]
   114	    async fn test_adapter_handle_error_completion() {
   115	        let handle = AdapterHandle::spawn("test-err", |_cancel| async move {
   116	            Err(crate::error::AppError::Terminal("test error".to_string()))
   117	        });
   118	
   119	        tokio::time::sleep(Duration::from_millis(10)).await;
   120	        assert!(handle.is_finished());
   121	        handle.shutdown().await;
   122	    }
   123	
   124	    #[tokio::test]
   125	    async fn test_adapter_handle_drop_aborts() {
   126	        let handle = AdapterHandle::spawn("test-drop", |cancel| async move {
   127	            cancel.cancelled().await;
   128	            Ok(())
   129	        });
   130	
   131	        // Drop without shutdown — should cancel + abort, not leak
   132	        drop(handle);
   133	        // If this test completes without hanging, Drop works correctly
   134	    }
   135	}

 succeeded in 0ms:
     1	use std::io::{self, stdout};
     2	
     3	use crossterm::{
     4	    execute,
     5	    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
     6	};
     7	use ratatui::{Terminal, backend::CrosstermBackend};
     8	use tokio::sync::{mpsc, watch};
     9	use tracing_appender::non_blocking::WorkerGuard;
    10	use tracing_appender::rolling;
    11	use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
    12	
    13	use devflow_tui::adapter::file_watcher;
    14	use devflow_tui::adapter::git_poller;
    15	use devflow_tui::adapter::handle::AdapterHandle;
    16	use devflow_tui::adapter::hooks_server;
    17	use devflow_tui::app::App;
    18	use devflow_tui::config::AppConfig;
    19	use devflow_tui::event::AppEvent;
    20	use devflow_tui::parser::models::{FlowState, GitSnapshot};
    21	use devflow_tui::service::token;
    22	
    23	/// RAII guard that restores terminal state on drop.
    24	struct TerminalGuard;
    25	
    26	impl TerminalGuard {
    27	    fn new() -> devflow_tui::error::Result<(Self, Terminal<CrosstermBackend<io::Stdout>>)> {
    28	        enable_raw_mode()?;
    29	        let mut stdout = stdout();
    30	        execute!(stdout, EnterAlternateScreen)?;
    31	        let backend = CrosstermBackend::new(stdout);
    32	        let terminal = Terminal::new(backend)?;
    33	        Ok((Self, terminal))
    34	    }
    35	}
    36	
    37	impl Drop for TerminalGuard {
    38	    fn drop(&mut self) {
    39	        let _ = disable_raw_mode();
    40	        let _ = execute!(stdout(), LeaveAlternateScreen);
    41	    }
    42	}
    43	
    44	#[tokio::main]
    45	async fn main() -> devflow_tui::error::Result<()> {
    46	    // Parse config
    47	    let args: Vec<String> = std::env::args().collect();
    48	    let config = AppConfig::from_args(&args)?;
    49	
    50	    // Initialize tracing — keep guard alive for log flushing on exit
    51	    let _log_guard = init_tracing(&config);
    52	
    53	    // Install panic hook to restore terminal
    54	    let original_hook = std::panic::take_hook();
    55	    std::panic::set_hook(Box::new(move |panic_info| {
    56	        let _ = disable_raw_mode();
    57	        let _ = execute!(stdout(), LeaveAlternateScreen);
    58	        original_hook(panic_info);
    59	    }));
    60	
    61	    // Setup terminal with RAII guard — cleanup guaranteed even on early error
    62	    let (_guard, mut terminal) = TerminalGuard::new()?;
    63	    tracing::info!(
    64	        "devflow-tui started (port: {}, demo: {})",
    65	        config.port,
    66	        config.demo
    67	    );
    68	
    69	    // Generate/load token
    70	    let tok = token::get_or_create_token(&config.project_dir, config.regenerate_token)?;
    71	
    72	    // Create channels
    73	    let (event_tx, event_rx) = mpsc::channel::<AppEvent>(256);
    74	    let (flow_state_tx, flow_state_rx) = watch::channel(FlowState::default());
    75	    let (git_snapshot_tx, git_snapshot_rx) = watch::channel(GitSnapshot::default());
    76	
    77	    // Get terminal size for App init
    78	    let size = terminal.size()?;
    79	
    80	    // Create App
    81	    let mut app = App::new(
    82	        size.width,
    83	        size.height,
    84	        event_tx.clone(),
    85	        tok.clone(),
    86	        config.project_dir.clone(),
    87	    );
    88	
    89	    if config.demo {
    90	        devflow_tui::demo::populate_demo_data(&mut app);
    91	    }
    92	
    93	    // Spawn adapters
    94	    let mut adapters = Vec::new();
    95	
    96	    if !config.demo {
    97	        let project_dir = config.project_dir.clone();
    98	        let fw_flow_tx = flow_state_tx;
    99	        let fw_event_tx = event_tx.clone();
   100	        adapters.push(AdapterHandle::spawn("file_watcher", |cancel| async move {
   101	            file_watcher::run(cancel, project_dir, fw_flow_tx, fw_event_tx).await
   102	        }));
   103	
   104	        let project_dir = config.project_dir.clone();
   105	        let gp_git_tx = git_snapshot_tx;
   106	        let gp_event_tx = event_tx.clone();
   107	        adapters.push(AdapterHandle::spawn("git_poller", |cancel| async move {
   108	            git_poller::run(cancel, project_dir, gp_git_tx, gp_event_tx).await
   109	        }));
   110	
   111	        let hs_event_tx = event_tx.clone();
   112	        let hs_token = tok;
   113	        let hs_port = config.port;
   114	        adapters.push(AdapterHandle::spawn("hooks_server", |cancel| async move {
   115	            hooks_server::run(cancel, hs_port, hs_token, hs_event_tx).await
   116	        }));
   117	    }
   118	
   119	    // Run event loop
   120	    devflow_tui::event_loop::run_event_loop(
   121	        &mut terminal,
   122	        &mut app,
   123	        event_rx,
   124	        flow_state_rx,
   125	        git_snapshot_rx,
   126	        adapters,
   127	    )
   128	    .await?;
   129	
   130	    tracing::info!("devflow-tui exited cleanly");
   131	    // _guard drops here → terminal restored
   132	    // _log_guard drops here → logs flushed
   133	
   134	    Ok(())
   135	}
   136	
   137	fn init_tracing(config: &AppConfig) -> WorkerGuard {
   138	    let log_dir = dirs::cache_dir()
   139	        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
   140	        .join("devflow-tui");
   141	
   142	    let file_appender = rolling::daily(&log_dir, "devflow-tui.log");
   143	    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
   144	
   145	    let filter = EnvFilter::try_from_env("DEVFLOW_TUI_LOG")
   146	        .unwrap_or_else(|_| EnvFilter::new(&config.log_level));
   147	
   148	    tracing_subscriber::registry()
   149	        .with(filter)
   150	        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
   151	        .init();
   152	
   153	    guard
   154	}

exec
/bin/zsh -lc "nl -ba src/error.rs | sed -n '1,240p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::path::PathBuf;
     2	
     3	pub type Result<T> = std::result::Result<T, AppError>;
     4	
     5	#[derive(Debug, thiserror::Error)]
     6	pub enum AppError {
     7	    // Infrastructure
     8	    #[error("I/O error: {0}")]
     9	    Io(#[from] std::io::Error),
    10	
    11	    #[error("Notify error: {0}")]
    12	    Notify(#[from] notify::Error),
    13	
    14	    #[error("Terminal error: {0}")]
    15	    Terminal(String),
    16	
    17	    // Parser
    18	    #[error("Failed to parse devflow-state at {file}: {detail}")]
    19	    ParseFlowState { file: PathBuf, detail: String },
    20	
    21	    #[error("Failed to parse session-summary at {file}: {detail}")]
    22	    ParseSessionSummary { file: PathBuf, detail: String },
    23	
    24	    #[error("Failed to parse audit log at {file}: {detail}")]
    25	    ParseAuditLog { file: PathBuf, detail: String },
    26	
    27	    // Git
    28	    #[error("Git command failed: {command} — {stderr}")]
    29	    GitCommand { command: String, stderr: String },
    30	
    31	    #[error("Git command timed out: {command}")]
    32	    GitTimeout { command: String },
    33	
    34	    // Hooks
    35	    #[error("Failed to bind hooks server on port {port}: {reason}")]
    36	    HooksServerBind { port: u16, reason: String },
    37	
    38	    #[error("Hooks token mismatch")]
    39	    HooksTokenMismatch,
    40	
    41	    #[error("Invalid hooks payload: {detail}")]
    42	    HooksPayloadInvalid { detail: String },
    43	
    44	    // Service
    45	    #[error("Clipboard unavailable")]
    46	    ClipboardUnavailable,
    47	
    48	    #[error("Token generation failed: {0}")]
    49	    TokenGeneration(String),
    50	
    51	    #[error("Failed to read config at {path}: {detail}")]
    52	    ConfigRead { path: PathBuf, detail: String },
    53	}
    54	
    55	#[cfg(test)]
    56	mod tests {
    57	    use super::*;
    58	
    59	    #[test]
    60	    fn test_error_display_io() {
    61	        let err = AppError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "missing"));
    62	        assert!(err.to_string().contains("I/O error"));
    63	    }
    64	
    65	    #[test]
    66	    fn test_error_display_parse_flow_state() {
    67	        let err = AppError::ParseFlowState {
    68	            file: PathBuf::from("devflow-state.md"),
    69	            detail: "unexpected format".to_string(),
    70	        };
    71	        assert!(err.to_string().contains("devflow-state"));
    72	        assert!(err.to_string().contains("unexpected format"));
    73	    }
    74	
    75	    #[test]
    76	    fn test_error_display_git_command() {
    77	        let err = AppError::GitCommand {
    78	            command: "git status".to_string(),
    79	            stderr: "not a git repo".to_string(),
    80	        };
    81	        assert!(err.to_string().contains("git status"));
    82	    }
    83	
    84	    #[test]
    85	    fn test_error_display_hooks_server_bind() {
    86	        let err = AppError::HooksServerBind {
    87	            port: 9100,
    88	            reason: "address in use".to_string(),
    89	        };
    90	        assert!(err.to_string().contains("9100"));
    91	    }
    92	
    93	    #[test]
    94	    fn test_error_display_clipboard() {
    95	        let err = AppError::ClipboardUnavailable;
    96	        assert_eq!(err.to_string(), "Clipboard unavailable");
    97	    }
    98	
    99	    #[test]
   100	    fn test_error_display_config_read() {
   101	        let err = AppError::ConfigRead {
   102	            path: PathBuf::from("/etc/config"),
   103	            detail: "permission denied".to_string(),
   104	        };
   105	        assert!(err.to_string().contains("/etc/config"));
   106	    }
   107	}

exec
/bin/zsh -lc "nl -ba src/action.rs | sed -n '1,260p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	#[derive(Debug, Clone, PartialEq, Eq)]
     2	pub enum Direction {
     3	    Up,
     4	    Down,
     5	    Left,
     6	    Right,
     7	}
     8	
     9	#[derive(Debug, Clone)]
    10	pub enum Action {
    11	    // Navigation
    12	    FocusNextPanel,
    13	    FocusPrevPanel,
    14	    FocusDirection(Direction),
    15	    ExpandPanel,
    16	    CollapsePanel,
    17	    OpenArtifactModal,
    18	
    19	    // Panel interaction
    20	    ScrollUp,
    21	    ScrollDown,
    22	    Select,
    23	
    24	    // Async (delegated to CommandRunner)
    25	    Refresh,
    26	    CopyToClipboard(String),
    27	
    28	    // System
    29	    Quit,
    30	}
    31	
    32	impl Action {
    33	    pub fn name(&self) -> &'static str {
    34	        match self {
    35	            Self::FocusNextPanel => "focus_next_panel",
    36	            Self::FocusPrevPanel => "focus_prev_panel",
    37	            Self::FocusDirection(_) => "focus_direction",
    38	            Self::ExpandPanel => "expand_panel",
    39	            Self::CollapsePanel => "collapse_panel",
    40	            Self::OpenArtifactModal => "open_artifact_modal",
    41	            Self::ScrollUp => "scroll_up",
    42	            Self::ScrollDown => "scroll_down",
    43	            Self::Select => "select",
    44	            Self::Refresh => "refresh",
    45	            Self::CopyToClipboard(_) => "copy_to_clipboard",
    46	            Self::Quit => "quit",
    47	        }
    48	    }
    49	
    50	    pub fn is_async(&self) -> bool {
    51	        matches!(self, Self::Refresh | Self::CopyToClipboard(_))
    52	    }
    53	}
    54	
    55	#[cfg(test)]
    56	mod tests {
    57	    use super::*;
    58	
    59	    #[test]
    60	    fn test_action_name() {
    61	        assert_eq!(Action::FocusNextPanel.name(), "focus_next_panel");
    62	        assert_eq!(Action::Quit.name(), "quit");
    63	        assert_eq!(Action::CopyToClipboard("x".into()).name(), "copy_to_clipboard");
    64	        assert_eq!(Action::FocusDirection(Direction::Up).name(), "focus_direction");
    65	    }
    66	
    67	    #[test]
    68	    fn test_action_is_async() {
    69	        assert!(Action::Refresh.is_async());
    70	        assert!(Action::CopyToClipboard("x".into()).is_async());
    71	        assert!(!Action::Quit.is_async());
    72	        assert!(!Action::ScrollUp.is_async());
    73	    }
    74	}

exec
/bin/zsh -lc "nl -ba src/command.rs | sed -n '1,340p'" in /Users/jay.ahn/projects/backend/devflow-tui
exec
/bin/zsh -lc "nl -ba src/event.rs | sed -n '1,320p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use tokio::sync::mpsc;
     2	
     3	use crate::action::Action;
     4	use crate::event::AppEvent;
     5	
     6	/// Lightweight async command runner for side-effect actions.
     7	///
     8	/// Handles: clipboard copy, refresh triggers.
     9	/// Sends completion/failure events back via channel.
    10	#[derive(Clone)]
    11	pub struct CommandRunner {
    12	    event_tx: mpsc::Sender<AppEvent>,
    13	}
    14	
    15	impl CommandRunner {
    16	    pub fn new(event_tx: mpsc::Sender<AppEvent>) -> Self {
    17	        Self { event_tx }
    18	    }
    19	
    20	    /// Execute an async action. Spawns a tokio task.
    21	    pub fn execute(&self, action: Action) {
    22	        let tx = self.event_tx.clone();
    23	        let action_name = action.name().to_string();
    24	
    25	        tokio::spawn(async move {
    26	            let result = match action {
    27	                Action::CopyToClipboard(text) => copy_to_clipboard(&text).await,
    28	                Action::Refresh => {
    29	                    // Refresh is handled by re-triggering file watcher parse
    30	                    // For now, just succeed
    31	                    Ok(())
    32	                }
    33	                _ => return, // Non-async actions don't come here
    34	            };
    35	
    36	            match result {
    37	                Ok(()) => {
    38	                    let _ = tx
    39	                        .send(AppEvent::CommandCompleted {
    40	                            action_name,
    41	                        })
    42	                        .await;
    43	                }
    44	                Err(e) => {
    45	                    let _ = tx
    46	                        .send(AppEvent::CommandFailed {
    47	                            action_name,
    48	                            error: e,
    49	                        })
    50	                        .await;
    51	                }
    52	            }
    53	        });
    54	    }
    55	}
    56	
    57	/// Copy text to clipboard using pbcopy (macOS).
    58	/// Falls back to writing to /tmp/devflow-tui-clipboard.txt.
    59	/// Clipboard timeout
    60	const CLIPBOARD_TIMEOUT_SECS: u64 = 5;
    61	
    62	async fn copy_to_clipboard(text: &str) -> std::result::Result<(), String> {
    63	    // Try pbcopy first (macOS) with timeout
    64	    let result = tokio::process::Command::new("pbcopy")
    65	        .stdin(std::process::Stdio::piped())
    66	        .kill_on_drop(true)
    67	        .spawn();
    68	
    69	    match result {
    70	        Ok(mut child) => {
    71	            if let Some(ref mut stdin) = child.stdin {
    72	                use tokio::io::AsyncWriteExt;
    73	                if let Err(e) = stdin.write_all(text.as_bytes()).await {
    74	                    return Err(format!("pbcopy write failed: {e}"));
    75	                }
    76	            }
    77	            drop(child.stdin.take());
    78	
    79	            // Wait with timeout to avoid hanging on stuck pbcopy
    80	            match tokio::time::timeout(
    81	                std::time::Duration::from_secs(CLIPBOARD_TIMEOUT_SECS),
    82	                child.wait(),
    83	            )
    84	            .await
    85	            {
    86	                Ok(Ok(status)) if status.success() => Ok(()),
    87	                Ok(Ok(status)) => Err(format!("pbcopy exited with: {status}")),
    88	                Ok(Err(e)) => Err(format!("pbcopy wait failed: {e}")),
    89	                Err(_) => Err("pbcopy timed out".to_string()),
    90	            }
    91	        }
    92	        Err(_) => {
    93	            // Fallback: write to secure temp file (not predictable path)
    94	            let dir = std::env::temp_dir();
    95	            let file_path = dir.join(format!(
    96	                "devflow-tui-clip-{}.txt",
    97	                std::process::id()
    98	            ));
    99	            tokio::fs::write(&file_path, text)
   100	                .await
   101	                .map_err(|e| format!("clipboard fallback failed: {e}"))?;
   102	            tracing::info!(
   103	                "clipboard unavailable, saved to {}",
   104	                file_path.display()
   105	            );
   106	            Ok(())
   107	        }
   108	    }
   109	}
   110	
   111	#[cfg(test)]
   112	mod tests {
   113	    use super::*;
   114	
   115	    #[tokio::test]
   116	    async fn test_command_runner_non_async_ignored() {
   117	        let (tx, mut rx) = mpsc::channel(16);
   118	        let runner = CommandRunner::new(tx);
   119	
   120	        // Non-async action should be ignored (no event sent)
   121	        runner.execute(Action::ScrollUp);
   122	        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
   123	
   124	        assert!(rx.try_recv().is_err());
   125	    }
   126	
   127	    #[tokio::test]
   128	    async fn test_command_runner_refresh() {
   129	        let (tx, mut rx) = mpsc::channel(16);
   130	        let runner = CommandRunner::new(tx);
   131	
   132	        runner.execute(Action::Refresh);
   133	        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
   134	
   135	        let event = rx.try_recv().unwrap();
   136	        match event {
   137	            AppEvent::CommandCompleted { action_name } => {
   138	                assert_eq!(action_name, "refresh");
   139	            }
   140	            other => panic!("unexpected event: {other:?}"),
   141	        }
   142	    }
   143	}

 succeeded in 0ms:
     1	use std::path::PathBuf;
     2	
     3	use crate::parser::models::{ArtifactFile, AuditEntry, FlowState, GitSnapshot, SessionSummary};
     4	
     5	#[derive(Debug)]
     6	pub enum AppEvent {
     7	    // File watcher
     8	    FlowStateChanged(FlowState),
     9	    SessionSummaryChanged(SessionSummary),
    10	    AuditLogAppended(Vec<AuditEntry>),
    11	    ArtifactListChanged(Vec<ArtifactFile>),
    12	
    13	    // Git polling
    14	    GitStatusUpdated(GitSnapshot),
    15	
    16	    // Hooks
    17	    AgentStarted {
    18	        agent_id: String,
    19	        agent_type: String,
    20	    },
    21	    AgentStopped {
    22	        agent_id: String,
    23	    },
    24	    ToolUseStarted {
    25	        tool_name: String,
    26	    },
    27	    ToolUseCompleted {
    28	        tool_name: String,
    29	    },
    30	    TurnCompleted {
    31	        last_message: String,
    32	    },
    33	
    34	    // System
    35	    HooksServerStarted {
    36	        port: u16,
    37	    },
    38	    HooksServerFailed {
    39	        reason: String,
    40	    },
    41	    FileWatcherError {
    42	        path: PathBuf,
    43	        error: String,
    44	    },
    45	    GitPollError {
    46	        error: String,
    47	    },
    48	    ParseError {
    49	        file: String,
    50	        error: String,
    51	    },
    52	    CommandCompleted {
    53	        action_name: String,
    54	    },
    55	    CommandFailed {
    56	        action_name: String,
    57	        error: String,
    58	    },
    59	}
    60	
    61	#[cfg(test)]
    62	mod tests {
    63	    use super::*;
    64	
    65	    #[test]
    66	    fn test_event_debug() {
    67	        let event = AppEvent::AgentStarted {
    68	            agent_id: "abc".to_string(),
    69	            agent_type: "Explore".to_string(),
    70	        };
    71	        let debug = format!("{event:?}");
    72	        assert!(debug.contains("AgentStarted"));
    73	        assert!(debug.contains("abc"));
    74	    }
    75	
    76	    #[test]
    77	    fn test_event_hooks_failed_debug() {
    78	        let event = AppEvent::HooksServerFailed {
    79	            reason: "port in use".to_string(),
    80	        };
    81	        let debug = format!("{event:?}");
    82	        assert!(debug.contains("HooksServerFailed"));
    83	    }
    84	}

exec
/bin/zsh -lc "nl -ba src/lib.rs | sed -n '1,220p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	pub mod action;
     2	pub mod adapter;
     3	pub mod app;
     4	pub mod command;
     5	pub mod component;
     6	pub mod config;
     7	pub mod demo;
     8	pub mod error;
     9	pub mod event;
    10	pub mod event_loop;
    11	pub mod panel;
    12	pub mod parser;
    13	pub mod port;
    14	pub mod service;
    15	pub mod ui;
    16	
    17	#[cfg(test)]
    18	pub(crate) mod test_helpers;

exec
/bin/zsh -lc "nl -ba src/component.rs | sed -n '1,220p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use crossterm::event::KeyEvent;
     2	use ratatui::{Frame, layout::Rect};
     3	
     4	use crate::action::Action;
     5	use crate::event::AppEvent;
     6	
     7	/// UI 패널이 구현하는 공통 trait.
     8	///
     9	/// nexttui Component trait 기반. `render`에 `focused` 매개변수 추가.
    10	pub trait Component {
    11	    /// 키 입력 처리. Action을 반환하면 App이 처리.
    12	    fn handle_key(&mut self, key: KeyEvent) -> Option<Action>;
    13	
    14	    /// 백그라운드 이벤트 처리. 패널 내부 상태 업데이트.
    15	    fn handle_event(&mut self, event: &AppEvent);
    16	
    17	    /// 패널 렌더링. focused=true면 Cyan 테두리.
    18	    fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool);
    19	}

exec
/bin/zsh -lc "nl -ba src/ui/layout.rs | sed -n '1,420p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use ratatui::layout::{Constraint, Layout, Rect};
     2	
     3	/// Layout mode determined by terminal size.
     4	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
     5	pub enum LayoutMode {
     6	    TooSmall,              // < 80x24
     7	    Compact,               // 80x24 ~ 119x29
     8	    Standard,              // 120x30 ~ 199x49
     9	    Wide,                  // 200x50+
    10	}
    11	
    12	/// Computed layout areas for each UI section.
    13	#[derive(Debug, Clone)]
    14	pub struct LayoutAreas {
    15	    pub header: Rect,
    16	    pub body: Rect,
    17	    pub status_bar: Rect,
    18	}
    19	
    20	/// Panel areas within the body, depending on layout mode.
    21	#[derive(Debug, Clone)]
    22	pub enum PanelAreas {
    23	    TooSmall {
    24	        message: Rect,
    25	    },
    26	    Compact {
    27	        /// Single panel fills the body
    28	        panel: Rect,
    29	    },
    30	    Standard {
    31	        workflow_map: Rect,
    32	        git_status: Rect,
    33	        agent_status: Rect,
    34	        audit_log: Rect,
    35	    },
    36	    Wide {
    37	        workflow_map: Rect,
    38	        git_status: Rect,
    39	        artifacts: Rect,
    40	        agent_status: Rect,
    41	        audit_log: Rect,
    42	        gate_alert: Rect,
    43	    },
    44	}
    45	
    46	/// Manages layout calculations based on terminal size.
    47	pub struct LayoutManager {
    48	    width: u16,
    49	    height: u16,
    50	}
    51	
    52	impl LayoutManager {
    53	    pub fn new(width: u16, height: u16) -> Self {
    54	        Self { width, height }
    55	    }
    56	
    57	    pub fn on_resize(&mut self, width: u16, height: u16) {
    58	        self.width = width;
    59	        self.height = height;
    60	    }
    61	
    62	    pub fn mode(&self) -> LayoutMode {
    63	        if self.width < 80 || self.height < 24 {
    64	            LayoutMode::TooSmall
    65	        } else if self.width < 120 || self.height < 30 {
    66	            LayoutMode::Compact
    67	        } else if self.width < 200 || self.height < 50 {
    68	            LayoutMode::Standard
    69	        } else {
    70	            LayoutMode::Wide
    71	        }
    72	    }
    73	
    74	    /// Calculate the top-level layout areas (header, body, status bar).
    75	    pub fn areas(&self, frame_size: Rect) -> LayoutAreas {
    76	        let [header, body, status_bar] = Layout::vertical([
    77	            Constraint::Length(1),
    78	            Constraint::Fill(1),
    79	            Constraint::Length(1),
    80	        ])
    81	        .areas(frame_size);
    82	
    83	        LayoutAreas {
    84	            header,
    85	            body,
    86	            status_bar,
    87	        }
    88	    }
    89	
    90	    /// Calculate panel areas within the body based on current layout mode.
    91	    pub fn panel_areas(&self, body: Rect) -> PanelAreas {
    92	        match self.mode() {
    93	            LayoutMode::TooSmall => PanelAreas::TooSmall { message: body },
    94	            LayoutMode::Compact => PanelAreas::Compact { panel: body },
    95	            LayoutMode::Standard => self.standard_panels(body),
    96	            LayoutMode::Wide => self.wide_panels(body),
    97	        }
    98	    }
    99	
   100	    fn standard_panels(&self, body: Rect) -> PanelAreas {
   101	        // Left(40%) | Right(fill)
   102	        let [left, right] = Layout::horizontal([
   103	            Constraint::Percentage(40),
   104	            Constraint::Fill(1),
   105	        ])
   106	        .areas(body);
   107	
   108	        // Right: Top(50%) | Bottom(fill)
   109	        let [right_top, right_bottom] = Layout::vertical([
   110	            Constraint::Percentage(50),
   111	            Constraint::Fill(1),
   112	        ])
   113	        .areas(right);
   114	
   115	        // Right bottom: Agent(40%) | Audit(fill)
   116	        let [agent, audit] = Layout::horizontal([
   117	            Constraint::Percentage(40),
   118	            Constraint::Fill(1),
   119	        ])
   120	        .areas(right_bottom);
   121	
   122	        PanelAreas::Standard {
   123	            workflow_map: left,
   124	            git_status: right_top,
   125	            agent_status: agent,
   126	            audit_log: audit,
   127	        }
   128	    }
   129	
   130	    fn wide_panels(&self, body: Rect) -> PanelAreas {
   131	        // Top row | Bottom row
   132	        let [top_row, bottom_row] = Layout::vertical([
   133	            Constraint::Percentage(50),
   134	            Constraint::Fill(1),
   135	        ])
   136	        .areas(body);
   137	
   138	        // Top: 3 columns
   139	        let [wf, git, art] = Layout::horizontal([
   140	            Constraint::Percentage(33),
   141	            Constraint::Percentage(34),
   142	            Constraint::Fill(1),
   143	        ])
   144	        .areas(top_row);
   145	
   146	        // Bottom: 3 columns
   147	        let [agent, audit, gate] = Layout::horizontal([
   148	            Constraint::Percentage(33),
   149	            Constraint::Percentage(34),
   150	            Constraint::Fill(1),
   151	        ])
   152	        .areas(bottom_row);
   153	
   154	        PanelAreas::Wide {
   155	            workflow_map: wf,
   156	            git_status: git,
   157	            artifacts: art,
   158	            agent_status: agent,
   159	            audit_log: audit,
   160	            gate_alert: gate,
   161	        }
   162	    }
   163	}
   164	
   165	/// Centered rectangle for modals/overlays.
   166	pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
   167	    let px = percent_x.min(100);
   168	    let py = percent_y.min(100);
   169	
   170	    let [_, center_v, _] = Layout::vertical([
   171	        Constraint::Percentage((100 - py) / 2),
   172	        Constraint::Percentage(py),
   173	        Constraint::Percentage((100 - py) / 2),
   174	    ])
   175	    .areas(area);
   176	
   177	    let [_, center, _] = Layout::horizontal([
   178	        Constraint::Percentage((100 - px) / 2),
   179	        Constraint::Percentage(px),
   180	        Constraint::Percentage((100 - px) / 2),
   181	    ])
   182	    .areas(center_v);
   183	
   184	    center
   185	}
   186	
   187	#[cfg(test)]
   188	mod tests {
   189	    use super::*;
   190	
   191	    #[test]
   192	    fn test_layout_mode_too_small() {
   193	        let lm = LayoutManager::new(79, 23);
   194	        assert_eq!(lm.mode(), LayoutMode::TooSmall);
   195	    }
   196	
   197	    #[test]
   198	    fn test_layout_mode_compact() {
   199	        let lm = LayoutManager::new(100, 25);
   200	        assert_eq!(lm.mode(), LayoutMode::Compact);
   201	    }
   202	
   203	    #[test]
   204	    fn test_layout_mode_standard() {
   205	        let lm = LayoutManager::new(150, 40);
   206	        assert_eq!(lm.mode(), LayoutMode::Standard);
   207	    }
   208	
   209	    #[test]
   210	    fn test_layout_mode_wide() {
   211	        let lm = LayoutManager::new(220, 60);
   212	        assert_eq!(lm.mode(), LayoutMode::Wide);
   213	    }
   214	
   215	    #[test]
   216	    fn test_areas_has_three_sections() {
   217	        let lm = LayoutManager::new(120, 30);
   218	        let areas = lm.areas(Rect::new(0, 0, 120, 30));
   219	        assert_eq!(areas.header.height, 1);
   220	        assert_eq!(areas.status_bar.height, 1);
   221	        assert!(areas.body.height > 0);
   222	    }
   223	
   224	    #[test]
   225	    fn test_standard_panels() {
   226	        let lm = LayoutManager::new(120, 30);
   227	        let areas = lm.areas(Rect::new(0, 0, 120, 30));
   228	        let panels = lm.panel_areas(areas.body);
   229	        assert!(matches!(panels, PanelAreas::Standard { .. }));
   230	    }
   231	
   232	    #[test]
   233	    fn test_wide_panels() {
   234	        let lm = LayoutManager::new(200, 50);
   235	        let areas = lm.areas(Rect::new(0, 0, 200, 50));
   236	        let panels = lm.panel_areas(areas.body);
   237	        assert!(matches!(panels, PanelAreas::Wide { .. }));
   238	    }
   239	
   240	    #[test]
   241	    fn test_centered_rect() {
   242	        let area = Rect::new(0, 0, 100, 50);
   243	        let center = centered_rect(80, 60, area);
   244	        assert!(center.x > 0);
   245	        assert!(center.y > 0);
   246	        assert!(center.width < 100);
   247	        assert!(center.height < 50);
   248	    }
   249	
   250	    #[test]
   251	    fn test_resize() {
   252	        let mut lm = LayoutManager::new(80, 24);
   253	        assert_eq!(lm.mode(), LayoutMode::Compact);
   254	        lm.on_resize(200, 50);
   255	        assert_eq!(lm.mode(), LayoutMode::Wide);
   256	    }
   257	
   258	    // ── Render tests ──
   259	
   260	    use crate::test_helpers::render_with;
   261	
   262	    #[test]
   263	    fn render_areas_non_overlapping() {
   264	        let terminal = render_with(120, 30, |frame, area| {
   265	            let lm = LayoutManager::new(area.width, area.height);
   266	            let areas = lm.areas(area);
   267	
   268	            // Header and status_bar should not overlap body
   269	            assert!(areas.header.y + areas.header.height <= areas.body.y);
   270	            assert!(areas.body.y + areas.body.height <= areas.status_bar.y);
   271	
   272	            // Render something to confirm no panic
   273	            let block = ratatui::widgets::Block::bordered();
   274	            frame.render_widget(block, areas.body);
   275	        });
   276	        let _buf = terminal.backend().buffer();
   277	    }
   278	
   279	    #[test]
   280	    fn render_centered_rect_position() {
   281	        let terminal = render_with(100, 50, |frame, area| {
   282	            let center = centered_rect(60, 60, area);
   283	            // Center should be within the area
   284	            assert!(center.x >= area.x);
   285	            assert!(center.y >= area.y);
   286	            assert!(center.x + center.width <= area.x + area.width);
   287	            assert!(center.y + center.height <= area.y + area.height);
   288	
   289	            let block = ratatui::widgets::Block::bordered()
   290	                .title("Modal");
   291	            frame.render_widget(ratatui::widgets::Clear, center);
   292	            frame.render_widget(block, center);
   293	        });
   294	        let buf = terminal.backend().buffer();
   295	        assert!(crate::test_helpers::buffer_contains_str(buf, "Modal"));
   296	    }
   297	
   298	    #[test]
   299	    fn render_too_small_mode_uses_full_body() {
   300	        let lm = LayoutManager::new(60, 20);
   301	        let areas = lm.areas(Rect::new(0, 0, 60, 20));
   302	        let panels = lm.panel_areas(areas.body);
   303	        match panels {
   304	            PanelAreas::TooSmall { message } => {
   305	                assert_eq!(message, areas.body);
   306	            }
   307	            _ => panic!("Expected TooSmall layout"),
   308	        }
   309	    }
   310	}

codex
I’m now scanning panel implementations and adapters for duplicated logic, hidden failure paths, and trait-shape issues that affect maintainability.
exec
/bin/zsh -lc "nl -ba src/panel/workflow_map.rs | sed -n '1,460p'" in /Users/jay.ahn/projects/backend/devflow-tui
exec
/bin/zsh -lc "nl -ba src/panel/agent_status.rs | sed -n '1,420p'" in /Users/jay.ahn/projects/backend/devflow-tui
exec
/bin/zsh -lc "nl -ba src/panel/git_status.rs | sed -n '1,380p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use crossterm::event::{KeyCode, KeyEvent};
     2	use ratatui::layout::{Constraint, Layout, Rect};
     3	use ratatui::style::Stylize;
     4	use ratatui::text::{Line, Span};
     5	use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph};
     6	use ratatui::Frame;
     7	
     8	use crate::action::Action;
     9	use crate::component::Component;
    10	use crate::event::AppEvent;
    11	use crate::parser::models::{GitChangeStatus, GitSnapshot};
    12	use crate::ui::theme::{self, Icons, Theme};
    13	
    14	pub struct GitStatusPanel {
    15	    snapshot: GitSnapshot,
    16	    changes_state: ListState,
    17	}
    18	
    19	impl Default for GitStatusPanel {
    20	    fn default() -> Self {
    21	        Self::new()
    22	    }
    23	}
    24	
    25	impl GitStatusPanel {
    26	    pub fn new() -> Self {
    27	        Self {
    28	            snapshot: GitSnapshot::default(),
    29	            changes_state: ListState::default(),
    30	        }
    31	    }
    32	
    33	    pub fn set_snapshot(&mut self, snapshot: GitSnapshot) {
    34	        self.snapshot = snapshot;
    35	        self.clamp_selection();
    36	    }
    37	
    38	    pub fn clamp_scroll(&mut self) {
    39	        self.clamp_selection();
    40	    }
    41	
    42	    fn clamp_selection(&mut self) {
    43	        let len = self.snapshot.changes.len();
    44	        if let Some(selected) = self.changes_state.selected()
    45	            && selected >= len
    46	            && len > 0
    47	        {
    48	            self.changes_state.select(Some(len - 1));
    49	        }
    50	    }
    51	}
    52	
    53	impl Component for GitStatusPanel {
    54	    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
    55	        let len = self.snapshot.changes.len();
    56	        match key.code {
    57	            KeyCode::Char('j') | KeyCode::Down => {
    58	                if len > 0 {
    59	                    let current = self.changes_state.selected().unwrap_or(0);
    60	                    self.changes_state.select(Some((current + 1).min(len - 1)));
    61	                }
    62	                None
    63	            }
    64	            KeyCode::Char('k') | KeyCode::Up => {
    65	                if len > 0 {
    66	                    let current = self.changes_state.selected().unwrap_or(0);
    67	                    self.changes_state.select(Some(current.saturating_sub(1)));
    68	                }
    69	                None
    70	            }
    71	            KeyCode::Enter => Some(Action::Select),
    72	            _ => None,
    73	        }
    74	    }
    75	
    76	    fn handle_event(&mut self, event: &AppEvent) {
    77	        if let AppEvent::GitStatusUpdated(snapshot) = event {
    78	            self.set_snapshot(snapshot.clone());
    79	        }
    80	    }
    81	
    82	    fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
    83	        let border_style = if focused {
    84	            Theme::focus_border()
    85	        } else {
    86	            Theme::unfocus_border()
    87	        };
    88	
    89	        let block = Block::bordered()
    90	            .border_type(BorderType::Rounded)
    91	            .title(theme::panel_title("Git Status", focused))
    92	            .border_style(border_style);
    93	
    94	        let inner = block.inner(area);
    95	        frame.render_widget(block, area);
    96	
    97	        if inner.height < 3 {
    98	            return;
    99	        }
   100	
   101	        // Split inner: header(3) | changes(fill) | bottom(commits + worktrees)
   102	        let header_h = 3u16;
   103	        let worktrees_h = if self.snapshot.worktrees.is_empty() {
   104	            0
   105	        } else {
   106	            self.snapshot.worktrees.len() as u16 + 2 // blank line + "Worktrees:" + entries
   107	        };
   108	        let commits_h = self.snapshot.commits.len().min(5) as u16 + 2 + worktrees_h;
   109	        let changes_h = inner.height.saturating_sub(header_h + commits_h);
   110	
   111	        let [header_area, changes_area, commits_area] = Layout::vertical([
   112	            Constraint::Length(header_h),
   113	            Constraint::Length(changes_h),
   114	            Constraint::Length(commits_h),
   115	        ])
   116	        .areas(inner);
   117	
   118	        // Header: branch + HEAD + diff stat
   119	        let header_lines = vec![
   120	            Line::from(vec![
   121	                Span::from("  Branch: ").dim(),
   122	                Span::from(self.snapshot.branch.as_str()).cyan().bold(),
   123	            ]),
   124	            Line::from(vec![
   125	                Span::from("  HEAD:   ").dim(),
   126	                Span::from(self.snapshot.head.as_str()).dim(),
   127	            ]),
   128	            Line::from(vec![
   129	                Span::from("  Diff:   ").dim(),
   130	                Span::from(format!("+{}", self.snapshot.diff_stat.additions)).green(),
   131	                Span::from(" / "),
   132	                Span::from(format!("-{}", self.snapshot.diff_stat.deletions)).red(),
   133	            ]),
   134	        ];
   135	        frame.render_widget(Paragraph::new(header_lines), header_area);
   136	
   137	        // Changes list
   138	        let change_items: Vec<ListItem> = self
   139	            .snapshot
   140	            .changes
   141	            .iter()
   142	            .map(|c| {
   143	                let (icon, style) = match c.status {
   144	                    GitChangeStatus::Staged => (Icons::staged(), Theme::staged()),
   145	                    GitChangeStatus::Unstaged => (Icons::unstaged(), Theme::unstaged()),
   146	                    GitChangeStatus::Untracked => (Icons::untracked(), Theme::untracked()),
   147	                    GitChangeStatus::Conflict => (Icons::conflict(), Theme::conflict()),
   148	                };
   149	                let diff_str = match (c.additions, c.deletions) {
   150	                    (Some(a), Some(d)) => format!(" +{a} -{d}"),
   151	                    _ => String::new(),
   152	                };
   153	                ListItem::new(Line::from(vec![
   154	                    Span::from(format!("  {icon} ")).style(style),
   155	                    Span::from(c.path.as_str()).style(style),
   156	                    Span::from(diff_str).dim(),
   157	                ]))
   158	            })
   159	            .collect();
   160	
   161	        let changes_list = List::new(change_items)
   162	            .highlight_style(Theme::highlight());
   163	        frame.render_stateful_widget(changes_list, changes_area, &mut self.changes_state);
   164	
   165	        // Recent commits + worktrees
   166	        let mut bottom_lines: Vec<Line> = Vec::new();
   167	
   168	        bottom_lines.push(Line::from("  Commits:").dim());
   169	        for c in &self.snapshot.commits {
   170	            bottom_lines.push(Line::from(vec![
   171	                Span::from(format!("    {} ", c.hash)).yellow(),
   172	                Span::from(c.message.as_str()).dim(),
   173	            ]));
   174	        }
   175	
   176	        if !self.snapshot.worktrees.is_empty() {
   177	            bottom_lines.push(Line::from(""));
   178	            bottom_lines.push(Line::from("  Worktrees:").dim());
   179	            for wt in &self.snapshot.worktrees {
   180	                let branch_str = wt
   181	                    .branch
   182	                    .as_deref()
   183	                    .unwrap_or("(detached)");
   184	                bottom_lines.push(Line::from(vec![
   185	                    Span::from("    ").dim(),
   186	                    Span::from(branch_str).cyan(),
   187	                    Span::from(format!("  {}", wt.path)).dim(),
   188	                ]));
   189	            }
   190	        }
   191	
   192	        frame.render_widget(Paragraph::new(bottom_lines), commits_area);
   193	    }
   194	}
   195	
   196	#[cfg(test)]
   197	mod tests {
   198	    use super::*;
   199	    use crate::parser::models::GitChange;
   200	
   201	    #[test]
   202	    fn test_clamp_scroll_empty() {
   203	        let mut panel = GitStatusPanel::new();
   204	        panel.changes_state.select(Some(5));
   205	        panel.clamp_scroll();
   206	        // No changes → selection stays (len=0 guard)
   207	    }
   208	
   209	    #[test]
   210	    fn test_set_snapshot_clamps() {
   211	        let mut panel = GitStatusPanel::new();
   212	        panel.changes_state.select(Some(10));
   213	        let snapshot = GitSnapshot {
   214	            changes: vec![GitChange {
   215	                status: GitChangeStatus::Staged,
   216	                path: "file.rs".to_string(),
   217	                additions: None,
   218	                deletions: None,
   219	            }],
   220	            ..Default::default()
   221	        };
   222	        panel.set_snapshot(snapshot);
   223	        assert_eq!(panel.changes_state.selected(), Some(0));
   224	    }
   225	
   226	    // ── Render tests ──
   227	
   228	    use crate::test_helpers::{buffer_contains_str, render_component};
   229	    use crate::parser::models::{DiffStat, GitCommit, GitWorktree};
   230	
   231	    #[test]
   232	    fn render_empty_snapshot_no_panic() {
   233	        let mut panel = GitStatusPanel::new();
   234	        let _terminal = render_component(&mut panel, 60, 20, true);
   235	    }
   236	
   237	    #[test]
   238	    fn render_branch_and_head() {
   239	        let mut panel = GitStatusPanel::new();
   240	        panel.set_snapshot(GitSnapshot {
   241	            branch: "feature/test".to_string(),
   242	            head: "abc1234".to_string(),
   243	            ..Default::default()
   244	        });
   245	        let terminal = render_component(&mut panel, 60, 20, true);
   246	        let buf = terminal.backend().buffer();
   247	        assert!(buffer_contains_str(buf, "feature/test"));
   248	        assert!(buffer_contains_str(buf, "abc1234"));
   249	    }
   250	
   251	    #[test]
   252	    fn render_diff_stat() {
   253	        let mut panel = GitStatusPanel::new();
   254	        panel.set_snapshot(GitSnapshot {
   255	            diff_stat: DiffStat { additions: 10, deletions: 5 },
   256	            ..Default::default()
   257	        });
   258	        let terminal = render_component(&mut panel, 60, 20, true);
   259	        let buf = terminal.backend().buffer();
   260	        assert!(buffer_contains_str(buf, "+10"));
   261	        assert!(buffer_contains_str(buf, "-5"));
   262	    }
   263	
   264	    #[test]
   265	    fn render_staged_change() {
   266	        let mut panel = GitStatusPanel::new();
   267	        panel.set_snapshot(GitSnapshot {
   268	            changes: vec![GitChange {
   269	                status: GitChangeStatus::Staged,
   270	                path: "src/main.rs".to_string(),
   271	                additions: Some(3),
   272	                deletions: Some(1),
   273	            }],
   274	            ..Default::default()
   275	        });
   276	        let terminal = render_component(&mut panel, 60, 20, true);
   277	        let buf = terminal.backend().buffer();
   278	        assert!(buffer_contains_str(buf, "S"));
   279	        assert!(buffer_contains_str(buf, "src/main.rs"));
   280	    }
   281	
   282	    #[test]
   283	    fn render_unstaged_change() {
   284	        let mut panel = GitStatusPanel::new();
   285	        panel.set_snapshot(GitSnapshot {
   286	            changes: vec![GitChange {
   287	                status: GitChangeStatus::Unstaged,
   288	                path: "lib.rs".to_string(),
   289	                additions: None,
   290	                deletions: None,
   291	            }],
   292	            ..Default::default()
   293	        });
   294	        let terminal = render_component(&mut panel, 60, 20, true);
   295	        let buf = terminal.backend().buffer();
   296	        assert!(buffer_contains_str(buf, "M"));
   297	        assert!(buffer_contains_str(buf, "lib.rs"));
   298	    }
   299	
   300	    #[test]
   301	    fn render_commits() {
   302	        let mut panel = GitStatusPanel::new();
   303	        panel.set_snapshot(GitSnapshot {
   304	            commits: vec![GitCommit {
   305	                hash: "deadbeef".to_string(),
   306	                message: "fix: resolve crash".to_string(),
   307	            }],
   308	            ..Default::default()
   309	        });
   310	        let terminal = render_component(&mut panel, 60, 20, true);
   311	        let buf = terminal.backend().buffer();
   312	        assert!(buffer_contains_str(buf, "deadbeef"));
   313	        assert!(buffer_contains_str(buf, "fix: resolve crash"));
   314	    }
   315	
   316	    #[test]
   317	    fn render_worktrees_visible() {
   318	        let mut panel = GitStatusPanel::new();
   319	        panel.set_snapshot(GitSnapshot {
   320	            commits: vec![GitCommit {
   321	                hash: "abc1234".to_string(),
   322	                message: "init".to_string(),
   323	            }],
   324	            worktrees: vec![GitWorktree {
   325	                path: "/tmp/wt".to_string(),
   326	                branch: Some("feature/wt".to_string()),
   327	            }],
   328	            ..Default::default()
   329	        });
   330	        let terminal = render_component(&mut panel, 60, 20, true);
   331	        let buf = terminal.backend().buffer();
   332	        assert!(buffer_contains_str(buf, "Worktrees:"));
   333	        assert!(buffer_contains_str(buf, "feature/wt"));
   334	    }
   335	}

 succeeded in 0ms:
     1	use crossterm::event::{KeyCode, KeyEvent};
     2	use ratatui::layout::Rect;
     3	use ratatui::style::Stylize;
     4	use ratatui::text::{Line, Text};
     5	use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
     6	use ratatui::Frame;
     7	
     8	use crate::action::Action;
     9	use crate::component::Component;
    10	use crate::event::AppEvent;
    11	use crate::parser::models::{Complexity, FlowState, SessionSummary, StageStatus, WorkMarker};
    12	use crate::ui::theme::{self, Icons, Theme};
    13	
    14	/// INCEPTION stage definitions with conditional visibility.
    15	struct StageInfo {
    16	    name: &'static str,
    17	    min_complexity: Option<Complexity>,
    18	}
    19	
    20	const INCEPTION_STAGES: &[StageInfo] = &[
    21	    StageInfo { name: "workspace-detection", min_complexity: None },
    22	    StageInfo { name: "complexity-declaration", min_complexity: None },
    23	    StageInfo { name: "requirements-analysis", min_complexity: None },
    24	    StageInfo { name: "user-stories", min_complexity: Some(Complexity::Standard) },
    25	    StageInfo { name: "nfr-requirements", min_complexity: Some(Complexity::Standard) },
    26	    StageInfo { name: "workflow-planning", min_complexity: None },
    27	    StageInfo { name: "application-design", min_complexity: Some(Complexity::Comprehensive) },
    28	    StageInfo { name: "units-generation", min_complexity: Some(Complexity::Comprehensive) },
    29	];
    30	
    31	const CONSTRUCTION_STAGES: &[StageInfo] = &[
    32	    StageInfo { name: "functional-design", min_complexity: Some(Complexity::Comprehensive) },
    33	    StageInfo { name: "code-generation", min_complexity: None },
    34	    StageInfo { name: "build-and-test", min_complexity: None },
    35	];
    36	
    37	pub struct WorkflowMapPanel {
    38	    flow_state: FlowState,
    39	    session_summary: SessionSummary,
    40	    syncing: bool,
    41	    pub scroll_offset: u16,
    42	}
    43	
    44	impl Default for WorkflowMapPanel {
    45	    fn default() -> Self {
    46	        Self::new()
    47	    }
    48	}
    49	
    50	impl WorkflowMapPanel {
    51	    pub fn new() -> Self {
    52	        Self {
    53	            flow_state: FlowState::default(),
    54	            session_summary: SessionSummary::default(),
    55	            syncing: false,
    56	            scroll_offset: 0,
    57	        }
    58	    }
    59	
    60	    pub fn set_flow_state(&mut self, state: FlowState) {
    61	        self.flow_state = state;
    62	        self.syncing = false;
    63	    }
    64	
    65	    pub fn set_session_summary(&mut self, summary: SessionSummary) {
    66	        self.session_summary = summary;
    67	    }
    68	
    69	    pub fn set_syncing(&mut self) {
    70	        self.syncing = true;
    71	    }
    72	
    73	    fn stage_status(&self, stage_name: &str) -> StageStatus {
    74	        if self.flow_state.stage == stage_name {
    75	            return StageStatus::Active;
    76	        }
    77	        if self.flow_state.completed_stages.iter().any(|s| s.name == stage_name) {
    78	            return StageStatus::Completed;
    79	        }
    80	        if self.flow_state.skipped_stages.iter().any(|s| s.name == stage_name) {
    81	            return StageStatus::Skipped;
    82	        }
    83	        StageStatus::Waiting
    84	    }
    85	
    86	    fn is_visible(&self, info: &StageInfo) -> bool {
    87	        match info.min_complexity {
    88	            None => true,
    89	            Some(Complexity::Standard) => matches!(
    90	                self.flow_state.complexity,
    91	                Complexity::Standard | Complexity::Comprehensive
    92	            ),
    93	            Some(Complexity::Comprehensive) => {
    94	                self.flow_state.complexity == Complexity::Comprehensive
    95	            }
    96	            Some(Complexity::Minimal) => true,
    97	        }
    98	    }
    99	
   100	    fn render_stages<'a>(&self, phase_name: &str, stages: &[StageInfo]) -> Vec<Line<'a>> {
   101	        let mut lines = vec![Line::from(phase_name.to_string()).bold()];
   102	
   103	        let visible: Vec<&StageInfo> = stages.iter().filter(|s| self.is_visible(s)).collect();
   104	
   105	        for (i, info) in visible.iter().enumerate() {
   106	            let status = self.stage_status(info.name);
   107	            let (icon, style) = match status {
   108	                StageStatus::Active => (Icons::active(), Theme::active()),
   109	                StageStatus::Completed => (Icons::done(), Theme::done()),
   110	                StageStatus::Waiting => (Icons::waiting(), Theme::waiting()),
   111	                StageStatus::Skipped => (Icons::skipped(), Theme::skipped()),
   112	            };
   113	
   114	            let prefix = if i == visible.len() - 1 { "  └── " } else { "  ├── " };
   115	            lines.push(Line::from(vec![
   116	                ratatui::text::Span::from(prefix).dim(),
   117	                theme::status_span(icon, info.name, style),
   118	            ]));
   119	        }
   120	
   121	        lines
   122	    }
   123	}
   124	
   125	impl Component for WorkflowMapPanel {
   126	    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
   127	        match key.code {
   128	            KeyCode::Char('j') | KeyCode::Down => {
   129	                self.scroll_offset = self.scroll_offset.saturating_add(1);
   130	                None
   131	            }
   132	            KeyCode::Char('k') | KeyCode::Up => {
   133	                self.scroll_offset = self.scroll_offset.saturating_sub(1);
   134	                None
   135	            }
   136	            KeyCode::Enter => Some(Action::OpenArtifactModal),
   137	            _ => None,
   138	        }
   139	    }
   140	
   141	    fn handle_event(&mut self, event: &AppEvent) {
   142	        match event {
   143	            AppEvent::FlowStateChanged(state) => self.set_flow_state(state.clone()),
   144	            AppEvent::SessionSummaryChanged(summary) => self.set_session_summary(summary.clone()),
   145	            AppEvent::ParseError { .. } => self.set_syncing(),
   146	            _ => {}
   147	        }
   148	    }
   149	
   150	    fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
   151	        let border_style = if focused {
   152	            Theme::focus_border()
   153	        } else {
   154	            Theme::unfocus_border()
   155	        };
   156	
   157	        let block = Block::bordered()
   158	            .border_type(BorderType::Rounded)
   159	            .title(theme::panel_title("Workflow Map", focused))
   160	            .border_style(border_style);
   161	
   162	        let inner = block.inner(area);
   163	
   164	        let mut lines: Vec<Line> = Vec::new();
   165	
   166	        if self.syncing {
   167	            lines.push(Line::from("  Syncing...").yellow());
   168	            lines.push(Line::from(""));
   169	        }
   170	
   171	        // INCEPTION stages
   172	        lines.extend(self.render_stages("  INCEPTION", INCEPTION_STAGES));
   173	        lines.push(Line::from(""));
   174	
   175	        // CONSTRUCTION stages
   176	        lines.extend(self.render_stages("  CONSTRUCTION", CONSTRUCTION_STAGES));
   177	        lines.push(Line::from(""));
   178	
   179	        // Completed Work
   180	        if !self.session_summary.completed_work.is_empty() {
   181	            lines.push(Line::from("  Completed Work:").dim());
   182	            for item in &self.session_summary.completed_work {
   183	                let (icon, style) = match item.marker {
   184	                    WorkMarker::Done => (Icons::done(), Theme::done()),
   185	                    WorkMarker::InProgress => ("◐", Theme::active()),
   186	                    WorkMarker::Pending => (Icons::waiting(), Theme::waiting()),
   187	                };
   188	                lines.push(Line::from(vec![
   189	                    ratatui::text::Span::from(format!("    {icon} ")).style(style),
   190	                    ratatui::text::Span::from(item.text.as_str()).style(style),
   191	                ]));
   192	            }
   193	            lines.push(Line::from(""));
   194	        }
   195	
   196	        // Key Decisions
   197	        if !self.session_summary.key_decisions.is_empty() {
   198	            lines.push(Line::from("  Key Decisions:").dim());
   199	            for decision in &self.session_summary.key_decisions {
   200	                lines.push(Line::from(format!("    • {decision}")).dim());
   201	            }
   202	            lines.push(Line::from(""));
   203	        }
   204	
   205	        // Next Steps
   206	        if !self.session_summary.next_steps.is_empty() {
   207	            lines.push(Line::from("  Next:").dim());
   208	            for step in &self.session_summary.next_steps {
   209	                lines.push(Line::from(format!("    {step}")).dim());
   210	            }
   211	        }
   212	
   213	        // Active unit info
   214	        if let Some(ref unit) = self.flow_state.active_unit {
   215	            lines.push(Line::from(""));
   216	            lines.push(Line::from(format!("  Active Unit: {unit}")).yellow());
   217	        }
   218	
   219	        let text = Text::from(lines);
   220	        let content = Paragraph::new(text)
   221	            .scroll((self.scroll_offset, 0))
   222	            .wrap(Wrap { trim: false });
   223	
   224	        frame.render_widget(block, area);
   225	        frame.render_widget(content, inner);
   226	    }
   227	}
   228	
   229	#[cfg(test)]
   230	mod tests {
   231	    use super::*;
   232	    use crate::parser::models::CompletedStage;
   233	
   234	    #[test]
   235	    fn test_stage_status_active() {
   236	        let mut panel = WorkflowMapPanel::new();
   237	        panel.flow_state.stage = "requirements-analysis".to_string();
   238	        assert_eq!(panel.stage_status("requirements-analysis"), StageStatus::Active);
   239	    }
   240	
   241	    #[test]
   242	    fn test_stage_status_completed() {
   243	        let mut panel = WorkflowMapPanel::new();
   244	        panel.flow_state.completed_stages.push(CompletedStage {
   245	            name: "workspace-detection".to_string(),
   246	            timestamp: None,
   247	        });
   248	        assert_eq!(panel.stage_status("workspace-detection"), StageStatus::Completed);
   249	    }
   250	
   251	    #[test]
   252	    fn test_visibility_minimal() {
   253	        let mut panel = WorkflowMapPanel::new();
   254	        panel.flow_state.complexity = Complexity::Minimal;
   255	        let info = StageInfo {
   256	            name: "user-stories",
   257	            min_complexity: Some(Complexity::Standard),
   258	        };
   259	        assert!(!panel.is_visible(&info));
   260	    }
   261	
   262	    #[test]
   263	    fn test_visibility_standard() {
   264	        let mut panel = WorkflowMapPanel::new();
   265	        panel.flow_state.complexity = Complexity::Standard;
   266	        let info = StageInfo {
   267	            name: "user-stories",
   268	            min_complexity: Some(Complexity::Standard),
   269	        };
   270	        assert!(panel.is_visible(&info));
   271	    }
   272	
   273	    // ── Render tests ──
   274	
   275	    use crate::test_helpers::{buffer_contains_str, render_component};
   276	    use crate::parser::models::WorkItem;
   277	
   278	    #[test]
   279	    fn render_default_state() {
   280	        let mut panel = WorkflowMapPanel::new();
   281	        let terminal = render_component(&mut panel, 60, 30, true);
   282	        let buf = terminal.backend().buffer();
   283	        assert!(buffer_contains_str(buf, "INCEPTION"));
   284	        assert!(buffer_contains_str(buf, "CONSTRUCTION"));
   285	        assert!(buffer_contains_str(buf, "○"));
   286	    }
   287	
   288	    #[test]
   289	    fn render_active_stage() {
   290	        let mut panel = WorkflowMapPanel::new();
   291	        panel.flow_state.stage = "requirements-analysis".to_string();
   292	        let terminal = render_component(&mut panel, 60, 30, true);
   293	        let buf = terminal.backend().buffer();
   294	        assert!(buffer_contains_str(buf, "●"));
   295	        assert!(buffer_contains_str(buf, "requirements-analysis"));
   296	    }
   297	
   298	    #[test]
   299	    fn render_completed_stage() {
   300	        let mut panel = WorkflowMapPanel::new();
   301	        panel.flow_state.completed_stages.push(CompletedStage {
   302	            name: "workspace-detection".to_string(),
   303	            timestamp: None,
   304	        });
   305	        let terminal = render_component(&mut panel, 60, 30, true);
   306	        let buf = terminal.backend().buffer();
   307	        assert!(buffer_contains_str(buf, "✓"));
   308	        assert!(buffer_contains_str(buf, "workspace-detection"));
   309	    }
   310	
   311	    #[test]
   312	    fn render_minimal_complexity() {
   313	        let mut panel = WorkflowMapPanel::new();
   314	        panel.flow_state.complexity = Complexity::Minimal;
   315	        let terminal = render_component(&mut panel, 60, 30, true);
   316	        let buf = terminal.backend().buffer();
   317	        // Standard-only stages should be hidden
   318	        assert!(!buffer_contains_str(buf, "user-stories"));
   319	        // Always-visible stages should be present
   320	        assert!(buffer_contains_str(buf, "workspace-detection"));
   321	    }
   322	
   323	    #[test]
   324	    fn render_comprehensive_complexity() {
   325	        let mut panel = WorkflowMapPanel::new();
   326	        panel.flow_state.complexity = Complexity::Comprehensive;
   327	        let terminal = render_component(&mut panel, 60, 30, true);
   328	        let buf = terminal.backend().buffer();
   329	        assert!(buffer_contains_str(buf, "application-design"));
   330	        assert!(buffer_contains_str(buf, "units-generation"));
   331	    }
   332	
   333	    #[test]
   334	    fn render_syncing_state() {
   335	        let mut panel = WorkflowMapPanel::new();
   336	        panel.set_syncing();
   337	        let terminal = render_component(&mut panel, 60, 30, true);
   338	        let buf = terminal.backend().buffer();
   339	        assert!(buffer_contains_str(buf, "Syncing..."));
   340	    }
   341	
   342	    #[test]
   343	    fn render_completed_work_and_decisions() {
   344	        let mut panel = WorkflowMapPanel::new();
   345	        panel.session_summary.completed_work.push(WorkItem {
   346	            text: "Setup project".to_string(),
   347	            marker: WorkMarker::Done,
   348	        });
   349	        panel.session_summary.key_decisions.push("Use ratatui".to_string());
   350	        let terminal = render_component(&mut panel, 60, 35, true);
   351	        let buf = terminal.backend().buffer();
   352	        assert!(buffer_contains_str(buf, "Completed Work:"));
   353	        assert!(buffer_contains_str(buf, "Setup project"));
   354	        assert!(buffer_contains_str(buf, "Key Decisions:"));
   355	        assert!(buffer_contains_str(buf, "Use ratatui"));
   356	    }
   357	}

 succeeded in 0ms:
     1	use std::collections::{HashMap, VecDeque};
     2	use std::time::Instant;
     3	
     4	use crossterm::event::{KeyCode, KeyEvent};
     5	use ratatui::layout::Rect;
     6	use ratatui::style::Stylize;
     7	use ratatui::text::{Line, Span};
     8	use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph};
     9	use ratatui::Frame;
    10	
    11	use crate::action::Action;
    12	use crate::component::Component;
    13	use crate::event::AppEvent;
    14	use crate::ui::theme::{self, Icons, Theme};
    15	
    16	const ORPHAN_TIMEOUT_SECS: u64 = 60;
    17	const MAX_HISTORY: usize = 100;
    18	
    19	#[derive(Debug, Clone)]
    20	enum AgentState {
    21	    Running { started: Instant },
    22	    Done,
    23	    Timeout,
    24	}
    25	
    26	#[derive(Debug, Clone)]
    27	struct AgentEntry {
    28	    seq: u64,
    29	    agent_id: String,
    30	    agent_type: String,
    31	    state: AgentState,
    32	}
    33	
    34	pub struct AgentStatusPanel {
    35	    agents: VecDeque<AgentEntry>,
    36	    running_map: HashMap<String, u64>, // agent_id → seq
    37	    next_seq: u64,
    38	    list_state: ListState,
    39	    hooks_active: bool,
    40	}
    41	
    42	impl Default for AgentStatusPanel {
    43	    fn default() -> Self {
    44	        Self::new()
    45	    }
    46	}
    47	
    48	impl AgentStatusPanel {
    49	    pub fn new() -> Self {
    50	        Self {
    51	            agents: VecDeque::new(),
    52	            running_map: HashMap::new(),
    53	            next_seq: 0,
    54	            list_state: ListState::default(),
    55	            hooks_active: false,
    56	        }
    57	    }
    58	
    59	    pub fn set_hooks_active(&mut self, active: bool) {
    60	        self.hooks_active = active;
    61	    }
    62	
    63	    fn find_by_seq(&self, seq: u64) -> Option<usize> {
    64	        self.agents.iter().position(|a| a.seq == seq)
    65	    }
    66	
    67	    fn agent_started(&mut self, agent_id: String, agent_type: String) {
    68	        // If same agent_id is already running, mark it as Done first
    69	        if let Some(old_seq) = self.running_map.remove(&agent_id)
    70	            && let Some(idx) = self.find_by_seq(old_seq)
    71	        {
    72	            self.agents[idx].state = AgentState::Done;
    73	        }
    74	
    75	        let seq = self.next_seq;
    76	        self.next_seq += 1;
    77	
    78	        let entry = AgentEntry {
    79	            seq,
    80	            agent_id: agent_id.clone(),
    81	            agent_type,
    82	            state: AgentState::Running {
    83	                started: Instant::now(),
    84	            },
    85	        };
    86	        self.agents.push_back(entry);
    87	        self.running_map.insert(agent_id, seq);
    88	
    89	        // Cap history — pop_front O(1)
    90	        while self.agents.len() > MAX_HISTORY {
    91	            if let Some(removed) = self.agents.pop_front() {
    92	                // Clean up running_map if the removed entry was still tracked
    93	                if let Some(&s) = self.running_map.get(&removed.agent_id)
    94	                    && s == removed.seq
    95	                {
    96	                    self.running_map.remove(&removed.agent_id);
    97	                }
    98	            }
    99	        }
   100	    }
   101	
   102	    fn agent_stopped(&mut self, agent_id: &str) {
   103	        if let Some(seq) = self.running_map.remove(agent_id)
   104	            && let Some(idx) = self.find_by_seq(seq)
   105	        {
   106	            self.agents[idx].state = AgentState::Done;
   107	        }
   108	    }
   109	
   110	    /// Check for orphan agents (running > 60s without stop).
   111	    pub fn check_timeouts(&mut self) -> bool {
   112	        let mut changed = false;
   113	        let now = Instant::now();
   114	        let mut timed_out_ids = Vec::new();
   115	
   116	        for (id, &seq) in &self.running_map {
   117	            if let Some(idx) = self.find_by_seq(seq)
   118	                && let AgentState::Running { started } = self.agents[idx].state
   119	                && now.duration_since(started).as_secs() >= ORPHAN_TIMEOUT_SECS
   120	            {
   121	                timed_out_ids.push(id.clone());
   122	            }
   123	        }
   124	
   125	        for id in timed_out_ids {
   126	            if let Some(seq) = self.running_map.remove(&id)
   127	                && let Some(idx) = self.find_by_seq(seq)
   128	            {
   129	                self.agents[idx].state = AgentState::Timeout;
   130	                changed = true;
   131	            }
   132	        }
   133	
   134	        changed
   135	    }
   136	
   137	    /// Returns true if any running agents exist.
   138	    pub fn has_running_agents(&self) -> bool {
   139	        !self.running_map.is_empty()
   140	    }
   141	
   142	    fn running_count(&self) -> usize {
   143	        self.running_map.len()
   144	    }
   145	
   146	    pub fn clamp_scroll(&mut self) {
   147	        let len = self.agents.len();
   148	        if let Some(selected) = self.list_state.selected()
   149	            && selected >= len
   150	            && len > 0
   151	        {
   152	            self.list_state.select(Some(len - 1));
   153	        }
   154	    }
   155	}
   156	
   157	impl Component for AgentStatusPanel {
   158	    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
   159	        let len = self.agents.len();
   160	        match key.code {
   161	            KeyCode::Char('j') | KeyCode::Down => {
   162	                if len > 0 {
   163	                    let cur = self.list_state.selected().unwrap_or(0);
   164	                    self.list_state.select(Some((cur + 1).min(len - 1)));
   165	                }
   166	                None
   167	            }
   168	            KeyCode::Char('k') | KeyCode::Up => {
   169	                if len > 0 {
   170	                    let cur = self.list_state.selected().unwrap_or(0);
   171	                    self.list_state.select(Some(cur.saturating_sub(1)));
   172	                }
   173	                None
   174	            }
   175	            _ => None,
   176	        }
   177	    }
   178	
   179	    fn handle_event(&mut self, event: &AppEvent) {
   180	        match event {
   181	            AppEvent::AgentStarted {
   182	                agent_id,
   183	                agent_type,
   184	            } => {
   185	                self.agent_started(agent_id.clone(), agent_type.clone());
   186	            }
   187	            AppEvent::AgentStopped { agent_id } => {
   188	                self.agent_stopped(agent_id);
   189	            }
   190	            AppEvent::HooksServerStarted { .. } => {
   191	                self.set_hooks_active(true);
   192	            }
   193	            AppEvent::HooksServerFailed { .. } => {
   194	                self.set_hooks_active(false);
   195	            }
   196	            _ => {}
   197	        }
   198	    }
   199	
   200	    fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
   201	        let border_style = if focused {
   202	            Theme::focus_border()
   203	        } else {
   204	            Theme::unfocus_border()
   205	        };
   206	
   207	        let block = Block::bordered()
   208	            .border_type(BorderType::Rounded)
   209	            .title(theme::panel_title("Agent Status", focused))
   210	            .border_style(border_style);
   211	
   212	        let inner = block.inner(area);
   213	        frame.render_widget(block, area);
   214	
   215	        if !self.hooks_active {
   216	            let msg = Paragraph::new(vec![
   217	                Line::from(""),
   218	                Line::from("  에이전트 추적을 위해").dim(),
   219	                Line::from("  hooks 설정이 필요합니다").dim(),
   220	                Line::from(""),
   221	                Line::from("  ? 키로 설정 안내 확인").dim(),
   222	            ]);
   223	            frame.render_widget(msg, inner);
   224	            return;
   225	        }
   226	
   227	        if self.agents.is_empty() {
   228	            let msg = Paragraph::new(Line::from("  대기 중...").dim());
   229	            frame.render_widget(msg, inner);
   230	            return;
   231	        }
   232	
   233	        let items: Vec<ListItem> = self
   234	            .agents
   235	            .iter()
   236	            .rev() // Most recent first
   237	            .map(|entry| {
   238	                let (icon, style, elapsed_str) = match &entry.state {
   239	                    AgentState::Running { started } => {
   240	                        let secs = Instant::now().duration_since(*started).as_secs();
   241	                        (Icons::active(), Theme::active(), format!("{secs}s"))
   242	                    }
   243	                    AgentState::Done => (Icons::done(), Theme::done(), "done".to_string()),
   244	                    AgentState::Timeout => {
   245	                        (Icons::timeout(), Theme::timeout(), "t/o".to_string())
   246	                    }
   247	                };
   248	
   249	                ListItem::new(Line::from(vec![
   250	                    Span::from(format!("  {icon} ")).style(style),
   251	                    Span::from(format!("{:<10}", entry.agent_type)).style(style),
   252	                    Span::from(format!(" {}", elapsed_str)).dim(),
   253	                ]))
   254	            })
   255	            .collect();
   256	
   257	        let total = self.agents.len();
   258	        let running = self.running_count();
   259	        let footer = format!("  Total: {total} ({running} running)");
   260	
   261	        let list = List::new(items).highlight_style(Theme::highlight());
   262	        frame.render_stateful_widget(list, inner, &mut self.list_state);
   263	
   264	        // Footer in the last line of inner
   265	        if inner.height > 1 {
   266	            let footer_area = Rect::new(inner.x, inner.y + inner.height - 1, inner.width, 1);
   267	            frame.render_widget(Paragraph::new(Line::from(footer).dim()), footer_area);
   268	        }
   269	    }
   270	}
   271	
   272	#[cfg(test)]
   273	mod tests {
   274	    use super::*;
   275	
   276	    #[test]
   277	    fn test_agent_lifecycle() {
   278	        let mut panel = AgentStatusPanel::new();
   279	        panel.set_hooks_active(true);
   280	
   281	        panel.agent_started("a1".to_string(), "Explore".to_string());
   282	        assert_eq!(panel.running_count(), 1);
   283	        assert!(panel.has_running_agents());
   284	
   285	        panel.agent_stopped("a1");
   286	        assert_eq!(panel.running_count(), 0);
   287	        assert!(!panel.has_running_agents());
   288	    }
   289	
   290	    #[test]
   291	    fn test_agent_timeout() {
   292	        let mut panel = AgentStatusPanel::new();
   293	        panel.set_hooks_active(true);
   294	
   295	        // Manually create an agent with old start time
   296	        let seq = panel.next_seq;
   297	        panel.next_seq += 1;
   298	        panel.agents.push_back(AgentEntry {
   299	            seq,
   300	            agent_id: "old".to_string(),
   301	            agent_type: "Plan".to_string(),
   302	            state: AgentState::Running {
   303	                started: Instant::now() - std::time::Duration::from_secs(61),
   304	            },
   305	        });
   306	        panel.running_map.insert("old".to_string(), seq);
   307	
   308	        let changed = panel.check_timeouts();
   309	        assert!(changed);
   310	        assert_eq!(panel.running_count(), 0);
   311	        assert!(matches!(panel.agents[0].state, AgentState::Timeout));
   312	    }
   313	
   314	    #[test]
   315	    fn test_max_history() {
   316	        let mut panel = AgentStatusPanel::new();
   317	        for i in 0..MAX_HISTORY + 5 {
   318	            panel.agent_started(format!("a{i}"), "Explore".to_string());
   319	            panel.agent_stopped(&format!("a{i}"));
   320	        }
   321	        assert_eq!(panel.agents.len(), MAX_HISTORY);
   322	        assert!(panel.running_map.is_empty());
   323	    }
   324	
   325	    #[test]
   326	    fn test_duplicate_agent_id() {
   327	        let mut panel = AgentStatusPanel::new();
   328	        panel.set_hooks_active(true);
   329	
   330	        panel.agent_started("a1".to_string(), "Explore".to_string());
   331	        assert_eq!(panel.running_count(), 1);
   332	
   333	        // Same ID started again — previous should become Done
   334	        panel.agent_started("a1".to_string(), "Plan".to_string());
   335	        assert_eq!(panel.running_count(), 1);
   336	        assert_eq!(panel.agents.len(), 2);
   337	        assert!(matches!(panel.agents[0].state, AgentState::Done));
   338	        assert!(matches!(panel.agents[1].state, AgentState::Running { .. }));
   339	    }
   340	
   341	    #[test]
   342	    fn test_hooks_inactive_message() {
   343	        let panel = AgentStatusPanel::new();
   344	        assert!(!panel.hooks_active);
   345	    }
   346	
   347	    // ── Render tests ──
   348	
   349	    use crate::test_helpers::{buffer_contains_str, render_component};
   350	
   351	    #[test]
   352	    fn render_hooks_inactive() {
   353	        let mut panel = AgentStatusPanel::new();
   354	        let terminal = render_component(&mut panel, 50, 12, true);
   355	        let buf = terminal.backend().buffer();
   356	        assert!(buffer_contains_str(buf, "hooks 설정이 필요합니다"));
   357	    }
   358	
   359	    #[test]
   360	    fn render_empty_agents() {
   361	        let mut panel = AgentStatusPanel::new();
   362	        panel.set_hooks_active(true);
   363	        let terminal = render_component(&mut panel, 50, 10, true);
   364	        let buf = terminal.backend().buffer();
   365	        assert!(buffer_contains_str(buf, "대기 중..."));
   366	    }
   367	
   368	    #[test]
   369	    fn render_with_running_agent() {
   370	        let mut panel = AgentStatusPanel::new();
   371	        panel.set_hooks_active(true);
   372	        panel.agent_started("a1".to_string(), "Explore".to_string());
   373	        let terminal = render_component(&mut panel, 50, 10, true);
   374	        let buf = terminal.backend().buffer();
   375	        assert!(buffer_contains_str(buf, "●"));
   376	        assert!(buffer_contains_str(buf, "Explore"));
   377	    }
   378	
   379	    #[test]
   380	    fn render_with_done_agent() {
   381	        let mut panel = AgentStatusPanel::new();
   382	        panel.set_hooks_active(true);
   383	        panel.agent_started("a1".to_string(), "Plan".to_string());
   384	        panel.agent_stopped("a1");
   385	        let terminal = render_component(&mut panel, 50, 10, true);
   386	        let buf = terminal.backend().buffer();
   387	        assert!(buffer_contains_str(buf, "✓"));
   388	        assert!(buffer_contains_str(buf, "done"));
   389	    }
   390	
   391	    #[test]
   392	    fn render_with_timeout_agent() {
   393	        let mut panel = AgentStatusPanel::new();
   394	        panel.set_hooks_active(true);
   395	        let seq = panel.next_seq;
   396	        panel.next_seq += 1;
   397	        panel.agents.push_back(AgentEntry {
   398	            seq,
   399	            agent_id: "old".to_string(),
   400	            agent_type: "Plan".to_string(),
   401	            state: AgentState::Running {
   402	                started: Instant::now() - std::time::Duration::from_secs(61),
   403	            },
   404	        });
   405	        panel.running_map.insert("old".to_string(), seq);
   406	        panel.check_timeouts();
   407	
   408	        let terminal = render_component(&mut panel, 50, 10, true);
   409	        let buf = terminal.backend().buffer();
   410	        assert!(buffer_contains_str(buf, "⏱"));
   411	        assert!(buffer_contains_str(buf, "t/o"));
   412	    }
   413	
   414	    #[test]
   415	    fn render_footer_count() {
   416	        let mut panel = AgentStatusPanel::new();
   417	        panel.set_hooks_active(true);
   418	        panel.agent_started("a1".to_string(), "Explore".to_string());
   419	        panel.agent_started("a2".to_string(), "Plan".to_string());
   420	        panel.agent_stopped("a1");

exec
/bin/zsh -lc "nl -ba src/panel/agent_status.rs | sed -n '420,520p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
   420	        panel.agent_stopped("a1");
   421	        let terminal = render_component(&mut panel, 50, 12, true);
   422	        let buf = terminal.backend().buffer();
   423	        assert!(buffer_contains_str(buf, "Total: 2 (1 running)"));
   424	    }
   425	}

exec
/bin/zsh -lc "nl -ba src/panel/audit_log.rs | sed -n '1,420p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use crossterm::event::{KeyCode, KeyEvent};
     2	use ratatui::layout::Rect;
     3	use ratatui::style::Stylize;
     4	use ratatui::text::{Line, Span};
     5	use ratatui::widgets::{
     6	    Block, BorderType, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
     7	};
     8	use ratatui::Frame;
     9	
    10	use crate::action::Action;
    11	use crate::component::Component;
    12	use crate::event::AppEvent;
    13	use crate::parser::models::AuditEntry;
    14	use crate::ui::theme::{self, Theme};
    15	
    16	const DEFAULT_BUFFER_CAP: usize = 1000;
    17	const MIN_BUFFER_CAP: usize = 100;
    18	const MAX_BUFFER_CAP: usize = 100_000;
    19	
    20	pub struct AuditLogPanel {
    21	    entries: Vec<AuditEntry>,
    22	    list_state: ListState,
    23	    scrollbar_state: ScrollbarState,
    24	    auto_scroll: bool,
    25	    buffer_cap: usize,
    26	}
    27	
    28	impl Default for AuditLogPanel {
    29	    fn default() -> Self {
    30	        Self::new()
    31	    }
    32	}
    33	
    34	impl AuditLogPanel {
    35	    pub fn new() -> Self {
    36	        let cap = std::env::var("DEVFLOW_TUI_LOG_BUFFER")
    37	            .ok()
    38	            .and_then(|v| v.parse::<usize>().ok())
    39	            .unwrap_or(DEFAULT_BUFFER_CAP);
    40	        Self::new_with_cap(cap)
    41	    }
    42	
    43	    pub fn new_with_cap(cap: usize) -> Self {
    44	        Self {
    45	            entries: Vec::new(),
    46	            list_state: ListState::default(),
    47	            scrollbar_state: ScrollbarState::default(),
    48	            auto_scroll: true,
    49	            buffer_cap: cap.clamp(MIN_BUFFER_CAP, MAX_BUFFER_CAP),
    50	        }
    51	    }
    52	
    53	    pub fn set_entries(&mut self, entries: Vec<AuditEntry>) {
    54	        self.entries = entries;
    55	        self.enforce_cap();
    56	        if self.auto_scroll && !self.entries.is_empty() {
    57	            self.list_state.select(Some(self.entries.len() - 1));
    58	        }
    59	        self.scrollbar_state = ScrollbarState::new(self.entries.len())
    60	            .position(self.list_state.selected().unwrap_or(0));
    61	    }
    62	
    63	    pub fn clamp_scroll(&mut self) {
    64	        let len = self.entries.len();
    65	        if let Some(selected) = self.list_state.selected()
    66	            && selected >= len
    67	            && len > 0
    68	        {
    69	            self.list_state.select(Some(len - 1));
    70	        }
    71	    }
    72	
    73	    fn enforce_cap(&mut self) {
    74	        if self.entries.len() > self.buffer_cap {
    75	            let excess = self.entries.len() - self.buffer_cap;
    76	            self.entries.drain(..excess);
    77	        }
    78	    }
    79	
    80	    fn format_entry(entry: &AuditEntry) -> ListItem<'_> {
    81	        let mut spans = Vec::new();
    82	
    83	        if let Some(ref ts) = entry.timestamp {
    84	            spans.push(Span::from(format!("  {ts} ")).style(Theme::timestamp()));
    85	        }
    86	
    87	        if let Some(ref stage) = entry.stage {
    88	            spans.push(Span::from(stage.as_str()).bold());
    89	        }
    90	
    91	        if let Some(ref choice) = entry.choice {
    92	            spans.push(Span::from(" → ").dim());
    93	            let style = if choice.starts_with('B') || choice.contains("Approve") {
    94	                Theme::done()
    95	            } else if choice.contains("Skip") || choice.contains("skip") {
    96	                Theme::skipped()
    97	            } else {
    98	                Theme::active()
    99	            };
   100	            spans.push(Span::from(choice.as_str()).style(style));
   101	        }
   102	
   103	        if spans.is_empty() {
   104	            // Raw/unrecognized line
   105	            spans.push(Span::from(format!("  {}", entry.raw_line)).dim());
   106	        }
   107	
   108	        ListItem::new(Line::from(spans))
   109	    }
   110	}
   111	
   112	impl Component for AuditLogPanel {
   113	    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
   114	        let len = self.entries.len();
   115	        match key.code {
   116	            KeyCode::Char('j') | KeyCode::Down => {
   117	                if len > 0 {
   118	                    let cur = self.list_state.selected().unwrap_or(0);
   119	                    let next = (cur + 1).min(len - 1);
   120	                    self.list_state.select(Some(next));
   121	                    self.auto_scroll = next == len - 1;
   122	                    self.scrollbar_state = self.scrollbar_state.position(next);
   123	                }
   124	                None
   125	            }
   126	            KeyCode::Char('k') | KeyCode::Up => {
   127	                if len > 0 {
   128	                    let cur = self.list_state.selected().unwrap_or(0);
   129	                    let next = cur.saturating_sub(1);
   130	                    self.list_state.select(Some(next));
   131	                    self.auto_scroll = false;
   132	                    self.scrollbar_state = self.scrollbar_state.position(next);
   133	                }
   134	                None
   135	            }
   136	            KeyCode::Char('G') => {
   137	                // Jump to bottom, re-enable auto scroll
   138	                if len > 0 {
   139	                    self.list_state.select(Some(len - 1));
   140	                    self.auto_scroll = true;
   141	                    self.scrollbar_state = self.scrollbar_state.position(len - 1);
   142	                }
   143	                None
   144	            }
   145	            _ => None,
   146	        }
   147	    }
   148	
   149	    fn handle_event(&mut self, event: &AppEvent) {
   150	        if let AppEvent::AuditLogAppended(entries) = event {
   151	            self.set_entries(entries.clone());
   152	        }
   153	    }
   154	
   155	    fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
   156	        let border_style = if focused {
   157	            Theme::focus_border()
   158	        } else {
   159	            Theme::unfocus_border()
   160	        };
   161	
   162	        let block = Block::bordered()
   163	            .border_type(BorderType::Rounded)
   164	            .title(theme::panel_title("Audit Log", focused))
   165	            .border_style(border_style);
   166	
   167	        let inner = block.inner(area);
   168	        frame.render_widget(block, area);
   169	
   170	        if self.entries.is_empty() {
   171	            let msg = ratatui::widgets::Paragraph::new(
   172	                Line::from("  감사 로그 대기 중...").dim(),
   173	            );
   174	            frame.render_widget(msg, inner);
   175	            return;
   176	        }
   177	
   178	        let items: Vec<ListItem> = self.entries.iter().map(Self::format_entry).collect();
   179	
   180	        let list = List::new(items).highlight_style(Theme::highlight());
   181	        frame.render_stateful_widget(list, inner, &mut self.list_state);
   182	
   183	        // Scrollbar
   184	        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
   185	        frame.render_stateful_widget(scrollbar, area, &mut self.scrollbar_state);
   186	    }
   187	}
   188	
   189	#[cfg(test)]
   190	mod tests {
   191	    use super::*;
   192	
   193	    fn make_entries(n: usize) -> Vec<AuditEntry> {
   194	        (0..n)
   195	            .map(|i| AuditEntry {
   196	                timestamp: Some(format!("2026-03-30T14:{i:02}")),
   197	                stage: Some(format!("stage-{i}")),
   198	                choice: Some("B (Approve)".to_string()),
   199	                raw_line: String::new(),
   200	            })
   201	            .collect()
   202	    }
   203	
   204	    #[test]
   205	    fn test_buffer_cap() {
   206	        let mut panel = AuditLogPanel::new_with_cap(200);
   207	        assert_eq!(panel.buffer_cap, 200);
   208	        panel.set_entries(make_entries(300));
   209	        assert_eq!(panel.entries.len(), 200);
   210	    }
   211	
   212	    #[test]
   213	    fn test_buffer_cap_clamp_lower() {
   214	        let panel = AuditLogPanel::new_with_cap(5);
   215	        assert_eq!(panel.buffer_cap, MIN_BUFFER_CAP);
   216	    }
   217	
   218	    #[test]
   219	    fn test_buffer_cap_clamp_upper() {
   220	        let panel = AuditLogPanel::new_with_cap(usize::MAX);
   221	        assert_eq!(panel.buffer_cap, MAX_BUFFER_CAP);
   222	    }
   223	
   224	    #[test]
   225	    fn test_auto_scroll() {
   226	        let mut panel = AuditLogPanel::new();
   227	        panel.set_entries(make_entries(5));
   228	        assert_eq!(panel.list_state.selected(), Some(4)); // Last item
   229	        assert!(panel.auto_scroll);
   230	    }
   231	
   232	    #[test]
   233	    fn test_manual_scroll_disables_auto() {
   234	        let mut panel = AuditLogPanel::new();
   235	        panel.set_entries(make_entries(10));
   236	
   237	        // Scroll up
   238	        panel.handle_key(KeyEvent::from(KeyCode::Char('k')));
   239	        assert!(!panel.auto_scroll);
   240	    }
   241	
   242	    #[test]
   243	    fn test_jump_to_bottom_re_enables_auto() {
   244	        let mut panel = AuditLogPanel::new();
   245	        panel.set_entries(make_entries(10));
   246	
   247	        panel.handle_key(KeyEvent::from(KeyCode::Char('k')));
   248	        assert!(!panel.auto_scroll);
   249	
   250	        panel.handle_key(KeyEvent::from(KeyCode::Char('G')));
   251	        assert!(panel.auto_scroll);
   252	        assert_eq!(panel.list_state.selected(), Some(9));
   253	    }
   254	
   255	    #[test]
   256	    fn test_format_entry_with_choice() {
   257	        let entry = AuditEntry {
   258	            timestamp: Some("14:20".to_string()),
   259	            stage: Some("requirements".to_string()),
   260	            choice: Some("B (Approve)".to_string()),
   261	            raw_line: String::new(),
   262	        };
   263	        let _item = AuditLogPanel::format_entry(&entry);
   264	        // Just verify it doesn't panic
   265	    }
   266	
   267	    #[test]
   268	    fn test_format_entry_raw() {
   269	        let entry = AuditEntry {
   270	            timestamp: None,
   271	            stage: None,
   272	            choice: None,
   273	            raw_line: "some unrecognized line".to_string(),
   274	        };
   275	        let _item = AuditLogPanel::format_entry(&entry);
   276	    }
   277	
   278	    // ── Render tests ──
   279	
   280	    use crate::test_helpers::{buffer_contains_str, render_component};
   281	
   282	    #[test]
   283	    fn render_empty_shows_waiting() {
   284	        let mut panel = AuditLogPanel::new();
   285	        let terminal = render_component(&mut panel, 50, 10, true);
   286	        let buf = terminal.backend().buffer();
   287	        assert!(buffer_contains_str(buf, "감사 로그 대기 중..."));
   288	    }
   289	
   290	    #[test]
   291	    fn render_with_entries() {
   292	        let mut panel = AuditLogPanel::new();
   293	        panel.set_entries(make_entries(3));
   294	        let terminal = render_component(&mut panel, 60, 12, true);
   295	        let buf = terminal.backend().buffer();
   296	        assert!(buffer_contains_str(buf, "stage-0"));
   297	        assert!(buffer_contains_str(buf, "Approve"));
   298	    }
   299	
   300	    #[test]
   301	    fn render_focused_border() {
   302	        let mut panel = AuditLogPanel::new();
   303	        let terminal = render_component(&mut panel, 50, 10, true);
   304	        let buf = terminal.backend().buffer();
   305	        assert!(buffer_contains_str(buf, "[ Audit Log ]"));
   306	    }
   307	
   308	    #[test]
   309	    fn render_unfocused_border() {
   310	        let mut panel = AuditLogPanel::new();
   311	        let terminal = render_component(&mut panel, 50, 10, false);
   312	        let buf = terminal.backend().buffer();
   313	        assert!(buffer_contains_str(buf, "  Audit Log  "));
   314	    }
   315	
   316	    #[test]
   317	    fn render_scrollbar_present() {
   318	        let mut panel = AuditLogPanel::new();
   319	        panel.set_entries(make_entries(20));
   320	        let terminal = render_component(&mut panel, 50, 10, true);
   321	        let buf = terminal.backend().buffer();
   322	        // Scrollbar uses block elements like "█" or "▐"
   323	        assert!(
   324	            buffer_contains_str(buf, "█")
   325	                || buffer_contains_str(buf, "▐")
   326	                || buffer_contains_str(buf, "│")
   327	                || buffer_contains_str(buf, "▮")
   328	        );
   329	    }
   330	
   331	    #[test]
   332	    fn render_raw_entry() {
   333	        let mut panel = AuditLogPanel::new();
   334	        panel.set_entries(vec![AuditEntry {
   335	            timestamp: None,
   336	            stage: None,
   337	            choice: None,
   338	            raw_line: "some raw log text".to_string(),
   339	        }]);
   340	        let terminal = render_component(&mut panel, 60, 10, true);
   341	        let buf = terminal.backend().buffer();
   342	        assert!(buffer_contains_str(buf, "some raw log text"));
   343	    }
   344	}

exec
/bin/zsh -lc "nl -ba src/adapter/file_watcher.rs | sed -n '1,360p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::path::{Path, PathBuf};
     2	use std::time::Duration;
     3	
     4	use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
     5	use tokio::sync::{mpsc, watch};
     6	use tokio_util::sync::CancellationToken;
     7	
     8	use crate::error::Result;
     9	use crate::event::AppEvent;
    10	use crate::parser::models::{ArtifactFile, FlowState};
    11	use crate::parser::{audit_log, devflow_state, session_summary};
    12	
    13	const DEBOUNCE_MS: u64 = 300;
    14	
    15	/// Run the file watcher adapter.
    16	///
    17	/// Watches devflow-docs/ for changes. On file change, re-parses and sends
    18	/// updated state via watch/mpsc channels.
    19	///
    20	/// If devflow-docs/ doesn't exist, waits for it to be created.
    21	pub async fn run(
    22	    cancel: CancellationToken,
    23	    project_dir: PathBuf,
    24	    flow_state_tx: watch::Sender<FlowState>,
    25	    event_tx: mpsc::Sender<AppEvent>,
    26	) -> Result<()> {
    27	    let devflow_dir = project_dir.join("devflow-docs");
    28	
    29	    // Wait for devflow-docs/ to exist
    30	    while !devflow_dir.exists() {
    31	        tokio::select! {
    32	            _ = cancel.cancelled() => return Ok(()),
    33	            _ = tokio::time::sleep(Duration::from_secs(1)) => {}
    34	        }
    35	    }
    36	
    37	    // Initial parse
    38	    parse_and_send(&devflow_dir, &flow_state_tx, &event_tx).await;
    39	
    40	    // Setup notify watcher
    41	    let (notify_tx, mut notify_rx) = mpsc::channel::<PathBuf>(64);
    42	
    43	    let dir_clone = devflow_dir.clone();
    44	    let mut watcher: RecommendedWatcher = notify::recommended_watcher(
    45	        move |res: std::result::Result<notify::Event, notify::Error>| {
    46	            if let Ok(event) = res
    47	                && matches!(
    48	                    event.kind,
    49	                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
    50	                )
    51	            {
    52	                for path in event.paths {
    53	                    if path.starts_with(&dir_clone)
    54	                        && notify_tx.try_send(path).is_err()
    55	                    {
    56	                        tracing::warn!("file_watcher: event channel full, some events may be coalesced");
    57	                    }
    58	                }
    59	            }
    60	        },
    61	    )?;
    62	
    63	    watcher.watch(&devflow_dir, RecursiveMode::Recursive)?;
    64	
    65	    // Debounce loop
    66	    let mut debounce_timer: Option<tokio::time::Instant> = None;
    67	
    68	    loop {
    69	        tokio::select! {
    70	            _ = cancel.cancelled() => {
    71	                tracing::info!("file_watcher: shutting down");
    72	                return Ok(());
    73	            }
    74	            Some(_path) = notify_rx.recv() => {
    75	                // Reset debounce timer on each event
    76	                debounce_timer = Some(tokio::time::Instant::now() + Duration::from_millis(DEBOUNCE_MS));
    77	            }
    78	            _ = async {
    79	                if let Some(deadline) = debounce_timer {
    80	                    tokio::time::sleep_until(deadline).await;
    81	                } else {
    82	                    // No timer set — wait forever (cancelled by other branches)
    83	                    std::future::pending::<()>().await;
    84	                }
    85	            } => {
    86	                debounce_timer = None;
    87	                parse_and_send(&devflow_dir, &flow_state_tx, &event_tx).await;
    88	            }
    89	        }
    90	    }
    91	}
    92	
    93	async fn parse_and_send(
    94	    devflow_dir: &Path,
    95	    flow_state_tx: &watch::Sender<FlowState>,
    96	    event_tx: &mpsc::Sender<AppEvent>,
    97	) {
    98	    // Parse devflow-state.md
    99	    let state_path = devflow_dir.join("devflow-state.md");
   100	    if state_path.exists() {
   101	        match tokio::fs::read_to_string(&state_path).await {
   102	            Ok(content) => {
   103	                let state = devflow_state::parse(&content);
   104	                let _ = flow_state_tx.send(state);
   105	            }
   106	            Err(e) => {
   107	                tracing::warn!("file_watcher: failed to read {}: {e}", state_path.display());
   108	                let _ = event_tx
   109	                    .send(AppEvent::ParseError {
   110	                        file: state_path.display().to_string(),
   111	                        error: e.to_string(),
   112	                    })
   113	                    .await;
   114	            }
   115	        }
   116	    }
   117	
   118	    // Parse session-summary.md
   119	    let summary_path = devflow_dir.join("session-summary.md");
   120	    if summary_path.exists() {
   121	        match tokio::fs::read_to_string(&summary_path).await {
   122	            Ok(content) => {
   123	                let summary = session_summary::parse(&content);
   124	                let _ = event_tx
   125	                    .send(AppEvent::SessionSummaryChanged(summary))
   126	                    .await;
   127	            }
   128	            Err(e) => {
   129	                tracing::warn!("file_watcher: failed to read {}: {e}", summary_path.display());
   130	            }
   131	        }
   132	    }
   133	
   134	    // Parse audit.md or devflow-audit.md
   135	    let audit_path = find_audit_file(devflow_dir);
   136	    if let Some(ref path) = audit_path {
   137	        match tokio::fs::read_to_string(path).await {
   138	            Ok(content) => {
   139	                let entries = audit_log::parse(&content);
   140	                let _ = event_tx.send(AppEvent::AuditLogAppended(entries)).await;
   141	            }
   142	            Err(e) => {
   143	                tracing::warn!("file_watcher: failed to read {}: {e}", path.display());
   144	            }
   145	        }
   146	    }
   147	
   148	    // Scan artifact files (sorted for deterministic UI)
   149	    let mut artifacts = scan_artifacts(devflow_dir).await;
   150	    artifacts.sort_by(|a, b| a.path.cmp(&b.path));
   151	    let _ = event_tx
   152	        .send(AppEvent::ArtifactListChanged(artifacts))
   153	        .await;
   154	}
   155	
   156	fn find_audit_file(devflow_dir: &Path) -> Option<PathBuf> {
   157	    let candidates = ["audit.md", "devflow-audit.md"];
   158	    for name in &candidates {
   159	        let path = devflow_dir.join(name);
   160	        if path.exists() {
   161	            return Some(path);
   162	        }
   163	    }
   164	    None
   165	}
   166	
   167	async fn scan_artifacts(devflow_dir: &Path) -> Vec<ArtifactFile> {
   168	    let mut artifacts = Vec::new();
   169	    for subdir in &["inception", "construction"] {
   170	        let dir = devflow_dir.join(subdir);
   171	        if dir.exists()
   172	            && let Ok(mut entries) = tokio::fs::read_dir(&dir).await
   173	        {
   174	            while let Ok(Some(entry)) = entries.next_entry().await {
   175	                let path = entry.path();
   176	                if path.extension().is_some_and(|ext| ext == "md") {
   177	                    let name = path
   178	                        .file_name()
   179	                        .map(|n| n.to_string_lossy().to_string())
   180	                        .unwrap_or_default();
   181	                    artifacts.push(ArtifactFile { path, name });
   182	                }
   183	            }
   184	        }
   185	    }
   186	    artifacts
   187	}
   188	
   189	// Note: Integration tests for file_watcher require tokio runtime + temp dirs.
   190	// These are better placed in tests/ directory or as #[tokio::test] with tempfile crate.
   191	// Unit-level tests cover the parsers (Unit 2). This module's correctness depends on
   192	// notify crate behavior which is tested via manual/integration testing.

exec
/bin/zsh -lc "nl -ba src/adapter/git_poller.rs | sed -n '1,360p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::path::PathBuf;
     2	use std::time::Duration;
     3	
     4	use tokio::process::Command;
     5	use tokio::sync::{mpsc, watch};
     6	use tokio_util::sync::CancellationToken;
     7	
     8	use crate::error::Result;
     9	use crate::event::AppEvent;
    10	use crate::parser::models::{DiffStat, GitChange, GitChangeStatus, GitCommit, GitSnapshot, GitWorktree};
    11	use crate::service::sanitizer::strip_ansi;
    12	
    13	const POLL_INTERVAL_SECS: u64 = 2;
    14	const GIT_TIMEOUT_SECS: u64 = 5;
    15	
    16	/// Run the git poller adapter.
    17	///
    18	/// Polls git status every 2 seconds via CLI subprocess.
    19	/// Sends GitSnapshot updates via watch channel.
    20	pub async fn run(
    21	    cancel: CancellationToken,
    22	    project_dir: PathBuf,
    23	    git_snapshot_tx: watch::Sender<GitSnapshot>,
    24	    event_tx: mpsc::Sender<AppEvent>,
    25	) -> Result<()> {
    26	    let mut interval = tokio::time::interval(Duration::from_secs(POLL_INTERVAL_SECS));
    27	
    28	    loop {
    29	        tokio::select! {
    30	            _ = cancel.cancelled() => {
    31	                tracing::info!("git_poller: shutting down");
    32	                return Ok(());
    33	            }
    34	            _ = interval.tick() => {
    35	                match poll_git(&project_dir).await {
    36	                    Ok(snapshot) => {
    37	                        let _ = git_snapshot_tx.send(snapshot);
    38	                    }
    39	                    Err(e) => {
    40	                        tracing::warn!("git_poller: {e}");
    41	                        let _ = event_tx.send(AppEvent::GitPollError {
    42	                            error: e.to_string(),
    43	                        }).await;
    44	                    }
    45	                }
    46	            }
    47	        }
    48	    }
    49	}
    50	
    51	async fn poll_git(dir: &PathBuf) -> Result<GitSnapshot> {
    52	    // Run essential commands, propagate first error for branch/head
    53	    let branch = git_cmd(dir, &["rev-parse", "--abbrev-ref", "HEAD"]).await;
    54	    let head = git_cmd(dir, &["rev-parse", "--short", "HEAD"]).await;
    55	
    56	    // If we can't even get branch/head, this isn't a git repo — propagate error
    57	    let branch = branch.map_err(|e| crate::error::AppError::GitCommand {
    58	        command: "git rev-parse --abbrev-ref HEAD".to_string(),
    59	        stderr: e,
    60	    })?;
    61	    let head = head.map_err(|e| crate::error::AppError::GitCommand {
    62	        command: "git rev-parse --short HEAD".to_string(),
    63	        stderr: e,
    64	    })?;
    65	
    66	    // Non-critical commands: log warnings on failure, use defaults
    67	    let status_out = git_cmd(dir, &["status", "--porcelain=v2"]).await.unwrap_or_else(|e| {
    68	        tracing::warn!("git status failed: {e}");
    69	        String::new()
    70	    });
    71	    let log_out = git_cmd(dir, &["log", "--oneline", "-n", "10"]).await.unwrap_or_else(|e| {
    72	        tracing::warn!("git log failed: {e}");
    73	        String::new()
    74	    });
    75	    let worktree_out = git_cmd(dir, &["worktree", "list", "--porcelain"]).await.unwrap_or_else(|e| {
    76	        tracing::warn!("git worktree list failed: {e}");
    77	        String::new()
    78	    });
    79	    let diff_out = git_cmd(dir, &["diff", "--stat"]).await.unwrap_or_else(|e| {
    80	        tracing::warn!("git diff --stat failed: {e}");
    81	        String::new()
    82	    });
    83	
    84	    Ok(GitSnapshot {
    85	        branch: strip_ansi(branch.trim()),
    86	        head: strip_ansi(head.trim()),
    87	        changes: parse_status_porcelain_v2(&status_out),
    88	        commits: parse_log_oneline(&log_out),
    89	        worktrees: parse_worktree_porcelain(&worktree_out),
    90	        diff_stat: parse_diff_stat(&diff_out),
    91	    })
    92	}
    93	
    94	async fn git_cmd(dir: &PathBuf, args: &[&str]) -> std::result::Result<String, String> {
    95	    let child = Command::new("git")
    96	        .args(args)
    97	        .current_dir(dir)
    98	        .kill_on_drop(true) // Prevent orphaned git processes
    99	        .stdout(std::process::Stdio::piped())
   100	        .stderr(std::process::Stdio::piped())
   101	        .spawn()
   102	        .map_err(|e| format!("git spawn error: {e}"))?;
   103	
   104	    let result = tokio::time::timeout(
   105	        Duration::from_secs(GIT_TIMEOUT_SECS),
   106	        child.wait_with_output(),
   107	    )
   108	    .await;
   109	
   110	    match result {
   111	        Ok(Ok(output)) => {
   112	            if output.status.success() {
   113	                Ok(strip_ansi(&String::from_utf8_lossy(&output.stdout)))
   114	            } else {
   115	                Err(strip_ansi(String::from_utf8_lossy(&output.stderr).trim()))
   116	            }
   117	        }
   118	        Ok(Err(e)) => Err(format!("git exec error: {e}")),
   119	        Err(_) => {
   120	            // Timeout — child is killed on drop
   121	            Err(format!("git command timed out: git {}", args.join(" ")))
   122	        }
   123	    }
   124	}
   125	
   126	// ── Parsers ──
   127	
   128	/// Parse `git status --porcelain=v2` output.
   129	pub fn parse_status_porcelain_v2(output: &str) -> Vec<GitChange> {
   130	    let mut changes = Vec::new();
   131	
   132	    for line in output.lines() {
   133	        let line = line.trim();
   134	        if line.is_empty() {
   135	            continue;
   136	        }
   137	
   138	        if let Some(rest) = line.strip_prefix("1 ") {
   139	            // Changed entry after "1 ": XY sub mH mI mW hH hI path
   140	            let parts: Vec<&str> = rest.splitn(8, ' ').collect();
   141	            if parts.len() >= 8 {
   142	                let xy = parts[0];
   143	                let path = parts[7].to_string();
   144	                let x = xy.as_bytes().first().copied().unwrap_or(b'.');
   145	                let y = xy.as_bytes().get(1).copied().unwrap_or(b'.');
   146	
   147	                if x != b'.' && x != b'?' {
   148	                    changes.push(GitChange {
   149	                        status: GitChangeStatus::Staged,
   150	                        path: path.clone(),
   151	                        additions: None,
   152	                        deletions: None,
   153	                    });
   154	                }
   155	                if y != b'.' && y != b'?' {
   156	                    changes.push(GitChange {
   157	                        status: GitChangeStatus::Unstaged,
   158	                        path,
   159	                        additions: None,
   160	                        deletions: None,
   161	                    });
   162	                }
   163	            }
   164	        } else if let Some(rest) = line.strip_prefix("2 ") {
   165	            // Renamed/copied after "2 ": XY sub mH mI mW hH hI Xscore path\torigPath
   166	            let parts: Vec<&str> = rest.splitn(9, ' ').collect();
   167	            if parts.len() >= 9 {
   168	                let path = parts[8].split('\t').next().unwrap_or("").to_string();
   169	                changes.push(GitChange {
   170	                    status: GitChangeStatus::Staged,
   171	                    path,
   172	                    additions: None,
   173	                    deletions: None,
   174	                });
   175	            }
   176	        } else if let Some(rest) = line.strip_prefix("u ") {
   177	            // Unmerged after "u ": XY sub m1 m2 m3 mW h1 h2 h3 path
   178	            let parts: Vec<&str> = rest.splitn(10, ' ').collect();
   179	            if parts.len() >= 10 {
   180	                changes.push(GitChange {
   181	                    status: GitChangeStatus::Conflict,
   182	                    path: parts[9].to_string(),
   183	                    additions: None,
   184	                    deletions: None,
   185	                });
   186	            }
   187	        } else if let Some(rest) = line.strip_prefix("? ") {
   188	            // Untracked
   189	            changes.push(GitChange {
   190	                status: GitChangeStatus::Untracked,
   191	                path: rest.to_string(),
   192	                additions: None,
   193	                deletions: None,
   194	            });
   195	        }
   196	    }
   197	
   198	    changes
   199	}
   200	
   201	/// Parse `git log --oneline -n 10` output.
   202	pub fn parse_log_oneline(output: &str) -> Vec<GitCommit> {
   203	    output
   204	        .lines()
   205	        .filter_map(|line| {
   206	            let line = line.trim();
   207	            if line.is_empty() {
   208	                return None;
   209	            }
   210	            let (hash, message) = line.split_once(' ').unwrap_or((line, ""));
   211	            Some(GitCommit {
   212	                hash: hash.to_string(),
   213	                message: message.to_string(),
   214	            })
   215	        })
   216	        .collect()
   217	}
   218	
   219	/// Parse `git worktree list --porcelain` output.
   220	pub fn parse_worktree_porcelain(output: &str) -> Vec<GitWorktree> {
   221	    let mut worktrees = Vec::new();
   222	    let mut current_path: Option<String> = None;
   223	    let mut current_branch: Option<String> = None;
   224	
   225	    for line in output.lines() {
   226	        if let Some(path) = line.strip_prefix("worktree ") {
   227	            // Flush previous
   228	            if let Some(p) = current_path.take() {
   229	                worktrees.push(GitWorktree {
   230	                    path: p,
   231	                    branch: current_branch.take(),
   232	                });
   233	            }
   234	            current_path = Some(path.to_string());
   235	            current_branch = None;
   236	        } else if let Some(branch) = line.strip_prefix("branch refs/heads/") {
   237	            current_branch = Some(branch.to_string());
   238	        } else if line.is_empty() {
   239	            // Blank line separates entries
   240	            if let Some(p) = current_path.take() {
   241	                worktrees.push(GitWorktree {
   242	                    path: p,
   243	                    branch: current_branch.take(),
   244	                });
   245	            }
   246	        }
   247	    }
   248	
   249	    // Flush last
   250	    if let Some(p) = current_path.take() {
   251	        worktrees.push(GitWorktree {
   252	            path: p,
   253	            branch: current_branch.take(),
   254	        });
   255	    }
   256	
   257	    worktrees
   258	}
   259	
   260	/// Parse `git diff --stat` output. Extract total additions/deletions from summary line.
   261	pub fn parse_diff_stat(output: &str) -> DiffStat {
   262	    let mut stat = DiffStat::default();
   263	
   264	    // Last line format: " N files changed, N insertions(+), N deletions(-)"
   265	    if let Some(last_line) = output.lines().last() {
   266	        for part in last_line.split(',') {
   267	            let part = part.trim();
   268	            if part.contains("insertion") {
   269	                if let Some(num) = part.split_whitespace().next() {
   270	                    stat.additions = num.parse().unwrap_or(0);
   271	                }
   272	            } else if part.contains("deletion")
   273	                && let Some(num) = part.split_whitespace().next()
   274	            {
   275	                stat.deletions = num.parse().unwrap_or(0);
   276	            }
   277	        }
   278	    }
   279	
   280	    stat
   281	}
   282	
   283	#[cfg(test)]
   284	mod tests {
   285	    use super::*;
   286	
   287	    #[test]
   288	    fn test_parse_status_porcelain_v2_staged_and_unstaged() {
   289	        // Real porcelain v2 format: 1 <XY> <sub> <mH> <mI> <mW> <hH> <hI> <path>
   290	        let output = "\
   291	1 M. N... 100644 100644 100644 abc1234567890123456789012345678901234567890 def4567890123456789012345678901234567890 src/main.rs
   292	1 .M N... 100644 100644 100644 abc1234567890123456789012345678901234567890 def4567890123456789012345678901234567890 src/lib.rs
   293	";
   294	        let changes = parse_status_porcelain_v2(output);
   295	        assert_eq!(changes.len(), 2);
   296	        assert_eq!(changes[0].status, GitChangeStatus::Staged);
   297	        assert_eq!(changes[0].path, "src/main.rs");
   298	        assert_eq!(changes[1].status, GitChangeStatus::Unstaged);
   299	        assert_eq!(changes[1].path, "src/lib.rs");
   300	    }
   301	
   302	    #[test]
   303	    fn test_parse_status_untracked() {
   304	        let output = "? new_file.rs\n? another.txt\n";
   305	        let changes = parse_status_porcelain_v2(output);
   306	        assert_eq!(changes.len(), 2);
   307	        assert_eq!(changes[0].status, GitChangeStatus::Untracked);
   308	        assert_eq!(changes[0].path, "new_file.rs");
   309	    }
   310	
   311	    #[test]
   312	    fn test_parse_status_conflict() {
   313	        // Real porcelain v2 unmerged: u <XY> <sub> <m1> <m2> <m3> <mW> <h1> <h2> <h3> <path>
   314	        let output = "\
   315	u UU N... 100644 100644 100644 100644 abc1234567890123456789012345678901234567890 def4567890123456789012345678901234567890 ghi4567890123456789012345678901234567890 src/conflict.rs
   316	";
   317	        let changes = parse_status_porcelain_v2(output);
   318	        assert_eq!(changes.len(), 1);
   319	        assert_eq!(changes[0].status, GitChangeStatus::Conflict);
   320	        assert_eq!(changes[0].path, "src/conflict.rs");
   321	    }
   322	
   323	    #[test]
   324	    fn test_parse_status_empty() {
   325	        let changes = parse_status_porcelain_v2("");
   326	        assert!(changes.is_empty());
   327	    }
   328	
   329	    #[test]
   330	    fn test_parse_log_oneline() {
   331	        let output = "a1b2c3d feat: add parser module\nd4e5f6a init: project setup\n";
   332	        let commits = parse_log_oneline(output);
   333	        assert_eq!(commits.len(), 2);
   334	        assert_eq!(commits[0].hash, "a1b2c3d");
   335	        assert_eq!(commits[0].message, "feat: add parser module");
   336	        assert_eq!(commits[1].hash, "d4e5f6a");
   337	    }
   338	
   339	    #[test]
   340	    fn test_parse_log_empty() {
   341	        let commits = parse_log_oneline("");
   342	        assert!(commits.is_empty());
   343	    }
   344	
   345	    #[test]
   346	    fn test_parse_worktree_porcelain() {
   347	        let output = "worktree /Users/jay/project\nHEAD abc123\nbranch refs/heads/main\n\n\
   348	                       worktree /Users/jay/project-feat\nHEAD def456\nbranch refs/heads/feature/tui\n\n";
   349	        let wts = parse_worktree_porcelain(output);
   350	        assert_eq!(wts.len(), 2);
   351	        assert_eq!(wts[0].path, "/Users/jay/project");
   352	        assert_eq!(wts[0].branch.as_deref(), Some("main"));
   353	        assert_eq!(wts[1].branch.as_deref(), Some("feature/tui"));
   354	    }
   355	
   356	    #[test]
   357	    fn test_parse_worktree_empty() {
   358	        let wts = parse_worktree_porcelain("");
   359	        assert!(wts.is_empty());
   360	    }

exec
/bin/zsh -lc "nl -ba src/adapter/git_poller.rs | sed -n '360,520p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
   360	    }
   361	
   362	    #[test]
   363	    fn test_parse_diff_stat() {
   364	        let output = " src/main.rs | 42 +++++++-\n src/lib.rs  | 18 +++\n 2 files changed, 57 insertions(+), 3 deletions(-)\n";
   365	        let stat = parse_diff_stat(output);
   366	        assert_eq!(stat.additions, 57);
   367	        assert_eq!(stat.deletions, 3);
   368	    }
   369	
   370	    #[test]
   371	    fn test_parse_diff_stat_empty() {
   372	        let stat = parse_diff_stat("");
   373	        assert_eq!(stat.additions, 0);
   374	        assert_eq!(stat.deletions, 0);
   375	    }
   376	
   377	    #[test]
   378	    fn test_parse_diff_stat_insertions_only() {
   379	        let output = " 1 file changed, 10 insertions(+)\n";
   380	        let stat = parse_diff_stat(output);
   381	        assert_eq!(stat.additions, 10);
   382	        assert_eq!(stat.deletions, 0);
   383	    }
   384	}

exec
/bin/zsh -lc "nl -ba src/adapter/hooks_server.rs | sed -n '1,420p'" in /Users/jay.ahn/projects/backend/devflow-tui
exec
/bin/zsh -lc "nl -ba src/config.rs | sed -n '1,340p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::collections::HashMap;
     2	use std::net::SocketAddr;
     3	use std::sync::Arc;
     4	
     5	use axum::{
     6	    Json,
     7	    extract::{DefaultBodyLimit, Query},
     8	    http::StatusCode,
     9	    response::IntoResponse,
    10	    routing::post,
    11	};
    12	use tokio::sync::mpsc;
    13	use tokio_util::sync::CancellationToken;
    14	
    15	use crate::error::Result;
    16	use crate::event::AppEvent;
    17	use crate::service::sanitizer::strip_ansi;
    18	
    19	const PORT_RANGE_START: u16 = 9100;
    20	const PORT_RANGE_END: u16 = 9110;
    21	
    22	/// Shared state for the hooks HTTP server.
    23	struct HooksState {
    24	    token: String,
    25	    event_tx: mpsc::Sender<AppEvent>,
    26	}
    27	
    28	/// Run the hooks HTTP server.
    29	///
    30	/// Tries to bind to the given port, falling back to 9100-9110 range.
    31	/// Returns the actual bound port via AppEvent::HooksServerStarted.
    32	pub async fn run(
    33	    cancel: CancellationToken,
    34	    preferred_port: u16,
    35	    token: String,
    36	    event_tx: mpsc::Sender<AppEvent>,
    37	) -> Result<()> {
    38	    let state = Arc::new(HooksState {
    39	        token,
    40	        event_tx: event_tx.clone(),
    41	    });
    42	
    43	    let app = axum::Router::new()
    44	        .route("/hook", post(handle_hook))
    45	        .layer(DefaultBodyLimit::max(64 * 1024)) // 64KB max request body
    46	        .with_state(state);
    47	
    48	    // Try preferred port first, then range
    49	    let mut ports_to_try = Vec::with_capacity(12);
    50	    ports_to_try.push(preferred_port);
    51	    for p in PORT_RANGE_START..=PORT_RANGE_END {
    52	        if p != preferred_port {
    53	            ports_to_try.push(p);
    54	        }
    55	    }
    56	
    57	    for port in ports_to_try {
    58	        let addr = SocketAddr::from(([127, 0, 0, 1], port));
    59	        match tokio::net::TcpListener::bind(addr).await {
    60	            Ok(listener) => {
    61	                tracing::info!("hooks_server: listening on {addr}");
    62	                let _ = event_tx
    63	                    .send(AppEvent::HooksServerStarted { port })
    64	                    .await;
    65	
    66	                // Run server with graceful shutdown
    67	                let server = axum::serve(listener, app);
    68	                tokio::select! {
    69	                    _ = cancel.cancelled() => {
    70	                        tracing::info!("hooks_server: shutting down");
    71	                    }
    72	                    result = server => {
    73	                        if let Err(e) = result {
    74	                            tracing::error!("hooks_server error: {e}");
    75	                        }
    76	                    }
    77	                }
    78	
    79	                return Ok(());
    80	            }
    81	            Err(e) => {
    82	                tracing::debug!("hooks_server: port {port} unavailable: {e}");
    83	            }
    84	        }
    85	    }
    86	
    87	    // All ports failed
    88	    let reason = format!("all ports {PORT_RANGE_START}-{PORT_RANGE_END} unavailable");
    89	    tracing::warn!("hooks_server: {reason}");
    90	    let _ = event_tx
    91	        .send(AppEvent::HooksServerFailed { reason })
    92	        .await;
    93	
    94	    Ok(())
    95	}
    96	
    97	/// Hook payload from Claude Code.
    98	#[derive(Debug, serde::Deserialize)]
    99	struct HookPayload {
   100	    hook_event_name: Option<String>,
   101	    agent_id: Option<String>,
   102	    agent_type: Option<String>,
   103	    tool_name: Option<String>,
   104	    last_assistant_message: Option<String>,
   105	}
   106	
   107	async fn handle_hook(
   108	    axum::extract::State(state): axum::extract::State<Arc<HooksState>>,
   109	    Query(params): Query<HashMap<String, String>>,
   110	    Json(payload): Json<serde_json::Value>,
   111	) -> impl IntoResponse {
   112	    // Token validation
   113	    let provided_token = params.get("token").map(|s| s.as_str()).unwrap_or("");
   114	    if !crate::service::token::validate_token(&state.token, provided_token) {
   115	        return StatusCode::FORBIDDEN;
   116	    }
   117	
   118	    // Parse payload
   119	    let hook: HookPayload = match serde_json::from_value(payload) {
   120	        Ok(h) => h,
   121	        Err(e) => {
   122	            tracing::warn!("hooks_server: invalid payload: {e}");
   123	            return StatusCode::BAD_REQUEST;
   124	        }
   125	    };
   126	
   127	    // Convert to AppEvent
   128	    let event_name = hook.hook_event_name.as_deref().unwrap_or("");
   129	    let event = match event_name {
   130	        "SubagentStart" => hook.agent_id.map(|id| AppEvent::AgentStarted {
   131	            agent_id: strip_ansi(&id),
   132	            agent_type: hook.agent_type.map(|t| strip_ansi(&t)).unwrap_or_default(),
   133	        }),
   134	        "SubagentStop" => hook.agent_id.map(|id| AppEvent::AgentStopped {
   135	            agent_id: strip_ansi(&id),
   136	        }),
   137	        "PreToolUse" => hook.tool_name.map(|name| AppEvent::ToolUseStarted {
   138	            tool_name: strip_ansi(&name),
   139	        }),
   140	        "PostToolUse" => hook.tool_name.map(|name| AppEvent::ToolUseCompleted {
   141	            tool_name: strip_ansi(&name),
   142	        }),
   143	        "Stop" => Some(AppEvent::TurnCompleted {
   144	            last_message: hook
   145	                .last_assistant_message
   146	                .map(|m| strip_ansi(&m))
   147	                .unwrap_or_default(),
   148	        }),
   149	        _ => {
   150	            tracing::debug!("hooks_server: unknown event: {event_name}");
   151	            None
   152	        }
   153	    };
   154	
   155	    if let Some(ev) = event {
   156	        let _ = state.event_tx.send(ev).await;
   157	    }
   158	
   159	    StatusCode::OK
   160	}
   161	
   162	#[cfg(test)]
   163	mod tests {
   164	    use super::*;
   165	    use axum::body::Body;
   166	    use axum::http::Request;
   167	    use tower::ServiceExt;
   168	
   169	    fn test_app(token: &str) -> (axum::Router, mpsc::Receiver<AppEvent>) {
   170	        let (tx, rx) = mpsc::channel(16);
   171	        let state = Arc::new(HooksState {
   172	            token: token.to_string(),
   173	            event_tx: tx,
   174	        });
   175	        let app = axum::Router::new()
   176	            .route("/hook", post(handle_hook))
   177	            .with_state(state);
   178	        (app, rx)
   179	    }
   180	
   181	    #[tokio::test]
   182	    async fn test_hook_valid_token_agent_started() {
   183	        let (app, mut rx) = test_app("secret123");
   184	
   185	        let body = serde_json::json!({
   186	            "hook_event_name": "SubagentStart",
   187	            "agent_id": "agent-abc",
   188	            "agent_type": "Explore"
   189	        });
   190	
   191	        let response = app
   192	            .oneshot(
   193	                Request::builder()
   194	                    .method("POST")
   195	                    .uri("/hook?token=secret123")
   196	                    .header("content-type", "application/json")
   197	                    .body(Body::from(serde_json::to_string(&body).unwrap()))
   198	                    .unwrap(),
   199	            )
   200	            .await
   201	            .unwrap();
   202	
   203	        assert_eq!(response.status(), StatusCode::OK);
   204	
   205	        let event = rx.try_recv().unwrap();
   206	        match event {
   207	            AppEvent::AgentStarted { agent_id, agent_type } => {
   208	                assert_eq!(agent_id, "agent-abc");
   209	                assert_eq!(agent_type, "Explore");
   210	            }
   211	            other => panic!("unexpected event: {other:?}"),
   212	        }
   213	    }
   214	
   215	    #[tokio::test]
   216	    async fn test_hook_invalid_token_403() {
   217	        let (app, _rx) = test_app("secret123");
   218	
   219	        let body = serde_json::json!({"hook_event_name": "Stop"});
   220	
   221	        let response = app
   222	            .oneshot(
   223	                Request::builder()
   224	                    .method("POST")
   225	                    .uri("/hook?token=wrongtoken")
   226	                    .header("content-type", "application/json")
   227	                    .body(Body::from(serde_json::to_string(&body).unwrap()))
   228	                    .unwrap(),
   229	            )
   230	            .await
   231	            .unwrap();
   232	
   233	        assert_eq!(response.status(), StatusCode::FORBIDDEN);
   234	    }
   235	
   236	    #[tokio::test]
   237	    async fn test_hook_missing_token_403() {
   238	        let (app, _rx) = test_app("secret123");
   239	
   240	        let body = serde_json::json!({"hook_event_name": "Stop"});
   241	
   242	        let response = app
   243	            .oneshot(
   244	                Request::builder()
   245	                    .method("POST")
   246	                    .uri("/hook")
   247	                    .header("content-type", "application/json")
   248	                    .body(Body::from(serde_json::to_string(&body).unwrap()))
   249	                    .unwrap(),
   250	            )
   251	            .await
   252	            .unwrap();
   253	
   254	        assert_eq!(response.status(), StatusCode::FORBIDDEN);
   255	    }
   256	
   257	    #[tokio::test]
   258	    async fn test_hook_stop_event() {
   259	        let (app, mut rx) = test_app("tok");
   260	
   261	        let body = serde_json::json!({
   262	            "hook_event_name": "Stop",
   263	            "last_assistant_message": "A) option\nB) option"
   264	        });
   265	
   266	        let response = app
   267	            .oneshot(
   268	                Request::builder()
   269	                    .method("POST")
   270	                    .uri("/hook?token=tok")
   271	                    .header("content-type", "application/json")
   272	                    .body(Body::from(serde_json::to_string(&body).unwrap()))
   273	                    .unwrap(),
   274	            )
   275	            .await
   276	            .unwrap();
   277	
   278	        assert_eq!(response.status(), StatusCode::OK);
   279	
   280	        let event = rx.try_recv().unwrap();
   281	        match event {
   282	            AppEvent::TurnCompleted { last_message } => {
   283	                assert!(last_message.contains("A) option"));
   284	            }
   285	            other => panic!("unexpected event: {other:?}"),
   286	        }
   287	    }
   288	
   289	    #[tokio::test]
   290	    async fn test_hook_ansi_sanitized() {
   291	        let (app, mut rx) = test_app("tok");
   292	
   293	        let body = serde_json::json!({
   294	            "hook_event_name": "SubagentStart",
   295	            "agent_id": "\x1b[31magent-x\x1b[0m",
   296	            "agent_type": "Plan"
   297	        });
   298	
   299	        let response = app
   300	            .oneshot(
   301	                Request::builder()
   302	                    .method("POST")
   303	                    .uri("/hook?token=tok")
   304	                    .header("content-type", "application/json")
   305	                    .body(Body::from(serde_json::to_string(&body).unwrap()))
   306	                    .unwrap(),
   307	            )
   308	            .await
   309	            .unwrap();
   310	
   311	        assert_eq!(response.status(), StatusCode::OK);
   312	
   313	        let event = rx.try_recv().unwrap();
   314	        match event {
   315	            AppEvent::AgentStarted { agent_id, .. } => {
   316	                assert_eq!(agent_id, "agent-x"); // ANSI stripped
   317	            }
   318	            other => panic!("unexpected event: {other:?}"),
   319	        }
   320	    }
   321	}

 succeeded in 0ms:
     1	use std::path::PathBuf;
     2	
     3	use crate::error::{AppError, Result};
     4	
     5	const DEFAULT_PORT: u16 = 9100;
     6	const DEFAULT_LOG_BUFFER: usize = 1000;
     7	
     8	#[derive(Debug, Clone)]
     9	pub struct AppConfig {
    10	    pub port: u16,
    11	    pub project_dir: PathBuf,
    12	    pub demo: bool,
    13	    pub regenerate_token: bool,
    14	    pub log_level: String,
    15	    pub log_buffer_size: usize,
    16	}
    17	
    18	impl Default for AppConfig {
    19	    fn default() -> Self {
    20	        Self {
    21	            port: DEFAULT_PORT,
    22	            project_dir: PathBuf::from("."),
    23	            demo: false,
    24	            regenerate_token: false,
    25	            log_level: "info".to_string(),
    26	            log_buffer_size: DEFAULT_LOG_BUFFER,
    27	        }
    28	    }
    29	}
    30	
    31	impl AppConfig {
    32	    pub fn from_args(args: &[String]) -> Result<Self> {
    33	        let mut config = Self::default();
    34	
    35	        // Apply environment variable overrides first
    36	        if let Ok(port) = std::env::var("DEVFLOW_TUI_PORT") {
    37	            config.port = port.parse::<u16>().map_err(|_| AppError::ConfigRead {
    38	                path: PathBuf::from("DEVFLOW_TUI_PORT"),
    39	                detail: format!("invalid port: {port}"),
    40	            })?;
    41	        }
    42	        if let Ok(log) = std::env::var("DEVFLOW_TUI_LOG") {
    43	            config.log_level = log;
    44	        }
    45	        if let Ok(buf) = std::env::var("DEVFLOW_TUI_LOG_BUFFER") {
    46	            config.log_buffer_size = buf.parse::<usize>().map_err(|_| AppError::ConfigRead {
    47	                path: PathBuf::from("DEVFLOW_TUI_LOG_BUFFER"),
    48	                detail: format!("invalid buffer size: {buf}"),
    49	            })?;
    50	        }
    51	
    52	        // Parse CLI args (override env vars)
    53	        let mut i = 1; // skip binary name
    54	        while i < args.len() {
    55	            match args[i].as_str() {
    56	                "--port" => {
    57	                    i += 1;
    58	                    let val = args.get(i).ok_or_else(|| AppError::ConfigRead {
    59	                        path: PathBuf::from("--port"),
    60	                        detail: "missing value".to_string(),
    61	                    })?;
    62	                    config.port = val.parse::<u16>().map_err(|_| AppError::ConfigRead {
    63	                        path: PathBuf::from("--port"),
    64	                        detail: format!("invalid port: {val}"),
    65	                    })?;
    66	                }
    67	                "--project-dir" => {
    68	                    i += 1;
    69	                    let val = args.get(i).ok_or_else(|| AppError::ConfigRead {
    70	                        path: PathBuf::from("--project-dir"),
    71	                        detail: "missing value".to_string(),
    72	                    })?;
    73	                    config.project_dir = PathBuf::from(val);
    74	                }
    75	                "--demo" => {
    76	                    config.demo = true;
    77	                }
    78	                "--regenerate-token" => {
    79	                    config.regenerate_token = true;
    80	                }
    81	                _ => {
    82	                    // Ignore unknown args
    83	                }
    84	            }
    85	            i += 1;
    86	        }
    87	
    88	        config.validate()?;
    89	        Ok(config)
    90	    }
    91	
    92	    fn validate(&self) -> Result<()> {
    93	        if self.port == 0 || self.port < 1024 {
    94	            return Err(AppError::ConfigRead {
    95	                path: PathBuf::from("--port"),
    96	                detail: format!("port must be 1024-65535, got {}", self.port),
    97	            });
    98	        }
    99	        if self.log_buffer_size == 0 {
   100	            return Err(AppError::ConfigRead {
   101	                path: PathBuf::from("DEVFLOW_TUI_LOG_BUFFER"),
   102	                detail: "log buffer size must be > 0".to_string(),
   103	            });
   104	        }
   105	        Ok(())
   106	    }
   107	
   108	    pub fn devflow_docs_dir(&self) -> PathBuf {
   109	        self.project_dir.join("devflow-docs")
   110	    }
   111	}
   112	
   113	#[cfg(test)]
   114	mod tests {
   115	    use super::*;
   116	
   117	    #[test]
   118	    fn test_config_defaults() {
   119	        let config = AppConfig::default();
   120	        assert_eq!(config.port, 9100);
   121	        assert_eq!(config.project_dir, PathBuf::from("."));
   122	        assert!(!config.demo);
   123	        assert!(!config.regenerate_token);
   124	        assert_eq!(config.log_level, "info");
   125	        assert_eq!(config.log_buffer_size, 1000);
   126	    }
   127	
   128	    #[test]
   129	    fn test_config_cli_override() {
   130	        let args = vec![
   131	            "devflow-tui".to_string(),
   132	            "--port".to_string(),
   133	            "9200".to_string(),
   134	            "--project-dir".to_string(),
   135	            "/tmp/proj".to_string(),
   136	            "--demo".to_string(),
   137	        ];
   138	        let config = AppConfig::from_args(&args).unwrap();
   139	        assert_eq!(config.port, 9200);
   140	        assert_eq!(config.project_dir, PathBuf::from("/tmp/proj"));
   141	        assert!(config.demo);
   142	    }
   143	
   144	    #[test]
   145	    fn test_config_missing_port_value() {
   146	        let args = vec!["devflow-tui".to_string(), "--port".to_string()];
   147	        let result = AppConfig::from_args(&args);
   148	        assert!(result.is_err());
   149	    }
   150	
   151	    #[test]
   152	    fn test_config_invalid_port() {
   153	        let args = vec![
   154	            "devflow-tui".to_string(),
   155	            "--port".to_string(),
   156	            "not_a_number".to_string(),
   157	        ];
   158	        let result = AppConfig::from_args(&args);
   159	        assert!(result.is_err());
   160	    }
   161	
   162	    #[test]
   163	    fn test_config_devflow_docs_dir() {
   164	        let mut config = AppConfig::default();
   165	        config.project_dir = PathBuf::from("/home/user/project");
   166	        assert_eq!(
   167	            config.devflow_docs_dir(),
   168	            PathBuf::from("/home/user/project/devflow-docs")
   169	        );
   170	    }
   171	
   172	    #[test]
   173	    fn test_config_regenerate_token() {
   174	        let args = vec![
   175	            "devflow-tui".to_string(),
   176	            "--regenerate-token".to_string(),
   177	        ];
   178	        let config = AppConfig::from_args(&args).unwrap();
   179	        assert!(config.regenerate_token);
   180	    }
   181	
   182	    #[test]
   183	    fn test_config_port_zero_rejected() {
   184	        let args = vec![
   185	            "devflow-tui".to_string(),
   186	            "--port".to_string(),
   187	            "0".to_string(),
   188	        ];
   189	        let result = AppConfig::from_args(&args);
   190	        assert!(result.is_err());
   191	    }
   192	
   193	    #[test]
   194	    fn test_config_port_low_rejected() {
   195	        let args = vec![
   196	            "devflow-tui".to_string(),
   197	            "--port".to_string(),
   198	            "80".to_string(),
   199	        ];
   200	        let result = AppConfig::from_args(&args);
   201	        assert!(result.is_err());
   202	    }
   203	
   204	    #[test]
   205	    fn test_config_unknown_args_ignored() {
   206	        let args = vec![
   207	            "devflow-tui".to_string(),
   208	            "--unknown".to_string(),
   209	            "value".to_string(),
   210	        ];
   211	        let config = AppConfig::from_args(&args).unwrap();
   212	        assert_eq!(config.port, 9100); // default preserved
   213	    }
   214	}

exec
/bin/zsh -lc "nl -ba src/parser/models.rs | sed -n '1,360p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::collections::HashMap;
     2	use std::fmt;
     3	use std::path::PathBuf;
     4	use std::str::FromStr;
     5	
     6	// ── Phase & Complexity ──
     7	
     8	#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
     9	pub enum Phase {
    10	    #[default]
    11	    Inception,
    12	    Construction,
    13	    Complete,
    14	    Finished,
    15	}
    16	
    17	impl fmt::Display for Phase {
    18	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    19	        match self {
    20	            Self::Inception => write!(f, "INCEPTION"),
    21	            Self::Construction => write!(f, "CONSTRUCTION"),
    22	            Self::Complete => write!(f, "complete"),
    23	            Self::Finished => write!(f, "finished"),
    24	        }
    25	    }
    26	}
    27	
    28	/// Error returned when parsing an unknown phase string.
    29	#[derive(Debug, Clone, PartialEq, Eq)]
    30	pub struct ParsePhaseError(pub String);
    31	
    32	impl fmt::Display for ParsePhaseError {
    33	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    34	        write!(f, "unknown phase: '{}'", self.0)
    35	    }
    36	}
    37	
    38	impl FromStr for Phase {
    39	    type Err = ParsePhaseError;
    40	
    41	    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    42	        match s.trim().to_uppercase().as_str() {
    43	            "INCEPTION" => Ok(Self::Inception),
    44	            "CONSTRUCTION" => Ok(Self::Construction),
    45	            "COMPLETE" => Ok(Self::Complete),
    46	            "FINISHED" => Ok(Self::Finished),
    47	            _ => Err(ParsePhaseError(s.trim().to_string())),
    48	        }
    49	    }
    50	}
    51	
    52	#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    53	pub enum Complexity {
    54	    Minimal,
    55	    #[default]
    56	    Standard,
    57	    Comprehensive,
    58	}
    59	
    60	impl fmt::Display for Complexity {
    61	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    62	        match self {
    63	            Self::Minimal => write!(f, "Minimal"),
    64	            Self::Standard => write!(f, "Standard"),
    65	            Self::Comprehensive => write!(f, "Comprehensive"),
    66	        }
    67	    }
    68	}
    69	
    70	/// Error returned when parsing an unknown complexity string.
    71	#[derive(Debug, Clone, PartialEq, Eq)]
    72	pub struct ParseComplexityError(pub String);
    73	
    74	impl fmt::Display for ParseComplexityError {
    75	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    76	        write!(f, "unknown complexity: '{}'", self.0)
    77	    }
    78	}
    79	
    80	impl FromStr for Complexity {
    81	    type Err = ParseComplexityError;
    82	
    83	    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    84	        match s.trim().to_lowercase().as_str() {
    85	            "minimal" => Ok(Self::Minimal),
    86	            "standard" => Ok(Self::Standard),
    87	            "comprehensive" => Ok(Self::Comprehensive),
    88	            _ => Err(ParseComplexityError(s.trim().to_string())),
    89	        }
    90	    }
    91	}
    92	
    93	// ── Stage Status ──
    94	
    95	#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    96	pub enum StageStatus {
    97	    Active,
    98	    Completed,
    99	    #[default]
   100	    Waiting,
   101	    Skipped,
   102	}
   103	
   104	// ── Work Marker ──
   105	
   106	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
   107	pub enum WorkMarker {
   108	    Done,       // [x]
   109	    InProgress, // [~]
   110	    Pending,    // [ ]
   111	}
   112	
   113	impl WorkMarker {
   114	    pub fn icon(&self) -> &'static str {
   115	        match self {
   116	            Self::Done => "✓",
   117	            Self::InProgress => "◐",
   118	            Self::Pending => "○",
   119	        }
   120	    }
   121	}
   122	
   123	// ── FlowState ──
   124	
   125	#[derive(Debug, Clone, Default)]
   126	pub struct CompletedStage {
   127	    pub name: String,
   128	    pub timestamp: Option<String>,
   129	}
   130	
   131	#[derive(Debug, Clone, Default)]
   132	pub struct ApprovedStage {
   133	    pub name: String,
   134	    pub depth: Option<String>,
   135	}
   136	
   137	#[derive(Debug, Clone, Default)]
   138	pub struct SkippedStage {
   139	    pub name: String,
   140	    pub reason: Option<String>,
   141	}
   142	
   143	#[derive(Debug, Clone, Default)]
   144	pub struct WorktreeInfo {
   145	    pub branch: Option<String>,
   146	    pub path: Option<String>,
   147	}
   148	
   149	#[derive(Debug, Clone, Default)]
   150	pub struct FlowState {
   151	    pub phase: Phase,
   152	    pub stage: String,
   153	    pub complexity: Complexity,
   154	    pub selected_approach: Option<String>,
   155	    pub completed_stages: Vec<CompletedStage>,
   156	    pub approved_stages: Vec<ApprovedStage>,
   157	    pub skipped_stages: Vec<SkippedStage>,
   158	    pub active_unit: Option<String>,
   159	    pub completed_units: Vec<String>,
   160	    pub worktree: WorktreeInfo,
   161	    pub extra_fields: HashMap<String, String>,
   162	}
   163	
   164	// ── SessionSummary ──
   165	
   166	#[derive(Debug, Clone)]
   167	pub struct WorkItem {
   168	    pub text: String,
   169	    pub marker: WorkMarker,
   170	}
   171	
   172	#[derive(Debug, Clone, Default)]
   173	pub struct SessionSummary {
   174	    pub last_updated: Option<String>,
   175	    pub commit: Option<String>,
   176	    pub phase: Option<Phase>,
   177	    pub stage: Option<String>,
   178	    pub complexity: Option<Complexity>,
   179	    pub key_decisions: Vec<String>,
   180	    pub completed_work: Vec<WorkItem>,
   181	    pub next_steps: Vec<String>,
   182	    pub for_next_session: Vec<String>,
   183	}
   184	
   185	// ── AuditEntry ──
   186	
   187	#[derive(Debug, Clone)]
   188	pub struct AuditEntry {
   189	    pub timestamp: Option<String>,
   190	    pub stage: Option<String>,
   191	    pub choice: Option<String>,
   192	    pub raw_line: String,
   193	}
   194	
   195	// ── ArtifactFile ──
   196	
   197	#[derive(Debug, Clone)]
   198	pub struct ArtifactFile {
   199	    pub path: PathBuf,
   200	    pub name: String,
   201	}
   202	
   203	// ── Git Models ──
   204	
   205	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
   206	pub enum GitChangeStatus {
   207	    Staged,
   208	    Unstaged,
   209	    Untracked,
   210	    Conflict,
   211	}
   212	
   213	#[derive(Debug, Clone)]
   214	pub struct GitChange {
   215	    pub status: GitChangeStatus,
   216	    pub path: String,
   217	    pub additions: Option<u32>,
   218	    pub deletions: Option<u32>,
   219	}
   220	
   221	#[derive(Debug, Clone)]
   222	pub struct GitCommit {
   223	    pub hash: String,
   224	    pub message: String,
   225	}
   226	
   227	#[derive(Debug, Clone)]
   228	pub struct GitWorktree {
   229	    pub path: String,
   230	    pub branch: Option<String>,
   231	}
   232	
   233	#[derive(Debug, Clone, Default)]
   234	pub struct DiffStat {
   235	    pub additions: u32,
   236	    pub deletions: u32,
   237	}
   238	
   239	#[derive(Debug, Clone, Default)]
   240	pub struct GitSnapshot {
   241	    pub branch: String,
   242	    pub head: String,
   243	    pub changes: Vec<GitChange>,
   244	    pub commits: Vec<GitCommit>,
   245	    pub worktrees: Vec<GitWorktree>,
   246	    pub diff_stat: DiffStat,
   247	}
   248	
   249	#[cfg(test)]
   250	mod tests {
   251	    use super::*;
   252	
   253	    #[test]
   254	    fn test_flow_state_default() {
   255	        let state = FlowState::default();
   256	        assert_eq!(state.phase, Phase::Inception);
   257	        assert!(state.stage.is_empty());
   258	        assert_eq!(state.complexity, Complexity::Standard);
   259	        assert!(state.completed_stages.is_empty());
   260	        assert!(state.extra_fields.is_empty());
   261	    }
   262	
   263	    #[test]
   264	    fn test_phase_display() {
   265	        assert_eq!(Phase::Inception.to_string(), "INCEPTION");
   266	        assert_eq!(Phase::Construction.to_string(), "CONSTRUCTION");
   267	        assert_eq!(Phase::Complete.to_string(), "complete");
   268	        assert_eq!(Phase::Finished.to_string(), "finished");
   269	    }
   270	
   271	    #[test]
   272	    fn test_phase_from_str() {
   273	        assert_eq!("INCEPTION".parse::<Phase>(), Ok(Phase::Inception));
   274	        assert_eq!("construction".parse::<Phase>(), Ok(Phase::Construction));
   275	        assert_eq!("Complete".parse::<Phase>(), Ok(Phase::Complete));
   276	        assert!("unknown".parse::<Phase>().is_err());
   277	    }
   278	
   279	    #[test]
   280	    fn test_complexity_display() {
   281	        assert_eq!(Complexity::Minimal.to_string(), "Minimal");
   282	        assert_eq!(Complexity::Standard.to_string(), "Standard");
   283	        assert_eq!(Complexity::Comprehensive.to_string(), "Comprehensive");
   284	    }
   285	
   286	    #[test]
   287	    fn test_complexity_from_str() {
   288	        assert_eq!("minimal".parse::<Complexity>(), Ok(Complexity::Minimal));
   289	        assert_eq!("Standard".parse::<Complexity>(), Ok(Complexity::Standard));
   290	        assert_eq!("COMPREHENSIVE".parse::<Complexity>(), Ok(Complexity::Comprehensive));
   291	        assert!("unknown".parse::<Complexity>().is_err());
   292	    }
   293	
   294	    #[test]
   295	    fn test_git_snapshot_default() {
   296	        let snap = GitSnapshot::default();
   297	        assert!(snap.branch.is_empty());
   298	        assert!(snap.changes.is_empty());
   299	        assert_eq!(snap.diff_stat.additions, 0);
   300	    }
   301	}

exec
/bin/zsh -lc "nl -ba src/service/hook_config.rs | sed -n '1,360p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::path::{Path, PathBuf};
     2	
     3	/// Result of checking Claude Code hooks configuration.
     4	#[derive(Debug, Clone, PartialEq, Eq)]
     5	pub enum HookConfigStatus {
     6	    /// Hooks are configured and endpoint matches.
     7	    Configured,
     8	    /// Hooks exist but endpoint doesn't match TUI port.
     9	    EndpointMismatch { configured_url: String },
    10	    /// No hooks configuration found.
    11	    NotConfigured,
    12	}
    13	
    14	/// Check if Claude Code hooks are configured for devflow-tui.
    15	///
    16	/// Checks two locations:
    17	/// 1. Project-level: .claude/settings.json
    18	/// 2. User-level: ~/.claude/settings.json
    19	pub fn check_hooks_config(project_dir: &Path, expected_port: u16, token: &str) -> HookConfigStatus {
    20	    let project_settings = project_dir.join(".claude").join("settings.json");
    21	    let user_settings = dirs::home_dir()
    22	        .map(|h| h.join(".claude").join("settings.json"))
    23	        .unwrap_or_else(|| PathBuf::from(""));
    24	
    25	    for path in &[project_settings, user_settings] {
    26	        if let Ok(content) = std::fs::read_to_string(path)
    27	            && let Some(status) = check_content(&content, expected_port, token)
    28	        {
    29	            return status;
    30	        }
    31	    }
    32	
    33	    HookConfigStatus::NotConfigured
    34	}
    35	
    36	fn check_content(content: &str, expected_port: u16, token: &str) -> Option<HookConfigStatus> {
    37	    // Check for matching port + token
    38	    let expected_url_base = format!("localhost:{expected_port}/hook?token={token}");
    39	    let expected_url_127 = format!("127.0.0.1:{expected_port}/hook?token={token}");
    40	
    41	    if content.contains(&expected_url_base) || content.contains(&expected_url_127) {
    42	        return Some(HookConfigStatus::Configured);
    43	    }
    44	
    45	    // Check port match but token mismatch
    46	    let port_base = format!("localhost:{expected_port}");
    47	    let port_127 = format!("127.0.0.1:{expected_port}");
    48	
    49	    if content.contains(&port_base) || content.contains(&port_127) {
    50	        return Some(HookConfigStatus::EndpointMismatch {
    51	            configured_url: format!("port {expected_port} matched but token mismatch"),
    52	        });
    53	    }
    54	
    55	    // Check any hooks URL with wrong port
    56	    if content.contains("localhost:") || content.contains("127.0.0.1:") {
    57	        for line in content.lines() {
    58	            let trimmed = line.trim();
    59	            if (trimmed.contains("localhost:") || trimmed.contains("127.0.0.1:"))
    60	                && trimmed.contains("http")
    61	            {
    62	                return Some(HookConfigStatus::EndpointMismatch {
    63	                    configured_url: trimmed.trim_matches('"').trim_matches(',').to_string(),
    64	                });
    65	            }
    66	        }
    67	    }
    68	
    69	    None
    70	}
    71	
    72	/// Generate a hooks configuration JSON snippet for the user to add to settings.json.
    73	pub fn generate_hooks_snippet(port: u16, token: &str) -> String {
    74	    format!(
    75	        r#"{{
    76	  "hooks": {{
    77	    "SubagentStart": [
    78	      {{
    79	        "matcher": ".*",
    80	        "hooks": [
    81	          {{
    82	            "type": "http",
    83	            "url": "http://localhost:{port}/hook?token={token}"
    84	          }}
    85	        ]
    86	      }}
    87	    ],
    88	    "SubagentStop": [
    89	      {{
    90	        "matcher": ".*",
    91	        "hooks": [
    92	          {{
    93	            "type": "http",
    94	            "url": "http://localhost:{port}/hook?token={token}"
    95	          }}
    96	        ]
    97	      }}
    98	    ],
    99	    "PreToolUse": [
   100	      {{
   101	        "matcher": ".*",
   102	        "hooks": [
   103	          {{
   104	            "type": "http",
   105	            "url": "http://localhost:{port}/hook?token={token}"
   106	          }}
   107	        ]
   108	      }}
   109	    ],
   110	    "PostToolUse": [
   111	      {{
   112	        "matcher": ".*",
   113	        "hooks": [
   114	          {{
   115	            "type": "http",
   116	            "url": "http://localhost:{port}/hook?token={token}"
   117	          }}
   118	        ]
   119	      }}
   120	    ],
   121	    "Stop": [
   122	      {{
   123	        "matcher": ".*",
   124	        "hooks": [
   125	          {{
   126	            "type": "http",
   127	            "url": "http://localhost:{port}/hook?token={token}"
   128	          }}
   129	        ]
   130	      }}
   131	    ]
   132	  }}
   133	}}"#
   134	    )
   135	}
   136	
   137	#[cfg(test)]
   138	mod tests {
   139	    use super::*;
   140	
   141	    #[test]
   142	    fn test_check_hooks_not_configured() {
   143	        let dir = tempfile::TempDir::new().unwrap();
   144	        let status = check_hooks_config(dir.path(), 9100, "testtoken");
   145	        assert_eq!(status, HookConfigStatus::NotConfigured);
   146	    }
   147	
   148	    #[test]
   149	    fn test_check_hooks_configured() {
   150	        let dir = tempfile::TempDir::new().unwrap();
   151	        let claude_dir = dir.path().join(".claude");
   152	        std::fs::create_dir_all(&claude_dir).unwrap();
   153	        let settings = claude_dir.join("settings.json");
   154	        std::fs::write(
   155	            &settings,
   156	            r#"{"hooks":{"SubagentStart":[{"hooks":[{"type":"http","url":"http://localhost:9100/hook?token=abc"}]}]}}"#,
   157	        ).unwrap();
   158	
   159	        let status = check_hooks_config(dir.path(), 9100, "abc");
   160	        assert_eq!(status, HookConfigStatus::Configured);
   161	    }
   162	
   163	    #[test]
   164	    fn test_check_hooks_token_mismatch() {
   165	        let dir = tempfile::TempDir::new().unwrap();
   166	        let claude_dir = dir.path().join(".claude");
   167	        std::fs::create_dir_all(&claude_dir).unwrap();
   168	        let settings = claude_dir.join("settings.json");
   169	        std::fs::write(
   170	            &settings,
   171	            r#"{"hooks":{"SubagentStart":[{"hooks":[{"type":"http","url":"http://localhost:9100/hook?token=oldtoken"}]}]}}"#,
   172	        ).unwrap();
   173	
   174	        // Port matches but token is different
   175	        let status = check_hooks_config(dir.path(), 9100, "newtoken");
   176	        assert!(matches!(status, HookConfigStatus::EndpointMismatch { .. }));
   177	    }
   178	
   179	    #[test]
   180	    fn test_check_hooks_endpoint_mismatch() {
   181	        let dir = tempfile::TempDir::new().unwrap();
   182	        let claude_dir = dir.path().join(".claude");
   183	        std::fs::create_dir_all(&claude_dir).unwrap();
   184	        let settings = claude_dir.join("settings.json");
   185	        std::fs::write(
   186	            &settings,
   187	            r#"{"hooks":{"SubagentStart":[{"hooks":[{"type":"http","url":"http://localhost:9200/hook?token=abc"}]}]}}"#,
   188	        ).unwrap();
   189	
   190	        let status = check_hooks_config(dir.path(), 9100, "abc");
   191	        assert!(matches!(status, HookConfigStatus::EndpointMismatch { .. }));
   192	    }
   193	
   194	    #[test]
   195	    fn test_generate_hooks_snippet() {
   196	        let snippet = generate_hooks_snippet(9100, "mytoken123");
   197	        assert!(snippet.contains("localhost:9100"));
   198	        assert!(snippet.contains("mytoken123"));
   199	        assert!(snippet.contains("SubagentStart"));
   200	        assert!(snippet.contains("SubagentStop"));
   201	        assert!(snippet.contains("Stop"));
   202	    }
   203	}

exec
/bin/zsh -lc "nl -ba src/service/token.rs | sed -n '1,360p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::path::{Path, PathBuf};
     2	
     3	use sha2::{Digest, Sha256};
     4	
     5	use crate::error::{AppError, Result};
     6	
     7	const SALT: &str = "devflow-tui-v1";
     8	const TOKEN_FILE: &str = ".tui-token";
     9	
    10	/// Get or create a stable token for the given project directory.
    11	///
    12	/// Token is deterministic: SHA-256(project_dir + salt).
    13	/// Stored in devflow-docs/.tui-token for persistence.
    14	pub fn get_or_create_token(project_dir: &Path, regenerate: bool) -> Result<String> {
    15	    let token_path = project_dir.join("devflow-docs").join(TOKEN_FILE);
    16	
    17	    if !regenerate
    18	        && let Ok(existing) = std::fs::read_to_string(&token_path)
    19	    {
    20	        let trimmed = existing.trim().to_string();
    21	        if !trimmed.is_empty() {
    22	            return Ok(trimmed);
    23	        }
    24	    }
    25	
    26	    let token = generate_token(project_dir);
    27	    write_token(&token_path, &token)?;
    28	    Ok(token)
    29	}
    30	
    31	/// Generate a deterministic token from project directory path.
    32	///
    33	/// Uses absolute path (not canonicalize) to avoid flakiness with symlinks,
    34	/// Docker mounts, or network filesystems where canonicalize may fail or
    35	/// return inconsistent results.
    36	pub fn generate_token(project_dir: &Path) -> String {
    37	    // Prefer absolute path over canonicalize for stability
    38	    let abs_path = if project_dir.is_absolute() {
    39	        project_dir.to_path_buf()
    40	    } else {
    41	        std::env::current_dir()
    42	            .map(|cwd| cwd.join(project_dir))
    43	            .unwrap_or_else(|_| project_dir.to_path_buf())
    44	    };
    45	
    46	    let mut hasher = Sha256::new();
    47	    hasher.update(abs_path.to_string_lossy().as_bytes());
    48	    hasher.update(SALT.as_bytes());
    49	    let result = hasher.finalize();
    50	    hex::encode(result)
    51	}
    52	
    53	fn write_token(path: &PathBuf, token: &str) -> Result<()> {
    54	    if let Some(parent) = path.parent() {
    55	        std::fs::create_dir_all(parent).map_err(|e| AppError::TokenGeneration(e.to_string()))?;
    56	    }
    57	    std::fs::write(path, token).map_err(|e| AppError::TokenGeneration(e.to_string()))?;
    58	    Ok(())
    59	}
    60	
    61	/// Validate a token against the expected value.
    62	pub fn validate_token(expected: &str, provided: &str) -> bool {
    63	    // Constant-time comparison to prevent timing attacks
    64	    if expected.len() != provided.len() {
    65	        return false;
    66	    }
    67	    expected
    68	        .bytes()
    69	        .zip(provided.bytes())
    70	        .fold(0u8, |acc, (a, b)| acc | (a ^ b))
    71	        == 0
    72	}
    73	
    74	#[cfg(test)]
    75	mod tests {
    76	    use super::*;
    77	    use tempfile::TempDir;
    78	
    79	    #[test]
    80	    fn test_same_dir_same_token() {
    81	        let dir = TempDir::new().unwrap();
    82	        let token1 = generate_token(dir.path());
    83	        let token2 = generate_token(dir.path());
    84	        assert_eq!(token1, token2);
    85	        assert_eq!(token1.len(), 64); // SHA-256 hex = 64 chars
    86	    }
    87	
    88	    #[test]
    89	    fn test_different_dir_different_token() {
    90	        let dir1 = TempDir::new().unwrap();
    91	        let dir2 = TempDir::new().unwrap();
    92	        let token1 = generate_token(dir1.path());
    93	        let token2 = generate_token(dir2.path());
    94	        assert_ne!(token1, token2);
    95	    }
    96	
    97	    #[test]
    98	    fn test_get_or_create_and_persist() {
    99	        let dir = TempDir::new().unwrap();
   100	        let devflow_dir = dir.path().join("devflow-docs");
   101	        std::fs::create_dir_all(&devflow_dir).unwrap();
   102	
   103	        let token1 = get_or_create_token(dir.path(), false).unwrap();
   104	        let token2 = get_or_create_token(dir.path(), false).unwrap();
   105	        assert_eq!(token1, token2); // Reads from file
   106	
   107	        // Regenerate
   108	        let token3 = get_or_create_token(dir.path(), true).unwrap();
   109	        assert_eq!(token1, token3); // Same dir → same hash
   110	    }
   111	
   112	    #[test]
   113	    fn test_validate_token() {
   114	        assert!(validate_token("abc123", "abc123"));
   115	        assert!(!validate_token("abc123", "abc124"));
   116	        assert!(!validate_token("abc123", "abc12"));
   117	    }
   118	}

exec
/bin/zsh -lc "nl -ba src/service/sanitizer.rs | sed -n '1,220p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	/// Strip ANSI escape sequences from a string.
     2	///
     3	/// Handles:
     4	/// - CSI sequences: ESC [ ... final_byte
     5	/// - OSC sequences: ESC ] ... ST
     6	/// - Simple escapes: ESC followed by single char
     7	pub fn strip_ansi(input: &str) -> String {
     8	    let mut result = String::with_capacity(input.len());
     9	    let mut chars = input.chars().peekable();
    10	
    11	    while let Some(ch) = chars.next() {
    12	        if ch == '\x1b' {
    13	            // ESC found — determine sequence type
    14	            match chars.peek() {
    15	                Some('[') => {
    16	                    // CSI sequence: ESC [ (params) final_byte
    17	                    // Final byte range per ECMA-48: 0x40..=0x7E
    18	                    chars.next(); // consume '['
    19	                    for c in chars.by_ref() {
    20	                        if ('\x40'..='\x7E').contains(&c) {
    21	                            break;
    22	                        }
    23	                    }
    24	                }
    25	                Some(']') => {
    26	                    // OSC sequence: ESC ] ... (ST = ESC \ or BEL)
    27	                    chars.next(); // consume ']'
    28	                    while let Some(c) = chars.next() {
    29	                        if c == '\x07' {
    30	                            break; // BEL terminator
    31	                        }
    32	                        if c == '\x1b' {
    33	                            if chars.peek() == Some(&'\\') {
    34	                                chars.next(); // consume '\'
    35	                            }
    36	                            break;
    37	                        }
    38	                    }
    39	                }
    40	                Some(_) => {
    41	                    // Simple escape: skip the next char
    42	                    chars.next();
    43	                }
    44	                None => {
    45	                    // Trailing ESC at end of string — skip
    46	                }
    47	            }
    48	        } else {
    49	            result.push(ch);
    50	        }
    51	    }
    52	
    53	    result
    54	}
    55	
    56	#[cfg(test)]
    57	mod tests {
    58	    use super::*;
    59	
    60	    #[test]
    61	    fn test_strip_ansi_escape_csi() {
    62	        let input = "\x1b[31mError\x1b[0m: something failed";
    63	        assert_eq!(strip_ansi(input), "Error: something failed");
    64	    }
    65	
    66	    #[test]
    67	    fn test_strip_ansi_escape_bold() {
    68	        let input = "\x1b[1m\x1b[36mBold Cyan\x1b[0m";
    69	        assert_eq!(strip_ansi(input), "Bold Cyan");
    70	    }
    71	
    72	    #[test]
    73	    fn test_no_ansi_passthrough() {
    74	        let input = "plain text with no escapes";
    75	        assert_eq!(strip_ansi(input), input);
    76	    }
    77	
    78	    #[test]
    79	    fn test_mixed_content() {
    80	        let input = "[14:20] \x1b[32m✓\x1b[0m workspace-detection → B (Approve)";
    81	        assert_eq!(
    82	            strip_ansi(input),
    83	            "[14:20] ✓ workspace-detection → B (Approve)"
    84	        );
    85	    }
    86	
    87	    #[test]
    88	    fn test_strip_osc_sequence() {
    89	        let input = "\x1b]0;window title\x07normal text";
    90	        assert_eq!(strip_ansi(input), "normal text");
    91	    }
    92	
    93	    #[test]
    94	    fn test_strip_trailing_esc() {
    95	        let input = "text\x1b";
    96	        assert_eq!(strip_ansi(input), "text");
    97	    }
    98	
    99	    #[test]
   100	    fn test_empty_string() {
   101	        assert_eq!(strip_ansi(""), "");
   102	    }
   103	}

exec
/bin/zsh -lc "nl -ba src/ui/header.rs | sed -n '1,220p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use ratatui::layout::Rect;
     2	use ratatui::style::Modifier;
     3	use ratatui::text::{Line, Span};
     4	use ratatui::widgets::Paragraph;
     5	use ratatui::Frame;
     6	
     7	use crate::ui::theme::Theme;
     8	
     9	/// Render the header bar at the top of the screen.
    10	pub fn render(
    11	    frame: &mut Frame,
    12	    area: Rect,
    13	    phase: &str,
    14	    hooks_active: bool,
    15	    hooks_port: Option<u16>,
    16	) {
    17	    let app_name = Span::from(" devflow-tui ").style(Theme::highlight());
    18	
    19	    let phase_span = Span::from(format!(" Phase: {phase} ")).style(Theme::focus_border());
    20	
    21	    let hooks_span = if hooks_active {
    22	        let port_str = hooks_port
    23	            .map(|p| format!(" {p}"))
    24	            .unwrap_or_default();
    25	        Span::from(format!(" Hooks: ●{port_str} ")).style(Theme::done())
    26	    } else {
    27	        Span::from(" Hooks: ○ ").style(Theme::error())
    28	    };
    29	
    30	    // Calculate padding — use Span::width() for correct unicode display width
    31	    let left_len = app_name.width();
    32	    let separator_len = 2; // "──"
    33	    let right_len = phase_span.width() + hooks_span.width();
    34	    let total = area.width as usize;
    35	    let pad = total.saturating_sub(left_len + right_len + separator_len);
    36	
    37	    let line = Line::from(vec![
    38	        app_name,
    39	        Span::from("─".repeat(pad)),
    40	        phase_span,
    41	        Span::from("──"),
    42	        hooks_span,
    43	    ]);
    44	
    45	    let widget = Paragraph::new(line).style(ratatui::style::Style::new().add_modifier(Modifier::DIM));
    46	    frame.render_widget(widget, area);
    47	}
    48	
    49	#[cfg(test)]
    50	mod tests {
    51	    use crate::test_helpers::{buffer_contains_str, render_with};
    52	
    53	    #[test]
    54	    fn render_default_state() {
    55	        let terminal = render_with(80, 1, |frame, area| {
    56	            super::render(frame, area, "INCEPTION", false, None);
    57	        });
    58	        let buf = terminal.backend().buffer();
    59	        assert!(buffer_contains_str(buf, "devflow-tui"));
    60	        assert!(buffer_contains_str(buf, "Phase:"));
    61	        assert!(buffer_contains_str(buf, "Hooks: ○"));
    62	    }
    63	
    64	    #[test]
    65	    fn render_hooks_active_with_port() {
    66	        let terminal = render_with(80, 1, |frame, area| {
    67	            super::render(frame, area, "CONSTRUCTION", true, Some(9464));
    68	        });
    69	        let buf = terminal.backend().buffer();
    70	        assert!(buffer_contains_str(buf, "Hooks: ●"));
    71	        assert!(buffer_contains_str(buf, "9464"));
    72	    }
    73	
    74	    #[test]
    75	    fn render_hooks_active_no_port() {
    76	        let terminal = render_with(80, 1, |frame, area| {
    77	            super::render(frame, area, "INCEPTION", true, None);
    78	        });
    79	        let buf = terminal.backend().buffer();
    80	        assert!(buffer_contains_str(buf, "Hooks: ●"));
    81	        assert!(!buffer_contains_str(buf, "9464"));
    82	    }
    83	
    84	    #[test]
    85	    fn render_long_phase_no_panic() {
    86	        let terminal = render_with(80, 1, |frame, area| {
    87	            super::render(frame, area, "VERY_LONG_PHASE_NAME_TESTING", false, None);
    88	        });
    89	        let buf = terminal.backend().buffer();
    90	        assert!(buffer_contains_str(buf, "devflow-tui"));
    91	    }
    92	
    93	    #[test]
    94	    fn render_narrow_terminal() {
    95	        // width=40 should not panic
    96	        let terminal = render_with(40, 1, |frame, area| {
    97	            super::render(frame, area, "INCEPTION", true, Some(8080));
    98	        });
    99	        let buf = terminal.backend().buffer();
   100	        assert!(buffer_contains_str(buf, "devflow-tui"));
   101	    }
   102	}

exec
/bin/zsh -lc "nl -ba src/ui/status_bar.rs | sed -n '1,260p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use ratatui::layout::Rect;
     2	use ratatui::style::Modifier;
     3	use ratatui::text::{Line, Span};
     4	use ratatui::widgets::Paragraph;
     5	use ratatui::Frame;
     6	
     7	use crate::ui::theme::key_hint;
     8	
     9	/// Render the status bar at the bottom of the screen.
    10	pub fn render(
    11	    frame: &mut Frame,
    12	    area: Rect,
    13	    focus_name: &str,
    14	    phase_stage: &str,
    15	    wide_mode: bool,
    16	) {
    17	    let left_span = Span::from(format!(" [{focus_name}]  {phase_stage}"));
    18	
    19	    let mut hints: Vec<Span> = Vec::new();
    20	    hints.extend(key_hint("Tab", "패널"));
    21	    if wide_mode {
    22	        hints.extend(key_hint("C-hjkl", "방향"));
    23	    }
    24	    hints.extend(key_hint("j/k", "스크롤"));
    25	    hints.extend(key_hint("f", "확대"));
    26	    hints.extend(key_hint("Enter", "상세"));
    27	    hints.extend(key_hint("r", "새로고침"));
    28	    hints.extend(key_hint("?", "도움말"));
    29	    hints.extend(key_hint("q", "종료"));
    30	
    31	    // Use Span::width() for correct unicode display width
    32	    let right_width: usize = hints.iter().map(|s| s.width()).sum();
    33	    let left_width = left_span.width();
    34	    let total_width = area.width as usize;
    35	    let pad = total_width.saturating_sub(left_width + right_width);
    36	
    37	    let mut spans = vec![left_span];
    38	    spans.push(Span::from(" ".repeat(pad)));
    39	    spans.extend(hints);
    40	
    41	    let line = Line::from(spans);
    42	    let bar_style = if super::theme::no_color() {
    43	        ratatui::style::Style::default().add_modifier(Modifier::REVERSED)
    44	    } else {
    45	        ratatui::style::Style::new().on_dark_gray().white()
    46	    };
    47	    let widget = Paragraph::new(line).style(bar_style);
    48	
    49	    frame.render_widget(widget, area);
    50	}
    51	
    52	#[cfg(test)]
    53	mod tests {
    54	    use crate::test_helpers::{buffer_contains_str, render_with};
    55	
    56	    #[test]
    57	    fn render_default() {
    58	        let terminal = render_with(160, 1, |frame, area| {
    59	            super::render(frame, area, "Audit Log", "INCEPTION > workspace-detection", false);
    60	        });
    61	        let buf = terminal.backend().buffer();
    62	        assert!(buffer_contains_str(buf, "[Audit Log]"));
    63	        assert!(buffer_contains_str(buf, "INCEPTION"));
    64	        assert!(buffer_contains_str(buf, "Tab"));
    65	        assert!(buffer_contains_str(buf, "q"));
    66	    }
    67	
    68	    #[test]
    69	    fn render_wide_mode_has_direction_hint() {
    70	        let terminal = render_with(120, 1, |frame, area| {
    71	            super::render(frame, area, "Git", "INCEPTION", true);
    72	        });
    73	        let buf = terminal.backend().buffer();
    74	        assert!(buffer_contains_str(buf, "C-hjkl"));
    75	    }
    76	
    77	    #[test]
    78	    fn render_narrow_mode_no_direction_hint() {
    79	        let terminal = render_with(80, 1, |frame, area| {
    80	            super::render(frame, area, "Git", "INCEPTION", false);
    81	        });
    82	        let buf = terminal.backend().buffer();
    83	        assert!(!buffer_contains_str(buf, "C-hjkl"));
    84	    }
    85	
    86	    #[test]
    87	    fn render_korean_text() {
    88	        let terminal = render_with(160, 1, |frame, area| {
    89	            super::render(frame, area, "Agent", "CONSTRUCTION", false);
    90	        });
    91	        let buf = terminal.backend().buffer();
    92	        // key_hint produces Korean descriptions
    93	        assert!(buffer_contains_str(buf, "패널"));
    94	        assert!(buffer_contains_str(buf, "스크롤"));
    95	    }
    96	}

exec
/bin/zsh -lc "nl -ba src/ui/help_overlay.rs | sed -n '1,300p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use ratatui::layout::Rect;
     2	use ratatui::text::{Line, Text};
     3	use ratatui::widgets::{Block, BorderType, Clear, Paragraph, Wrap};
     4	use ratatui::Frame;
     5	
     6	use crate::ui::layout::centered_rect;
     7	use crate::ui::theme::Theme;
     8	
     9	/// Render the help overlay (? key).
    10	pub fn render(frame: &mut Frame, area: Rect) {
    11	    let modal_area = centered_rect(60, 70, area);
    12	
    13	    // Clear background
    14	    frame.render_widget(Clear, modal_area);
    15	
    16	    let title_style = Theme::highlight();
    17	    let key_style = Theme::focus_border();
    18	    let dim_style = Theme::waiting();
    19	
    20	    let help_text = Text::from(vec![
    21	        Line::from("  Keyboard Shortcuts").style(title_style),
    22	        Line::from(""),
    23	        Line::from(vec![
    24	            "  Tab / Shift+Tab  ".into(),
    25	            "패널 간 순환".into(),
    26	        ]).style(key_style),
    27	        Line::from(vec![
    28	            "  Ctrl+h/j/k/l    ".into(),
    29	            "방향 네비게이션 (와이드)".into(),
    30	        ]).style(key_style),
    31	        Line::from(vec![
    32	            "  j / k  ↑ / ↓    ".into(),
    33	            "리스트 스크롤".into(),
    34	        ]).style(key_style),
    35	        Line::from(vec![
    36	            "  Enter            ".into(),
    37	            "상세 보기 / 산출물 모달".into(),
    38	        ]).style(key_style),
    39	        Line::from(vec![
    40	            "  f                ".into(),
    41	            "패널 전체화면 확대".into(),
    42	        ]).style(key_style),
    43	        Line::from(vec![
    44	            "  Esc              ".into(),
    45	            "복귀 / 모달 닫기".into(),
    46	        ]).style(key_style),
    47	        Line::from(vec![
    48	            "  r                ".into(),
    49	            "수동 새로고침".into(),
    50	        ]).style(key_style),
    51	        Line::from(vec![
    52	            "  ?                ".into(),
    53	            "이 도움말 표시/닫기".into(),
    54	        ]).style(key_style),
    55	        Line::from(vec![
    56	            "  q                ".into(),
    57	            "종료".into(),
    58	        ]).style(key_style),
    59	        Line::from(""),
    60	        Line::from("  Press ? or Esc to close").style(dim_style),
    61	    ]);
    62	
    63	    let block = Block::bordered()
    64	        .border_type(BorderType::Rounded)
    65	        .title("  Help  ")
    66	        .border_style(Theme::focus_border());
    67	
    68	    let widget = Paragraph::new(help_text)
    69	        .block(block)
    70	        .wrap(Wrap { trim: false });
    71	
    72	    frame.render_widget(widget, modal_area);
    73	}
    74	
    75	#[cfg(test)]
    76	mod tests {
    77	    use crate::test_helpers::{buffer_contains_str, render_with};
    78	
    79	    #[test]
    80	    fn render_contains_shortcuts() {
    81	        let terminal = render_with(80, 24, |frame, area| {
    82	            super::render(frame, area);
    83	        });
    84	        let buf = terminal.backend().buffer();
    85	        assert!(buffer_contains_str(buf, "Tab"));
    86	        assert!(buffer_contains_str(buf, "Enter"));
    87	        assert!(buffer_contains_str(buf, "q"));
    88	        assert!(buffer_contains_str(buf, "Esc"));
    89	    }
    90	
    91	    #[test]
    92	    fn render_has_border_and_title() {
    93	        let terminal = render_with(80, 24, |frame, area| {
    94	            super::render(frame, area);
    95	        });
    96	        let buf = terminal.backend().buffer();
    97	        assert!(buffer_contains_str(buf, "Help"));
    98	        assert!(buffer_contains_str(buf, "╭") || buffer_contains_str(buf, "┌"));
    99	    }
   100	
   101	    #[test]
   102	    fn render_small_area_no_panic() {
   103	        // Should not panic even on a small terminal
   104	        let _terminal = render_with(40, 12, |frame, area| {
   105	            super::render(frame, area);
   106	        });
   107	    }
   108	}

exec
/bin/zsh -lc "nl -ba src/ui/theme.rs | sed -n '1,360p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::sync::LazyLock;
     2	
     3	use ratatui::style::{Modifier, Style, Stylize};
     4	use ratatui::text::Span;
     5	
     6	static NO_COLOR: LazyLock<bool> = LazyLock::new(|| std::env::var("NO_COLOR").is_ok());
     7	
     8	/// Check if NO_COLOR environment variable is set (cached at first call).
     9	pub fn no_color() -> bool {
    10	    *NO_COLOR
    11	}
    12	
    13	/// Color tokens for the application.
    14	pub struct Theme;
    15	
    16	impl Theme {
    17	    pub fn active() -> Style {
    18	        if no_color() {
    19	            Style::default().add_modifier(Modifier::BOLD)
    20	        } else {
    21	            Style::new().yellow().bold()
    22	        }
    23	    }
    24	
    25	    pub fn done() -> Style {
    26	        if no_color() {
    27	            Style::default()
    28	        } else {
    29	            Style::new().green()
    30	        }
    31	    }
    32	
    33	    pub fn waiting() -> Style {
    34	        if no_color() {
    35	            Style::default().dim()
    36	        } else {
    37	            Style::new().dark_gray()
    38	        }
    39	    }
    40	
    41	    pub fn skipped() -> Style {
    42	        if no_color() {
    43	            Style::default().dim()
    44	        } else {
    45	            Style::new().dark_gray()
    46	        }
    47	    }
    48	
    49	    pub fn error() -> Style {
    50	        if no_color() {
    51	            Style::default().add_modifier(Modifier::BOLD)
    52	        } else {
    53	            Style::new().red()
    54	        }
    55	    }
    56	
    57	    pub fn timeout() -> Style {
    58	        if no_color() {
    59	            Style::default().add_modifier(Modifier::BOLD)
    60	        } else {
    61	            Style::new().red()
    62	        }
    63	    }
    64	
    65	    pub fn focus_border() -> Style {
    66	        if no_color() {
    67	            Style::default().add_modifier(Modifier::BOLD)
    68	        } else {
    69	            Style::new().cyan()
    70	        }
    71	    }
    72	
    73	    pub fn unfocus_border() -> Style {
    74	        if no_color() {
    75	            Style::default().dim()
    76	        } else {
    77	            Style::new().dark_gray()
    78	        }
    79	    }
    80	
    81	    pub fn disabled() -> Style {
    82	        if no_color() {
    83	            Style::default().dim()
    84	        } else {
    85	            Style::new().dark_gray().dim()
    86	        }
    87	    }
    88	
    89	    pub fn highlight() -> Style {
    90	        if no_color() {
    91	            Style::default().add_modifier(Modifier::BOLD)
    92	        } else {
    93	            Style::new().white().bold()
    94	        }
    95	    }
    96	
    97	    pub fn timestamp() -> Style {
    98	        if no_color() {
    99	            Style::default()
   100	        } else {
   101	            Style::new().cyan()
   102	        }
   103	    }
   104	
   105	    pub fn gate_alert() -> Style {
   106	        if no_color() {
   107	            Style::default().add_modifier(Modifier::BOLD)
   108	        } else {
   109	            Style::new().yellow().bold()
   110	        }
   111	    }
   112	
   113	    pub fn staged() -> Style {
   114	        if no_color() { Style::default() } else { Style::new().green() }
   115	    }
   116	
   117	    pub fn unstaged() -> Style {
   118	        if no_color() { Style::default() } else { Style::new().yellow() }
   119	    }
   120	
   121	    pub fn untracked() -> Style {
   122	        if no_color() { Style::default().dim() } else { Style::new().dark_gray() }
   123	    }
   124	
   125	    pub fn conflict() -> Style {
   126	        if no_color() {
   127	            Style::default().add_modifier(Modifier::BOLD)
   128	        } else {
   129	            Style::new().red().bold()
   130	        }
   131	    }
   132	}
   133	
   134	/// Create a status span with icon and text, respecting NO_COLOR.
   135	pub fn status_span<'a>(icon: &'a str, text: &'a str, style: Style) -> Span<'a> {
   136	    Span::styled(format!("{icon} {text}"), style)
   137	}
   138	
   139	/// Panel title formatting based on focus state.
   140	pub fn panel_title(name: &str, focused: bool) -> String {
   141	    if focused {
   142	        format!("[ {name} ]")
   143	    } else {
   144	        format!("  {name}  ")
   145	    }
   146	}
   147	
   148	/// Status icons for NO_COLOR compatibility.
   149	pub struct Icons;
   150	
   151	impl Icons {
   152	    pub fn active() -> &'static str { "●" }
   153	    pub fn done() -> &'static str { "✓" }
   154	    pub fn waiting() -> &'static str { "○" }
   155	    pub fn skipped() -> &'static str { "–" }
   156	    pub fn error() -> &'static str { "✗" }
   157	    pub fn timeout() -> &'static str { "⏱" }
   158	    pub fn staged() -> &'static str { "S" }
   159	    pub fn unstaged() -> &'static str { "M" }
   160	    pub fn untracked() -> &'static str { "?" }
   161	    pub fn conflict() -> &'static str { "C!" }
   162	}
   163	
   164	/// Key hint spans for status bar.
   165	pub fn key_hint<'a>(key: &'a str, desc: &'a str) -> Vec<Span<'a>> {
   166	    if no_color() {
   167	        vec![
   168	            Span::from(format!(" {key} ")).add_modifier(Modifier::BOLD),
   169	            Span::from(format!("{desc} ")),
   170	        ]
   171	    } else {
   172	        vec![
   173	            Span::from(format!(" {key} ")).cyan().bold(),
   174	            Span::from(format!("{desc} ")).dim(),
   175	        ]
   176	    }
   177	}
   178	
   179	#[cfg(test)]
   180	mod tests {
   181	    use super::*;
   182	
   183	    #[test]
   184	    fn test_panel_title_focused() {
   185	        assert_eq!(panel_title("Git Status", true), "[ Git Status ]");
   186	    }
   187	
   188	    #[test]
   189	    fn test_panel_title_unfocused() {
   190	        assert_eq!(panel_title("Git Status", false), "  Git Status  ");
   191	    }
   192	
   193	    #[test]
   194	    fn test_status_span_format() {
   195	        let span = status_span("✓", "workspace-detection", Theme::done());
   196	        assert!(span.content.contains("✓ workspace-detection"));
   197	    }
   198	
   199	    #[test]
   200	    fn test_key_hint_produces_two_spans() {
   201	        let spans = key_hint("Tab", "패널");
   202	        assert_eq!(spans.len(), 2);
   203	    }
   204	
   205	    #[test]
   206	    fn render_status_span_in_buffer() {
   207	        use ratatui::text::Line;
   208	        use ratatui::widgets::Paragraph;
   209	        use crate::test_helpers::{buffer_contains_str, render_with};
   210	
   211	        let terminal = render_with(40, 1, |frame, area| {
   212	            let span = status_span(Icons::done(), "workspace-detection", Theme::done());
   213	            let widget = Paragraph::new(Line::from(span));
   214	            frame.render_widget(widget, area);
   215	        });
   216	        let buf = terminal.backend().buffer();
   217	        assert!(buffer_contains_str(buf, "✓"));
   218	        assert!(buffer_contains_str(buf, "workspace-detection"));
   219	    }
   220	
   221	    #[test]
   222	    fn render_key_hint_in_buffer() {
   223	        use ratatui::text::Line;
   224	        use ratatui::widgets::Paragraph;
   225	        use crate::test_helpers::{buffer_contains_str, render_with};
   226	
   227	        let terminal = render_with(20, 1, |frame, area| {
   228	            let spans = key_hint("Tab", "패널");
   229	            let widget = Paragraph::new(Line::from(spans));
   230	            frame.render_widget(widget, area);
   231	        });
   232	        let buf = terminal.backend().buffer();
   233	        assert!(buffer_contains_str(buf, "Tab"));
   234	        assert!(buffer_contains_str(buf, "패널"));
   235	    }
   236	}

codex
I’ve collected most of the implementation context; I’m doing a final pass through module boundaries and test strategy to ensure the review calls out real maintainability and correctness risks, not style-only notes.
exec
/bin/zsh -lc "nl -ba src/panel/mod.rs | sed -n '1,200p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	pub mod agent_status;
     2	pub mod audit_log;
     3	pub mod git_status;
     4	pub mod workflow_map;

exec
/bin/zsh -lc "nl -ba src/adapter/mod.rs | sed -n '1,200p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	pub mod handle;
     2	pub mod file_watcher;
     3	pub mod git_poller;
     4	pub mod hooks_server;

exec
/bin/zsh -lc "nl -ba src/parser/devflow_state.rs | sed -n '1,340p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use crate::parser::models::{
     2	    ApprovedStage, Complexity, CompletedStage, FlowState, Phase, SkippedStage, WorktreeInfo,
     3	};
     4	use crate::service::sanitizer::strip_ansi;
     5	
     6	/// Parse devflow-state.md content into FlowState.
     7	///
     8	/// Tolerant parsing: unknown sections are stored in extra_fields.
     9	/// Empty or minimal content returns default FlowState.
    10	pub fn parse(content: &str) -> FlowState {
    11	    let content = strip_ansi(content);
    12	    let mut state = FlowState::default();
    13	    let mut current_section: Option<String> = None;
    14	    let mut section_lines: Vec<String> = Vec::new();
    15	
    16	    for line in content.lines() {
    17	        if let Some(header) = line.strip_prefix("## ") {
    18	            // Flush previous section
    19	            if let Some(ref section) = current_section {
    20	                apply_section(&mut state, section, &section_lines);
    21	            }
    22	            current_section = Some(header.trim().to_string());
    23	            section_lines.clear();
    24	        } else if line.starts_with("# ") {
    25	            // Top-level header — ignore (e.g., "# DevFlow State")
    26	        } else if current_section.is_some() {
    27	            section_lines.push(line.to_string());
    28	        }
    29	    }
    30	
    31	    // Flush last section
    32	    if let Some(ref section) = current_section {
    33	        apply_section(&mut state, section, &section_lines);
    34	    }
    35	
    36	    state
    37	}
    38	
    39	fn apply_section(state: &mut FlowState, section: &str, lines: &[String]) {
    40	    let value = lines
    41	        .iter()
    42	        .map(|l| l.trim())
    43	        .filter(|l| !l.is_empty())
    44	        .collect::<Vec<_>>()
    45	        .join("\n");
    46	
    47	    match section {
    48	        "Current Phase" => {
    49	            match value.parse::<Phase>() {
    50	                Ok(phase) => state.phase = phase,
    51	                Err(e) => {
    52	                    tracing::warn!("devflow-state: {e}, falling back to default");
    53	                    state.phase = Phase::default();
    54	                }
    55	            }
    56	        }
    57	        "Current Stage" => {
    58	            state.stage = value.to_string();
    59	        }
    60	        "Complexity" => {
    61	            match value.parse::<Complexity>() {
    62	                Ok(c) => state.complexity = c,
    63	                Err(e) => {
    64	                    tracing::warn!("devflow-state: {e}, falling back to default");
    65	                    state.complexity = Complexity::default();
    66	                }
    67	            }
    68	        }
    69	        "Selected Approach" => {
    70	            if !value.is_empty() && value != "(pending)" {
    71	                state.selected_approach = Some(value.to_string());
    72	            }
    73	        }
    74	        "Completed Stages" => {
    75	            state.completed_stages = parse_completed_stages(lines);
    76	        }
    77	        "Approved Stages" => {
    78	            state.approved_stages = parse_approved_stages(lines);
    79	        }
    80	        "Skipped Stages" => {
    81	            state.skipped_stages = parse_skipped_stages(lines);
    82	        }
    83	        "Active Unit" => {
    84	            if !value.is_empty() && value != "(pending)" {
    85	                state.active_unit = Some(value.to_string());
    86	            }
    87	        }
    88	        "Completed Units" => {
    89	            state.completed_units = lines
    90	                .iter()
    91	                .filter_map(|l| l.trim().strip_prefix("- ").map(|s| s.trim().to_string()))
    92	                .collect();
    93	        }
    94	        "Worktree" => {
    95	            state.worktree = parse_worktree(lines);
    96	        }
    97	        _ => {
    98	            // Unknown section → store as extra field
    99	            if !value.is_empty() {
   100	                state.extra_fields.insert(section.to_string(), value);
   101	            }
   102	        }
   103	    }
   104	}
   105	
   106	fn parse_completed_stages(lines: &[String]) -> Vec<CompletedStage> {
   107	    lines
   108	        .iter()
   109	        .filter_map(|l| {
   110	            let trimmed = l.trim().strip_prefix("- ")?;
   111	            let (name, timestamp) = if let Some((n, t)) = trimmed.rsplit_once('(') {
   112	                (n.trim().to_string(), Some(t.trim_end_matches(')').trim().to_string()))
   113	            } else {
   114	                (trimmed.trim().to_string(), None)
   115	            };
   116	            Some(CompletedStage { name, timestamp })
   117	        })
   118	        .collect()
   119	}
   120	
   121	fn parse_approved_stages(lines: &[String]) -> Vec<ApprovedStage> {
   122	    lines
   123	        .iter()
   124	        .filter_map(|l| {
   125	            let trimmed = l.trim().strip_prefix("- ")?;
   126	            let (name, depth) = if let Some((n, d)) = trimmed.split_once("—") {
   127	                let depth_val = d.trim().strip_prefix("depth:").map(|s| s.trim().to_string());
   128	                (n.trim().to_string(), depth_val)
   129	            } else {
   130	                (trimmed.trim().to_string(), None)
   131	            };
   132	            Some(ApprovedStage { name, depth })
   133	        })
   134	        .collect()
   135	}
   136	
   137	fn parse_skipped_stages(lines: &[String]) -> Vec<SkippedStage> {
   138	    lines
   139	        .iter()
   140	        .filter_map(|l| {
   141	            let trimmed = l.trim().strip_prefix("- ")?;
   142	            let (name, reason) = if let Some((n, r)) = trimmed.split_once("—") {
   143	                let reason_val = r.trim().strip_prefix("reason:").map(|s| s.trim().to_string());
   144	                (n.trim().to_string(), reason_val)
   145	            } else {
   146	                (trimmed.trim().to_string(), None)
   147	            };
   148	            Some(SkippedStage { name, reason })
   149	        })
   150	        .collect()
   151	}
   152	
   153	fn parse_worktree(lines: &[String]) -> WorktreeInfo {
   154	    let mut info = WorktreeInfo::default();
   155	    for line in lines {
   156	        let trimmed = line.trim();
   157	        if let Some((key, val)) = trimmed.split_once(':') {
   158	            match key.trim().to_lowercase().as_str() {
   159	                "branch" => info.branch = Some(val.trim().to_string()),
   160	                "path" => info.path = Some(val.trim().to_string()),
   161	                _ => {}
   162	            }
   163	        }
   164	    }
   165	    info
   166	}
   167	
   168	#[cfg(test)]
   169	mod tests {
   170	    use super::*;
   171	
   172	    #[test]
   173	    fn test_golden_minimal() {
   174	        let content = r#"# DevFlow State
   175	
   176	## Current Phase
   177	INCEPTION
   178	
   179	## Current Stage
   180	(pending)
   181	
   182	## Complexity
   183	(pending)
   184	
   185	## Selected Approach
   186	(pending)
   187	"#;
   188	        let state = parse(content);
   189	        assert_eq!(state.phase, Phase::Inception);
   190	        assert_eq!(state.stage, "(pending)");
   191	        assert_eq!(state.complexity, Complexity::Standard); // (pending) → default
   192	        assert!(state.selected_approach.is_none());
   193	    }
   194	
   195	    #[test]
   196	    fn test_golden_standard() {
   197	        let content = r#"# DevFlow State
   198	
   199	## Current Phase
   200	INCEPTION
   201	
   202	## Current Stage
   203	requirements-analysis
   204	
   205	## Complexity
   206	Standard
   207	
   208	## Selected Approach
   209	B (계층별 유닛 분해)
   210	
   211	## Completed Stages
   212	- workspace-detection (2026-03-30T14:18)
   213	- complexity-declaration (2026-03-30T14:19)
   214	
   215	## Approved Stages
   216	- code-generation — depth: Standard
   217	- build-and-test — depth: Standard
   218	
   219	## Skipped Stages
   220	- user-stories — reason: Minimal complexity
   221	"#;
   222	        let state = parse(content);
   223	        assert_eq!(state.phase, Phase::Inception);
   224	        assert_eq!(state.stage, "requirements-analysis");
   225	        assert_eq!(state.complexity, Complexity::Standard);
   226	        assert_eq!(state.selected_approach.as_deref(), Some("B (계층별 유닛 분해)"));
   227	        assert_eq!(state.completed_stages.len(), 2);
   228	        assert_eq!(state.completed_stages[0].name, "workspace-detection");
   229	        assert_eq!(
   230	            state.completed_stages[0].timestamp.as_deref(),
   231	            Some("2026-03-30T14:18")
   232	        );
   233	        assert_eq!(state.approved_stages.len(), 2);
   234	        assert_eq!(state.approved_stages[0].depth.as_deref(), Some("Standard"));
   235	        assert_eq!(state.skipped_stages.len(), 1);
   236	        assert_eq!(
   237	            state.skipped_stages[0].reason.as_deref(),
   238	            Some("Minimal complexity")
   239	        );
   240	    }
   241	
   242	    #[test]
   243	    fn test_golden_construction() {
   244	        let content = r#"# DevFlow State
   245	
   246	## Current Phase
   247	CONSTRUCTION
   248	
   249	## Current Stage
   250	code-generation
   251	
   252	## Complexity
   253	Standard
   254	
   255	## Active Unit
   256	unit-03-file-watcher
   257	
   258	## Completed Units
   259	- unit-01-foundation
   260	- unit-02-parser
   261	
   262	## Worktree
   263	branch: feature/devflow-tui
   264	path: /Users/jay/projects/backend/devflow-tui
   265	"#;
   266	        let state = parse(content);
   267	        assert_eq!(state.phase, Phase::Construction);
   268	        assert_eq!(state.stage, "code-generation");
   269	        assert_eq!(state.active_unit.as_deref(), Some("unit-03-file-watcher"));
   270	        assert_eq!(state.completed_units.len(), 2);
   271	        assert_eq!(state.completed_units[0], "unit-01-foundation");
   272	        assert_eq!(state.worktree.branch.as_deref(), Some("feature/devflow-tui"));
   273	    }
   274	
   275	    #[test]
   276	    fn test_golden_nonstandard() {
   277	        let content = r#"# DevFlow State
   278	
   279	## Current Phase
   280	INCEPTION
   281	
   282	## Current Stage
   283	workflow-planning
   284	
   285	## Complexity
   286	Standard
   287	
   288	## Project Root
   289	/Users/jay/projects/backend
   290	
   291	## Finishing Choice
   292	B (PR pending)
   293	
   294	## PR URL
   295	https://github.com/user/repo/pull/42
   296	"#;
   297	        let state = parse(content);
   298	        assert_eq!(state.phase, Phase::Inception);
   299	        assert_eq!(
   300	            state.extra_fields.get("Project Root").map(|s| s.as_str()),
   301	            Some("/Users/jay/projects/backend")
   302	        );
   303	        assert_eq!(
   304	            state.extra_fields.get("Finishing Choice").map(|s| s.as_str()),
   305	            Some("B (PR pending)")
   306	        );
   307	        assert_eq!(
   308	            state.extra_fields.get("PR URL").map(|s| s.as_str()),
   309	            Some("https://github.com/user/repo/pull/42")
   310	        );
   311	    }
   312	
   313	    #[test]
   314	    fn test_golden_empty() {
   315	        let state = parse("");
   316	        assert_eq!(state.phase, Phase::Inception);
   317	        assert!(state.stage.is_empty());
   318	        assert_eq!(state.complexity, Complexity::Standard);
   319	    }
   320	
   321	    #[test]
   322	    fn test_ansi_stripped() {
   323	        let content = "# DevFlow State\n\n## Current Phase\n\x1b[32mCONSTRUCTION\x1b[0m\n";
   324	        let state = parse(content);
   325	        assert_eq!(state.phase, Phase::Construction);
   326	    }
   327	}

exec
/bin/zsh -lc "nl -ba src/parser/session_summary.rs | sed -n '1,360p'" in /Users/jay.ahn/projects/backend/devflow-tui
exec
/bin/zsh -lc "nl -ba src/parser/audit_log.rs | sed -n '1,340p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use crate::parser::models::{Complexity, Phase, SessionSummary, WorkItem, WorkMarker};
     2	use crate::service::sanitizer::strip_ansi;
     3	
     4	/// Parse session-summary.md content into SessionSummary.
     5	pub fn parse(content: &str) -> SessionSummary {
     6	    let content = strip_ansi(content);
     7	    let mut summary = SessionSummary::default();
     8	    let mut current_section: Option<String> = None;
     9	    let mut section_lines: Vec<String> = Vec::new();
    10	
    11	    for line in content.lines() {
    12	        if let Some(header) = line.strip_prefix("## ") {
    13	            if let Some(ref section) = current_section {
    14	                apply_section(&mut summary, section, &section_lines);
    15	            }
    16	            current_section = Some(header.trim().to_string());
    17	            section_lines.clear();
    18	        } else if line.starts_with("# ") {
    19	            // Top-level header — ignore
    20	        } else if line.starts_with("**") && current_section.is_none() {
    21	            // Metadata lines before first section (e.g., **Last Updated**)
    22	            parse_metadata_line(&mut summary, line);
    23	        } else if let Some(ref _section) = current_section {
    24	            section_lines.push(line.to_string());
    25	        }
    26	    }
    27	
    28	    if let Some(ref section) = current_section {
    29	        apply_section(&mut summary, section, &section_lines);
    30	    }
    31	
    32	    summary
    33	}
    34	
    35	fn parse_metadata_line(summary: &mut SessionSummary, line: &str) {
    36	    if let Some(rest) = line.strip_prefix("**Last Updated**:") {
    37	        summary.last_updated = Some(rest.trim().to_string());
    38	    } else if let Some(rest) = line.strip_prefix("**Commit**:") {
    39	        summary.commit = Some(rest.trim().to_string());
    40	    }
    41	}
    42	
    43	fn apply_section(summary: &mut SessionSummary, section: &str, lines: &[String]) {
    44	    match section {
    45	        "Current State" => {
    46	            for line in lines {
    47	                let trimmed = line.trim().strip_prefix("- ").unwrap_or(line.trim());
    48	                if let Some(val) = trimmed.strip_prefix("Phase:") {
    49	                    match val.trim().parse::<Phase>() {
    50	                        Ok(phase) => summary.phase = Some(phase),
    51	                        Err(e) => tracing::warn!("session-summary: {e}"),
    52	                    }
    53	                } else if let Some(val) = trimmed.strip_prefix("Stage:") {
    54	                    summary.stage = Some(val.trim().to_string());
    55	                } else if let Some(val) = trimmed.strip_prefix("Complexity:") {
    56	                    match val.trim().parse::<Complexity>() {
    57	                        Ok(c) => summary.complexity = Some(c),
    58	                        Err(e) => tracing::warn!("session-summary: {e}"),
    59	                    }
    60	                } else if let Some(val) = trimmed.strip_prefix("Commit:") {
    61	                    summary.commit = Some(val.trim().to_string());
    62	                }
    63	            }
    64	        }
    65	        "Key Decisions" => {
    66	            summary.key_decisions = lines
    67	                .iter()
    68	                .map(|l| l.trim())
    69	                .filter(|l| !l.is_empty())
    70	                .map(|l| l.strip_prefix("- ").unwrap_or(l).to_string())
    71	                .collect();
    72	        }
    73	        "Completed Work" | "INCEPTION" | "CONSTRUCTION" => {
    74	            let items = parse_work_items(lines);
    75	            summary.completed_work.extend(items);
    76	        }
    77	        "Next Steps" => {
    78	            summary.next_steps = lines
    79	                .iter()
    80	                .map(|l| l.trim())
    81	                .filter(|l| !l.is_empty())
    82	                .map(|l| l.strip_prefix("- ").unwrap_or(l).to_string())
    83	                .collect();
    84	        }
    85	        "For Next Session" => {
    86	            summary.for_next_session = lines
    87	                .iter()
    88	                .map(|l| l.trim())
    89	                .filter(|l| !l.is_empty())
    90	                .map(|l| l.strip_prefix("- ").unwrap_or(l).to_string())
    91	                .collect();
    92	        }
    93	        _ => {
    94	            // Unknown section — ignore
    95	        }
    96	    }
    97	}
    98	
    99	fn parse_work_items(lines: &[String]) -> Vec<WorkItem> {
   100	    lines
   101	        .iter()
   102	        .filter_map(|line| {
   103	            let trimmed = line.trim();
   104	            let trimmed = trimmed.strip_prefix("- ").unwrap_or(trimmed);
   105	            parse_single_work_item(trimmed)
   106	        })
   107	        .collect()
   108	}
   109	
   110	fn parse_single_work_item(trimmed: &str) -> Option<WorkItem> {
   111	    let markers: &[(&str, WorkMarker)] = &[
   112	        ("[x] ", WorkMarker::Done),
   113	        ("[~] ", WorkMarker::InProgress),
   114	        ("[ ] ", WorkMarker::Pending),
   115	    ];
   116	
   117	    for (prefix, marker) in markers {
   118	        if let Some(rest) = trimmed.strip_prefix(prefix) {
   119	            return Some(WorkItem {
   120	                text: rest.to_string(),
   121	                marker: *marker,
   122	            });
   123	        }
   124	    }
   125	
   126	    None
   127	}
   128	
   129	#[cfg(test)]
   130	mod tests {
   131	    use super::*;
   132	
   133	    #[test]
   134	    fn test_golden_inception_session() {
   135	        let content = r#"# Session Summary
   136	
   137	**Last Updated**: 2026-03-30T14:40:00+09:00
   138	**Commit**: 2a3cbee
   139	
   140	## Current State
   141	- Phase: INCEPTION
   142	- Stage: requirements-analysis
   143	- Complexity: Standard
   144	
   145	## Key Decisions
   146	- 2026-03-30T14:18 workspace-detection B — Greenfield
   147	- 2026-03-30T14:19 complexity A→Standard
   148	
   149	## Completed Work
   150	### INCEPTION
   151	- [x] workspace-detection — Greenfield detected
   152	- [~] requirements-analysis — in progress
   153	- [ ] workflow-planning — pending
   154	
   155	## Next Steps
   156	- Complete requirements-analysis
   157	
   158	## For Next Session
   159	- Review open questions in requirements
   160	"#;
   161	        let summary = parse(content);
   162	        assert_eq!(summary.last_updated.as_deref(), Some("2026-03-30T14:40:00+09:00"));
   163	        assert_eq!(summary.commit.as_deref(), Some("2a3cbee"));
   164	        assert_eq!(summary.phase, Some(Phase::Inception));
   165	        assert_eq!(summary.stage.as_deref(), Some("requirements-analysis"));
   166	        assert_eq!(summary.complexity, Some(Complexity::Standard));
   167	        assert_eq!(summary.key_decisions.len(), 2);
   168	        assert_eq!(summary.completed_work.len(), 3);
   169	        assert_eq!(summary.completed_work[0].marker, WorkMarker::Done);
   170	        assert_eq!(summary.completed_work[1].marker, WorkMarker::InProgress);
   171	        assert_eq!(summary.completed_work[2].marker, WorkMarker::Pending);
   172	        assert_eq!(summary.next_steps.len(), 1);
   173	        assert_eq!(summary.for_next_session.len(), 1);
   174	    }
   175	
   176	    #[test]
   177	    fn test_golden_construction_session() {
   178	        let content = r#"# Session Summary
   179	
   180	**Last Updated**: 2026-03-30
   181	
   182	## Current State
   183	- Phase: CONSTRUCTION
   184	- Stage: code-generation
   185	
   186	## Key Decisions
   187	- Approach B selected
   188	
   189	## Completed Work
   190	### INCEPTION
   191	- [x] workspace-detection — done
   192	- [x] requirements-analysis — done
   193	### CONSTRUCTION
   194	- [x] code-generation unit-01 — done
   195	- [~] code-generation unit-02 — in progress
   196	
   197	## Next Steps
   198	- Complete unit-02 parser
   199	"#;
   200	        let summary = parse(content);
   201	        assert_eq!(summary.phase, Some(Phase::Construction));
   202	        assert_eq!(summary.completed_work.len(), 4);
   203	        assert_eq!(summary.completed_work[3].marker, WorkMarker::InProgress);
   204	        assert!(summary.completed_work[3].text.contains("unit-02"));
   205	    }
   206	
   207	    #[test]
   208	    fn test_golden_empty_session() {
   209	        let summary = parse("");
   210	        assert!(summary.phase.is_none());
   211	        assert!(summary.key_decisions.is_empty());
   212	        assert!(summary.completed_work.is_empty());
   213	    }
   214	}

 succeeded in 0ms:
     1	use crate::parser::models::AuditEntry;
     2	use crate::service::sanitizer::strip_ansi;
     3	
     4	/// Parse audit.md / devflow-audit.md content into Vec<AuditEntry>.
     5	///
     6	/// Supports two formats:
     7	/// - Brief: `[timestamp] stage — choice`
     8	/// - Detailed: `## Stage` + `**Timestamp**:` + metadata
     9	///
    10	/// Unrecognized lines are preserved as raw_line.
    11	pub fn parse(content: &str) -> Vec<AuditEntry> {
    12	    let content = strip_ansi(content);
    13	    let mut entries = Vec::new();
    14	    let mut current_stage: Option<String> = None;
    15	    let mut current_timestamp: Option<String> = None;
    16	    let mut current_choice: Option<String> = None;
    17	
    18	    for line in content.lines() {
    19	        let trimmed = line.trim();
    20	
    21	        if trimmed.is_empty() {
    22	            continue;
    23	        }
    24	
    25	        // Detailed format: ## Stage header
    26	        if let Some(stage) = trimmed.strip_prefix("## ") {
    27	            // Flush previous detailed entry
    28	            flush_detailed(
    29	                &mut entries,
    30	                &mut current_stage,
    31	                &mut current_timestamp,
    32	                &mut current_choice,
    33	            );
    34	            current_stage = Some(stage.trim().to_string());
    35	            continue;
    36	        }
    37	
    38	        // Inside detailed entry — check if we've left the block
    39	        if current_stage.is_some() {
    40	            if let Some(val) = trimmed.strip_prefix("**Timestamp**:") {
    41	                current_timestamp = Some(val.trim().to_string());
    42	                continue;
    43	            }
    44	            if let Some(val) = trimmed.strip_prefix("**User Input**:") {
    45	                current_choice = Some(val.trim().to_string());
    46	                continue;
    47	            }
    48	            if trimmed.starts_with("**") {
    49	                // Other metadata fields — skip
    50	                continue;
    51	            }
    52	            // Non-metadata line inside block: this means the detailed block has ended.
    53	            // Flush the current detailed entry, then process this line normally.
    54	            flush_detailed(
    55	                &mut entries,
    56	                &mut current_stage,
    57	                &mut current_timestamp,
    58	                &mut current_choice,
    59	            );
    60	            // Fall through to brief/unrecognized parsing below
    61	        }
    62	
    63	        // Brief format: [timestamp] stage — choice
    64	        if let Some(entry) = try_parse_brief(trimmed) {
    65	            entries.push(entry);
    66	            continue;
    67	        }
    68	
    69	        // Unrecognized line — preserve as raw
    70	        entries.push(AuditEntry {
    71	            timestamp: None,
    72	            stage: None,
    73	            choice: None,
    74	            raw_line: trimmed.to_string(),
    75	        });
    76	    }
    77	
    78	    // Flush last detailed entry
    79	    flush_detailed(
    80	        &mut entries,
    81	        &mut current_stage,
    82	        &mut current_timestamp,
    83	        &mut current_choice,
    84	    );
    85	
    86	    entries
    87	}
    88	
    89	fn flush_detailed(
    90	    entries: &mut Vec<AuditEntry>,
    91	    stage: &mut Option<String>,
    92	    timestamp: &mut Option<String>,
    93	    choice: &mut Option<String>,
    94	) {
    95	    if let Some(s) = stage.take() {
    96	        entries.push(AuditEntry {
    97	            timestamp: timestamp.take(),
    98	            stage: Some(s),
    99	            choice: choice.take(),
   100	            raw_line: String::new(),
   101	        });
   102	    }
   103	    *timestamp = None;
   104	    *choice = None;
   105	}
   106	
   107	fn try_parse_brief(line: &str) -> Option<AuditEntry> {
   108	    // Format: [timestamp] stage — choice
   109	    // or: [timestamp] stage → choice
   110	    let rest = line.strip_prefix('[')?;
   111	    let (ts, rest) = rest.split_once(']')?;
   112	    let rest = rest.trim();
   113	
   114	    let (stage, choice) = if let Some((s, c)) = rest.split_once('—') {
   115	        (Some(s.trim().to_string()), Some(c.trim().to_string()))
   116	    } else if let Some((s, c)) = rest.split_once('→') {
   117	        (Some(s.trim().to_string()), Some(c.trim().to_string()))
   118	    } else {
   119	        (Some(rest.to_string()), None)
   120	    };
   121	
   122	    Some(AuditEntry {
   123	        timestamp: Some(ts.trim().to_string()),
   124	        stage,
   125	        choice,
   126	        raw_line: line.to_string(),
   127	    })
   128	}
   129	
   130	#[cfg(test)]
   131	mod tests {
   132	    use super::*;
   133	
   134	    #[test]
   135	    fn test_golden_brief_audit() {
   136	        let content = r#"[2026-03-30T14:18] workspace-detection → B (Approve)
   137	[2026-03-30T14:19] complexity-declaration → Standard
   138	[2026-03-30T14:25] requirements-analysis — B (Approve, assumptions noted)
   139	"#;
   140	        let entries = parse(content);
   141	        assert_eq!(entries.len(), 3);
   142	        assert_eq!(entries[0].timestamp.as_deref(), Some("2026-03-30T14:18"));
   143	        assert_eq!(entries[0].stage.as_deref(), Some("workspace-detection"));
   144	        assert_eq!(entries[0].choice.as_deref(), Some("B (Approve)"));
   145	        assert_eq!(entries[2].choice.as_deref(), Some("B (Approve, assumptions noted)"));
   146	    }
   147	
   148	    #[test]
   149	    fn test_golden_detailed_audit() {
   150	        let content = r#"## workspace-detection
   151	**Timestamp**: 2026-03-30T14:18:00+09:00
   152	**User Input**: B (Approve)
   153	**AI Response**: Greenfield detected
   154	**Context**: Initial workspace analysis
   155	
   156	## complexity-declaration
   157	**Timestamp**: 2026-03-30T14:19:00+09:00
   158	**User Input**: Standard
   159	"#;
   160	        let entries = parse(content);
   161	        assert_eq!(entries.len(), 2);
   162	        assert_eq!(entries[0].stage.as_deref(), Some("workspace-detection"));
   163	        assert_eq!(
   164	            entries[0].timestamp.as_deref(),
   165	            Some("2026-03-30T14:18:00+09:00")
   166	        );
   167	        assert_eq!(entries[0].choice.as_deref(), Some("B (Approve)"));
   168	        assert_eq!(entries[1].stage.as_deref(), Some("complexity-declaration"));
   169	    }
   170	
   171	    #[test]
   172	    fn test_golden_mixed_audit() {
   173	        // Note: "Some unrecognized line here" is inside the detailed block
   174	        // (after **User Input** but before the next brief entry),
   175	        // so it gets absorbed by the detailed parser (skipped as other content).
   176	        // To test unrecognized lines, they must appear outside any ## block.
   177	        let content = r#"[2026-03-30T14:18] workspace-detection → B (Approve)
   178	
   179	## requirements-analysis
   180	**Timestamp**: 2026-03-30T14:25
   181	**User Input**: B (Approve)
   182	
   183	[2026-03-30T14:30] workflow-planning — B (Approve)
   184	Some unrecognized line outside blocks
   185	"#;
   186	        let entries = parse(content);
   187	        assert_eq!(entries.len(), 4);
   188	
   189	        // Brief entry
   190	        assert_eq!(entries[0].stage.as_deref(), Some("workspace-detection"));
   191	
   192	        // Detailed entry (flushed when next non-## line encountered outside block)
   193	        assert_eq!(entries[1].stage.as_deref(), Some("requirements-analysis"));
   194	        assert_eq!(entries[1].timestamp.as_deref(), Some("2026-03-30T14:25"));
   195	
   196	        // Brief entry
   197	        assert_eq!(entries[2].stage.as_deref(), Some("workflow-planning"));
   198	
   199	        // Unrecognized line (outside any block)
   200	        assert!(entries[3].stage.is_none());
   201	        assert_eq!(entries[3].raw_line, "Some unrecognized line outside blocks");
   202	    }
   203	
   204	    #[test]
   205	    fn test_empty_audit() {
   206	        let entries = parse("");
   207	        assert!(entries.is_empty());
   208	    }
   209	
   210	    #[test]
   211	    fn test_ansi_stripped_in_audit() {
   212	        let content = "[\x1b[36m2026-03-30\x1b[0m] \x1b[32mworkspace\x1b[0m → B";
   213	        let entries = parse(content);
   214	        assert_eq!(entries.len(), 1);
   215	        assert_eq!(entries[0].timestamp.as_deref(), Some("2026-03-30"));
   216	        assert_eq!(entries[0].stage.as_deref(), Some("workspace"));
   217	    }
   218	}

exec
/bin/zsh -lc "nl -ba src/demo.rs | sed -n '1,280p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	use std::time::{Duration, Instant};
     2	
     3	use crate::app::App;
     4	use crate::event::AppEvent;
     5	use crate::parser::models::{
     6	    AuditEntry, CompletedStage, Complexity, DiffStat, FlowState, GitChange, GitChangeStatus,
     7	    GitCommit, GitSnapshot, Phase, SessionSummary, WorkItem, WorkMarker,
     8	};
     9	
    10	/// Populate the app with sample data for --demo mode.
    11	pub fn populate_demo_data(app: &mut App) {
    12	    // FlowState
    13	    let flow_state = FlowState {
    14	        phase: Phase::Construction,
    15	        stage: "code-generation".to_string(),
    16	        complexity: Complexity::Standard,
    17	        completed_stages: vec![
    18	            CompletedStage {
    19	                name: "workspace-detection".to_string(),
    20	                timestamp: None,
    21	            },
    22	            CompletedStage {
    23	                name: "requirements-analysis".to_string(),
    24	                timestamp: None,
    25	            },
    26	            CompletedStage {
    27	                name: "workflow-planning".to_string(),
    28	                timestamp: None,
    29	            },
    30	        ],
    31	        active_unit: Some("Unit 11".to_string()),
    32	        ..Default::default()
    33	    };
    34	    app.handle_flow_state(flow_state);
    35	
    36	    // SessionSummary
    37	    let summary = SessionSummary {
    38	        completed_work: vec![
    39	            WorkItem {
    40	                text: "Phase 1: Foundation + Parser".to_string(),
    41	                marker: WorkMarker::Done,
    42	            },
    43	            WorkItem {
    44	                text: "Phase 2: Data Adapters".to_string(),
    45	                marker: WorkMarker::Done,
    46	            },
    47	            WorkItem {
    48	                text: "Phase 3: UI Panels".to_string(),
    49	                marker: WorkMarker::InProgress,
    50	            },
    51	        ],
    52	        key_decisions: vec![
    53	            "Approach B: layered unit decomposition".to_string(),
    54	            "ratatui 0.30 + crossterm 0.29".to_string(),
    55	        ],
    56	        next_steps: vec!["Phase 4: App Integration".to_string()],
    57	        ..Default::default()
    58	    };
    59	    app.workflow_map.set_session_summary(summary);
    60	
    61	    // GitSnapshot
    62	    let git_snapshot = GitSnapshot {
    63	        branch: "feature/unit-11".to_string(),
    64	        head: "abc1234".to_string(),
    65	        diff_stat: DiffStat {
    66	            additions: 420,
    67	            deletions: 35,
    68	        },
    69	        changes: vec![
    70	            GitChange {
    71	                status: GitChangeStatus::Staged,
    72	                path: "src/app.rs".to_string(),
    73	                additions: Some(200),
    74	                deletions: Some(0),
    75	            },
    76	            GitChange {
    77	                status: GitChangeStatus::Staged,
    78	                path: "src/event_loop.rs".to_string(),
    79	                additions: Some(80),
    80	                deletions: Some(0),
    81	            },
    82	            GitChange {
    83	                status: GitChangeStatus::Unstaged,
    84	                path: "src/main.rs".to_string(),
    85	                additions: Some(50),
    86	                deletions: Some(20),
    87	            },
    88	            GitChange {
    89	                status: GitChangeStatus::Untracked,
    90	                path: "src/demo.rs".to_string(),
    91	                additions: None,
    92	                deletions: None,
    93	            },
    94	            GitChange {
    95	                status: GitChangeStatus::Unstaged,
    96	                path: "Cargo.toml".to_string(),
    97	                additions: Some(5),
    98	                deletions: Some(2),
    99	            },
   100	        ],
   101	        commits: vec![
   102	            GitCommit {
   103	                hash: "abc1234".to_string(),
   104	                message: "feat: add App struct and event loop".to_string(),
   105	            },
   106	            GitCommit {
   107	                hash: "def5678".to_string(),
   108	                message: "feat: add TestBackend render tests".to_string(),
   109	            },
   110	            GitCommit {
   111	                hash: "ghi9012".to_string(),
   112	                message: "refactor: render(&mut self) migration".to_string(),
   113	            },
   114	        ],
   115	        worktrees: Vec::new(),
   116	    };
   117	    app.handle_git_snapshot(git_snapshot);
   118	
   119	    // AuditLog entries
   120	    let audit_entries: Vec<AuditEntry> = (0..10)
   121	        .map(|i| AuditEntry {
   122	            timestamp: Some(format!("2026-03-31T14:{i:02}:00")),
   123	            stage: Some(
   124	                [
   125	                    "workspace-detection",
   126	                    "requirements-analysis",
   127	                    "user-stories",
   128	                    "workflow-planning",
   129	                    "application-design",
   130	                    "units-generation",
   131	                    "code-generation",
   132	                    "build-and-test",
   133	                    "code-generation",
   134	                    "build-and-test",
   135	                ][i]
   136	                .to_string(),
   137	            ),
   138	            choice: Some(if i % 3 == 0 {
   139	                "B (Approve)".to_string()
   140	            } else {
   141	                "B".to_string()
   142	            }),
   143	            raw_line: String::new(),
   144	        })
   145	        .collect();
   146	    app.audit_log.set_entries(audit_entries);
   147	
   148	    // Agents — simulate hooks active + agents
   149	    app.hooks_active = true;
   150	    app.hooks_port = Some(9100);
   151	    app.agent_status.set_hooks_active(true);
   152	
   153	    // Running agent
   154	    app.handle_event(AppEvent::AgentStarted {
   155	        agent_id: "explore-1".to_string(),
   156	        agent_type: "Explore".to_string(),
   157	    });
   158	
   159	    // Done agent (start then stop)
   160	    app.handle_event(AppEvent::AgentStarted {
   161	        agent_id: "plan-1".to_string(),
   162	        agent_type: "Plan".to_string(),
   163	    });
   164	    app.handle_event(AppEvent::AgentStopped {
   165	        agent_id: "plan-1".to_string(),
   166	    });
   167	
   168	    // Timeout agent (manually create old entry)
   169	    app.handle_event(AppEvent::AgentStarted {
   170	        agent_id: "old-agent".to_string(),
   171	        agent_type: "code-review".to_string(),
   172	    });
   173	}
   174	
   175	#[cfg(test)]
   176	mod tests {
   177	    use super::*;
   178	    use std::path::PathBuf;
   179	    use tokio::sync::mpsc;
   180	
   181	    #[test]
   182	    fn test_populate_demo_no_panic() {
   183	        let (tx, _rx) = mpsc::channel(16);
   184	        let mut app = App::new(120, 30, tx, "token".to_string(), PathBuf::from("/tmp"));
   185	        populate_demo_data(&mut app);
   186	        assert_eq!(app.phase, "CONSTRUCTION");
   187	        assert!(app.hooks_active);
   188	        assert!(app.agent_status.has_running_agents());
   189	    }
   190	}

exec
/bin/zsh -lc "cat clippy.toml && echo '---' && cat Cargo.toml" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
allow-unwrap-in-tests = true
allow-expect-in-tests = true
---
[package]
name = "devflow-tui"
version = "0.1.0"
edition = "2024"

[dependencies]
ratatui = "0.30"
crossterm = { version = "0.29", features = ["event-stream"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time", "process", "io-util", "fs"] }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
notify = "8"
thiserror = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-appender = "0.2"
chrono = { version = "0.4", features = ["serde"] }
futures = "0.3"
dirs = "6"
sha2 = "0.10"
hex = "0.4"
tokio-util = "0.7"

[dev-dependencies]
tempfile = "3"
tower = "0.5"
http-body-util = "0.1"

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true

[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
enum_glob_use = "deny"

codex
I’m running the test and lint baseline now to ground the review in actual warnings/failures, then I’ll deliver severity-ranked findings with exact file/line references.
exec
/bin/zsh -lc 'cargo test -q' in /Users/jay.ahn/projects/backend/devflow-tui
exec
/bin/zsh -lc 'cargo clippy -q --all-targets --all-features' in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 409ms:
warning: unused imports: `Duration` and `Instant`
 --> src/demo.rs:1:17
  |
1 | use std::time::{Duration, Instant};
  |                 ^^^^^^^^  ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: field `event_tx` is never read
  --> src/app.rs:62:5
   |
42 | pub struct App {
   |            --- field in this struct
...
62 |     event_tx: mpsc::Sender<AppEvent>,
   |     ^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `buffer_row_text` is never used
  --> src/test_helpers.rs:53:8
   |
53 | pub fn buffer_row_text(buf: &Buffer, row: u16) -> String {
   |        ^^^^^^^^^^^^^^^

warning: field assignment outside of initializer for an instance created with Default::default()
   --> src/config.rs:165:9
    |
165 |         config.project_dir = PathBuf::from("/home/user/project");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: consider initializing the variable with `config::AppConfig { project_dir: PathBuf::from("/home/user/project"), ..Default::default() }` and removing relevant reassignments
   --> src/config.rs:164:9
    |
164 |         let mut config = AppConfig::default();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#field_reassign_with_default
    = note: `#[warn(clippy::field_reassign_with_default)]` on by default


 succeeded in 3635ms:
warning: unused imports: `Duration` and `Instant`
 --> src/demo.rs:1:17
  |
1 | use std::time::{Duration, Instant};
  |                 ^^^^^^^^  ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: field `event_tx` is never read
  --> src/app.rs:62:5
   |
42 | pub struct App {
   |            --- field in this struct
...
62 |     event_tx: mpsc::Sender<AppEvent>,
   |     ^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `buffer_row_text` is never used
  --> src/test_helpers.rs:53:8
   |
53 | pub fn buffer_row_text(buf: &Buffer, row: u16) -> String {
   |        ^^^^^^^^^^^^^^^


running 182 tests
....................................................................................... 87/182
....................................................................................... 174/182
........
test result: ok. 182 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


exec
/bin/zsh -lc "nl -ba src/test_helpers.rs | sed -n '1,220p'" in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
     1	#![cfg(test)]
     2	
     3	use ratatui::backend::TestBackend;
     4	use ratatui::buffer::Buffer;
     5	use ratatui::layout::Rect;
     6	use ratatui::Terminal;
     7	
     8	use crate::component::Component;
     9	
    10	/// Component 트레이트 구현체를 TestBackend에 렌더링하고 Terminal을 반환한다.
    11	#[allow(clippy::unwrap_used)]
    12	pub fn render_component(
    13	    component: &mut dyn Component,
    14	    width: u16,
    15	    height: u16,
    16	    focused: bool,
    17	) -> Terminal<TestBackend> {
    18	    let backend = TestBackend::new(width, height);
    19	    let mut terminal = Terminal::new(backend).unwrap();
    20	    terminal
    21	        .draw(|frame| {
    22	            let area = frame.area();
    23	            component.render(frame, area, focused);
    24	        })
    25	        .unwrap();
    26	    terminal
    27	}
    28	
    29	/// 자유 함수(header::render 등)를 클로저로 받아 TestBackend에 렌더링한다.
    30	#[allow(clippy::unwrap_used)]
    31	pub fn render_with<F>(width: u16, height: u16, f: F) -> Terminal<TestBackend>
    32	where
    33	    F: FnOnce(&mut ratatui::Frame, Rect),
    34	{
    35	    let backend = TestBackend::new(width, height);
    36	    let mut terminal = Terminal::new(backend).unwrap();
    37	    terminal
    38	        .draw(|frame| {
    39	            let area = frame.area();
    40	            f(frame, area);
    41	        })
    42	        .unwrap();
    43	    terminal
    44	}
    45	
    46	/// 버퍼의 전체 텍스트에서 needle 문자열이 포함되어 있는지 확인한다.
    47	pub fn buffer_contains_str(buf: &Buffer, needle: &str) -> bool {
    48	    let full_text = buffer_full_text(buf);
    49	    full_text.contains(needle)
    50	}
    51	
    52	/// 특정 행의 텍스트를 추출한다 (trailing 공백 제거, wide char 패딩 스킵).
    53	pub fn buffer_row_text(buf: &Buffer, row: u16) -> String {
    54	    let area = buf.area();
    55	    let mut text = String::new();
    56	    let mut col = area.x;
    57	    while col < area.x + area.width {
    58	        let cell = &buf[(col, row)];
    59	        let sym = cell.symbol();
    60	        text.push_str(sym);
    61	        col += display_width_of_first_char(sym);
    62	    }
    63	    text.trim_end().to_string()
    64	}
    65	
    66	/// 첫 번째 문자의 display width를 반환한다 (CJK/한글 = 2, 나머지 = 1).
    67	fn display_width_of_first_char(s: &str) -> u16 {
    68	    s.chars().next().map_or(1, |c| {
    69	        if ('\u{1100}'..='\u{115F}').contains(&c)
    70	            || ('\u{2E80}'..='\u{A4CF}').contains(&c)
    71	            || ('\u{AC00}'..='\u{D7A3}').contains(&c)
    72	            || ('\u{F900}'..='\u{FAFF}').contains(&c)
    73	            || ('\u{FE10}'..='\u{FE6F}').contains(&c)
    74	            || ('\u{FF01}'..='\u{FF60}').contains(&c)
    75	            || ('\u{FFE0}'..='\u{FFE6}').contains(&c)
    76	            || c > '\u{1F000}'
    77	        {
    78	            2
    79	        } else {
    80	            1
    81	        }
    82	    })
    83	}
    84	
    85	/// 버퍼 전체 텍스트를 한 문자열로 합친다.
    86	/// Wide character 뒤의 패딩 셀을 건너뛰어 연속된 텍스트로 만든다.
    87	fn buffer_full_text(buf: &Buffer) -> String {
    88	    let area = buf.area();
    89	    let mut text = String::new();
    90	    for row in area.y..area.y + area.height {
    91	        let mut col = area.x;
    92	        while col < area.x + area.width {
    93	            let cell = &buf[(col, row)];
    94	            let sym = cell.symbol();
    95	            text.push_str(sym);
    96	            col += display_width_of_first_char(sym);
    97	        }
    98	    }
    99	    text
   100	}

exec
/bin/zsh -lc 'rg -n "OpenArtifactModal|ArtifactListChanged|GitStatusUpdated|FlowStateChanged" src -S' in /Users/jay.ahn/projects/backend/devflow-tui
 succeeded in 0ms:
src/event.rs:8:    FlowStateChanged(FlowState),
src/event.rs:11:    ArtifactListChanged(Vec<ArtifactFile>),
src/event.rs:14:    GitStatusUpdated(GitSnapshot),
src/action.rs:17:    OpenArtifactModal,
src/action.rs:40:            Self::OpenArtifactModal => "open_artifact_modal",
src/app.rs:285:            AppEvent::FlowStateChanged(state) => {
src/adapter/file_watcher.rs:152:        .send(AppEvent::ArtifactListChanged(artifacts))
src/panel/git_status.rs:77:        if let AppEvent::GitStatusUpdated(snapshot) = event {
src/panel/workflow_map.rs:136:            KeyCode::Enter => Some(Action::OpenArtifactModal),
src/panel/workflow_map.rs:143:            AppEvent::FlowStateChanged(state) => self.set_flow_state(state.clone()),

codex
## Critical
- No critical issues found.

## High
- **`AdapterHandle::shutdown` timeout path does not abort the task (potential detached/leaking task).** In timeout, the `JoinHandle` is moved into `timeout(...)` and lost on `Err(_)`, so the task is detached instead of aborted; the comment claims Drop will abort, but `self.join` is already `None`. See [src/adapter/handle.rs:53](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/handle.rs:53), [src/adapter/handle.rs:57](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/handle.rs:57), [src/adapter/handle.rs:67](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/handle.rs:67), [src/adapter/handle.rs:72](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/handle.rs:72).
- **Hook “copy config” behavior is wrong on mismatch path.** UI says press `c` to copy new settings, but mismatch stores a diagnostic string/URL and `copy_hooks_snippet` copies that string instead of generated JSON snippet. See [src/app.rs:335](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:335), [src/app.rs:337](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:337), [src/service/hook_config.rs:50](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:50), [src/service/hook_config.rs:63](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:63).

## Medium
- **Advertised `Enter` behavior is dropped.** `WorkflowMapPanel` emits `Action::OpenArtifactModal`, but `App::execute_action` has no handler, so the action is silently ignored. See [src/panel/workflow_map.rs:136](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/workflow_map.rs:136), [src/action.rs:17](/Users/jay.ahn/projects/backend/devflow-tui/src/action.rs:17), [src/app.rs:248](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:248).
- **Navigation hints and actual behavior diverge.** Help/status suggest directional `Ctrl+h/j/k/l`, but app logic only handles left/right and no-ops up/down. See [src/ui/status_bar.rs:22](/Users/jay.ahn/projects/backend/devflow-tui/src/ui/status_bar.rs:22), [src/ui/help_overlay.rs:28](/Users/jay.ahn/projects/backend/devflow-tui/src/ui/help_overlay.rs:28), [src/app.rs:255](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:255), [src/app.rs:261](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:261).
- **Adapter failure warnings can spam every loop iteration.** Finished adapters are warned on every pass without dedupe/throttling, creating noisy logs and masking signal. See [src/event_loop.rs:89](/Users/jay.ahn/projects/backend/devflow-tui/src/event_loop.rs:89).
- **Synchronous disk I/O in app event path can block UI responsiveness.** Hook config check performs blocking `std::fs::read_to_string` from event handling. See [src/app.rs:279](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:279), [src/service/hook_config.rs:26](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:26).
- **Hook config validation is brittle substring matching.** It can mis-detect config status and gives weak diagnostics; parsing JSON schema would be more robust. See [src/service/hook_config.rs:36](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:36), [src/service/hook_config.rs:56](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:56).
- **Event model has drift/unhandled paths.** Runtime mostly uses watch channels for flow/git, but event variants still exist and artifact events are emitted without consumers. This increases complexity and confuses ownership. See [src/event_loop.rs:60](/Users/jay.ahn/projects/backend/devflow-tui/src/event_loop.rs:60), [src/event.rs:8](/Users/jay.ahn/projects/backend/devflow-tui/src/event.rs:8), [src/event.rs:11](/Users/jay.ahn/projects/backend/devflow-tui/src/event.rs:11), [src/adapter/file_watcher.rs:152](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/file_watcher.rs:152).

## Low
- **Dead code / lint noise indicates cleanup debt.** Unused imports, unused `App.event_tx`, and unused helper function. See [src/demo.rs:1](/Users/jay.ahn/projects/backend/devflow-tui/src/demo.rs:1), [src/app.rs:62](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:62), [src/test_helpers.rs:53](/Users/jay.ahn/projects/backend/devflow-tui/src/test_helpers.rs:53).
- **Non-idiomatic parameter types and repeated panel chrome code.** `&PathBuf` where `&Path` is more idiomatic, and repeated border/title style setup across panels hurts DRY. See [src/adapter/git_poller.rs:51](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/git_poller.rs:51), [src/adapter/git_poller.rs:94](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/git_poller.rs:94), [src/panel/workflow_map.rs:151](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/workflow_map.rs:151), [src/panel/git_status.rs:83](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/git_status.rs:83), [src/panel/agent_status.rs:201](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/agent_status.rs:201), [src/panel/audit_log.rs:156](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/audit_log.rs:156).

## Open Questions / Assumptions
- Is `OpenArtifactModal` intentionally deferred to Unit 12? If yes, current key hints should be gated to avoid promising unavailable behavior.
- Should `c` always copy a full generated hook snippet (including mismatch cases), or intentionally copy the currently detected mismatched URL/details?

## Baseline Check
- `cargo test -q`: pass (182 tests).
- `cargo clippy -q --all-targets --all-features`: pass with warnings noted above.
tokens used
105,658
## Critical
- No critical issues found.

## High
- **`AdapterHandle::shutdown` timeout path does not abort the task (potential detached/leaking task).** In timeout, the `JoinHandle` is moved into `timeout(...)` and lost on `Err(_)`, so the task is detached instead of aborted; the comment claims Drop will abort, but `self.join` is already `None`. See [src/adapter/handle.rs:53](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/handle.rs:53), [src/adapter/handle.rs:57](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/handle.rs:57), [src/adapter/handle.rs:67](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/handle.rs:67), [src/adapter/handle.rs:72](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/handle.rs:72).
- **Hook “copy config” behavior is wrong on mismatch path.** UI says press `c` to copy new settings, but mismatch stores a diagnostic string/URL and `copy_hooks_snippet` copies that string instead of generated JSON snippet. See [src/app.rs:335](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:335), [src/app.rs:337](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:337), [src/service/hook_config.rs:50](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:50), [src/service/hook_config.rs:63](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:63).

## Medium
- **Advertised `Enter` behavior is dropped.** `WorkflowMapPanel` emits `Action::OpenArtifactModal`, but `App::execute_action` has no handler, so the action is silently ignored. See [src/panel/workflow_map.rs:136](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/workflow_map.rs:136), [src/action.rs:17](/Users/jay.ahn/projects/backend/devflow-tui/src/action.rs:17), [src/app.rs:248](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:248).
- **Navigation hints and actual behavior diverge.** Help/status suggest directional `Ctrl+h/j/k/l`, but app logic only handles left/right and no-ops up/down. See [src/ui/status_bar.rs:22](/Users/jay.ahn/projects/backend/devflow-tui/src/ui/status_bar.rs:22), [src/ui/help_overlay.rs:28](/Users/jay.ahn/projects/backend/devflow-tui/src/ui/help_overlay.rs:28), [src/app.rs:255](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:255), [src/app.rs:261](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:261).
- **Adapter failure warnings can spam every loop iteration.** Finished adapters are warned on every pass without dedupe/throttling, creating noisy logs and masking signal. See [src/event_loop.rs:89](/Users/jay.ahn/projects/backend/devflow-tui/src/event_loop.rs:89).
- **Synchronous disk I/O in app event path can block UI responsiveness.** Hook config check performs blocking `std::fs::read_to_string` from event handling. See [src/app.rs:279](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:279), [src/service/hook_config.rs:26](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:26).
- **Hook config validation is brittle substring matching.** It can mis-detect config status and gives weak diagnostics; parsing JSON schema would be more robust. See [src/service/hook_config.rs:36](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:36), [src/service/hook_config.rs:56](/Users/jay.ahn/projects/backend/devflow-tui/src/service/hook_config.rs:56).
- **Event model has drift/unhandled paths.** Runtime mostly uses watch channels for flow/git, but event variants still exist and artifact events are emitted without consumers. This increases complexity and confuses ownership. See [src/event_loop.rs:60](/Users/jay.ahn/projects/backend/devflow-tui/src/event_loop.rs:60), [src/event.rs:8](/Users/jay.ahn/projects/backend/devflow-tui/src/event.rs:8), [src/event.rs:11](/Users/jay.ahn/projects/backend/devflow-tui/src/event.rs:11), [src/adapter/file_watcher.rs:152](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/file_watcher.rs:152).

## Low
- **Dead code / lint noise indicates cleanup debt.** Unused imports, unused `App.event_tx`, and unused helper function. See [src/demo.rs:1](/Users/jay.ahn/projects/backend/devflow-tui/src/demo.rs:1), [src/app.rs:62](/Users/jay.ahn/projects/backend/devflow-tui/src/app.rs:62), [src/test_helpers.rs:53](/Users/jay.ahn/projects/backend/devflow-tui/src/test_helpers.rs:53).
- **Non-idiomatic parameter types and repeated panel chrome code.** `&PathBuf` where `&Path` is more idiomatic, and repeated border/title style setup across panels hurts DRY. See [src/adapter/git_poller.rs:51](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/git_poller.rs:51), [src/adapter/git_poller.rs:94](/Users/jay.ahn/projects/backend/devflow-tui/src/adapter/git_poller.rs:94), [src/panel/workflow_map.rs:151](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/workflow_map.rs:151), [src/panel/git_status.rs:83](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/git_status.rs:83), [src/panel/agent_status.rs:201](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/agent_status.rs:201), [src/panel/audit_log.rs:156](/Users/jay.ahn/projects/backend/devflow-tui/src/panel/audit_log.rs:156).

## Open Questions / Assumptions
- Is `OpenArtifactModal` intentionally deferred to Unit 12? If yes, current key hints should be gated to avoid promising unavailable behavior.
- Should `c` always copy a full generated hook snippet (including mismatch cases), or intentionally copy the currently detected mismatched URL/details?

## Baseline Check
- `cargo test -q`: pass (182 tests).
- `cargo clippy -q --all-targets --all-features`: pass with warnings noted above.
