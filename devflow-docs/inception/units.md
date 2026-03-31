# Units Generation

## Unit 분해 원칙
- 각 유닛은 독립적으로 빌드/테스트 가능
- 의존성은 아래→위 방향 (상위 유닛이 하위 유닛에 의존)
- TDD Iron Law: 각 유닛은 실패 테스트부터 작성

## 의존성 그래프

```
Unit 1: 프로젝트 기반 + 이벤트/액션 모델
   │
   ├── Unit 2: 파일 파서
   │      │
   │      └── Unit 3: 파일 감시 어댑터
   │
   ├── Unit 4: Git 어댑터
   │
   ├── Unit 5: Hooks HTTP 서버 + 서비스
   │
   ├── Unit 6: Theme + 공통 UI 위젯
   │      │
   │      ├── Unit 7: 워크플로우 맵 패널
   │      ├── Unit 8: Git 상태 패널
   │      ├── Unit 9: 에이전트 상태 패널
   │      └── Unit 10: 감사 로그 패널
   │
   └── Unit 11: App 통합 + 레이아웃 + 네비게이션
          │
          └── Unit 12: [P1] 산출물 미리보기 + 게이트 알림
```

## 구현 순서

### Phase 1: 기반 (Unit 1~2)

#### Unit 1: 프로젝트 기반 + 이벤트/액션 모델
**범위:**
- `cargo init --name devflow-tui`
- `Cargo.toml` 의존성 설정
- `.gitignore`, `clippy.toml` (deny unwrap_used, expect_used)
- `error.rs` — `AppError` enum + `thiserror` derive
- `event.rs` — `AppEvent` enum 정의
- `action.rs` — `Action`, `Direction` enum 정의
- `component.rs` — `Component` trait 정의
- `config.rs` — CLI 옵션 (`--port`, `--project-dir`, `--demo`, `--regenerate-token`) + 환경변수
- `main.rs` — 터미널 셋업/복원 + panic hook + tracing 초기화 (이벤트 루프는 Unit 11)
- `lib.rs` — 모듈 export

**산출물:** 컴파일 가능한 Cargo 프로젝트, `cargo test` + `cargo clippy` 통과
**테스트:** Config 파싱 테스트, AppError Display 테스트

---

#### Unit 2: 파일 파서
**범위:**
- `parser/models.rs` — `FlowState`, `SessionSummary`, `AuditEntry`, `ArtifactFile` 도메인 모델
- `parser/devflow_state.rs` — devflow-state.md 파서 (관용 파싱, 비표준 필드 raw key-value)
- `parser/session_summary.rs` — session-summary.md 파서 (`[x]`/`[~]`/`[ ]` 마커)
- `parser/audit_log.rs` — audit.md / devflow-audit.md 파서 (혼합 포맷)
- `service/sanitizer.rs` — ANSI escape sequence 제거

**의존성:** Unit 1 (models, error)
**산출물:** 순수 함수 파서, I/O 없음
**테스트:** 골든 테스트 필수 — 실제 devflow-state.md/session-summary.md/audit.md 변형 샘플 포함. fake clock normalization (타임스탬프 치환)

---

### Phase 2: 데이터 어댑터 (Unit 3~5, 병렬 가능)

#### Unit 3: 파일 감시 어댑터
**범위:**
- `port/watcher.rs` — `FileWatcher` enum dispatch (Real, Mock)
- `adapter/file_watcher.rs` — notify crate + debounce(300ms). CancellationToken + AdapterHandle 패턴
- 파일 변경 감지 → 파서 호출 → `watch::Sender<FlowState>` / `mpsc::Sender<AppEvent>` 전송
- devflow-docs/ 미존재 시 대기 + 생성 감지 시 활성화
- 파싱 실패 시 이전 유효 상태 유지 + `AppEvent::ParseError` 전송

**의존성:** Unit 1 (이벤트 모델), Unit 2 (파서)
**산출물:** 장수명 백그라운드 태스크
**테스트:** Mock watcher 테스트, debounce 동작 테스트 (tokio::time::pause)

