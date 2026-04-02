use std::path::PathBuf;

use crate::parser::models::{ArtifactFile, AuditEntry, FlowState, GitSnapshot, SessionSummary, TokenSnapshot};

#[derive(Debug)]
pub enum AppEvent {
    // File watcher
    FlowStateChanged(FlowState),
    SessionSummaryChanged(SessionSummary),
    AuditLogAppended(Vec<AuditEntry>),
    ArtifactListChanged(Vec<ArtifactFile>),

    // Git polling
    GitStatusUpdated(GitSnapshot),

    // Hooks
    AgentStarted {
        agent_id: String,
        agent_type: String,
    },
    AgentStopped {
        agent_id: String,
    },
    ToolUseStarted {
        tool_name: String,
    },
    ToolUseCompleted {
        tool_name: String,
    },
    TurnCompleted {
        last_message: String,
    },

    // JSONL token watcher
    TokenUsageUpdated(TokenSnapshot),

    // System
    HooksServerStarted {
        port: u16,
    },
    HooksServerFailed {
        reason: String,
    },
    FileWatcherError {
        path: PathBuf,
        error: String,
    },
    GitPollError {
        error: String,
    },
    ParseError {
        file: String,
        error: String,
    },
    CommandCompleted {
        action_name: String,
    },
    CommandFailed {
        action_name: String,
        error: String,
    },
    AdapterCrashed {
        name: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_debug() {
        let event = AppEvent::AgentStarted {
            agent_id: "abc".to_string(),
            agent_type: "Explore".to_string(),
        };
        let debug = format!("{event:?}");
        assert!(debug.contains("AgentStarted"));
        assert!(debug.contains("abc"));
    }

    #[test]
    fn test_event_hooks_failed_debug() {
        let event = AppEvent::HooksServerFailed {
            reason: "port in use".to_string(),
        };
        let debug = format!("{event:?}");
        assert!(debug.contains("HooksServerFailed"));
    }
}
