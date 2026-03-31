# Code Plan: Unit 11 — App 통합 + 이벤트 루프 + 레이아웃 + 네비게이션

## 개요
모든 어댑터, 패널, UI 위젯을 통합하여 동작하는 TUI 대시보드를 완성한다.

## 선행 조건 (완료)
- render(&mut self) 마이그레이션 완료 (152 tests pass)
- Unit 1~10 구현 + Phase 1~3 리뷰/fix 완료

## 파일 목록

### Step 1: App 상태 구조체
- **파일**: `src/app.rs` (신규)
- **작업**:
  - `FocusPane` enum: WorkflowMap, GitStatus, AgentStatus, AuditLog
    - P1 패널(ArtifactPreview, GateAlert)은 Unit 12에서 추가
  - `InputMode` enum: Normal, Expanded, HelpOverlay
    - ArtifactModal은 Unit 12에서 추가
  - `HookSetupState` enum: Unknown, Checking, Configured, NotConfigured { snippet: String }, Mismatch { detail: String }
  - `App` struct:
    - `should_quit: bool`
    - `input_mode: InputMode`
    - `focus: FocusPane`
    - 4개 패널 필드 (WorkflowMapPanel, GitStatusPanel, AgentStatusPanel, AuditLogPanel)
    - `layout: LayoutManager`
    - `hooks_active: bool`, `hooks_port: Option<u16>`
    - `hook_setup: HookSetupState` — hooks 설정 상태 추적
    - `token: String` — hooks 토큰 (스니펫 생성용)
    - `phase: String`, `stage: String` (header/status_bar 렌더링용)
    - `command_runner: CommandRunner`
    - `event_tx: mpsc::Sender<AppEvent>` (CommandRunner 생성용)
  - `App::new(width, height, event_tx, token)` 생성자 — 토큰도 받음
  - `App::available_panels(&self) -> Vec<FocusPane>`: 레이아웃 모드에 따라 가용 패널 반환
  - `App::ensure_valid_focus(&mut self)`: 포커스가 사용 불가 패널이면 첫 번째로 이동
- **테스트**:
  - `test_available_panels_compact`: Compact 모드에서 4개 패널
  - `test_available_panels_standard`: Standard 모드에서 4개 패널
  - `test_ensure_valid_focus`: 유효하지 않은 포커스 보정

### Step 2: 키 입력 처리 + 액션 실행
- **파일**: `src/app.rs` (Step 1에 추가)
- **작업**:
  - `App::handle_key(&mut self, key: KeyEvent) -> bool`:
    - InputMode::HelpOverlay: Esc/? → Normal 복귀, true 반환
    - InputMode::Expanded: Esc/f → Normal 복귀, true 반환
    - InputMode::Normal:
      - `q` → should_quit = true
      - `?` → HelpOverlay 모드
      - `f` → Expanded 모드
      - `Tab` → FocusNextPanel
      - `Shift+Tab` → FocusPrevPanel
      - `Ctrl+h/j/k/l` → FocusDirection(Left/Down/Up/Right)
      - `r` → Refresh (event_tx로 재파싱 트리거)
      - `c` → hooks 스니펫 클립보드 복사 (NotConfigured/Mismatch 상태일 때만)
      - 나머지 → 포커스 패널의 handle_key 위임, Action 있으면 실행
    - 반환값: 렌더링 필요 여부 (true/false)
  - `App::execute_action(&mut self, action: Action)`:
    - FocusNextPanel/FocusPrevPanel: 포커스 순환
    - FocusDirection: 방향 네비게이션 (Standard는 Left/Right만, Wide는 2x3 그리드)
    - ExpandPanel/CollapsePanel: InputMode 전환
    - Quit: should_quit = true
    - 비동기 액션: command_runner.execute()
- **테스트**:
  - `test_handle_key_quit`: q → should_quit
  - `test_handle_key_tab_focus`: Tab 순환
  - `test_handle_key_help_overlay`: ? 토글
  - `test_handle_key_expand`: f 토글
  - `test_focus_direction_standard`: Ctrl+h/l 동작
  - `test_help_overlay_esc_returns_normal`: Esc → Normal
  - `test_handle_key_c_copies_snippet`: c 키 → 미설정 시 복사 트리거
  - `test_handle_key_c_noop_when_configured`: c 키 → 설정 완료 시 무시

