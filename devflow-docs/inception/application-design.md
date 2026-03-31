# Application Design

## Part 1: 구조 설계

### 모듈 구조

```
devflow-tui/src/
├── main.rs                    # 엔트리: 터미널 셋업, 로깅, 이벤트 루프 시작
├── app.rs                     # App 상태: FocusPane, InputMode, 패널 라우팅, render
├── lib.rs                     # 공개 모듈 export
├── event.rs                   # AppEvent enum (데이터 소스 → UI)
├── action.rs                  # Action enum (UI → 직접 처리 또는 CommandRunner)
├── command.rs                 # CommandRunner: async side effect 실행 (clipboard, refresh)
├── event_loop.rs              # tokio::select 메인 루프
├── config.rs                  # CLI 옵션 + 환경변수 + 설정
├── error.rs                   # AppError, Result 타입
│
├── component.rs               # Component trait 정의
│
├── parser/                    # 파일 파서 (순수 함수, I/O 없음)
│   ├── mod.rs
│   ├── devflow_state.rs       # devflow-state.md 파서
│   ├── session_summary.rs     # session-summary.md 파서
│   ├── audit_log.rs           # audit.md / devflow-audit.md 파서
│   └── models.rs              # 파서 출력 도메인 모델
│
├── adapter/                   # 데이터 소스 어댑터 (Port 구현체)
│   ├── mod.rs
│   ├── file_watcher.rs        # notify + debounce → FileChanged 이벤트
│   ├── git_poller.rs          # tokio::interval + git CLI → GitPolled 이벤트
│   └── hooks_server.rs        # axum HTTP 서버 → HookReceived 이벤트
│
├── port/                      # 포트 (trait 정의)
│   ├── mod.rs
│   ├── watcher.rs             # FileWatcher trait
│   ├── git.rs                 # GitProvider trait
│   ├── hooks.rs               # HooksReceiver trait
│   └── mock.rs                # 테스트용 mock 구현
│
├── service/                   # 비즈니스 로직
│   ├── mod.rs
│   ├── hook_config.rs         # Hook Config Detection 서비스
│   ├── token.rs               # 프로젝트 기반 stable token 생성/검증
│   └── sanitizer.rs           # ANSI escape 제거
│
├── panel/                     # UI 패널 (Component 구현체)
│   ├── mod.rs
│   ├── workflow_map.rs        # 워크플로우 진행 맵 (FR-1)
│   ├── git_status.rs          # Git 상태 패널 (FR-2)
│   ├── agent_status.rs        # 에이전트 상태 패널 (FR-3)
│   ├── audit_log.rs           # 감사 로그 뷰 (FR-4)
│   ├── artifact_preview.rs    # [P1] 산출물 미리보기 (FR-5)
│   └── gate_alert.rs          # [P1] 게이트 알림 (FR-6)
│
├── ui/                        # 공통 UI 위젯
│   ├── mod.rs
│   ├── layout.rs              # LayoutManager (표준/와이드/축소)
│   ├── theme.rs               # 컬러 팔레트 + NO_COLOR 추상화 + Stylize 헬퍼
│   ├── status_bar.rs          # 상태바 + 단축키 힌트
│   ├── header.rs              # 헤더 (Phase 표시 + hooks 상태)
│   └── help_overlay.rs        # ? 키 도움말 오버레이
│
└── demo.rs                    # --demo 모드 샘플 데이터
```

### 핵심 Trait 정의

#### Component Trait
```rust
pub trait Component {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action>;
    fn handle_event(&mut self, event: &AppEvent);
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool);
}
```
> nexttui 대비 변경: `render`에 `focused: bool` 추가 (포커스 패널 테두리 색상 구분)

#### Port Traits — enum dispatch 전략
테스트 용이성을 위해 trait object(`dyn`) 대신 enum dispatch 패턴 사용. async trait의 object safety 문제를 회피.

```rust
// Git 데이터 포트 — enum dispatch
pub enum GitProvider {
    Cli(CliGitProvider),
    Mock(MockGitProvider),
}

impl GitProvider {
    pub async fn snapshot(&self) -> Result<GitSnapshot> {
        match self {
            Self::Cli(p) => p.snapshot().await,
            Self::Mock(p) => p.snapshot().await,
        }
    }
}
```

#### 장수명 어댑터 — CancellationToken + JoinHandle
FileWatcher와 HooksServer는 장수명 백그라운드 태스크. lifecycle을 명시적으로 관리.

