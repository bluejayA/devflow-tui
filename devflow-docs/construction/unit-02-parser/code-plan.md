# Code Plan: Unit 2 — 파일 파서

## 개요
devflow-state.md, session-summary.md, audit.md 파서 구현. 순수 함수, I/O 없음. 골든 테스트 필수.

## 파일 목록 및 구현 순서

### Step 1: ANSI 새니타이저
- **파일**: `src/service/mod.rs`, `src/service/sanitizer.rs`
- **작업**: ANSI escape sequence 제거 함수
- **테스트**: `test_strip_ansi_escape`, `test_no_ansi_passthrough`, `test_mixed_content`

### Step 2: devflow-state.md 파서
- **파일**: `src/parser/devflow_state.rs`
- **작업**:
  - `## Section` 헤더 기반 섹션 분리
  - 각 섹션별 파싱 (Phase, Stage, Complexity, list items, key-value)
  - 알 수 없는 섹션 → extra_fields HashMap
  - 비표준 필드 (Selected Approach, Project Root 등) 처리
  - 빈 파일/손상 파일 → AppError::ParseFlowState
- **테스트**: 골든 테스트 5개
  - `golden_minimal.md` — Minimal 상태
  - `golden_standard.md` — Standard (전체 필드)
  - `golden_construction.md` — CONSTRUCTION 진행 중
  - `golden_nonstandard.md` — 비표준 필드 포함
  - `golden_empty.md` — 빈 파일 (에러 아닌 기본값)

### Step 3: session-summary.md 파서
- **파일**: `src/parser/session_summary.rs`
- **작업**:
  - `## Section` 기반 섹션 분리
  - Key Decisions 리스트 파싱
  - Completed Work `[x]`/`[~]`/`[ ]` 마커 파싱
  - Next Steps, For Next Session 파싱
  - Current State 메타데이터 (Phase, Stage, Complexity, Commit)
- **테스트**: 골든 테스트 3개
  - `golden_inception_session.md` — INCEPTION 진행 중
  - `golden_construction_session.md` — CONSTRUCTION 진행 중
  - `golden_empty_session.md` — 빈 파일

### Step 4: audit.md 파서
- **파일**: `src/parser/audit_log.rs`
- **작업**:
  - 간략 형식 파싱: `[timestamp] stage — choice`
  - 상세 형식 파싱: `## Stage` + `**Timestamp**:` + 메타데이터
  - 혼합 포맷 자동 감지
  - 인식 불가 라인 → raw_line 보존
  - ANSI 새니타이즈 적용
- **테스트**: 골든 테스트 3개
  - `golden_brief_audit.md` — 간략 형식만
  - `golden_detailed_audit.md` — 상세 형식만
  - `golden_mixed_audit.md` — 혼합 + 인식 불가 라인

### Step 5: parser/mod.rs 통합
- **파일**: `src/parser/mod.rs` 업데이트
- **작업**: 서브모듈 export, 편의 함수 (parse_flow_state, parse_session_summary, parse_audit_log)

## Verification Contract

### 완료 체크리스트
- [ ] `cargo test` 전체 통과
- [ ] `cargo clippy -- -D warnings` 경고 0개
- [ ] 골든 테스트 11개 포함
- [ ] 모든 파서가 순수 함수 (I/O 없음, &str 입력)
- [ ] 빈/손상 입력에 패닉 없이 기본값 또는 에러 반환
- [ ] ANSI escape 제거가 모든 파서 출력에 적용

### Validation Commands
```bash
cargo test -- parser
cargo test -- service
cargo clippy -- -D warnings
```
