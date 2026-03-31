# User Stories

## Actor
- **Developer**: aidlc-devflow 플러그인을 사용하여 프로젝트를 개발하는 개발자. Claude Code와 별도 터미널에서 TUI 대시보드를 통해 워크플로우 진행 상황을 모니터링한다.

---

## Epic 1: 워크플로우 모니터링

### US-1: 워크플로우 진행 상태 확인 [P0]
**As a** Developer,
**I want to** INCEPTION/CONSTRUCTION 단계별 진행 상태를 트리 형태로 한눈에 보고 싶다,
**So that** 현재 어떤 단계에 있고, 어떤 단계가 남았는지 파악하여 작업 흐름을 제어할 수 있다.

**Acceptance Criteria:**
- [ ] 현재 Phase(INCEPTION/CONSTRUCTION/complete/finished)가 레이블로 표시된다
- [ ] 각 스테이지의 상태가 컬러 토큰으로 구분된다: 활성(Yellow), 완료(Green), 대기(Gray), 스킵(DarkGray+Strikethrough)
- [ ] Complexity에 따라 조건부 스테이지가 표시/숨김 처리된다
- [ ] devflow-state.md가 변경되면 300ms debounce 후 UI에 반영된다
- [ ] 파일이 중간 쓰기 상태일 때 이전 유효 상태를 유지하고 "Syncing..." 표시
- [ ] 알 수 없는 `##` 섹션은 무시하고, 비표준 필드(Selected Approach, Project Root 등)는 "기타" 영역에 raw key-value로 표시된다

### US-2: 세션 진행 이력 확인 [P0]
**As a** Developer,
**I want to** 이번 세션에서 내린 결정과 완료된 작업을 확인하고 싶다,
**So that** 세션 재개 시 맥락을 빠르게 파악할 수 있다.

**Acceptance Criteria:**
- [ ] session-summary.md의 Key Decisions 목록이 표시된다
- [ ] Completed Work의 마커가 시각적으로 구분된다: `[x]`(Green ✓), `[~]`(Yellow ◐), `[ ]`(Gray ○)
- [ ] Next Steps가 표시된다

---

## Epic 2: Git 추적

### US-3: Git 상태 모니터링 [P0]
**As a** Developer,
**I want to** 현재 브랜치, 최근 커밋, 변경 파일, worktree를 실시간으로 보고 싶다,
**So that** devflow가 수행한 코드 변경을 Claude Code 화면 전환 없이 파악할 수 있다.

**Acceptance Criteria:**
- [ ] 현재 브랜치명과 HEAD commit hash가 표시된다
- [ ] 최근 10개 커밋 히스토리가 표시된다
- [ ] staged(Green), unstaged(Yellow), untracked(Gray), conflict(Red+Bold) 파일이 컬러 구분 표시된다
- [ ] additions(Green +N) / deletions(Red -N) 변화량이 요약 표시된다
- [ ] `git worktree list` 결과가 브랜치명, 경로와 함께 표시된다
- [ ] 공통 Git 폴링 어댑터(tokio::interval 2초)로 모든 Git 데이터를 수집한다

> **Note**: 기존 US-4(Worktree)를 US-3에 통합. 동일 Git 폴링 파이프라인을 공유하므로 분리 의미 없음.

---

## Epic 3: 에이전트 추적

### US-4: 에이전트 실행 상태 확인 [P0]
**As a** Developer,
**I want to** Claude의 서브에이전트가 언제 스폰되고 완료되는지 실시간으로 보고 싶다,
**So that** 병렬 에이전트 작업의 진행 상황을 파악하고 대기 시간을 예측할 수 있다.

**Acceptance Criteria:**
- [ ] 실행 중 에이전트가 타입(Explore, Plan 등)과 함께 목록 표시된다
- [ ] SubagentStart의 `agent_id`와 SubagentStop의 `agent_id`를 매칭하여 라이프사이클을 추적한다
- [ ] 에이전트 상태가 컬러 토큰으로 구분된다: running(Yellow ●), done(Green ✓), timeout(Red ⏱)
- [ ] parallel agent 동시 실행 시 각각 개별 행으로 표시된다
- [ ] 60초 내 Stop 미수신 시 "timeout" 상태로 전환된다
- [ ] TUI 재시작 시 에이전트 목록이 초기화된다 (hooks 이벤트는 비영속)

