# Council Code Review Synthesis — Unit 11 (App Integration)

## Reviewers
- **Spec Reviewer (Claude)**: Spec compliance
- **Codex**: Code quality, structure, DRY
- **Gemini**: Security, edge cases, resource management

## Gate Decision: CONDITIONAL

## Rationale
구현의 전체적인 구조와 품질은 양호하나 (182 tests, clippy clean), High 이슈 2건과 actionable Medium 이슈 3건을 수정해야 합니다.

---

## Action Items (수정 필수)

### High

| # | 이슈 | 소스 | 파일 |
|---|------|------|------|
| H1 | **copy_hooks_snippet Mismatch 경로 버그** — Mismatch 시 diagnostic 문자열을 복사하는데, 새 스니펫을 생성해서 복사해야 함 | Codex + Spec | `app.rs:335` |
| H2 | **AdapterHandle::shutdown timeout 시 task leak** — timeout 경과 후 JoinHandle이 drop되어 task가 detached됨 (abort 필요) | Codex | `adapter/handle.rs` (기존 코드, Unit 5) |

### Medium (수정 권장)

| # | 이슈 | 소스 | 파일 |
|---|------|------|------|
| M1 | **Adapter crash 경고 spam** — is_finished() 체크가 매 루프마다 warn 발생, dedup 필요 | Codex | `event_loop.rs:89` |
| M2 | **ensure_valid_focus panic 가능성** — available_panels()가 빈 벡터 반환 시 index panic | Gemini | `app.rs` |
| M3 | **누락 테스트 2건** — test_handle_key_c_copies_snippet, test_check_hooks_configured | Spec | `app.rs` tests |

### Low (기록)

| # | 이슈 | 소스 |
|---|------|------|
| L1 | Dead code: unused `event_tx` field, unused Duration import in demo.rs | Codex |
| L2 | Token 예측 가능성 (deterministic SHA-256) — 로컬 전용이므로 위험도 낮음 | Gemini |
| L3 | on_resize 시 scroll_offset 리셋 (0으로) — clamp이 더 나은 UX | Gemini |
| L4 | OpenArtifactModal 무시됨 — Unit 12 범위이므로 현재 OK | Codex |
| L5 | Ctrl+j/k가 noop인데 help에 표시 — Standard에서는 의도된 동작 | Codex |

### Dismissed (기각)

| 이슈 | 사유 |
|------|------|
| Gemini CRITICAL: 토큰 예측 가능성 | 로컬 전용 도구 (127.0.0.1 바인딩), 위협 모델에서 같은 머신의 악의적 프로세스는 범위 밖 |
| Gemini HIGH: 토큰 파일 퍼미션 | 유효하지만 Unit 5 범위의 기존 코드 — 별도 이슈로 추적 |
| Codex: hook_config 서브스트링 매칭 취약 | 기존 코드 (Unit 5), 현재 Unit 11 범위가 아님 |
| Codex: 동기 디스크 I/O | check_hooks_config은 서버 시작 시 1회만 호출, 성능 영향 미미 |