---

#### Unit 4: Git 어댑터
**범위:**
- `port/git.rs` — `GitProvider` enum dispatch (Cli, Mock)
- `adapter/git_poller.rs` — tokio::interval(2s) + `tokio::process::Command`로 git CLI 호출
  - `git status --porcelain=v2`
  - `git log --oneline -n 10`
  - `git worktree list --porcelain`
  - `git diff --stat`
  - `git rev-parse --abbrev-ref HEAD` + `git rev-parse --short HEAD`
- 결과를 `GitSnapshot` 모델로 파싱 → `watch::Sender<GitSnapshot>` 전송
- 동시 실행 방지 (이전 호출 완료 전 스킵), timeout 5초
- 실패 시 이전 상태 유지 + `AppEvent::GitPollError` 전송

**의존성:** Unit 1 (이벤트 모델)
**산출물:** 장수명 백그라운드 폴러
**테스트:** Mock git 출력 파싱 테스트, timeout 테스트

---

#### Unit 5: Hooks HTTP 서버 + 서비스
**범위:**
- `port/hooks.rs` — `HooksReceiver` enum dispatch (Axum, Mock)
- `adapter/hooks_server.rs` — axum HTTP 서버
  - POST `/hook` endpoint, `?token=<TOKEN>` 검증 (403)
  - JSON payload 파싱 → `AppEvent` 변환 (AgentStarted/Stopped, ToolUse, TurnCompleted)
  - ANSI 새니타이즈 적용
  - CancellationToken + AdapterHandle, graceful shutdown (3초)
  - 포트 바인딩 실패 시 9100~9110 자동 탐색, 전부 실패 시 비활성 모드
- `service/token.rs` — 프로젝트 기반 stable token (SHA-256 + salt), `--regenerate-token`
- `service/hook_config.rs` — `~/.claude/settings.json` / `.claude/settings.json` hooks 설정 검사, JSON 스니펫 생성
- `command.rs` — CommandRunner (clipboard 복사, 파일 fallback)

**의존성:** Unit 1 (이벤트 모델)
**산출물:** axum 서버 + 서비스 레이어
**테스트:** 토큰 생성/검증 테스트, hooks 설정 감지 테스트, HTTP endpoint 통합 테스트 (axum test utilities)

---

### Phase 3: UI 패널 (Unit 6~10, 6 선행 후 7~10 병렬 가능)

#### Unit 6: Theme + 공통 UI 위젯
**범위:**
- `ui/theme.rs` — 컬러 팔레트 상수, NO_COLOR 감지, `status_span()` 등 Stylize 헬퍼
- `ui/layout.rs` — `LayoutManager` (Compact/Standard/Wide 모드, `on_resize`, `areas()`)
  - `Constraint::Fill(1)` 우선 패턴
  - `centered_rect()` 모달 유틸
- `ui/status_bar.rs` — 포커스 패널명 + Phase>Stage + 단축키 힌트
- `ui/header.rs` — 앱 이름 + Phase + Hooks 상태
- `ui/help_overlay.rs` — `?` 키 오버레이 (Clear + Paragraph)
- Block 패턴: `BorderType::Rounded`, 포커스 Cyan/비포커스 DarkGray

**의존성:** Unit 1 (Component trait)
**산출물:** 재사용 가능한 UI 기반
**테스트:** LayoutManager 단위 테스트 (각 breakpoint별 영역 계산), theme NO_COLOR 테스트

---

#### Unit 7: 워크플로우 맵 패널
**범위:**
- `panel/workflow_map.rs` — `WorkflowMapPanel` (Component 구현)
  - INCEPTION/CONSTRUCTION 트리 렌더링 (Paragraph + Line/Span)
  - `FlowState` 기반 스테이지 상태 표시 (active/done/waiting/skipped)
  - Complexity 기반 조건부 스테이지 표시/숨김
  - Key Decisions, Next Steps 표시
  - session-summary.md 데이터 표시 (`[x]`/`[~]`/`[ ]`)
  - "Syncing..." 표시 (파싱 실패 시)