### US-5: Hooks 미설정 시 안내 [P0]
**As a** Developer,
**I want to** hooks가 설정되지 않았을 때 명확한 안내를 받고 싶다,
**So that** 에이전트 추적 기능을 활성화하는 방법을 알 수 있다.

**Acceptance Criteria:**
- [ ] hooks 미설정 시 에이전트 패널에 "에이전트 추적을 위해 hooks 설정이 필요합니다" 안내 메시지가 표시된다
- [ ] 패널 테두리가 DarkGray로 비활성 상태 표시된다
- [ ] hooks 설정 상태 판별은 US-9(Hooks 설정 관리)의 Hook Config Detection 서비스를 사용한다

---

## Epic 4: 감사 로그

### US-6: 실시간 감사 로그 모니터링 [P0]
**As a** Developer,
**I want to** devflow의 감사 로그를 실시간으로 tail하며 보고 싶다,
**So that** 워크플로우에서 어떤 결정이 내려지고 있는지 즉시 파악할 수 있다.

**Acceptance Criteria:**
- [ ] audit.md 또는 devflow-audit.md 중 존재하는 파일을 자동 탐색한다
- [ ] 새 엔트리 추가 시 자동 스크롤 및 하이라이트된다
- [ ] 타임스탬프(Cyan)와 이벤트 유형별(선택=Green, 스킵=DarkGray, 에러=Red) 컬러가 구분된다
- [ ] 간략/상세 혼합 포맷이 모두 파싱된다
- [ ] 인식 불가 라인은 원문 그대로 표시된다
- [ ] 메모리에 최근 500줄만 유지된다
- [ ] ANSI escape sequence가 제거된 상태로 표시된다

---

## Epic 5: 산출물 관리

### US-7: 산출물 목록 및 미리보기 [P1]
**As a** Developer,
**I want to** devflow가 생성한 inception/construction 산출물을 목록으로 보고 내용을 미리보기하고 싶다,
**So that** 설계 문서와 코드 계획을 별도 에디터 없이 확인할 수 있다.

**Acceptance Criteria:**
- [ ] inception/, construction/ 하위 .md 파일 목록이 트리 형태로 표시된다
- [ ] 파일 선택(Enter) 시 내용이 미리보기 패널에 표시된다
- [ ] 미리보기 패널에서 j/k로 콘텐츠 스크롤이 가능하다 (f 확대 모드에서도 동일)
- [ ] 파일 변경 시 자동 새로고침된다

---

## Epic 6: 게이트 알림

### US-8: 게이트 대기 알림 수신 [P1]
**As a** Developer,
**I want to** Claude가 A/B 선택을 기다리고 있을 때 시각적 알림을 받고 싶다,
**So that** 다른 작업 중에도 입력이 필요한 시점을 놓치지 않을 수 있다.

**Acceptance Criteria:**
- [ ] Stop hook의 last_assistant_message에서 A)/B)/C) 패턴이 감지된다
- [ ] PostToolUse + AskUserQuestion matcher로도 게이트가 감지된다
- [ ] 감지 시 게이트 유형과 선택지가 표시된다
- [ ] 시각적 알림: 게이트 패널 테두리 깜빡임(Yellow/White 교대, 1초 주기)
- [ ] **알림 종료 조건**: 다음 Stop hook 수신 시(= Claude가 새 턴 시작) 또는 devflow-state.md의 Stage가 변경되면 자동 해제. 수동 해제: `Esc` 키
- [ ] hooks 미설정 시 session-summary.md의 Next Steps로 대체 표시된다

---

## Epic 7: 설정 및 환경

### US-9: Hooks 설정 관리 [P0]
**As a** Developer,
**I want to** TUI 시작 시 hooks 설정 상태를 확인하고, 미설정 시 설정 방법을 안내받고 싶다,
**So that** 최소한의 노력으로 실시간 기능을 활성화할 수 있다.

**Acceptance Criteria:**
- [ ] **Hook Config Detection 서비스**: `~/.claude/settings.json` 또는 프로젝트 `.claude/settings.json`에서 hooks 설정 존재 여부를 검사하고 결과를 다른 컴포넌트(US-4, US-5, US-8)에 제공한다
- [ ] 미설정 시 ephemeral token 포함 JSON 스니펫이 표시된다
- [ ] ephemeral token이 TUI 시작 시 랜덤 생성되고, hooks URL에 `?token=<TOKEN>` 포함된다
- [ ] axum 미들웨어에서 토큰 미일치 요청은 403 응답한다
- [ ] 클립보드 복사가 지원되고, `pbcopy` 미존재 시 `/tmp/devflow-tui-hooks.json`에 저장 + 경로 안내
- [ ] 설정된 endpoint와 TUI 서버 포트 일치가 검증된다
- [ ] TUI 종료 시 hooks 설정은 유지된다 (사용자가 수동 제거)

