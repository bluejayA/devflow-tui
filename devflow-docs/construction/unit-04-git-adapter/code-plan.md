# Code Plan: Unit 4 — Git 어댑터

## 개요
tokio::interval(2s) + git CLI subprocess로 Git 상태 폴링. GitSnapshot 모델로 파싱 → watch 채널 전송.

## 파일 목록

### Step 1: GitProvider 포트 + enum dispatch
- **파일**: `src/port/git.rs`
- **작업**: `GitProvider` enum (Cli, Mock), `snapshot()` 메서드
- **테스트**: Mock provider 기본 동작 테스트

### Step 2: Git CLI 출력 파서
- **파일**: `src/adapter/git_poller.rs` (파서 부분)
- **작업**:
  - `parse_status_porcelain_v2(&str) -> Vec<GitChange>` — staged/unstaged/untracked/conflict 파싱
  - `parse_log_oneline(&str) -> Vec<GitCommit>` — hash + message
  - `parse_worktree_porcelain(&str) -> Vec<GitWorktree>` — path + branch
  - `parse_diff_stat(&str) -> DiffStat` — additions/deletions 합산
- **테스트**: 각 git 출력 형식의 골든 테스트

### Step 3: Git 폴러 구현
- **파일**: `src/adapter/git_poller.rs` (폴러 부분)
- **작업**:
  - `run(cancel, project_dir, git_snapshot_tx)` async 함수
  - tokio::interval(2s) + tokio::process::Command
  - 동시 실행 방지 (이전 완료 전 스킵)
  - timeout 5초 (초과 시 스킵 + GitPollError)
  - 실패 시 이전 상태 유지
- **테스트**: Mock 환경에서 폴링 주기 + timeout 테스트

## Verification Contract
- [ ] git status/log/worktree/diff-stat 파서 골든 테스트 통과
- [ ] `cargo clippy -- -D warnings` 경고 0개
