# Workflow Plan

## Selected Approach
**B) 계층별 유닛 분해**

인프라 계층 → 데이터 계층 → UI 계층으로 분해. 각 계층 내에서 독립 유닛으로 구현.

### 선택 근거
- Unit 1~3 완료 후 즉시 워크플로우 맵 동작 확인 가능 (빠른 피드백)
- 각 유닛이 독립 테스트 가능 (TDD Iron Law 준수에 자연스러움)
- Port/Adapter 패턴과 정렬 (어댑터별 유닛 분리)

## Approaches Considered

### A) 단일 유닛 순차 구현
- 장점: 단순, 통합 이슈 적음
- 단점: 초기 피드백 느림
- **미선택**: 동작하는 TUI를 빨리 확인하고 싶음

### B) 계층별 유닛 분해 ✅
- 장점: 점진적 피드백, 독립 테스트, TDD 친화
- 단점: 유닛 간 인터페이스 사전 설계 필요
- **선택됨**

### C) 기능 슬라이스 (수직 분할)
- 장점: 기능 단위 완결성
- 단점: 이벤트 버스/앱 구조 불안정, 리팩토링 반복
- **미선택**: 인프라가 없는 greenfield에서 수직 슬라이스는 리스크

## Unit Breakdown (예정)

```
Unit 1:  프로젝트 기반 + 이벤트 버스
Unit 2:  파일 파서
Unit 3:  파일 감시 어댑터
Unit 4:  Git 어댑터
Unit 5:  Hooks HTTP 서버
Unit 6:  워크플로우 맵 패널
Unit 7:  Git 상태 패널
Unit 8:  에이전트 상태 패널
Unit 9:  감사 로그 패널
Unit 10: 설정/네비게이션/레이아웃
Unit 11: [P1] 산출물 미리보기 + 게이트 알림
```

상세 유닛 분해는 units-generation 스테이지에서 수행.

## Approved Stages

| 스테이지 | 깊이 | 포함 여부 |
|---------|------|----------|
| application-design | Standard | ✅ 포함 — 구조 설계 + UI 디자인 통합 |
| units-generation | Standard | ✅ 포함 — 11개 유닛 의존성 및 순서 정의 |
| code-generation | Standard | ✅ 포함 — TDD 기반 구현 |
| build-and-test | Standard | ✅ 포함 — 전체 빌드/테스트 검증 |

## Stage Depths

| 스테이지 | 깊이 | 근거 |
|---------|------|------|
| application-design | Standard | **구조 설계**: trait 정의 + 모듈 구조 + 이벤트 버스. **UI 디자인**: 레이아웃 와이어프레임(표준/와이드/축소), 패널별 위젯 배치, 상태바 디자인, 컬러 팔레트 |
| units-generation | Standard | 유닛 목록 + 의존성 + 구현 순서 |
| code-generation | Standard | Plan(파일/메서드/테스트 목록) → Generate(TDD RED-GREEN-REFACTOR) |
| build-and-test | Standard | cargo build + cargo test + cargo clippy |

## Environment Setup

### 프로젝트 디렉토리
- `backend/devflow-tui/` — 새 Cargo 프로젝트 생성
- `cargo init --name devflow-tui`

### Git
- backend/ 에 git이 없으므로 `devflow-tui/` 내부에서 `git init`
- 브랜치: `main` (신규 프로젝트이므로 worktree 불필요)

### 초기 설정
- `.gitignore`: target/, .env*, *.pem, devflow-docs/.tui-token
- `Cargo.toml`: 기본 의존성 설정
- `clippy.toml`: unwrap_used, expect_used deny