```rust
pub struct AdapterHandle {
    cancel: CancellationToken,
    join: JoinHandle<Result<()>>,
}

impl AdapterHandle {
    /// 어댑터 시작. 내부에서 tokio::spawn + CancellationToken 전달.
    pub fn spawn<F, Fut>(f: F) -> Self
    where
        F: FnOnce(CancellationToken) -> Fut,
        Fut: Future<Output = Result<()>> + Send + 'static,
    { ... }

    /// Graceful shutdown. cancel 신호 후 join 대기 (timeout 3초).
    pub async fn shutdown(self) -> Result<()> { ... }

    /// JoinHandle 감시용. panic/에러 시 AppEvent 전파.
    pub fn is_finished(&self) -> bool { ... }
}
```

**사용 예:**
```rust
// FileWatcher 시작
let watcher_handle = AdapterHandle::spawn(|cancel| async move {
    file_watcher::run(cancel, paths, event_tx).await
});

// HooksServer 시작
let hooks_handle = AdapterHandle::spawn(|cancel| async move {
    hooks_server::run(cancel, port, token, event_tx).await
});
```

### CommandRunner — 경량 async side effect 실행

Action 중 async 처리가 필요한 것(clipboard, refresh)을 위한 경량 실행기.

```rust
pub struct CommandRunner {
    event_tx: mpsc::Sender<AppEvent>,
}

impl CommandRunner {
    pub fn execute(&self, action: Action) {
        let tx = self.event_tx.clone();
        tokio::spawn(async move {
            let result = match action {
                Action::CopyToClipboard(text) => copy_to_clipboard(&text).await,
                Action::Refresh => { /* 파일 재읽기 트리거 */ Ok(()) },
                _ => return, // 동기 액션은 여기 오지 않음
            };
            match result {
                Ok(()) => { let _ = tx.send(AppEvent::CommandCompleted { action_name: action.name() }).await; }
                Err(e) => { let _ = tx.send(AppEvent::CommandFailed { action_name: action.name(), error: e.to_string() }).await; }
            }
        });
    }
}
```

### 이벤트/액션 모델

#### AppEvent (데이터 → UI)
```rust
pub enum AppEvent {
    // 파일 감시
    FlowStateChanged(FlowState),
    SessionSummaryChanged(SessionSummary),
    AuditLogAppended(Vec<AuditEntry>),
    ArtifactListChanged(Vec<ArtifactFile>),

    // Git 폴링
    GitStatusUpdated(GitSnapshot),

    // Hooks
    AgentStarted { agent_id: String, agent_type: String },
    AgentStopped { agent_id: String },
    ToolUseStarted { tool_name: String },
    ToolUseCompleted { tool_name: String },
    TurnCompleted { last_message: String },

    // 시스템
    HooksServerStarted { port: u16 },
    HooksServerFailed { reason: String },
    FileWatcherError { path: PathBuf, error: String },
    GitPollError { error: String },
    ParseError { file: String, error: String },
    CommandCompleted { action_name: String },
    CommandFailed { action_name: String, error: String },

    // tick은 이벤트로 전달하지 않음 — event_loop에서 직접 처리
}
```

#### Action (UI → App/CommandRunner)
```rust
pub enum Action {
    // 네비게이션
    FocusNextPanel,
    FocusPrevPanel,
    FocusDirection(Direction),  // Ctrl+h/j/k/l 방향 네비게이션
    ExpandPanel,
    CollapsePanel,
    OpenArtifactModal,          // 표준 레이아웃에서 Enter로 산출물 모달

    // 패널 내 조작
    ScrollUp,
    ScrollDown,
    Select,

    // 비동기 (CommandRunner로 위임)
    Refresh,
    CopyToClipboard(String),

    // 시스템
    Quit,
}

pub enum Direction {
    Up, Down, Left, Right,
}
```

### 이벤트 채널 아키텍처

단일 `mpsc` 대신, 데이터 특성에 맞는 채널 분리:

```rust
// 상태형 데이터 — tokio::watch (최신 값만 유지, coalescing 자동)
let (flow_state_tx, flow_state_rx) = watch::channel(FlowState::default());
let (git_snapshot_tx, git_snapshot_rx) = watch::channel(GitSnapshot::default());

// 이산형 이벤트 — bounded mpsc (순서 보장, backpressure)
let (event_tx, event_rx) = mpsc::channel::<AppEvent>(256);
```

**이벤트 루프에서 통합:**
```rust
tokio::select! {
    key = key_events.next() => { ... }
    _ = tick.tick() => { app.on_tick(); }
    Ok(()) = flow_state_rx.changed() => {
        app.handle_flow_state(*flow_state_rx.borrow());
        needs_render = true;
    }
    Ok(()) = git_snapshot_rx.changed() => {
        app.handle_git_snapshot(*git_snapshot_rx.borrow());
        needs_render = true;
    }
    event = event_rx.recv() => {
        if let Some(ev) = event {
            app.handle_event(ev);
            needs_render = true;
        }
    }
}
```

**backpressure 정책:**
- `mpsc` 채널 256 버퍼. full 시 `try_send` 실패 → oldest drop + tracing::warn
- `watch` 채널은 자동 coalescing (중간 값 버림, 최신 값만 유지)

