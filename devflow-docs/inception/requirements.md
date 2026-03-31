# Requirements Analysis

## User Intent
aidlc-devflow 플러그인 사용자에게 워크플로우 진행 상황, Git 변화, Claude 에이전트 상태를 실시간으로 시각화하는 독립 TUI 대시보드를 제공한다.

## Functional Requirements

### FR-1: 워크플로우 진행 맵 [P0]
- INCEPTION/CONSTRUCTION 2단계 페이즈와 하위 스테이지를 트리 형태로 시각화
- 현재 활성 스테이지 하이라이트, 완료/대기/스킵 상태 구분
- Complexity 레벨(Minimal/Standard/Comprehensive)에 따른 조건부 스테이지 표시/숨김
- devflow-state.md 파싱으로 현재 Phase/Stage/Complexity/Completed Stages/Approved Stages/Skipped Stages/Active Unit/Completed Units/Worktree 반영
- session-summary.md 파싱으로 Key Decisions, Completed Work(`[x]`/`[~]`/`[ ]` 마커), Next Steps 표시
- **파싱 정책**: 알 수 없는 `##` 섹션은 무시 (관용 파싱). 비표준 필드(Selected Approach, Project Root 등) 발견 시 raw key-value로 표시
- **파싱 안전성**: 파일이 중간 쓰기 상태(truncated/corrupt)일 경우 이전 유효 상태를 유지하고 "Syncing..." 표시. 다음 감시 주기에 재시도

### FR-2: Git 상태 패널 [P0]
- 현재 브랜치명, HEAD commit hash 표시
- git worktree 목록 및 상태
- 최근 커밋 히스토리 (최소 10개)
- 변경 파일 목록 (staged/unstaged/untracked 구분)
- diff 변화량 요약 (additions/deletions)
- **데이터 수집**: git CLI subprocess 기반 tokio::interval 폴링 (2초 주기). notify 파일 감시 대상에서 .git/ 제외

### FR-3: 에이전트 상태 패널 [P0]
- 실행 중/완료/대기 에이전트 목록 표시
- 에이전트 타입(Explore, Plan, code-review 등 Claude Code 공식 subagent_type) 구분
- parallel agent 동시 실행 시 각 에이전트별 상태 시각화
- SubagentStart/SubagentStop hooks의 `agent_id`로 Start↔Stop 매칭하여 라이프사이클 추적
- **Orphan 복구**: SubagentStop이 60초 내 도착하지 않으면 해당 에이전트를 "timeout" 상태로 전환. TUI 재시작 시 진행 중 에이전트 목록 초기화 (hooks 이벤트는 비영속)
- **Hooks 미설정 시**: "에이전트 추적을 위해 hooks 설정이 필요합니다" 안내 메시지 표시. 패널은 비활성 상태로 유지

### FR-4: 감사 로그 뷰 [P0]
- audit.md 또는 devflow-audit.md 실시간 테일 (양쪽 파일명 모두 탐색)
- 새 로그 엔트리 자동 스크롤 및 하이라이트
- 타임스탬프, 이벤트 유형별 컬러 구분
- **혼합 포맷 파싱**: 간략 형식(`[timestamp] stage — choice`)과 상세 형식(`## Stage` + 메타데이터) 양쪽 지원. 인식 불가 라인은 원문 그대로 표시
- **메모리 캡**: 최근 500줄만 메모리 버퍼에 유지. 오래된 엔트리는 자동 제거. 전체 로그는 파일에서 원본 유지
- **입력 새니타이즈**: 외부 소스 문자열(audit.md, hooks payload)에서 ANSI escape sequence 제거 후 표시

### FR-5: 산출물 목록/미리보기 [P1]
- devflow-docs/inception/, construction/ 하위 파일 목록
- 선택한 파일의 내용 미리보기 (마크다운 렌더링 또는 원문)
- 파일 변경 시 자동 새로고침