**의존성:** Unit 1, Unit 2 (FlowState, SessionSummary 모델), Unit 6 (theme, Block)
**산출물:** 독립 렌더링 가능한 패널
**테스트:** 다양한 FlowState 입력에 대한 렌더링 스냅샷 테스트

---

#### Unit 8: Git 상태 패널
**범위:**
- `panel/git_status.rs` — `GitStatusPanel` (Component 구현)
  - Branch/HEAD 표시 (Paragraph)
  - 변경 파일 Table + TableState (staged Green, unstaged Yellow, untracked Gray, conflict Red+Bold)
  - 커밋 히스토리 List + ListState
  - Worktree 목록
  - diff 변화량 (+N/-N)
  - j/k 스크롤, Enter 상세

**의존성:** Unit 1, Unit 6 (theme, Block)
**산출물:** 독립 렌더링 가능한 패널
**테스트:** GitSnapshot mock 데이터 렌더링 테스트

---

#### Unit 9: 에이전트 상태 패널
**범위:**
- `panel/agent_status.rs` — `AgentStatusPanel` (Component 구현)
  - Table + TableState (상태 아이콘, 타입, agent_id, elapsed)
  - agent_id 기반 Start↔Stop 매칭 HashMap
  - orphan timeout (60초 → timeout 상태)
  - elapsed time 갱신 (`on_tick` 연동)
  - hooks 미설정 시 비활성 안내 메시지
  - Total 카운트 (N running, N done)

**의존성:** Unit 1, Unit 6 (theme, Block)
**산출물:** 독립 렌더링 가능한 패널
**테스트:** 에이전트 라이프사이클 테스트 (start→stop, timeout, 중복 id)

---

#### Unit 10: 감사 로그 패널
**범위:**
- `panel/audit_log.rs` — `AuditLogPanel` (Component 구현)
  - List + ListState + Scrollbar + ScrollbarState
  - 메모리 캡 1000줄 (환경변수 설정 가능)
  - 새 엔트리 자동 스크롤 (`ListState::select(Some(last))`)
  - 타임스탬프 Cyan, 이벤트 유형별 컬러
  - ANSI escape 제거된 상태로 표시
  - j/k 수동 스크롤 (자동 스크롤 일시 중지, 새 엔트리 시 재개)

**의존성:** Unit 1, Unit 2 (AuditEntry 모델), Unit 6 (theme, Block)
**산출물:** 독립 렌더링 가능한 패널
**테스트:** 버퍼 캡 테스트, 자동/수동 스크롤 전환 테스트

---

### Phase 4: 통합 (Unit 11)

#### Unit 11: App 통합 + 이벤트 루프 + 레이아웃 + 네비게이션
**범위:**
- `app.rs` — `App` struct 조립
  - FocusPane, InputMode 상태 머신
  - `available_panels()`, `ensure_valid_focus()`, `focus_direction()`
  - `handle_key()` → Action 분기 (동기: 직접 처리, 비동기: CommandRunner)
  - `handle_event()` → 패널별 이벤트 라우팅
  - `handle_flow_state()`, `handle_git_snapshot()` — watch 채널 핸들러
  - `on_tick()` — elapsed time, orphan timeout, 게이트 강조. 변경 시 true 반환
  - `on_resize()` — LayoutManager + ListState/TableState 재보정
  - `render()` — LayoutManager.areas() 기반 패널 배치 + 모달 오버레이
- `event_loop.rs` — `run_event_loop()`
  - tokio::select! (key_events, tick 250ms, flow_state_rx, git_snapshot_rx, event_rx)
  - adapter supervisor (JoinHandle.is_finished)
  - needs_render + no-change frame skip
  - graceful shutdown
- `main.rs` 완성 — 어댑터 시작, 채널 생성, 이벤트 루프 실행
- `demo.rs` — `--demo` 모드 샘플 데이터