### Step 3: 이벤트 처리 + 상태 업데이트 + Hooks 설정 확인
- **파일**: `src/app.rs` (Step 2에 추가)
- **작업**:
  - `App::handle_event(&mut self, event: AppEvent)`:
    - 모든 패널에 `handle_event` 브로드캐스트
    - 특수 이벤트 처리:
      - HooksServerStarted → hooks_active=true, hooks_port 설정 → **hooks config 확인 트리거**
      - HooksServerFailed → hooks_active=false, hook_setup=Unknown
      - FlowStateChanged → phase/stage 업데이트
  - `App::check_hooks_config(&mut self)`:
    - HooksServerStarted 수신 후 호출
    - `hook_config::check_hooks_config(project_dir, port, token)` 호출
    - 결과에 따라 `hook_setup` 상태 전환:
      - Configured → HookSetupState::Configured
      - NotConfigured → HookSetupState::NotConfigured { snippet: generate_hooks_snippet(port, token) }
      - EndpointMismatch → HookSetupState::Mismatch { detail }
    - NotConfigured/Mismatch 시 status_bar에 "c: hooks 설정 복사" 힌트 추가
  - `App::copy_hooks_snippet(&mut self)`:
    - hook_setup이 NotConfigured일 때 스니펫을 클립보드로 복사 (CommandRunner 사용)
    - 복사 실패 시 `/tmp/devflow-tui-hooks.json` 파일 저장 후 경로 안내
  - `App::handle_flow_state(&mut self, state: FlowState)`:
    - workflow_map에 set_flow_state
    - phase/stage 문자열 업데이트
  - `App::handle_git_snapshot(&mut self, snapshot: GitSnapshot)`:
    - git_status에 set_snapshot
  - `App::on_tick(&mut self) -> bool`:
    - agent_status.check_timeouts()
    - 변경 있으면 true
  - `App::on_resize(&mut self, w: u16, h: u16)`:
    - layout.on_resize(w, h)
    - ensure_valid_focus()
    - 각 패널 clamp_scroll()
  - `App::trigger_refresh(&self)`:
    - event_tx로 FlowStateChanged/GitStatusUpdated 재요청 이벤트 전송
    - 또는 file_watcher/git_poller에 refresh 시그널 (단순 구현: 파일 재파싱 직접 실행 후 이벤트 전송)
- **테스트**:
  - `test_handle_event_hooks_started`: HooksServerStarted 이벤트 → hooks_active + check_hooks_config 호출
  - `test_check_hooks_configured`: Configured 상태 전환
  - `test_check_hooks_not_configured`: NotConfigured → 스니펫 생성
  - `test_handle_event_broadcast`: 이벤트가 모든 패널에 전달
  - `test_on_resize_revalidates_focus`: 리사이즈 후 포커스 보정
  - `test_on_tick_returns_false_when_no_change`: 변경 없으면 false

### Step 4: 렌더링
- **파일**: `src/app.rs` (Step 3에 추가)
- **작업**:
  - `App::render(&mut self, frame: &mut Frame)`:
    - `layout.areas(frame.area())` → header, body, status_bar 영역
    - header::render(frame, header_area, phase, hooks_active, hooks_port)
    - status_bar::render(frame, status_bar_area, focus_name, phase_stage, wide_mode)
    - hooks 미설정 배너: HookSetupState가 NotConfigured/Mismatch일 때 body 상단 1줄에 경고 + "c 키로 설정 복사" 안내 렌더링
    - `layout.panel_areas(body)` 분기:
      - TooSmall: "터미널이 너무 작습니다 (최소 80x24)" 메시지
      - Compact: 포커스 패널 1개만 렌더링
      - Standard: 4개 패널 배치
      - Wide: 4개 패널 + P1 영역은 빈 Block
    - InputMode::HelpOverlay: help_overlay::render(frame, frame.area())
    - InputMode::Expanded: 포커스 패널을 body 전체에 렌더링
  - `App::focus_name(&self) -> &str`: 현재 포커스 패널 이름
  - `App::focused_panel_mut(&mut self) -> &mut dyn Component`: 포커스 패널 참조