### FR-6: 게이트 알림 [P1]
- `Stop` hook의 `last_assistant_message`에서 게이트 텍스트 패턴 매칭으로 게이트 대기 감지
  - 패턴: `A)`, `B)`, `C)` 선택지가 포함된 메시지
  - 추가 탐지: `PostToolUse` + `AskUserQuestion` matcher
- 현재 대기 중인 게이트 유형과 선택지(A/B/C 등) 표시
- 시각적 알림 (깜빡임 또는 색상 변경)
- **Hooks 미설정 시**: 게이트 알림 비활성. session-summary.md의 `## Next Steps`를 대체 정보로 표시

### FR-7: Hooks 설정 관리 [P0]
- TUI 첫 실행 시 Claude Code hooks 설정 상태 확인
  - `~/.claude/settings.json` 또는 프로젝트 `.claude/settings.json` 에서 hooks 존재 여부 검사
- 미설정 시: 필요한 hooks 설정 JSON 스니펫을 화면에 표시하고 클립보드 복사 지원
  - **클립보드 fallback**: `pbcopy`(macOS) 미존재 시 `/tmp/devflow-tui-hooks.json`에 파일 저장 + 경로 안내
- 설정된 hooks의 HTTP endpoint가 TUI 서버 포트와 일치하는지 검증
- **ephemeral token 인증**: TUI 시작 시 랜덤 토큰 생성, hooks 스니펫의 URL에 `?token=<TOKEN>` 포함. axum 미들웨어에서 토큰 검증. 미일치 요청은 403 응답
- TUI 종료 시에도 hooks 설정은 유지 (사용자가 수동 제거)

### FR-8: 키보드 네비게이션 [P0]
- **패널 전환**: `Tab`/`Shift+Tab`으로 패널 순환 (워크플로우 맵 → Git → 에이전트 → 감사 로그 → 산출물 → 게이트)
- **패널 내 조작**: `j`/`k` 또는 `↑`/`↓`로 리스트 스크롤, `Enter`로 상세 보기
- **패널 확대/복귀**: `f`(focus)로 현재 패널 전체화면 확대, `Esc`로 대시보드 뷰 복귀
- **글로벌 단축키**: `q` 종료, `r` 수동 새로고침, `?` 키바인딩 도움말 오버레이
- **현재 포커스 패널**: 테두리 색상 변경으로 시각적 구분

### FR-9: 적응형 레이아웃 [P0]
- **최소 터미널 크기**: 120x30. 미달 시 "터미널을 120x30 이상으로 확대해주세요" 경고 표시
- **표준 레이아웃** (120x30+): 좌측 워크플로우 맵 + 우측 상단 Git/에이전트 + 우측 하단 감사 로그
- **와이드 레이아웃** (200x50+): 6개 패널 모두 표시
- **터미널 리사이즈**: crossterm resize 이벤트로 실시간 레이아웃 재배치

## Priority Levels
| 레벨 | 의미 |
|------|------|
| P0 | MVP 필수. 없으면 출시 불가 |
| P1 | MVP 권장. 시간 내 가능하면 포함, 아니면 v1.1 |
| P2 | 후순위. 향후 버전에서 추가 |

## Data Sources

### 파일 감시 (Source of Truth)
notify crate(FSEvents 기반)로 이벤트 감지 + debounce 적용. .git/ 디렉토리는 감시 대상에서 **제외** (FR-2는 별도 폴링).

| 파일 | 용도 | debounce |
|------|------|----------|
| devflow-state.md | Phase/Stage/Complexity 및 전체 워크플로우 상태 | 300ms |
| session-summary.md | 키 결정, 체크포인트(`[x]`/`[~]`/`[ ]`), 재개 정보 | 300ms |
| audit.md / devflow-audit.md | 감사 로그 (양쪽 파일명 탐색) | 300ms |
| devflow-docs/inception/*.md | 산출물 목록 | 1s |
| devflow-docs/construction/**/*.md | 산출물 목록 | 1s |

