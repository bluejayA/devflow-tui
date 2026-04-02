# Test Instructions

## Unit Tests
Run: `cargo test`
Expected: 315 tests pass, 0 failures

## Test Breakdown by Module
| Module | Tests | Coverage |
|--------|-------|----------|
| parser (devflow_state, session_summary, audit_log, models) | 22 | Golden data, edge cases |
| parser/jsonl_token | 18 | JSONL parse, accumulate, burn rate, timestamps |
| service (sanitizer, token, hook_config) | 16 | ANSI strip, token gen, config detect |
| adapter (handle, git_poller, hooks_server) | 22 | Spawn/shutdown, git parsing, HTTP |
| adapter/jsonl_watcher | 7 | File discovery, depth limit, incremental read |
| config | 9 | CLI args, env vars, validation |
| error, event, action | 10 | Display, debug, naming |
| ui (theme, layout, header, status_bar, help_overlay) | 30 | Render tests with TestBackend |
| panel (workflow_map, git_status, agent_status, audit_log) | 52 | State + render tests |
| panel/gate_alert | 21 | Pattern detect, events, render |
| panel/token_usage | 10 | Render, format, events, scroll |
| app | 36 | Key handling, events, hooks config, render modes |
| demo | 1 | No-panic data population |
| command | 2 | Async execution |

## Coverage Report
Run: `cargo llvm-cov --lib --html`
Output: `target/llvm-cov/html/index.html`

## Manual Verification
1. `cargo run -- --demo` — verify TUI renders correctly in terminal
2. Resize terminal to test layout modes:
   - < 80x24: "too small" message
   - 80x24: Compact (single panel)
   - 120x30: Standard (5 panels: Workflow, Git, Agent, Audit, Token Usage)
   - 200x50: Wide (7 panels + Artifacts + Gate Alert + Token Usage)
3. Press Tab to cycle panels, `?` for help overlay, `f` to expand, `q` to quit
4. Token Usage panel should show "JSONL 대기 중..." in demo mode (no live JSONL)
