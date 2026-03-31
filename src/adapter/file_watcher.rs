use std::path::{Path, PathBuf};
use std::time::Duration;

use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::{mpsc, watch};
use tokio_util::sync::CancellationToken;

use crate::error::Result;
use crate::event::AppEvent;
use crate::parser::models::{ArtifactFile, FlowState};
use crate::parser::{audit_log, devflow_state, session_summary};

const DEBOUNCE_MS: u64 = 300;

/// Run the file watcher adapter.
///
/// Watches devflow-docs/ for changes. On file change, re-parses and sends
/// updated state via watch/mpsc channels.
///
/// If devflow-docs/ doesn't exist, waits for it to be created.
pub async fn run(
    cancel: CancellationToken,
    project_dir: PathBuf,
    flow_state_tx: watch::Sender<FlowState>,
    event_tx: mpsc::Sender<AppEvent>,
) -> Result<()> {
    let devflow_dir = project_dir.join("devflow-docs");

    // Wait for devflow-docs/ to exist
    while !devflow_dir.exists() {
        tokio::select! {
            _ = cancel.cancelled() => return Ok(()),
            _ = tokio::time::sleep(Duration::from_secs(1)) => {}
        }
    }

    // Initial parse
    parse_and_send(&devflow_dir, &flow_state_tx, &event_tx).await;

    // Setup notify watcher
    let (notify_tx, mut notify_rx) = mpsc::channel::<PathBuf>(64);

    let dir_clone = devflow_dir.clone();
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(
        move |res: std::result::Result<notify::Event, notify::Error>| {
            if let Ok(event) = res
                && matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
                )
            {
                for path in event.paths {
                    if path.starts_with(&dir_clone)
                        && notify_tx.try_send(path).is_err()
                    {
                        tracing::warn!("file_watcher: event channel full, some events may be coalesced");
                    }
                }
            }
        },
    )?;

    watcher.watch(&devflow_dir, RecursiveMode::Recursive)?;

    // Debounce loop
    let mut debounce_timer: Option<tokio::time::Instant> = None;

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                tracing::info!("file_watcher: shutting down");
                return Ok(());
            }
            Some(_path) = notify_rx.recv() => {
                // Reset debounce timer on each event
                debounce_timer = Some(tokio::time::Instant::now() + Duration::from_millis(DEBOUNCE_MS));
            }
            _ = async {
                if let Some(deadline) = debounce_timer {
                    tokio::time::sleep_until(deadline).await;
                } else {
                    // No timer set — wait forever (cancelled by other branches)
                    std::future::pending::<()>().await;
                }
            } => {
                debounce_timer = None;
                parse_and_send(&devflow_dir, &flow_state_tx, &event_tx).await;
            }
        }
    }
}

async fn parse_and_send(
    devflow_dir: &Path,
    flow_state_tx: &watch::Sender<FlowState>,
    event_tx: &mpsc::Sender<AppEvent>,
) {
    // Parse devflow-state.md
    let state_path = devflow_dir.join("devflow-state.md");
    if state_path.exists() {
        match tokio::fs::read_to_string(&state_path).await {
            Ok(content) => {
                let state = devflow_state::parse(&content);
                let _ = flow_state_tx.send(state);
            }
            Err(e) => {
                tracing::warn!("file_watcher: failed to read {}: {e}", state_path.display());
                let _ = event_tx
                    .send(AppEvent::ParseError {
                        file: state_path.display().to_string(),
                        error: e.to_string(),
                    })
                    .await;
            }
        }
    }

    // Parse session-summary.md
    let summary_path = devflow_dir.join("session-summary.md");
    if summary_path.exists() {
        match tokio::fs::read_to_string(&summary_path).await {
            Ok(content) => {
                let summary = session_summary::parse(&content);
                let _ = event_tx
                    .send(AppEvent::SessionSummaryChanged(summary))
                    .await;
            }
            Err(e) => {
                tracing::warn!("file_watcher: failed to read {}: {e}", summary_path.display());
            }
        }
    }

    // Parse audit.md or devflow-audit.md
    let audit_path = find_audit_file(devflow_dir);
    if let Some(ref path) = audit_path {
        match tokio::fs::read_to_string(path).await {
            Ok(content) => {
                let entries = audit_log::parse(&content);
                let _ = event_tx.send(AppEvent::AuditLogAppended(entries)).await;
            }
            Err(e) => {
                tracing::warn!("file_watcher: failed to read {}: {e}", path.display());
            }
        }
    }

    // Scan artifact files (sorted for deterministic UI)
    let mut artifacts = scan_artifacts(devflow_dir).await;
    artifacts.sort_by(|a, b| a.path.cmp(&b.path));
    let _ = event_tx
        .send(AppEvent::ArtifactListChanged(artifacts))
        .await;
}

fn find_audit_file(devflow_dir: &Path) -> Option<PathBuf> {
    let candidates = ["audit.md", "devflow-audit.md"];
    for name in &candidates {
        let path = devflow_dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

async fn scan_artifacts(devflow_dir: &Path) -> Vec<ArtifactFile> {
    let mut artifacts = Vec::new();
    for subdir in &["inception", "construction"] {
        let dir = devflow_dir.join(subdir);
        if dir.exists()
            && let Ok(mut entries) = tokio::fs::read_dir(&dir).await
        {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "md") {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    artifacts.push(ArtifactFile { path, name });
                }
            }
        }
    }
    artifacts
}

// Note: Integration tests for file_watcher require tokio runtime + temp dirs.
// These are better placed in tests/ directory or as #[tokio::test] with tempfile crate.
// Unit-level tests cover the parsers (Unit 2). This module's correctness depends on
// notify crate behavior which is tested via manual/integration testing.