### 이벤트 루프 구조 (수정)

```rust
pub async fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
    mut event_rx: mpsc::Receiver<AppEvent>,
    mut flow_state_rx: watch::Receiver<FlowState>,
    mut git_snapshot_rx: watch::Receiver<GitSnapshot>,
    adapter_handles: Vec<AdapterHandle>,
) -> Result<()> {
    let mut key_events = EventStream::new();
    let tick_rate = Duration::from_millis(50);
    let mut tick = tokio::time::interval(tick_rate);
    let mut needs_render = true;

    loop {
        tokio::select! {
            // Branch 1: 키 입력 + 리사이즈
            Some(Ok(event)) = key_events.next() => {
                match event {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        needs_render = app.handle_key(key);
                    }
                    Event::Resize(w, h) => {
                        app.on_resize(w, h);
                        needs_render = true;
                    }
                    _ => {}
                }
            }

            // Branch 2: tick — 시간 기반 상태 업데이트
            _ = tick.tick() => {
                if app.on_tick() {
                    needs_render = true;
                }
            }

            // Branch 3: 상태형 채널 (flow state)
            Ok(()) = flow_state_rx.changed() => {
                app.handle_flow_state(flow_state_rx.borrow().clone());
                needs_render = true;
            }

            // Branch 4: 상태형 채널 (git snapshot)
            Ok(()) = git_snapshot_rx.changed() => {
                app.handle_git_snapshot(git_snapshot_rx.borrow().clone());
                needs_render = true;
            }

            // Branch 5: 이산형 이벤트 채널
            event = event_rx.recv() => {
                match event {
                    Some(ev) => {
                        app.handle_event(ev);
                        needs_render = true;
                    }
                    None => { app.should_quit = true; }
                }
            }
        }

        // Adapter supervisor: panic/에러 감지
        for handle in &adapter_handles {
            if handle.is_finished() {
                // JoinHandle 완료 = adapter crash
                // AppEvent::AdapterCrashed 처리 → 사용자 알림
            }
        }

        // Conditional render (no-change frame skip)
        if needs_render {
            terminal.draw(|f| app.render(f))?;
            needs_render = false;
        }

        if app.should_quit { break; }
    }

    // Graceful shutdown
    for handle in adapter_handles {
        handle.shutdown().await.ok();
    }

    Ok(())
}
```

**`App::on_tick()` 역할:**
- 에이전트 elapsed time 갱신 (`12s` → `13s`)
- 게이트 알림 깜빡임 토글 (Yellow ↔ White, 1초 주기)
- orphan 에이전트 timeout 체크 (60초)
- 변경 발생 시 `true` 반환, 아니면 `false` (no-change frame skip)

### App 상태 구조

```rust
pub enum FocusPane {
    WorkflowMap,
    GitStatus,
    AgentStatus,
    AuditLog,
    ArtifactPreview,  // 와이드 레이아웃에서만
    GateAlert,        // 와이드 레이아웃에서만
}

pub enum InputMode {
    Normal,         // 패널 탐색
    Expanded,       // 패널 전체화면
    HelpOverlay,    // ? 도움말
    ArtifactModal,  // 표준 레이아웃에서 산출물 모달
}

pub struct App {
    pub should_quit: bool,
    pub input_mode: InputMode,
    pub focus: FocusPane,

    // 패널 컴포넌트
    workflow_map: WorkflowMapPanel,
    git_status: GitStatusPanel,
    agent_status: AgentStatusPanel,
    audit_log: AuditLogPanel,
    artifact_preview: ArtifactPreviewPanel,  // 항상 존재, 표시 여부는 레이아웃이 결정
    gate_alert: GateAlertPanel,             // 항상 존재, 표시 여부는 레이아웃이 결정

    // 인프라
    layout: LayoutManager,
    status_bar: StatusBar,
    header: Header,
    hooks_active: bool,
    hooks_port: Option<u16>,
    command_runner: CommandRunner,

    // 이벤트
    event_tx: mpsc::Sender<AppEvent>,
}

impl App {
    /// Tab 순환 시 현재 레이아웃에서 사용 가능한 패널만 순환
    fn available_panels(&self) -> Vec<FocusPane> {
        match self.layout.mode() {
            LayoutMode::Compact => vec![WorkflowMap, GitStatus, AgentStatus, AuditLog],
            LayoutMode::Standard => vec![WorkflowMap, GitStatus, AgentStatus, AuditLog],
            LayoutMode::Wide => vec![WorkflowMap, GitStatus, ArtifactPreview, AgentStatus, AuditLog, GateAlert],
        }
    }

    /// 방향 네비게이션 (와이드 레이아웃 2x3 그리드)
    fn focus_direction(&mut self, dir: Direction) {
        // 와이드: 2행 x 3열 그리드 매핑
        // 표준/축소: Up/Down은 noop, Left/Right는 Tab과 동일
    }

    /// 포커스 안전성: 현재 포커스가 사용 불가 패널이면 첫 번째 가용 패널로 이동
    fn ensure_valid_focus(&mut self) {
        let available = self.available_panels();
        if !available.contains(&self.focus) {
            self.focus = available[0];
        }
    }
}
```