- **테스트** (TestBackend 사용):
  - `render_compact_mode`: 80x24에서 패널 1개만 표시
  - `render_standard_mode`: 120x30에서 4개 패널 표시
  - `render_too_small`: 60x20에서 경고 메시지
  - `render_help_overlay`: HelpOverlay 모드에서 도움말 표시
  - `render_expanded_mode`: Expanded 모드에서 패널 전체화면
  - `render_hooks_not_configured_banner`: hooks 미설정 시 배너 표시
  - `render_hooks_configured_no_banner`: hooks 설정 완료 시 배너 미표시

### Step 5: 이벤트 루프
- **파일**: `src/event_loop.rs` (신규)
- **작업**:
  - `run_event_loop()` async 함수:
    - 인자: `terminal, app, event_rx, flow_state_rx, git_snapshot_rx, adapter_handles`
    - `tokio::select!` 5개 브랜치:
      1. `key_events.next()` — KeyEventKind::Press만 처리, Resize 이벤트
      2. `tick.tick()` (250ms) — app.on_tick()
      3. `flow_state_rx.changed()` — app.handle_flow_state()
      4. `git_snapshot_rx.changed()` — app.handle_git_snapshot()
      5. `event_rx.recv()` — app.handle_event(), None이면 quit
    - Adapter supervisor: is_finished() 검사, 로깅
    - Conditional render: needs_render가 true일 때만 terminal.draw()
    - `app.should_quit` → 루프 종료
    - Graceful shutdown: adapter_handles 순회하며 shutdown().await
- **테스트**:
  - 이벤트 루프는 통합 테스트 성격이므로, 개별 브랜치 로직은 App 테스트에서 커버
  - `test_event_loop_quit_on_q`: q 키 입력 시 종료 (crossterm mock 필요 여부 판단)

### Step 6: main.rs 완성
- **파일**: `src/main.rs` (기존 수정)
- **작업**:
  - 채널 생성: `mpsc::channel(256)`, `watch::channel(FlowState/GitSnapshot)`
  - 토큰 생성: `get_or_create_token()`
  - 3개 어댑터 spawn (AdapterHandle)
  - App 생성
  - `run_event_loop()` 호출
  - tokio runtime: `#[tokio::main]`
- **테스트**: main.rs는 조립만 하므로 별도 테스트 없음 (통합 테스트는 --demo 모드로)

### Step 7: demo 모드
- **파일**: `src/demo.rs` (신규)
- **작업**:
  - `populate_demo_data(app: &mut App)`:
    - 샘플 FlowState (CONSTRUCTION, code-generation, 일부 completed)
    - 샘플 SessionSummary (completed work 3개, key decisions 2개)
    - 샘플 GitSnapshot (branch, 변경파일 5개, 커밋 3개)
    - 샘플 AuditEntry 10개
    - 에이전트 3개 (1 running, 1 done, 1 timeout)
  - main.rs에서 `--demo` 플래그 시 어댑터 대신 demo data 사용
- **테스트**:
  - `test_populate_demo_no_panic`: 패닉 없이 데이터 주입

### Step 8: lib.rs 업데이트
- **파일**: `src/lib.rs`
- **작업**: `pub mod app;`, `pub mod event_loop;`, `pub mod demo;` 추가

## 채널 아키텍처

```
watch::channel<FlowState>    ← file_watcher
watch::channel<GitSnapshot>  ← git_poller
mpsc::channel<AppEvent>(256) ← file_watcher + git_poller + hooks_server + command_runner
```

이벤트 루프에서 통합:
- watch: `changed()` → app.handle_flow_state / handle_git_snapshot
- mpsc: `recv()` → app.handle_event (모든 패널에 브로드캐스트)

## 키 바인딩 맵

| 키 | Normal | Expanded | HelpOverlay |
|----|--------|----------|-------------|
| q | Quit | — | — |
| ? | → HelpOverlay | — | → Normal |
| f | → Expanded | → Normal | — |
| Esc | — | → Normal | → Normal |
| Tab | FocusNext | — | — |
| Shift+Tab | FocusPrev | — | — |
| Ctrl+h/j/k/l | FocusDirection | — | — |
| j/k/↑/↓ | 패널 위임 | 패널 위임 | — |
| Enter | 패널 위임 | 패널 위임 | — |
| r | Refresh (재파싱 트리거) | — | — |
| c | Hooks 스니펫 복사 (미설정 시) | — | — |

## 검증 방법

```bash
cargo test --lib              # 전체 테스트
cargo clippy --all-targets    # clippy
cargo run -- --demo           # 데모 모드 시각 확인
```
