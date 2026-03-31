# Workspace Detection

## Type
Greenfield

## Project Name
devflow-tui (가칭)

## Technology Stack
| 항목 | 스택 |
|------|------|
| 언어 | Rust (edition 2024) |
| TUI 프레임워크 | ratatui 0.30 + crossterm 0.29 |
| Async 런타임 | tokio 1 (full) |
| 직렬화 | serde + serde_json + serde_yaml |
| 에러 처리 | thiserror 2 |
| 로깅 | tracing + tracing-subscriber + tracing-appender |
| 아키텍처 | TEA + Port/Adapter + Component trait |
| 테스트 | cargo test + clippy strict |

## Reference Project
- nexttui (/Users/jay.ahn/projects/infra/nexttui) — 동일 기술스택, 아키텍처 패턴 준수 대상

## Domain Context
| 영역 | 설명 |
|------|------|
| 워크플로우 시각화 | INCEPTION/CONSTRUCTION 단계별 진행 상태, 게이트 패턴, 산출물 모니터링 |
| 상태 파싱 | devflow-state.md, session-summary.md, audit.md 실시간 파싱 |
| Git 추적 | branch 상태, worktree 현황, commit 히스토리, diff 변화량 |
| Claude 진행 상태 | Claude Code 세션의 현재 작업 상태, 실행 중인 스킬, 게이트 대기 여부 |
| 에이전트 시각화 | subagent 스폰/완료 상태, parallel agent 동시 실행 현황, 각 에이전트별 태스크와 진행률 |

## Working Directory
/Users/jay.ahn/projects/backend
