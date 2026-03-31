# Code Plan: Unit 12 — GateAlert 패널 + Wide 레이아웃 통합

## 개요
게이트 대기 감지 + 알림 패널을 구현하고, Wide 레이아웃에서 직접 표시한다.
ArtifactPreviewPanel은 v1.1로 이관 — 현재 placeholder Block 유지.

## 범위 (축소)
- GateAlertPanel 구현 (FR-6)
- App에 GateAlert FocusPane 추가 (Wide 모드)
- Wide 레이아웃에서 GateAlert 패널 렌더링
- ArtifactPreview는 이관 (placeholder 유지)

## 파일 목록

### Step 1: GateAlertPanel 기본 구조
- **파일**: `src/panel/gate_alert.rs` (신규)
- **작업**:
  - `GateAlert` struct:
    - `active: bool` — 게이트 대기 중 여부
    - `gate_text: String` — 감지된 게이트 메시지
    - `choices: Vec<String>` — 파싱된 선택지 (A, B, C...)
    - `hooks_active: bool` — hooks 설정 여부
    - `next_steps: Vec<String>` — hooks 미설정 시 대체 표시용
  - `GateAlertPanel::new()` 생성자
  - `GateAlertPanel::set_hooks_active(bool)`
  - `GateAlertPanel::set_next_steps(Vec<String>)`
  - `detect_gate(message: &str) -> Option<(String, Vec<String>)>`:
    - `A)`, `B)`, `C)` 패턴 매칭 (정규식 없이 간단한 라인 스캔)
    - 선택지 텍스트 추출
  - `clear_gate(&mut self)`: 게이트 알림 해제
- **테스트**:
  - `test_detect_gate_ab_pattern`: "A) 옵션1\nB) 옵션2" 감지
  - `test_detect_gate_abc_pattern`: 3개 선택지 감지
  - `test_detect_gate_no_pattern`: 일반 메시지 → None
  - `test_detect_gate_inline`: "A) ... B) ..." 한 줄 패턴

### Step 2: GateAlertPanel 이벤트 처리
- **파일**: `src/panel/gate_alert.rs` (Step 1에 추가)
- **작업**:
  - `Component::handle_event` 구현:
    - `TurnCompleted { last_message }` → `detect_gate(last_message)`로 게이트 감지
    - `ToolUseCompleted { tool_name: "AskUserQuestion" }` → 게이트 활성화 (generic)
    - `FlowStateChanged` → stage 변경 시 `clear_gate()` (게이트 종료 조건)
    - `HooksServerStarted` → hooks_active = true
    - `HooksServerFailed` → hooks_active = false
    - `SessionSummaryChanged` → next_steps 업데이트
  - `Component::handle_key`: Esc → clear_gate (수동 해제)
- **테스트**:
  - `test_turn_completed_activates_gate`: TurnCompleted 이벤트 → active=true
  - `test_stage_change_clears_gate`: FlowStateChanged → active=false
  - `test_esc_clears_gate`: Esc 키 → active=false
  - `test_ask_user_question_activates`: AskUserQuestion 도구 → active

### Step 3: GateAlertPanel 렌더링
- **파일**: `src/panel/gate_alert.rs` (Step 2에 추가)
- **작업**:
  - `Component::render` 구현:
    - hooks 미설정 시: "게이트 감지를 위해 hooks 설정이 필요합니다" + next_steps 표시
    - 게이트 비활성: "대기 중..." 또는 마지막 게이트 정보 (dim)
    - 게이트 활성: Yellow+Bold `▶ GATE` 마커 + 선택지 목록
    - 패널 테두리: focused Cyan / unfocused DarkGray (기존 패턴)
- **테스트** (TestBackend):
  - `render_hooks_inactive`: hooks 미설정 메시지
  - `render_no_gate`: "대기 중..." 표시
  - `render_active_gate`: "▶" 마커 + 선택지 표시
  - `render_next_steps_fallback`: hooks 미설정 + next_steps 표시

### Step 4: App 통합 — Wide 레이아웃 + FocusPane
- **파일**: `src/app.rs` (수정), `src/panel/mod.rs` (수정)
- **작업**:
  - `FocusPane::GateAlert` variant 추가
  - `App` struct에 `gate_alert: GateAlertPanel` 필드 추가
  - `App::available_panels()`: Wide 모드에서 GateAlert 포함
  - `App::handle_event()`: gate_alert에도 이벤트 브로드캐스트
  - `App::on_tick()`: gate_alert 관련 tick 처리 (필요 시)
  - `App::render()`: Wide 모드에서 placeholder 대신 gate_alert.render() 호출
  - `App::focus_name()`: "Gate Alert" 추가
  - `App::focused_panel_mut()`: GateAlert 분기 추가
- **테스트**:
  - `test_available_panels_wide_includes_gate`: Wide 모드에서 GateAlert 포함
  - `test_gate_alert_event_broadcast`: TurnCompleted가 gate_alert에 전달
  - `render_wide_mode_gate_alert`: Wide 모드에서 Gate Alert 패널 제목 확인

### Step 5: panel/mod.rs 업데이트
- **파일**: `src/panel/mod.rs`
- **작업**: `pub mod gate_alert;` 추가

## 게이트 감지 로직

```
Stop hook → TurnCompleted { last_message }
  → detect_gate(last_message)
    → 라인 스캔: "A)" 또는 "B)" 패턴 존재?
    → 존재: choices 추출, gate_text 저장, active=true
    → 미존재: 무시

PostToolUse → ToolUseCompleted { tool_name }
  → tool_name == "AskUserQuestion"?
    → active=true (generic gate, choices 없음)

게이트 종료:
  - FlowStateChanged (stage 변경) → clear
  - 다음 TurnCompleted (새 게이트 또는 일반 메시지) → 업데이트/clear
  - Esc 키 → 수동 clear
```

## 검증 방법

```bash
cargo test --lib              # 전체 테스트
cargo clippy --all-targets    # clippy
cargo run -- --demo           # Wide 터미널에서 Gate Alert 패널 확인
```