### 에러 Taxonomy

```rust
pub enum AppError {
    // 인프라
    Io(std::io::Error),
    Terminal(String),

    // 파서
    ParseFlowState { file: PathBuf, detail: String },
    ParseSessionSummary { file: PathBuf, detail: String },
    ParseAuditLog { file: PathBuf, detail: String },

    // Git
    GitCommand { command: String, stderr: String },
    GitTimeout { command: String },

    // Hooks
    HooksServerBind { port: u16, reason: String },
    HooksTokenMismatch,
    HooksPayloadInvalid { detail: String },

    // 서비스
    ClipboardUnavailable,
    TokenGeneration(String),
    ConfigRead { path: PathBuf, detail: String },
}
```

### 데이터 흐름 (수정)

```
┌──────────────────────────────────────────────────────────────┐
│                      Event Sources                            │
│                                                               │
│  ┌──────────┐   ┌──────────┐   ┌──────────────────┐         │
│  │  notify   │   │  git CLI │   │  axum HTTP       │         │
│  │  watcher  │   │  poller  │   │  hooks server    │         │
│  └────┬─────┘   └────┬─────┘   └────────┬─────────┘         │
│       │               │                  │                    │
│  watch::Sender    watch::Sender     mpsc::Sender             │
│  (FlowState,      (GitSnapshot)     (AppEvent)               │
│   SessionSummary)                                             │
│       │               │                  │                    │
└───────┼───────────────┼──────────────────┼────────────────────┘
        ▼               ▼                  ▼
┌────────────────────────────────────────────────────┐
│                    event_loop                       │
│                   tokio::select!                    │
│                                                     │
│  key_events ────────────────────────────────┐      │
│  tick (50ms) ───────────────────────────────┤      │
│  flow_state_rx.changed() ───────────────────┤      │
│  git_snapshot_rx.changed() ─────────────────┤      │
│  event_rx.recv() ───────────────────────────┤      │
│                                              │      │
│  adapter_handles supervisor ─────────────────┘      │
└─────────────────────┬──────────────────────────────┘
                      ▼
              ┌────────────────┐
              │      App       │
              │                │
              │  handle_key()  │──→ Action (동기: 직접 처리)
              │                │──→ Action (비동기: CommandRunner)
              │  handle_event()│──→ 패널 상태 업데이트
              │  on_tick()     │──→ 타이머/깜빡임/orphan 체크
              │  render()      │──→ terminal.draw()
              └────────────────┘
```

---

## Part 2: UI 디자인

### 컬러 팔레트

| 토큰 | 용도 | 색상 | NO_COLOR 시 |
|------|------|------|-------------|
| `active` | 현재 활성 스테이지/에이전트 | Yellow | `●` |
| `done` | 완료 | Green | `✓` |
| `waiting` | 대기 | Gray | `○` |
| `skipped` | 스킵 | DarkGray | `–` |
| `error` | 에러/충돌 | Red | `✗` |
| `timeout` | 타임아웃 | Red | `⏱` |
| `focus_border` | 포커스 패널 테두리 | Cyan | `>>` 접두어 + 헤더 `[ Panel Name ]` |
| `unfocus_border` | 비포커스 테두리 | DarkGray | 일반 테두리 + 헤더 `  Panel Name  ` |
| `disabled` | 비활성 패널 | DarkGray | `[disabled]` |
| `highlight` | 새 항목 강조 | White+Bold | `*` 접두어 |
| `timestamp` | 타임스탬프 | Cyan | 그대로 |
| `gate_alert` | 게이트 알림 강조 | Yellow+Bold (깜빡임 대신 지속 강조) | `>>> GATE <<<` |
| `staged` | Git staged | Green | `S` |
| `unstaged` | Git unstaged | Yellow | `M` |
| `untracked` | Git untracked | Gray | `?` |
| `conflict` | Git conflict | Red+Bold | `C!` |

> **깜빡임 제거**: 접근성/터미널 호환성 문제로 blink 대신 Yellow+Bold 지속 강조 + 좌측 `▶` 마커로 시각적 주목.

### 표준 레이아웃 (120x30+)