### US-10: 키보드로 패널 탐색 [P0]
**As a** Developer,
**I want to** 키보드만으로 모든 패널을 탐색하고 상세 정보를 확인하고 싶다,
**So that** 마우스 없이 효율적으로 대시보드를 사용할 수 있다.

**Acceptance Criteria:**
- [ ] Tab/Shift+Tab으로 P0 패널 간 순환이 된다 (워크플로우 맵 → Git → 에이전트 → 감사 로그)
- [ ] P1 패널(산출물, 게이트)은 와이드 레이아웃에서만 Tab 순환에 포함된다
- [ ] j/k 또는 ↑/↓로 리스트 스크롤이 된다
- [ ] Enter로 선택 항목의 상세 정보가 표시된다 (커밋 상세, 에이전트 상세, 로그 엔트리 상세)
- [ ] f로 패널 전체화면 확대, Esc로 복귀된다
- [ ] 전체화면 모드에서 j/k는 콘텐츠 스크롤로 동작한다
- [ ] q로 종료, r로 수동 새로고침, ?로 도움말이 동작한다
- [ ] 현재 포커스 패널이 테두리 Cyan으로 구분된다
- [ ] 하단 상태바에 주요 단축키 힌트가 항상 표시된다 (Tab:패널 j/k:스크롤 f:확대 ?:도움말)

### US-11: 터미널 크기 적응 [P0]
**As a** Developer,
**I want to** 다양한 터미널 크기에서 대시보드가 적절히 표시되길 원한다,
**So that** tmux 분할 환경이나 작은 터미널에서도 사용할 수 있다.

**Acceptance Criteria:**
- [ ] 120x30 미만 시 경고 메시지가 표시된다: "터미널을 120x30 이상으로 확대해주세요 (현재: NxM)"
- [ ] 120x30+ 에서 표준 레이아웃: 좌측 워크플로우 맵(40%) + 우측 상단 Git/에이전트(60%의 상반) + 우측 하단 감사 로그(60%의 하반) — P0 패널만
- [ ] 200x50+ 에서 와이드 레이아웃: P0 4패널 + P1 2패널(산출물, 게이트) 추가
- [ ] 터미널 리사이즈 시 crossterm resize 이벤트로 실시간 레이아웃 재배치된다

### US-12: devflow 미설정 디렉토리 안내 [P0]
**As a** Developer,
**I want to** devflow가 설정되지 않은 디렉토리에서 TUI를 실행했을 때 명확한 안내를 받고 싶다,
**So that** 올바른 디렉토리에서 실행하거나 devflow를 시작하는 방법을 알 수 있다.

**Acceptance Criteria:**
- [ ] `devflow-docs/` 디렉토리 미존재 시 "devflow 프로젝트가 감지되지 않았습니다" 안내 표시
- [ ] "devflow-docs/ 디렉토리가 있는 프로젝트 경로에서 실행하거나, Claude Code에서 'devflow 시작해줘'로 새 프로젝트를 시작하세요" 가이드 표시
- [ ] devflow-docs/가 이후 생성되면 자동으로 감지하여 대시보드 활성화 (파일 감시로 탐지)

---

## Story Map 요약

| Epic | P0 Stories | P1 Stories |
|------|-----------|-----------|
| 워크플로우 모니터링 | US-1, US-2 | — |
| Git 추적 | US-3 | — |
| 에이전트 추적 | US-4, US-5 | — |
| 감사 로그 | US-6 | — |
| 산출물 관리 | — | US-7 |
| 게이트 알림 | — | US-8 |
| 설정 및 환경 | US-9, US-10, US-11, US-12 | — |
| **합계** | **10** | **2** |

## Review History
- **v1**: 초안 (12 stories)
- **v2**: agent-council 리뷰 반영 — US-3/US-4 통합, P0/P1 의존성 해소, FR 누락 AC 보충, 책임 경계 명확화, 콘텐츠 스크롤/게이트 종료 조건/Git 충돌 상태/초기 안내 추가. 번호 재정렬
