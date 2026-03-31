# Non-Functional Requirements

> **측정 기준**: 모든 성능/메모리 목표는 Release 빌드, macOS Apple Silicon(M1+) 기준. p95 백분위수 적용. warm 상태(첫 렌더 이후) 기준 (시작 시간만 cold).

## NFR-1: 성능 (Performance)

### NFR-1.1: UI 반응성
- 키 입력 후 화면 갱신까지 p95 **16ms 이내** (60fps 기준)
- ratatui immediate-mode 렌더링 모델 적용:
  - 매 tick마다 `terminal.draw()` 호출, ratatui 내부 backend diff가 변경된 셀만 업데이트
  - 상태 변경 없는 tick에서는 draw 호출 자체를 스킵 (no-change frame skip)
  - draw tick 주기: 기본 **50ms** (20fps). 이벤트 수신 시 즉시 추가 draw 트리거

### NFR-1.2: 데이터 갱신 지연
- **단일 파일 저장(single-save) 기준**: 파일 변경 이벤트 → UI 반영 p95 **500ms 이내** (300ms debounce + 200ms 처리)
- **burst write 기준**: 연속 쓰기가 settle(마지막 쓰기 후 300ms 무변경)된 시점부터 UI 반영 p95 **200ms 이내**
- Hooks HTTP 이벤트 → UI 반영: p95 **100ms 이내**
- Git 폴링 → UI 반영: **2초 주기 + p95 200ms 처리**

### NFR-1.3: 시작 시간
- TUI 기동 ~ 첫 화면 렌더: cold p95 **1초 이내** (axum 서버 기동 포함)
- devflow-docs/ 초기 파싱: 파일 10개 기준 p95 **200ms 이내**
- 파일 50개 이상인 대규모 프로젝트: 초기 파싱을 background tokio task로 오프로드, UI는 즉시 렌더 (파싱 완료 전 "Loading..." 표시)

### NFR-1.4: Git 폴링 부하
- git subprocess 호출은 2초 주기로 제한. 동시 실행 방지 (이전 호출 완료 전 다음 호출 스킵)
- git 명령 실행 timeout: **5초** (초과 시 스킵, 다음 주기에 재시도)

### NFR-1.5: CPU 사용량
- idle 상태 (이벤트 없음): CPU **< 3%**
- 활성 상태 (파일 변경 + hooks 수신): CPU 스파이크 허용하되 p95 **< 15%**
- idle 시 불필요한 폴링/렌더 루프 방지: 이벤트 기반 대기 (tokio::select)

## NFR-2: 메모리 (Memory)

### NFR-2.1: 메모리 사용량
- **idle 시나리오** (TUI 기동 후 파일 변경 없음, hooks 이벤트 없음): RSS p95 **30MB 이하**
- **active 시나리오** (파일 감시 + hooks 수신 + Git 폴링 동시 활성, 에이전트 5개 실행 중): RSS p95 **50MB 이하**
- 측정: `ps -o rss` 또는 `/proc/self/status` VmRSS, 10초 간격 60회 샘플링

### NFR-2.2: 메모리 캡
- 감사 로그 버퍼: 최근 **1000줄** (환경변수 `DEVFLOW_TUI_LOG_BUFFER`로 설정 가능)
- 에이전트 히스토리: 최근 **100개** 엔트리 (timeout 포함)
- Git 커밋 히스토리: **10개** 고정
- 이벤트 채널 버퍼: bounded **256개** (초과 시 oldest drop + tracing::warn 로그)

## NFR-3: 안정성 (Reliability)

### NFR-3.1: 에러 복구
- 파일 파싱 실패 → 이전 유효 상태 유지 + "Syncing..." 표시 + 다음 주기 재시도
- Git subprocess 실패 → 이전 상태 유지 + 로그 기록 + 다음 주기 재시도
- axum 서버 기동 실패 → 자동 대체 포트 탐색 (9100~9110 범위). 전부 실패 시 hooks 비활성 모드 + 경고 표시 + 상태바에 "Hooks: OFF (port unavailable)" 지속 표시
- axum task panic → **JoinHandle supervisor**가 감지하여 TUI 이벤트 루프에 에러 전파. 단일 종료 경로로 터미널 복원 보장

### NFR-3.2: Graceful Shutdown
- `q` 키 또는 SIGINT/SIGTERM → axum 서버 shutdown(최대 3초) → 터미널 상태 복원 → 프로세스 종료
- panic 발생 시 panic hook으로 터미널 raw mode 해제 + alternate screen 복원
- **단일 종료 경로**: 정상 종료/panic/signal 모두 동일한 cleanup 함수를 거침

### NFR-3.3: 파일 감시 안정성
- notify watcher 오류 시 자동 재생성 (최대 3회, 이후 fallback으로 5초 폴링)
- devflow-docs/ 삭제 후 재생성 시 자동 감지하여 모니터링 재개

### NFR-3.4: Backpressure 정책
- 이벤트 채널 overflow 시: oldest drop + tracing::warn + UI에 "Events dropping — high activity" 일시 표시
- hooks HTTP burst 시: axum은 요청을 순차 처리, 큐 초과 시 503 응답
- notify overflow 시: fallback 폴링으로 전환 (NFR-3.3과 동일 경로)
- SubagentStart/Stop 매칭 유실 시: orphan timeout(60초)으로 일관성 복구

