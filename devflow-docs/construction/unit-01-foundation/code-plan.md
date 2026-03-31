# Code Plan: Unit 1 — 프로젝트 기반 + 이벤트/액션 모델

## 개요
Cargo 프로젝트 초기화 + 핵심 타입 정의. 이 유닛 완료 후 모든 후속 유닛이 의존하는 기반이 확립된다.

## 파일 목록 및 구현 순서

### Step 1: 프로젝트 초기화
- **파일**: `Cargo.toml`, `.gitignore`, `clippy.toml`
- **작업**: `cargo init --name devflow-tui`, 의존성 설정, clippy strict 설정
- **테스트**: `cargo check` 통과

### Step 2: 에러 타입
- **파일**: `src/error.rs`
- **작업**: `AppError` enum (thiserror derive), `Result<T>` type alias
- **테스트**: `test_error_display` — 각 variant의 Display 출력 검증

### Step 3: 도메인 모델 (파서 출력 타입)
- **파일**: `src/parser/models.rs`, `src/parser/mod.rs`
- **작업**:
  - `FlowState` — phase, stage, complexity, completed_stages, approved_stages, skipped_stages, active_unit, completed_units, worktree, extra_fields
  - `SessionSummary` — key_decisions, completed_work (with marker enum), next_steps
  - `AuditEntry` — timestamp, stage, choice, raw_line (혼합 포맷 대응)
  - `ArtifactFile` — path, name, modified
  - `GitSnapshot` — branch, head, changes (staged/unstaged/untracked/conflict), commits, worktrees, diff_stat
  - `GitChange` — status enum, path, additions, deletions
  - `GitCommit` — hash, message
  - `GitWorktree` — path, branch
  - `DiffStat` — additions, deletions
  - `WorkMarker` enum — Done, InProgress, Pending
  - `StageStatus` enum — Active, Completed, Waiting, Skipped
  - `Phase` enum — Inception, Construction, Complete, Finished
  - `Complexity` enum — Minimal, Standard, Comprehensive
- **테스트**: `test_flow_state_default`, `test_phase_display`, `test_complexity_display`

### Step 4: AppEvent enum
- **파일**: `src/event.rs`
- **작업**: 전체 AppEvent enum 정의 (설계 문서 기반)
  - 파일 감시: FlowStateChanged, SessionSummaryChanged, AuditLogAppended, ArtifactListChanged
  - Git: GitStatusUpdated
  - Hooks: AgentStarted, AgentStopped, ToolUseStarted, ToolUseCompleted, TurnCompleted
  - 시스템: HooksServerStarted, HooksServerFailed, FileWatcherError, GitPollError, ParseError, CommandCompleted, CommandFailed
- **테스트**: `test_event_debug` — Debug trait 출력 검증

### Step 5: Action enum
- **파일**: `src/action.rs`
- **작업**: Action, Direction enum 정의
  - 네비게이션: FocusNextPanel, FocusPrevPanel, FocusDirection, ExpandPanel, CollapsePanel, OpenArtifactModal
  - 패널 내: ScrollUp, ScrollDown, Select
  - 비동기: Refresh, CopyToClipboard
  - 시스템: Quit
- **테스트**: `test_action_name` — 각 Action의 name() 메서드 검증

### Step 6: Component trait
- **파일**: `src/component.rs`
- **작업**: Component trait 정의 (handle_key, handle_event, render with focused)
- **테스트**: trait 정의만이므로 별도 테스트 불필요 (구현체에서 테스트)

### Step 7: Config
- **파일**: `src/config.rs`
- **작업**:
  - `AppConfig` struct — port, project_dir, demo, regenerate_token, log_level, log_buffer_size
  - CLI 파싱 (clap 또는 수동 args)
  - 환경변수 fallback: `DEVFLOW_TUI_PORT`, `DEVFLOW_TUI_LOG`, `DEVFLOW_TUI_LOG_BUFFER`
  - 기본값: port=9100, project_dir=현재 디렉토리, demo=false
- **테스트**: `test_config_defaults`, `test_config_env_override`, `test_config_cli_override`

### Step 8: main.rs 기본 구조
- **파일**: `src/main.rs`, `src/lib.rs`
- **작업**:
  - 터미널 셋업: `enable_raw_mode`, `EnterAlternateScreen`, `CrosstermBackend`
  - 터미널 복원: `disable_raw_mode`, `LeaveAlternateScreen` (cleanup 함수)
  - panic hook: `std::panic::set_hook` → cleanup 호출
  - tracing 초기화: `tracing-subscriber` + `tracing-appender` (rolling daily)
  - `lib.rs`: 모듈 export (error, event, action, component, parser, config)
  - main 함수: config 로드 → tracing 초기화 → 터미널 셋업 → (이벤트 루프는 Unit 11) → cleanup
- **테스트**: `cargo build` + `cargo clippy` 통과 확인

## Verification Contract

### 완료 체크리스트
- [ ] `cargo build` 성공
- [ ] `cargo test` 전체 통과
- [ ] `cargo clippy` 경고 0개
- [ ] 모든 public 타입에 Debug derive
- [ ] `unwrap()` / `expect()` 사용 없음 (테스트 제외)
- [ ] 모든 enum variant에 적절한 필드 정의
- [ ] `FlowState::default()` 가 유효한 초기 상태 반환
- [ ] panic hook이 터미널 상태 복원

### Validation Commands
```bash
cargo build
cargo test
cargo clippy -- -D warnings
```
