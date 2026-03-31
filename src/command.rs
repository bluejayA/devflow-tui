use tokio::sync::mpsc;

use crate::action::Action;
use crate::event::AppEvent;

/// Lightweight async command runner for side-effect actions.
///
/// Handles: clipboard copy, refresh triggers.
/// Sends completion/failure events back via channel.
#[derive(Clone)]
pub struct CommandRunner {
    event_tx: mpsc::Sender<AppEvent>,
}

impl CommandRunner {
    pub fn new(event_tx: mpsc::Sender<AppEvent>) -> Self {
        Self { event_tx }
    }

    /// Execute an async action. Spawns a tokio task.
    pub fn execute(&self, action: Action) {
        let tx = self.event_tx.clone();
        let action_name = action.name().to_string();

        tokio::spawn(async move {
            let result = match action {
                Action::CopyToClipboard(text) => copy_to_clipboard(&text).await,
                Action::Refresh => {
                    // Refresh is handled by re-triggering file watcher parse
                    // For now, just succeed
                    Ok(())
                }
                _ => return, // Non-async actions don't come here
            };

            match result {
                Ok(()) => {
                    let _ = tx
                        .send(AppEvent::CommandCompleted {
                            action_name,
                        })
                        .await;
                }
                Err(e) => {
                    let _ = tx
                        .send(AppEvent::CommandFailed {
                            action_name,
                            error: e,
                        })
                        .await;
                }
            }
        });
    }
}

/// Copy text to clipboard using pbcopy (macOS).
/// Falls back to writing to /tmp/devflow-tui-clipboard.txt.
/// Clipboard timeout
const CLIPBOARD_TIMEOUT_SECS: u64 = 5;

async fn copy_to_clipboard(text: &str) -> std::result::Result<(), String> {
    // Try pbcopy first (macOS) with timeout
    let result = tokio::process::Command::new("pbcopy")
        .stdin(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn();

    match result {
        Ok(mut child) => {
            if let Some(ref mut stdin) = child.stdin {
                use tokio::io::AsyncWriteExt;
                if let Err(e) = stdin.write_all(text.as_bytes()).await {
                    return Err(format!("pbcopy write failed: {e}"));
                }
            }
            drop(child.stdin.take());

            // Wait with timeout to avoid hanging on stuck pbcopy
            match tokio::time::timeout(
                std::time::Duration::from_secs(CLIPBOARD_TIMEOUT_SECS),
                child.wait(),
            )
            .await
            {
                Ok(Ok(status)) if status.success() => Ok(()),
                Ok(Ok(status)) => Err(format!("pbcopy exited with: {status}")),
                Ok(Err(e)) => Err(format!("pbcopy wait failed: {e}")),
                Err(_) => Err("pbcopy timed out".to_string()),
            }
        }
        Err(_) => {
            // Fallback: write to temp file with restrictive permissions (0600)
            let dir = std::env::temp_dir();
            let file_path = dir.join(format!(
                "devflow-tui-clip-{}.txt",
                std::process::id()
            ));
            tokio::fs::write(&file_path, text)
                .await
                .map_err(|e| format!("clipboard fallback failed: {e}"))?;

            // Set restrictive permissions — file may contain auth token
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let perms = std::fs::Permissions::from_mode(0o600);
                std::fs::set_permissions(&file_path, perms)
                    .map_err(|e| format!("failed to set file permissions: {e}"))?;
            }

            tracing::info!(
                "clipboard unavailable, saved to {}",
                file_path.display()
            );
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_command_runner_non_async_ignored() {
        let (tx, mut rx) = mpsc::channel(16);
        let runner = CommandRunner::new(tx);

        // Non-async action should be ignored (no event sent)
        runner.execute(Action::ScrollUp);
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn test_command_runner_refresh() {
        let (tx, mut rx) = mpsc::channel(16);
        let runner = CommandRunner::new(tx);

        runner.execute(Action::Refresh);
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let event = rx.try_recv().unwrap();
        match event {
            AppEvent::CommandCompleted { action_name } => {
                assert_eq!(action_name, "refresh");
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }
}