**의존성:** Unit 1~10 전체
**산출물:** 동작하는 TUI 대시보드 (P0 기능 완전)
**테스트:** 키 입력 시나리오 테스트, 레이아웃 breakpoint 테스트, focus 순환 테스트

---

### Phase 5: P1 확장 (Unit 12)

#### Unit 12: [P1] 산출물 미리보기 + 게이트 알림
**범위:**
- `panel/artifact_preview.rs` — `ArtifactPreviewPanel` (Component 구현)
  - 좌: 파일 목록 List + ListState
  - 우: Paragraph + `.scroll((offset, 0))`
  - Tab으로 좌/우 포커스 전환
  - 파일 변경 시 자동 새로고침
- `panel/gate_alert.rs` — `GateAlertPanel` (Component 구현)
  - Stop hook의 last_assistant_message 패턴 매칭 (A)/B)/C))
  - PostToolUse + AskUserQuestion 추가 탐지
  - Yellow+Bold 지속 강조 + `▶` 마커
  - 알림 종료: 다음 Stop hook / Stage 변경 / 수동 Esc
  - hooks 미설정 시 session-summary Next Steps 대체
- App에 ArtifactModal 모드 추가 (표준 레이아웃 Enter → 모달)
- 와이드 레이아웃에서 패널로 직접 표시

**의존성:** Unit 11 (App 통합)
**산출물:** P1 기능 완성
**테스트:** 게이트 패턴 매칭 테스트, 알림 종료 조건 테스트

---

## 요약

| Unit | 이름 | Phase | 의존성 | 병렬 가능 |
|------|------|-------|--------|----------|
| 1 | 프로젝트 기반 + 이벤트/액션 모델 | 1 | 없음 | — |
| 2 | 파일 파서 | 1 | Unit 1 | — |
| 3 | 파일 감시 어댑터 | 2 | Unit 1, 2 | ✅ (4, 5와) |
| 4 | Git 어댑터 | 2 | Unit 1 | ✅ (3, 5와) |
| 5 | Hooks HTTP 서버 + 서비스 | 2 | Unit 1 | ✅ (3, 4와) |
| 6 | Theme + 공통 UI 위젯 | 3 | Unit 1 | — |
| 7 | 워크플로우 맵 패널 | 3 | Unit 1, 2, 6 | ✅ (8, 9, 10과) |
| 8 | Git 상태 패널 | 3 | Unit 1, 6 | ✅ (7, 9, 10과) |
| 9 | 에이전트 상태 패널 | 3 | Unit 1, 6 | ✅ (7, 8, 10과) |
| 10 | 감사 로그 패널 | 3 | Unit 1, 2, 6 | ✅ (7, 8, 9와) |
| 11 | App 통합 + 이벤트 루프 | 4 | Unit 1~10 | — |
| 12 | [P1] 산출물 + 게이트 | 5 | Unit 11 | — |

## 구현 일정 (Phase 기준)

```
Phase 1: Unit 1 → Unit 2                    (순차)        → 3자 리뷰 → Fix
Phase 2: Unit 3 + Unit 4 + Unit 5           (병렬)        → 3자 리뷰 → Fix
Phase 3: Unit 6 → Unit 7 + 8 + 9 + 10      (6 선행, 병렬) → 3자 리뷰 → Fix
Phase 4: Unit 11                             (통합)        → 3자 리뷰 → Fix
Phase 5: Unit 12                             (P1 확장)     → 3자 리뷰 → Fix
```

## 리뷰 정책

- **각 Phase 완료 시 agent-council 기반 3자 리뷰 필수**
- 리뷰 관점: Codex (코드 품질/아키텍처), Gemini (보안/UX/완전성), Claude (종합 판정)
- 리뷰 결과의 Critical/High 이슈는 반드시 Fix 후 다음 Phase 진행
- Medium 이슈는 해당 Phase 내 Fix 권장, 다음 Phase에서 Fix 허용
- Low 이슈는 기록 후 판단