### Git CLI 폴링 (FR-2 전용)
tokio::interval 기반 2초 주기 폴링. notify 이벤트 감시 대신 git subprocess 직접 호출.

| 명령 | 용도 |
|------|------|
| `git status --porcelain=v2` | staged/unstaged/untracked 파일 |
| `git log --oneline -n 10` | 최근 커밋 히스토리 |
| `git worktree list --porcelain` | worktree 목록 |
| `git diff --stat` | 변경량 요약 |
| `git rev-parse --abbrev-ref HEAD` | 현재 브랜치 |
| `git rev-parse --short HEAD` | HEAD commit hash |

### Hooks HTTP (실시간 이벤트 보강)
TUI가 localhost에 경량 HTTP 서버를 내장. 기본 포트 9100 (--port 옵션으로 변경 가능). ephemeral token 인증 적용.

**서버 생명주기**:
- 기동 시 포트 바인딩 실패 → hooks 비활성 모드로 전환 (파일 감시만 동작), 경고 메시지 표시
- TUI 종료 시 graceful shutdown (진행 중 요청 완료 대기, 최대 3초 timeout)
- axum panic → TUI 이벤트 루프에 에러 전파, 터미널 상태 복원 후 에러 메시지 출력

**입력 검증**: 모든 hooks payload에서 ANSI escape sequence 제거, JSON 파싱 실패 시 무시 (로그만 기록)

| Hook 이벤트 | 용도 | Hooks 미설정 시 대체 |
|---|---|---|
| SubagentStart | 에이전트 스폰 감지 | 패널 비활성 + 안내 메시지 |
| SubagentStop | 에이전트 완료 (agent_id로 Start 매칭) | 패널 비활성 + 안내 메시지 |
| PreToolUse | 스킬/도구 실행 시작 | 없음 (파일 감시로 결과만 확인) |
| PostToolUse | 스킬/도구 실행 완료 | 없음 (파일 감시로 결과만 확인) |
| Stop | 턴 완료 + last_assistant_message로 게이트 감지 | session-summary.md의 Next Steps로 대체 |

### 이벤트 버스 아키텍처
파일 감시 이벤트와 Hooks HTTP 이벤트를 단일 `tokio::mpsc` 채널로 통합.

- **이벤트 타입**: `FileChanged(path)` | `HookReceived(event)` | `GitPolled(data)` | `Tick`
- **충돌 해결**: 수신 시각(monotonic clock) 기준 순서 처리. 동일 데이터에 대해 Hooks 이벤트가 파일 감시보다 먼저 도착하면 Hooks 우선 반영, 이후 파일 감시 이벤트는 diff 없으면 무시
- **렌더링**: dirty-flag 기반. 이벤트 처리 후 변경된 컴포넌트만 리렌더

### Hooks 의존도 매핑
| FR | 파일 감시만 | Hooks 추가 시 |
|----|-----------|--------------|
| FR-1 워크플로우 맵 | **완전 동작** | 스테이지 전환 즉시 반영 |
| FR-2 Git 상태 | **완전 동작** (폴링) | 해당 없음 |
| FR-3 에이전트 상태 | **비활성** (안내 메시지) | **완전 동작** |
| FR-4 감사 로그 | **완전 동작** | 해당 없음 |
| FR-5 산출물 | **완전 동작** | 해당 없음 |
| FR-6 게이트 알림 | **부분 동작** (Next Steps 표시) | **완전 동작** (패턴 매칭) |
| FR-7 Hooks 설정 | **완전 동작** (설정 안내) | **완전 동작** (상태 검증) |

## devflow-state.md 파싱 스키마