```
┌─ devflow-tui ─────────────────────────────── Phase: INCEPTION ── Hooks: ● 9100 ─┐
│                                                                                   │
│  ┌─[ Workflow Map ]────────────┐  ┌─ Git Status ──────────────────────────────┐  │
│  │                              │  │                                           │  │
│  │  INCEPTION                   │  │  Branch: feature/devflow-tui              │  │
│  │  ├── ✓ workspace-detection   │  │  HEAD:   a1b2c3d                          │  │
│  │  ├── ✓ complexity (Standard) │  │                                           │  │
│  │  ├── ✓ requirements-analysis │  │  Changes:                                 │  │
│  │  ├── ✓ user-stories          │  │    S src/main.rs         +42  -3          │  │
│  │  ├── ✓ nfr-requirements      │  │    M src/parser/mod.rs   +18  -0          │  │
│  │  ├── ✓ workflow-planning     │  │    ? src/new_file.rs                      │  │
│  │  ├── ● application-design    │  │                                           │  │
│  │  ├── ○ units-generation      │  │  Recent Commits:                          │  │
│  │  │                           │  │    a1b2c3d feat: add parser module         │  │
│  │  CONSTRUCTION                │  │    d4e5f6a init: project setup             │  │
│  │  ├── ○ code-generation       │  │                                           │  │
│  │  ├── ○ build-and-test        │  │  Worktrees:                               │  │
│  │  │                           │  │    main     /projects/backend/devflow-tui  │  │
│  │                              │  │                                           │  │
│  │  Key Decisions:              │  └───────────────────────────────────────────┘  │
│  │    • complexity → Standard   │  ┌─ Agent Status ── ┌─ Audit Log ───────────┐  │
│  │    • approach → B (계층별)    │  │                   │                       │  │
│  │                              │  │  ● Explore  12s   │  14:20 requirements   │  │
│  │  Next: units-generation      │  │  ✓ Plan     done  │    → B (Approve)      │  │
│  │                              │  │  ⏱ Review  t/o   │  14:18 complexity     │  │
│  │  Enter: 산출물 보기           │  │                   │    → Standard          │  │
│  │                              │  │                   │  14:15 workspace      │  │
│  │                              │  │                   │    → B (Approve)      │  │
│  └──────────────────────────────┘  └───────────────────┴───────────────────────┘  │
│                                                                                   │
├───────────────────────────────────────────────────────────────────────────────────┤
│ Tab:패널  Ctrl+hjkl:방향  j/k:스크롤  f:확대  Enter:상세  r:새로고침  ?:도움말    │
└───────────────────────────────────────────────────────────────────────────────────┘
```

> **포커스 표시**: `[ Workflow Map ]` (대괄호+볼드) vs `  Git Status  ` (일반). NO_COLOR 시에도 구분 가능.
> **산출물 접근**: 표준 레이아웃에서 Workflow Map에 포커스 → `Enter` → ArtifactModal 모드로 산출물 미리보기 오버레이

### 산출물 모달 (표준 레이아웃에서 Enter)

```
┌─ devflow-tui ─────────────────── Phase: INCEPTION ── Hooks: ● 9100 ─┐
│                                                                       │
│  ┌─ Artifacts ──────────────────────────────────────────────────────┐ │
│  │                                                                   │ │
│  │  inception/                    │  # Requirements Analysis         │ │
│  │  ├── workspace.md              │                                  │ │
│  │  ├── requirements.md      ←    │  ## User Intent                  │ │
│  │  ├── user-stories.md           │  aidlc-devflow 플러그인          │ │
│  │  ├── nfr-requirements.md       │  사용자에게 워크플로우 진행      │ │
│  │  ├── workflow-plan.md          │  상황, Git 변화, Claude          │ │
│  │  └── application-design.md     │  에이전트 상태를 실시간으로      │ │
│  │  construction/                 │  시각화하는 독립 TUI 대시보드를  │ │
│  │  └── (empty)                   │  제공한다.                        │ │
│  │                                │                                  │ │
│  │                                │  ## Functional Requirements      │ │
│  │                                │  ...                             │ │
│  │                                │                                  │ │
│  └────────────────────────────────┴──────────────────────────────────┘ │
│                                                                       │
├───────────────────────────────────────────────────────────────────────┤
│ Esc:닫기  j/k:스크롤  Tab:파일↔미리보기  ?:도움말                     │
└───────────────────────────────────────────────────────────────────────┘
```

### 와이드 레이아웃 (200x50+)

