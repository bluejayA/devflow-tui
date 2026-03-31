# Final Council Code Review Synthesis — CONSTRUCTION 완료 전 최종 리뷰

## Reviewers
- **Codex**: Code quality + Architecture + Security
- **Gemini**: Rate limited (429), 불참
- **Claude (의장)**: Security + Edge-case 독립 리뷰 + Synthesis

## Gate Decision: PASS (CONDITIONAL → 1건 수정 권장)

## Rationale
Critical 이슈 0건. High 이슈는 모두 기존 코드(Unit 5 범위) 또는 위협 모델 범위 밖(로컬 전용).
Phase 4+5에서 새로 작성된 코드(app.rs, event_loop.rs, gate_alert.rs, demo.rs)에 대한 신규 이슈는 actionable 1건뿐.

---

## Action Items

### 수정 권장 (1건)

| # | 이슈 | 소스 | 조치 |
|---|------|------|------|
| 1 | **클립보드 fallback 파일 퍼미션** — `/tmp/devflow-tui-clip-*.txt`가 0644로 생성되어 토큰 노출 가능 | Claude + Codex | `std::fs::set_permissions` 또는 `tempfile` 사용으로 0600 설정 |

### 기존 코드 이슈 (추적만, 이번 fix 범위 아님)

| # | 이슈 | 범위 | 비고 |
|---|------|------|------|
| H-legacy1 | 토큰 예측 가능성 (deterministic SHA-256) | Unit 5 | 로컬 전용이므로 Low risk. v1.1에서 CSPRNG 전환 검토 |
| H-legacy2 | --regenerate-token이 동일 값 재생성 | Unit 5 | 위와 동일 이슈. 토큰 방식 변경 시 함께 수정 |
| H-new1 | Adapter crash 시 UI 상태 미전환 | Phase 4 | hooks_active가 stale 유지 가능. 단 adapter 재시작 메커니즘이 없으므로 현재는 observability-only로 적절 |
| M-legacy1 | hook_config 서브스트링 매칭 false positive | Unit 5 | 기존 코드. JSON 파서 전환은 v1.1 검토 |

### 기각

| 이슈 | 사유 |
|------|------|
| App pub 필드 | demo.rs, test에서 직접 접근 필요. 현재 단일 바이너리 프로젝트에서 캡슐화 이점 낮음 |
| Gate detection permissive | sequential A→B→C 검증 추가 완료. 추가 제약은 실사용 후 데이터 기반 판단 |
| Event input error silent ignore | crossterm EventStream의 None/Err는 terminal detach 상황. quit 처리가 더 적절하나 edge case |
| validate_token length leak | SHA-256 고정 64 hex. 길이가 이미 공개 정보 |

## 전체 보안 점검 결과 (Claude)

Edge case 16개 항목 점검 완료:
- devflow-docs 미존재, 포트 고갈, 악의적 JSON, 대용량 body, adapter panic, terminal 복구, 채널 종료, resize race, Unicode, symlink, 프로세스 timeout 등 — 모두 적절하게 처리됨

**코드베이스 전반의 방어 코딩 수준이 높음**: unwrap 없음(테스트 제외), kill_on_drop + timeout 일관 적용, RAII 패턴.
