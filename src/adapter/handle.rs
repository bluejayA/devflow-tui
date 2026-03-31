use std::future::Future;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::error::Result;

/// RAII handle for a long-running background adapter task.
///
/// On drop, cancels the token and aborts the task if still running.
/// For graceful shutdown, call `shutdown()` explicitly before drop.
pub struct AdapterHandle {
    cancel: CancellationToken,
    join: Option<JoinHandle<Result<()>>>,
    name: &'static str,
}

impl Drop for AdapterHandle {
    fn drop(&mut self) {
        self.cancel.cancel();
        if let Some(join) = self.join.take()
            && !join.is_finished()
        {
            tracing::warn!("adapter '{}': dropped without shutdown, aborting task", self.name);
            join.abort();
        }
    }
}

impl AdapterHandle {
    /// Spawn a background adapter task.
    ///
    /// The closure receives a CancellationToken that it should poll for graceful shutdown.
    pub fn spawn<F, Fut>(name: &'static str, f: F) -> Self
    where
        F: FnOnce(CancellationToken) -> Fut,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let cancel = CancellationToken::new();
        let token = cancel.clone();
        let join = tokio::spawn(f(token));

        Self {
            cancel,
            join: Some(join),
            name,
        }
    }

    /// Graceful shutdown: cancel the token and wait up to 3 seconds.
    /// On timeout, explicitly aborts the task.
    pub async fn shutdown(mut self) {
        self.cancel.cancel();
        if let Some(join) = self.join.take() {
            let sleep = tokio::time::sleep(Duration::from_secs(3));
            tokio::pin!(sleep);
            tokio::pin!(join);

            tokio::select! {
                result = &mut join => {
                    match result {
                        Ok(Ok(())) => {
                            tracing::info!("adapter '{}' shut down cleanly", self.name);
                        }
                        Ok(Err(e)) => {
                            tracing::warn!("adapter '{}' exited with error: {e}", self.name);
                        }
                        Err(join_err) => {
                            tracing::error!("adapter '{}' panicked: {join_err}", self.name);
                        }
                    }
                }
                () = &mut sleep => {
                    tracing::warn!("adapter '{}' shutdown timed out, aborting", self.name);
                    join.abort();
                }
            }
        }
    }

    /// Check if the background task has finished (panic or completion).
    pub fn is_finished(&self) -> bool {
        self.join.as_ref().is_some_and(|j| j.is_finished())
    }

    /// Get the adapter name.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adapter_handle_spawn_and_shutdown() {
        let handle = AdapterHandle::spawn("test", |cancel| async move {
            cancel.cancelled().await;
            Ok(())
        });

        assert!(!handle.is_finished());
        handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_adapter_handle_immediate_completion() {
        let handle = AdapterHandle::spawn("test-fast", |_cancel| async move { Ok(()) });

        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(handle.is_finished());
        handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_adapter_handle_error_completion() {
        let handle = AdapterHandle::spawn("test-err", |_cancel| async move {
            Err(crate::error::AppError::Terminal("test error".to_string()))
        });

        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(handle.is_finished());
        handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_adapter_handle_drop_aborts() {
        let handle = AdapterHandle::spawn("test-drop", |cancel| async move {
            cancel.cancelled().await;
            Ok(())
        });

        // Drop without shutdown — should cancel + abort, not leak
        drop(handle);
        // If this test completes without hanging, Drop works correctly
    }
}