```
┌─ devflow-tui ──────────────────────────────────────────── Phase: CONSTRUCTION ── Hooks: ● 9100 ─┐
│                                                                                                   │
│  ┌─[ Workflow Map ]──────────┐  ┌─ Git Status ──────────────┐  ┌─ Artifacts ──────────────────┐  │
│  │                            │  │                            │  │                              │  │
│  │  INCEPTION                 │  │  Branch: feat/devflow-tui  │  │  inception/                  │  │
│  │  ├── ✓ workspace-detection │  │  HEAD:   a1b2c3d           │  │  ├── workspace.md            │  │
│  │  ├── ✓ complexity          │  │                            │  │  ├── requirements.md    ←    │  │
│  │  ├── ✓ requirements        │  │  Changes:                  │  │  ├── user-stories.md         │  │
│  │  ├── ✓ workflow-planning   │  │    S src/main.rs    +42 -3 │  │  ├── nfr-requirements.md     │  │
│  │  ├── ✓ application-design  │  │    M src/parser.rs  +18 -0 │  │  ├── workflow-plan.md        │  │
│  │  ├── ✓ units-generation    │  │                            │  │  └── application-design.md   │  │
│  │  │                         │  │  Commits:                  │  │  construction/                │  │
│  │  CONSTRUCTION              │  │    a1b2c3d add parser      │  │  └── (empty)                 │  │
│  │  ├── ● code-generation     │  │    d4e5f6a init setup      │  │                              │  │
│  │  │   └── Unit 3/11         │  │                            │  │  ── Preview ──               │  │
│  │  ├── ○ build-and-test      │  │  Worktrees: 1              │  │  # Requirements Analysis     │  │
│  │  │                         │  │    main /backend/devflow   │  │  ## User Intent               │  │
│  │  │                         │  │                            │  │  aidlc-devflow 플러그인...    │  │
│  │                            │  │                            │  │                              │  │
│  └────────────────────────────┘  └────────────────────────────┘  └──────────────────────────────┘  │
│                                                                                                   │
│  ┌─ Agent Status ─────────────┐  ┌─ Audit Log ───────────────┐  ┌─ Gate Alert ────────────────┐  │
│  │                            │  │                            │  │                              │  │
│  │  ● Explore    agent-a 15s  │  │  14:32 code-generation     │  │  ▶ >>> GATE WAITING <<<     │  │
│  │  ● Plan       agent-b  8s │  │    → B (Approve Plan)      │  │                              │  │
│  │  ✓ Explore    agent-c done │  │  14:28 units-generation    │  │  A) 수정 요청                │  │
│  │  ⏱ Review    agent-d t/o  │  │    → B (Approve)           │  │  B) 승인 → 다음 단계 진행    │  │
│  │                            │  │  14:25 application-design  │  │                              │  │
│  │  Total: 4 (2 running)      │  │    → B (Approve)           │  │  Since: 14:35 (2m ago)       │  │
│  │                            │  │                            │  │                              │  │
│  └────────────────────────────┘  └────────────────────────────┘  └──────────────────────────────┘  │
│                                                                                                   │
├───────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Tab:패널  Ctrl+hjkl:방향  j/k:스크롤  f:확대  Enter:상세  r:새로고침  ?:도움말  q:종료            │
└───────────────────────────────────────────────────────────────────────────────────────────────────┘
```

> **방향 네비게이션**: 와이드 2x3 그리드에서 `Ctrl+h/j/k/l`로 공간 이동. Tab은 순차 순환도 유지.

### 축소 레이아웃 (80x24 ~ 119x29)

```
┌─ devflow-tui ────── INCEPTION ── Hooks:● ─┐
│                                            │
│  ┌─[ Workflow Map ]──────────────────────┐ │
│  │                                        │ │
│  │  INCEPTION                             │ │
│  │  ├── ✓ workspace-detection             │ │
│  │  ├── ✓ complexity (Standard)           │ │
│  │  ├── ✓ requirements-analysis           │ │
│  │  ├── ● application-design              │ │
│  │  ├── ○ units-generation                │ │
│  │  │                                     │ │
│  │  CONSTRUCTION                          │ │
│  │  ├── ○ code-generation                 │ │
│  │  ├── ○ build-and-test                  │ │
│  │  │                                     │ │
│  │  Next: units-generation                │ │
│  │                                        │ │
│  └────────────────────────────────────────┘ │
│                                            │
│  축소 뷰 — 120x30 이상에서 전체 대시보드    │
├────────────────────────────────────────────┤
│ Tab:패널전환  f:확대  ?:도움말  q:종료      │
└────────────────────────────────────────────┘
```

> 축소 뷰에서 Tab으로 Git/에이전트/감사로그 패널 전환 (한 번에 하나만 표시)

### LayoutManager 배치 규칙

| 터미널 크기 | 레이아웃 | 패널 구성 |
|------------|---------|----------|
| < 80x24 | 경고만 | "터미널을 80x24 이상으로 확대해주세요" |
| 80x24 ~ 119x29 | 축소 | 단일 패널 + Tab 전환 |
| 120x30 ~ 199x49 | 표준 | 좌(40%): Workflow Map, 우상(60%×50%): Git+Agent, 우하(60%×50%): Audit Log |
| 200x50+ | 와이드 | 2행×3열: 상단(Workflow+Git+Artifacts), 하단(Agent+Audit+Gate) |