```
## Current Phase        → enum: INCEPTION | CONSTRUCTION | complete | finished
## Current Stage        → string (스테이지 ID)
## Complexity           → enum: Minimal | Standard | Comprehensive
## Selected Approach    → string (선택된 접근법, 비표준 필드)
## Completed Stages     → list: "- stage-name (timestamp)"
## Approved Stages      → list: "- stage-name — depth: Standard"
## Skipped Stages       → list: "- stage-name — reason: ..."
## Active Unit          → string (현재 구현 중인 유닛명)
## Completed Units      → list: "- unit-name"
## Worktree             → key-value: branch, path
## Extension Configuration → key-value (플러그인 확장 설정)
## Project Root         → string (비표준, 일부 프로젝트에서 사용)
## Finishing Choice     → string (비표준, PR 관련)
## PR URL               → string (비표준, PR URL)
```
알 수 없는 `##` 섹션 → raw key-value로 파싱, UI에서 "기타" 영역에 표시.

## Technology Stack
| 항목 | 스택 |
|------|------|
| 언어 | Rust (edition 2024) |
| TUI 프레임워크 | ratatui 0.30 + crossterm 0.29 |
| Async 런타임 | tokio 1 (rt-multi-thread, macros, sync, time, process, io-util, fs) |
| HTTP 서버 | axum (hooks 수신용, localhost 전용, ephemeral token 인증) |
| 파일 감시 | notify crate (FSEvents/inotify) + debounce |
| 직렬화 | serde + serde_json + serde_yaml |
| Git 연동 | git CLI subprocess (tokio::process::Command) |
| 에러 처리 | thiserror 2 |
| 로깅 | tracing + tracing-subscriber + tracing-appender |
| 아키텍처 | TEA + Port/Adapter + Component trait (nexttui 준수) |
| 테스트 | cargo test + clippy strict |

## Execution Environment
- 독립 터미널에서 실행 (Claude Code와 별도)
- tmux/split 터미널로 나란히 배치하는 사용 패턴
- Claude Code 세션과 같은 프로젝트 디렉토리에서 실행
- 최소 터미널 크기: 120x30

## Design Principles
- devflow-state.md가 워크플로우 상태의 single source of truth
- Hooks는 실시간 이벤트 보강 (hooks 없이도 파일 감시만으로 기본 동작)
- nexttui 아키텍처 패턴 준수 (Component trait, TEA, Port/Adapter). 단, nexttui의 인프라 서비스(AuthManager, RbacGuard 등)는 도메인이 다르므로 적용하지 않음. 공유할 패턴: App→Router→Component 계층, Worker→Event 비동기 패턴, ModuleRegistry 동적 등록 (MVP에서는 정적 컴포넌트 조합으로 시작, 모듈 확장 시 동적 등록 도입)
- Hooks 미설정 시 FR별 명시된 degradation 동작 수행
- 모든 외부 입력(파일 내용, hooks payload)에 ANSI escape sequence 새니타이즈 적용
- 단일 이벤트 버스로 파일 감시/Hooks/Git 폴링 이벤트 통합, dirty-flag 기반 렌더링

## Assumptions
- devflow-docs/ 디렉토리가 TUI 실행 디렉토리 하위에 존재
- Git 저장소가 초기화되어 있음
- Claude Code hooks는 사용자가 수동 설정 (TUI가 토큰 포함 설정 스니펫 제공)

## Impact Analysis
- **스코프**: 신규 Rust 프로젝트 (기존 코드 영향 없음)
- **리스크**: hooks HTTP 서버의 포트 충돌 가능성 → --port 옵션으로 설정 가능. 기동 실패 시 hooks 비활성 모드로 폴백
- **의존성**: aidlc-devflow 플러그인의 파일 포맷이 변경되면 파서 업데이트 필요. 관용 파싱 정책으로 완충

## Review History
- **v1**: 초안 작성
- **v2**: spec-reviewer 리뷰 반영 (게이트 감지 수정, 파싱 스키마 확장, hooks 설정 FR 추가, degradation 매핑)
- **v3**: agent-council 리뷰 반영 (Codex: Git 폴링 분리, 이벤트 버스, git CLI 확정, orphan 복구, axum 생명주기 / Gemini: token 인증, ANSI 새니타이즈, 키보드 네비게이션, 적응형 레이아웃, 메모리 캡)