## NFR-4: 보안 (Security)

### NFR-4.1: Hooks 서버 인증
- **프로젝트 기반 stable token**: 프로젝트 디렉토리 경로의 SHA-256 해시 + 고정 salt로 결정적 토큰 생성. TUI 재시작 시에도 동일 토큰 유지 → settings.json 재수정 불필요
- 토큰 저장: `devflow-docs/.tui-token` 파일에 저장 (.gitignore 대상)
- 모든 hooks endpoint에 `?token=<TOKEN>` 쿼리 파라미터 검증
- 미일치/미제공 시 403 응답 (body 없음)
- 토큰 재생성: `--regenerate-token` CLI 옵션으로 강제 갱신 가능

### NFR-4.2: 입력 새니타이즈
- 외부 소스(파일 내용, hooks JSON payload)에서 ANSI escape sequence 제거
- hooks JSON 파싱 실패 시 요청 무시 + tracing::warn 로그

### NFR-4.3: 네트워크 바인딩
- axum 서버는 `127.0.0.1`에만 바인딩 (0.0.0.0 금지)
- 외부 네트워크 접근 불가

## NFR-5: 사용성 (Usability)

### NFR-5.1: 학습 곡선
- 하단 상태바에 주요 단축키 힌트 항상 표시
- `?` 키로 전체 키바인딩 오버레이
- 첫 실행 시 hooks 미설정이면 설정 가이드 자동 표시

### NFR-5.2: 터미널 호환성
- **최소 터미널 크기**: 120x30
- **축소 터미널 대응** (80x24 ~ 119x29): 단일 패널 모드 — 워크플로우 맵만 표시, Tab으로 다른 패널 전환 (한 번에 하나만). 상태바에 "축소 뷰 — 120x30 이상에서 전체 대시보드 표시" 안내
- **최소 미만** (80x24 미만): "터미널을 80x24 이상으로 확대해주세요" 경고만 표시
- 지원 터미널: macOS Terminal, iTerm2, Alacritty, WezTerm, tmux
- 256 color 지원 (true color 선호, 미지원 시 256 color fallback)
- **NO_COLOR 환경변수 존중**: NO_COLOR 설정 시 모든 컬러 비활성, 텍스트 라벨만으로 상태 구분

### NFR-5.3: 접근성
- 모든 상태 정보에 컬러 외 텍스트 라벨 병행 (컬러만으로 구분하지 않음)
- 예: ✓(완료), ●(실행중), ⏱(타임아웃), ○(대기)

## NFR-6: 유지보수성 (Maintainability)

### NFR-6.1: 코드 품질
- `cargo clippy` 경고 0개 (deny: unwrap_used, expect_used)
- 테스트 커버리지: 파서 모듈 **90%+**, UI 로직 **70%+**
- 파서에 골든 테스트 필수 (devflow-state.md/session-summary.md/audit.md 실제 변형 샘플)
- **테스트 harness 요구**: fake clock(tokio::time::pause), fixture replay, golden output normalization(타임스탬프/경로 치환)으로 CI deterministic 보장

### NFR-6.2: 모듈 구조
- Port/Adapter 패턴으로 데이터 소스 추상화 (파일 감시, Git, Hooks를 trait 기반으로 분리)
- Component trait로 UI 패널 독립 (추가/제거 시 다른 패널 영향 없음)
- MVP는 정적 컴포넌트 조합, 향후 ModuleRegistry 동적 등록 확장

### NFR-6.3: 설정 가능성
- CLI 옵션: `--port <PORT>` (기본 9100), `--project-dir <PATH>` (기본 현재 디렉토리), `--demo` (샘플 데이터 모드), `--regenerate-token` (토큰 재생성)
- 환경 변수: `DEVFLOW_TUI_LOG` (로그 레벨), `DEVFLOW_TUI_PORT`, `DEVFLOW_TUI_LOG_BUFFER` (로그 버퍼 크기)

## NFR-7: 이식성 (Portability)

### NFR-7.1: 플랫폼 지원
- **Primary**: macOS (Darwin) — 개발 및 테스트 주 환경
- **Secondary**: Linux — CI/서버 환경 지원
- Windows: 지원 계획 없음 (crossterm은 Windows 지원하지만 우선순위 외)

### NFR-7.2: 빌드 및 의존성
- `cargo build --release` 단일 바이너리 생성
- **필수 외부 의존성**: git CLI (PATH에 존재 필요)
- **선택적 외부 의존성**: `pbcopy`(macOS 클립보드). 미존재 시 파일 저장 fallback — 크래시 없음

## Review History
- **v1**: 초안 (7 카테고리, 18 세부 항목)
- **v2**: agent-council 리뷰 반영 — ratatui immediate-mode 렌더링 모델, SLO p95 측정 조건, burst write/single-save 구분, CPU NFR 추가, JoinHandle supervisor, 대체 포트 탐색, 축소 터미널 대응, stable token, NO_COLOR, 테스트 harness, backpressure 정책