### 상태바 구성

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│ [Workflow Map]  INCEPTION > application-design  │  Tab Ctrl+hjkl j/k f Enter ? q │
└──────────────────────────────────────────────────────────────────────────────────┘
```

- 좌측: 현재 포커스 패널명 + Phase > Stage 경로
- 우측: 단축키 힌트 (항상 표시)

### 헤더 구성

```
┌─ devflow-tui ─────────────────── Phase: INCEPTION ── Hooks: ● 9100 ─┐
```

- 좌측: 앱 이름
- 중앙: 현재 Phase
- 우측: Hooks 상태 (● Green: 활성, ○ Red: 비활성) + 포트 번호

### 패널 간 전환 순서

**표준 레이아웃 (Tab 순환):**
```
Workflow Map → Git Status → Agent Status → Audit Log → (반복)
```

**와이드 레이아웃 (Tab 순환):**
```
Workflow Map → Git Status → Artifacts → Agent Status → Audit Log → Gate Alert → (반복)
```

**와이드 레이아웃 (Ctrl+hjkl 방향 이동):**
```
┌──────────────┬──────────────┬──────────────┐
│ Workflow Map │  Git Status  │  Artifacts   │
├──────────────┼──────────────┼──────────────┤
│ Agent Status │  Audit Log   │  Gate Alert  │
└──────────────┴──────────────┴──────────────┘
Ctrl+l: 오른쪽, Ctrl+h: 왼쪽, Ctrl+j: 아래, Ctrl+k: 위
```

**축소 레이아웃 (Tab 전환):**
```
Workflow Map → Git Status → Agent Status → Audit Log → (반복)
```
각 Tab에서 해당 패널만 전체 영역에 표시.

### devflow 미설정 시 초기 화면

```
┌─ devflow-tui ──────────────────────────────────────────── Hooks: ○ ─┐
│                                                                      │
│                                                                      │
│              devflow 프로젝트가 감지되지 않았습니다                    │
│                                                                      │
│              devflow-docs/ 디렉토리가 있는 프로젝트 경로에서          │
│              실행하거나, Claude Code에서                               │
│              'devflow 시작해줘'로 새 프로젝트를 시작하세요            │
│                                                                      │
│              현재 경로: /Users/jay.ahn/projects/backend               │
│                                                                      │
│              devflow-docs/ 생성 시 자동으로 감지합니다                 │
│              ● 감시 중...                                             │
│                                                                      │
│                                                                      │
├──────────────────────────────────────────────────────────────────────┤
│ q:종료                                                               │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Part 3: Implementation Notes (code-generation 시 참조)

> 이 섹션은 구현 단계에서 각 유닛의 code-plan 작성 및 코드 생성 시 체크리스트로 활용한다.

### 3.1 ratatui 위젯 매핑

| 패널 | 위젯 | 상태 타입 | 비고 |
|------|------|----------|------|
| Workflow Map | `Paragraph` + `Line`/`Span` 조합 | 없음 (정적 렌더링) | `├──`, `└──` 접두어 + `Span` 스타일로 트리 구현. ratatui에 내장 Tree 없음 |
| Git Status (변경 파일) | `Table` + `TableState` | `TableState` | 컬럼: 상태 아이콘, 파일 경로, +N/-N. 선택/스크롤 자동 |
| Git Status (커밋) | `List` + `ListState` | `ListState` | 최근 10개 커밋 |
| Agent Status | `Table` + `TableState` | `TableState` | 컬럼: 상태 아이콘, 타입, agent_id, elapsed |
| Audit Log | `List` + `ListState` + `Scrollbar` | `ListState` + `ScrollbarState` | 자동 스크롤: `ListState::select(Some(last))`. Scrollbar 우측 배치 |
| Artifact Preview (파일 목록) | `List` + `ListState` | `ListState` | 좌측 패널 |
| Artifact Preview (내용) | `Paragraph` + `.scroll((offset, 0))` | `u16` offset | 우측 패널. `Paragraph::scroll()`로 스크롤 |
| Gate Alert | `Paragraph` + `Alignment::Center` | 없음 | 단순 텍스트 중앙 정렬 |
| Help Overlay | `Paragraph` + `Clear` | 없음 | 모달 오버레이 |
| Status Bar | `Line` + `Span` 조합 | 없음 | nexttui 패턴 준수 |

### 3.2 스타일 패턴

**필수: `Stylize` trait 사용**
```rust
use ratatui::style::Stylize;

// 올바름
"✓ workspace-detection".green()
"● application-design".yellow().bold()
"○ units-generation".dark_gray()

// 금지 — 장황한 Style 구성
Span::styled("text", Style::default().fg(Color::Green))
```

**필수: `ui/theme.rs` 모듈 추가**

