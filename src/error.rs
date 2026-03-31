use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // Infrastructure
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Notify error: {0}")]
    Notify(#[from] notify::Error),

    #[error("Terminal error: {0}")]
    Terminal(String),

    // Parser
    #[error("Failed to parse devflow-state at {file}: {detail}")]
    ParseFlowState { file: PathBuf, detail: String },

    #[error("Failed to parse session-summary at {file}: {detail}")]
    ParseSessionSummary { file: PathBuf, detail: String },

    #[error("Failed to parse audit log at {file}: {detail}")]
    ParseAuditLog { file: PathBuf, detail: String },

    // Git
    #[error("Git command failed: {command} — {stderr}")]
    GitCommand { command: String, stderr: String },

    #[error("Git command timed out: {command}")]
    GitTimeout { command: String },

    // Hooks
    #[error("Failed to bind hooks server on port {port}: {reason}")]
    HooksServerBind { port: u16, reason: String },

    #[error("Hooks token mismatch")]
    HooksTokenMismatch,

    #[error("Invalid hooks payload: {detail}")]
    HooksPayloadInvalid { detail: String },

    // Service
    #[error("Clipboard unavailable")]
    ClipboardUnavailable,

    #[error("Token generation failed: {0}")]
    TokenGeneration(String),

    #[error("Failed to read config at {path}: {detail}")]
    ConfigRead { path: PathBuf, detail: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_io() {
        let err = AppError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "missing"));
        assert!(err.to_string().contains("I/O error"));
    }

    #[test]
    fn test_error_display_parse_flow_state() {
        let err = AppError::ParseFlowState {
            file: PathBuf::from("devflow-state.md"),
            detail: "unexpected format".to_string(),
        };
        assert!(err.to_string().contains("devflow-state"));
        assert!(err.to_string().contains("unexpected format"));
    }

    #[test]
    fn test_error_display_git_command() {
        let err = AppError::GitCommand {
            command: "git status".to_string(),
            stderr: "not a git repo".to_string(),
        };
        assert!(err.to_string().contains("git status"));
    }

    #[test]
    fn test_error_display_hooks_server_bind() {
        let err = AppError::HooksServerBind {
            port: 9100,
            reason: "address in use".to_string(),
        };
        assert!(err.to_string().contains("9100"));
    }

    #[test]
    fn test_error_display_clipboard() {
        let err = AppError::ClipboardUnavailable;
        assert_eq!(err.to_string(), "Clipboard unavailable");
    }

    #[test]
    fn test_error_display_config_read() {
        let err = AppError::ConfigRead {
            path: PathBuf::from("/etc/config"),
            detail: "permission denied".to_string(),
        };
        assert!(err.to_string().contains("/etc/config"));
    }
}
