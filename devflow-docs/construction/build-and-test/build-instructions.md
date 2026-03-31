# Build Instructions

## Prerequisites
- Rust 1.94+ (edition 2024)
- cargo (comes with Rust)

## Steps
1. `cargo build` — dev build
2. `cargo build --release` — optimized release build (LTO, single codegen unit, stripped)

## Expected Output
```
Finished `dev` profile [unoptimized + debuginfo]
```

Release build produces a single binary at `target/release/devflow-tui`.

## Clippy
```
cargo clippy --all-targets
```
Must pass with 0 errors. Project enforces `unwrap_used = "deny"` and `expect_used = "deny"`.
