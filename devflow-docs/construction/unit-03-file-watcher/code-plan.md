# Code Plan: Unit 3 — 파일 감시 어댑터

## 개요
notify crate + debounce로 devflow-docs/ 파일 변경 감시. 변경 시 파서 호출 → watch/mpsc 채널로 이벤트 전송.

## 파일 목록

### Step 1: AdapterHandle 유틸
- **파일**: `src/adapter/mod.rs`, `src/adapter/handle.rs`
- **작업**: `AdapterHandle` struct (CancellationToken + JoinHandle), `spawn()`, `shutdown()`, `is_finished()`
- **테스트**: spawn/shutdown 라이프사이클 테스트 (tokio::time::pause)

### Step 2: FileWatcher 포트 + enum dispatch
- **파일**: `src/port/mod.rs`, `src/port/watcher.rs`
- **작업**: `FileWatcherPort` enum (Real, Mock), `start()` 메서드
- **테스트**: Mock watcher 기본 동작 테스트

### Step 3: FileWatcher 어댑터 구현
- **파일**: `src/adapter/file_watcher.rs`
- **작업**:
  - `run(cancel, paths, event_tx, flow_state_tx)` async 함수
  - notify::recommended_watcher + debounce(300ms)
  - 파일 변경 감지 → 해당 파서 호출 → 채널 전송
  - devflow-docs/ 미존재 시 대기 + 생성 감지
  - 파싱 실패 시 이전 상태 유지 + AppEvent::ParseError 전송
  - CancellationToken으로 graceful 종료
- **테스트**: 임시 디렉토리에 파일 쓰기 → 이벤트 수신 확인 (통합 테스트)

## Verification Contract
- [ ] `cargo test -- adapter` 통과
- [ ] `cargo test -- port` 통과
- [ ] `cargo clippy -- -D warnings` 경고 0개
