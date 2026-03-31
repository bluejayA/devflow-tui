use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Json,
    extract::{DefaultBodyLimit, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::error::Result;
use crate::event::AppEvent;
use crate::service::sanitizer::strip_ansi;

const PORT_RANGE_START: u16 = 9100;
const PORT_RANGE_END: u16 = 9110;

/// Shared state for the hooks HTTP server.
struct HooksState {
    token: String,
    event_tx: mpsc::Sender<AppEvent>,
}

/// Run the hooks HTTP server.
///
/// Tries to bind to the given port, falling back to 9100-9110 range.
/// Returns the actual bound port via AppEvent::HooksServerStarted.
pub async fn run(
    cancel: CancellationToken,
    preferred_port: u16,
    token: String,
    event_tx: mpsc::Sender<AppEvent>,
) -> Result<()> {
    let state = Arc::new(HooksState {
        token,
        event_tx: event_tx.clone(),
    });

    let app = axum::Router::new()
        .route("/hook", post(handle_hook))
        .layer(DefaultBodyLimit::max(64 * 1024)) // 64KB max request body
        .with_state(state);

    // Try preferred port first, then range
    let mut ports_to_try = Vec::with_capacity(12);
    ports_to_try.push(preferred_port);
    for p in PORT_RANGE_START..=PORT_RANGE_END {
        if p != preferred_port {
            ports_to_try.push(p);
        }
    }

    for port in ports_to_try {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                tracing::info!("hooks_server: listening on {addr}");
                let _ = event_tx
                    .send(AppEvent::HooksServerStarted { port })
                    .await;

                // Run server with graceful shutdown
                let server = axum::serve(listener, app);
                tokio::select! {
                    _ = cancel.cancelled() => {
                        tracing::info!("hooks_server: shutting down");
                    }
                    result = server => {
                        if let Err(e) = result {
                            tracing::error!("hooks_server error: {e}");
                        }
                    }
                }

                return Ok(());
            }
            Err(e) => {
                tracing::debug!("hooks_server: port {port} unavailable: {e}");
            }
        }
    }

    // All ports failed
    let reason = format!("all ports {PORT_RANGE_START}-{PORT_RANGE_END} unavailable");
    tracing::warn!("hooks_server: {reason}");
    let _ = event_tx
        .send(AppEvent::HooksServerFailed { reason })
        .await;

    Ok(())
}

/// Hook payload from Claude Code.
#[derive(Debug, serde::Deserialize)]
struct HookPayload {
    hook_event_name: Option<String>,
    agent_id: Option<String>,
    agent_type: Option<String>,
    tool_name: Option<String>,
    last_assistant_message: Option<String>,
}

async fn handle_hook(
    axum::extract::State(state): axum::extract::State<Arc<HooksState>>,
    Query(params): Query<HashMap<String, String>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    // Token validation
    let provided_token = params.get("token").map(|s| s.as_str()).unwrap_or("");
    if !crate::service::token::validate_token(&state.token, provided_token) {
        return StatusCode::FORBIDDEN;
    }

    // Parse payload
    let hook: HookPayload = match serde_json::from_value(payload) {
        Ok(h) => h,
        Err(e) => {
            tracing::warn!("hooks_server: invalid payload: {e}");
            return StatusCode::BAD_REQUEST;
        }
    };

    // Convert to AppEvent
    let event_name = hook.hook_event_name.as_deref().unwrap_or("");
    let event = match event_name {
        "SubagentStart" => hook.agent_id.map(|id| AppEvent::AgentStarted {
            agent_id: strip_ansi(&id),
            agent_type: hook.agent_type.map(|t| strip_ansi(&t)).unwrap_or_default(),
        }),
        "SubagentStop" => hook.agent_id.map(|id| AppEvent::AgentStopped {
            agent_id: strip_ansi(&id),
        }),
        "PreToolUse" => hook.tool_name.map(|name| AppEvent::ToolUseStarted {
            tool_name: strip_ansi(&name),
        }),
        "PostToolUse" => hook.tool_name.map(|name| AppEvent::ToolUseCompleted {
            tool_name: strip_ansi(&name),
        }),
        "Stop" => Some(AppEvent::TurnCompleted {
            last_message: hook
                .last_assistant_message
                .map(|m| strip_ansi(&m))
                .unwrap_or_default(),
        }),
        _ => {
            tracing::debug!("hooks_server: unknown event: {event_name}");
            None
        }
    };

    if let Some(ev) = event {
        let _ = state.event_tx.send(ev).await;
    }

    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    fn test_app(token: &str) -> (axum::Router, mpsc::Receiver<AppEvent>) {
        let (tx, rx) = mpsc::channel(16);
        let state = Arc::new(HooksState {
            token: token.to_string(),
            event_tx: tx,
        });
        let app = axum::Router::new()
            .route("/hook", post(handle_hook))
            .with_state(state);
        (app, rx)
    }

    #[tokio::test]
    async fn test_hook_valid_token_agent_started() {
        let (app, mut rx) = test_app("secret123");

        let body = serde_json::json!({
            "hook_event_name": "SubagentStart",
            "agent_id": "agent-abc",
            "agent_type": "Explore"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/hook?token=secret123")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let event = rx.try_recv().unwrap();
        match event {
            AppEvent::AgentStarted { agent_id, agent_type } => {
                assert_eq!(agent_id, "agent-abc");
                assert_eq!(agent_type, "Explore");
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_hook_invalid_token_403() {
        let (app, _rx) = test_app("secret123");

        let body = serde_json::json!({"hook_event_name": "Stop"});

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/hook?token=wrongtoken")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_hook_missing_token_403() {
        let (app, _rx) = test_app("secret123");

        let body = serde_json::json!({"hook_event_name": "Stop"});

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/hook")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_hook_stop_event() {
        let (app, mut rx) = test_app("tok");

        let body = serde_json::json!({
            "hook_event_name": "Stop",
            "last_assistant_message": "A) option\nB) option"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/hook?token=tok")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let event = rx.try_recv().unwrap();
        match event {
            AppEvent::TurnCompleted { last_message } => {
                assert!(last_message.contains("A) option"));
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_hook_ansi_sanitized() {
        let (app, mut rx) = test_app("tok");

        let body = serde_json::json!({
            "hook_event_name": "SubagentStart",
            "agent_id": "\x1b[31magent-x\x1b[0m",
            "agent_type": "Plan"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/hook?token=tok")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let event = rx.try_recv().unwrap();
        match event {
            AppEvent::AgentStarted { agent_id, .. } => {
                assert_eq!(agent_id, "agent-x"); // ANSI stripped
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }
}
