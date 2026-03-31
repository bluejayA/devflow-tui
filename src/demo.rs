use crate::app::App;
use crate::event::AppEvent;
use crate::parser::models::{
    AuditEntry, CompletedStage, Complexity, DiffStat, FlowState, GitChange, GitChangeStatus,
    GitCommit, GitSnapshot, Phase, SessionSummary, WorkItem, WorkMarker,
};

/// Populate the app with sample data for --demo mode.
pub fn populate_demo_data(app: &mut App) {
    // FlowState
    let flow_state = FlowState {
        phase: Phase::Construction,
        stage: "code-generation".to_string(),
        complexity: Complexity::Standard,
        completed_stages: vec![
            CompletedStage {
                name: "workspace-detection".to_string(),
                timestamp: None,
            },
            CompletedStage {
                name: "requirements-analysis".to_string(),
                timestamp: None,
            },
            CompletedStage {
                name: "workflow-planning".to_string(),
                timestamp: None,
            },
        ],
        active_unit: Some("Unit 11".to_string()),
        ..Default::default()
    };
    app.handle_flow_state(flow_state);

    // SessionSummary
    let summary = SessionSummary {
        completed_work: vec![
            WorkItem {
                text: "Phase 1: Foundation + Parser".to_string(),
                marker: WorkMarker::Done,
            },
            WorkItem {
                text: "Phase 2: Data Adapters".to_string(),
                marker: WorkMarker::Done,
            },
            WorkItem {
                text: "Phase 3: UI Panels".to_string(),
                marker: WorkMarker::InProgress,
            },
        ],
        key_decisions: vec![
            "Approach B: layered unit decomposition".to_string(),
            "ratatui 0.30 + crossterm 0.29".to_string(),
        ],
        next_steps: vec!["Phase 4: App Integration".to_string()],
        ..Default::default()
    };
    app.workflow_map.set_session_summary(summary);

    // GitSnapshot
    let git_snapshot = GitSnapshot {
        branch: "feature/unit-11".to_string(),
        head: "abc1234".to_string(),
        diff_stat: DiffStat {
            additions: 420,
            deletions: 35,
        },
        changes: vec![
            GitChange {
                status: GitChangeStatus::Staged,
                path: "src/app.rs".to_string(),
                additions: Some(200),
                deletions: Some(0),
            },
            GitChange {
                status: GitChangeStatus::Staged,
                path: "src/event_loop.rs".to_string(),
                additions: Some(80),
                deletions: Some(0),
            },
            GitChange {
                status: GitChangeStatus::Unstaged,
                path: "src/main.rs".to_string(),
                additions: Some(50),
                deletions: Some(20),
            },
            GitChange {
                status: GitChangeStatus::Untracked,
                path: "src/demo.rs".to_string(),
                additions: None,
                deletions: None,
            },
            GitChange {
                status: GitChangeStatus::Unstaged,
                path: "Cargo.toml".to_string(),
                additions: Some(5),
                deletions: Some(2),
            },
        ],
        commits: vec![
            GitCommit {
                hash: "abc1234".to_string(),
                message: "feat: add App struct and event loop".to_string(),
            },
            GitCommit {
                hash: "def5678".to_string(),
                message: "feat: add TestBackend render tests".to_string(),
            },
            GitCommit {
                hash: "ghi9012".to_string(),
                message: "refactor: render(&mut self) migration".to_string(),
            },
        ],
        worktrees: Vec::new(),
    };
    app.handle_git_snapshot(git_snapshot);

    // AuditLog entries
    let audit_entries: Vec<AuditEntry> = (0..10)
        .map(|i| AuditEntry {
            timestamp: Some(format!("2026-03-31T14:{i:02}:00")),
            stage: Some(
                [
                    "workspace-detection",
                    "requirements-analysis",
                    "user-stories",
                    "workflow-planning",
                    "application-design",
                    "units-generation",
                    "code-generation",
                    "build-and-test",
                    "code-generation",
                    "build-and-test",
                ][i]
                .to_string(),
            ),
            choice: Some(if i % 3 == 0 {
                "B (Approve)".to_string()
            } else {
                "B".to_string()
            }),
            raw_line: String::new(),
        })
        .collect();
    app.audit_log.set_entries(audit_entries);

    // Agents — simulate hooks active + agents
    app.hooks_active = true;
    app.hooks_port = Some(9100);
    app.agent_status.set_hooks_active(true);

    // Running agent
    app.handle_event(AppEvent::AgentStarted {
        agent_id: "explore-1".to_string(),
        agent_type: "Explore".to_string(),
    });

    // Done agent (start then stop)
    app.handle_event(AppEvent::AgentStarted {
        agent_id: "plan-1".to_string(),
        agent_type: "Plan".to_string(),
    });
    app.handle_event(AppEvent::AgentStopped {
        agent_id: "plan-1".to_string(),
    });

    // Timeout agent (manually create old entry)
    app.handle_event(AppEvent::AgentStarted {
        agent_id: "old-agent".to_string(),
        agent_type: "code-review".to_string(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tokio::sync::mpsc;

    #[test]
    fn test_populate_demo_no_panic() {
        let (tx, _rx) = mpsc::channel(16);
        let mut app = App::new(120, 30, tx, "token".to_string(), PathBuf::from("/tmp"));
        populate_demo_data(&mut app);
        assert_eq!(app.phase, "CONSTRUCTION");
        assert!(app.hooks_active);
        assert!(app.agent_status.has_running_agents());
    }
}
