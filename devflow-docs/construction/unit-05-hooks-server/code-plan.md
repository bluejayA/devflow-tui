# Code Plan: Unit 5 — Hooks HTTP 서버 + 서비스

## 개요
axum HTTP 서버 (hooks 수신), stable token 생성/검증, hooks 설정 감지, CommandRunner.

## 파일 목록

### Step 1: Token 서비스
- **파일**: `src/service/token.rs`
- **작업**: 프로젝트 디렉토리 기반 stable token (SHA-256 + salt), `--regenerate-token` 시 파일 삭제 후 재생성, `.tui-token` 파일 관리
- **테스트**: 동일 디렉토리 → 동일 토큰, 다른 디렉토리 → 다른 토큰, 재생성 테스트

### Step 2: Hook Config Detection
- **파일**: `src/service/hook_config.rs`
- **작업**: `~/.claude/settings.json` + `.claude/settings.json` hooks 설정 검사, JSON 스니펫 생성 (토큰 포함)
- **테스트**: 설정 파일 존재/미존재 시나리오, 스니펫 생성 검증

### Step 3: HooksReceiver 포트 + enum dispatch
- **파일**: `src/port/hooks.rs`
- **작업**: `HooksReceiverPort` enum (Axum, Mock)
- **테스트**: Mock 기본 동작

### Step 4: Hooks HTTP 서버 구현
- **파일**: `src/adapter/hooks_server.rs`
- **작업**:
  - `run(cancel, port, token, event_tx)` async 함수
  - POST `/hook` endpoint
  - `?token=<TOKEN>` 쿼리 파라미터 검증 (403)
  - JSON payload → AppEvent 변환
  - ANSI 새니타이즈 적용
  - 포트 바인딩 실패 시 9100~9110 탐색
  - CancellationToken graceful shutdown (3초)
- **테스트**: axum test utilities로 endpoint 테스트 (토큰 검증, payload 파싱, 403 응답)

### Step 5: CommandRunner
- **파일**: `src/command.rs`
- **작업**: 비동기 side effect 실행 (clipboard 복사, 파일 fallback)
- **테스트**: CommandCompleted/CommandFailed 이벤트 생성 테스트

## Verification Contract
- [ ] 토큰 생성/검증 테스트 통과
- [ ] hooks 설정 감지 테스트 통과
- [ ] HTTP endpoint 통합 테스트 통과
- [ ] `cargo clippy -- -D warnings` 경고 0개