모듈 구조에 `ui/theme.rs`를 추가하여 NO_COLOR 추상화를 중앙 관리:
```rust
// ui/theme.rs
pub fn status_span(text: &str, color: Color, icon: &str) -> Span<'_> {
    if std::env::var("NO_COLOR").is_ok() {
        Span::raw(format!("{icon} {text}"))
    } else {
        Span::styled(format!("{icon} {text}"), Style::new().fg(color))
    }
}
```

모든 패널은 `theme::status_span()` 등을 통해 스타일을 적용한다. 패널 내부에서 직접 `Color::Green` 등을 하드코딩하지 않는다.

### 3.3 Block 테두리

```rust
Block::bordered()
    .border_type(BorderType::Rounded)  // ╭╮╰╯ — 와이어프레임의 ┌┐└┘ 대신
    .title(if focused { format!("[ {title} ]") } else { format!("  {title}  ") })
    .border_style(if focused { Style::new().cyan() } else { Style::new().dark_gray() })
```
- `BorderType::Rounded` 사용 (nexttui 패턴 준수)
- 포커스 시 `[ Title ]` + Cyan 테두리, 비포커스 시 `  Title  ` + DarkGray

### 3.4 모달 오버레이

산출물 모달, 도움말 오버레이에서 반드시 `Clear` 위젯으로 배경을 지운 후 렌더:
```rust
fn render_modal(frame: &mut Frame, content: impl Widget, percent_x: u16, percent_y: u16) {
    let area = centered_rect(percent_x, percent_y, frame.area());
    frame.render_widget(Clear, area);  // 배경 클리어 필수
    frame.render_widget(content, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let [_, center, _] = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ]).areas(area);
    let [_, center, _] = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ]).areas(center);
    center
}
```

### 3.5 Layout API 패턴

`Constraint::Fill(1)`을 `Constraint::Percentage`보다 선호. 반올림 오류로 1px 갭 방지:
```rust
// 표준 레이아웃 예시
let [header, body, status_bar] = Layout::vertical([
    Constraint::Length(1),
    Constraint::Fill(1),    // Percentage 대신 Fill
    Constraint::Length(1),
]).areas(frame.area());

let [left, right] = Layout::horizontal([
    Constraint::Percentage(40),
    Constraint::Fill(1),    // 나머지 60%를 Fill로
]).areas(body);

let [right_top, right_bottom] = Layout::vertical([
    Constraint::Percentage(50),
    Constraint::Fill(1),
]).areas(right);
```

### 3.6 Scrollbar 연동

감사 로그, 산출물 미리보기에 `Scrollbar` 위젯 추가:
```rust
let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
let mut scrollbar_state = ScrollbarState::new(total_items).position(current_position);
frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
```

### 3.7 Tick 주기

이벤트 루프 tick을 **250ms**로 설정 (설계 문서의 50ms에서 변경):
```rust
let tick_rate = Duration::from_millis(250);
```
- 에이전트 elapsed time은 초 단위 표시이므로 250ms면 충분
- 50ms는 CPU 낭비 (초당 20회 불필요 draw 가능성)
- 게이트 강조 토글도 1초 주기이므로 250ms tick 4회로 충분

### 3.8 리사이즈 시 상태 재보정

`App::on_resize()`에서 `ListState`/`TableState`의 offset이 새 크기 범위를 벗어나지 않도록 재보정:
```rust
fn on_resize(&mut self, width: u16, height: u16) {
    self.layout.on_resize(width, height);
    self.ensure_valid_focus();
    // 각 패널의 상태 재보정
    self.audit_log.clamp_scroll();
    self.git_status.clamp_scroll();
    self.agent_status.clamp_scroll();
}
```

### 3.9 Key Binding 표시 패턴

상태바의 단축키 힌트는 nexttui/ratatui 관용 패턴:
```rust
let help = Line::from(vec![
    " Tab ".bold().cyan(),
    "패널 ".dim(),
    " j/k ".bold().cyan(),
    "스크롤 ".dim(),
    " f ".bold().cyan(),
    "확대 ".dim(),
    " ? ".bold().cyan(),
    "도움말 ".dim(),
    " q ".bold().cyan(),
    "종료 ".dim(),
]);
```

---

## Review History
- **v1**: 초안 (구조 설계 + UI 디자인)
- **v2**: agent-council 리뷰 반영 — 이벤트 루프 match 수정, tick 시간 업데이트, CancellationToken+JoinHandle lifecycle, CommandRunner 추가, enum dispatch, watch+mpsc 채널 분리, 에러 taxonomy, 포커스 안전성, 방향 네비게이션, 산출물 모달, blink→Bold 대체
- **v3**: ratatui 0.30 리뷰 반영 — Part 3 Implementation Notes 추가 (위젯 매핑, Stylize/theme.rs, Rounded 테두리, Clear 모달, Fill constraint, Scrollbar, tick 250ms, 리사이즈 재보정)
