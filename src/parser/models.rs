use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

// ── Phase & Complexity ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Phase {
    #[default]
    Inception,
    Construction,
    Complete,
    Finished,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inception => write!(f, "INCEPTION"),
            Self::Construction => write!(f, "CONSTRUCTION"),
            Self::Complete => write!(f, "complete"),
            Self::Finished => write!(f, "finished"),
        }
    }
}

/// Error returned when parsing an unknown phase string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePhaseError(pub String);

impl fmt::Display for ParsePhaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown phase: '{}'", self.0)
    }
}

impl FromStr for Phase {
    type Err = ParsePhaseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "INCEPTION" => Ok(Self::Inception),
            "CONSTRUCTION" => Ok(Self::Construction),
            "COMPLETE" => Ok(Self::Complete),
            "FINISHED" => Ok(Self::Finished),
            _ => Err(ParsePhaseError(s.trim().to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Complexity {
    Minimal,
    #[default]
    Standard,
    Comprehensive,
}

impl fmt::Display for Complexity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minimal => write!(f, "Minimal"),
            Self::Standard => write!(f, "Standard"),
            Self::Comprehensive => write!(f, "Comprehensive"),
        }
    }
}

/// Error returned when parsing an unknown complexity string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseComplexityError(pub String);

impl fmt::Display for ParseComplexityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown complexity: '{}'", self.0)
    }
}

impl FromStr for Complexity {
    type Err = ParseComplexityError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "minimal" => Ok(Self::Minimal),
            "standard" => Ok(Self::Standard),
            "comprehensive" => Ok(Self::Comprehensive),
            _ => Err(ParseComplexityError(s.trim().to_string())),
        }
    }
}

// ── Stage Status ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StageStatus {
    Active,
    Completed,
    #[default]
    Waiting,
    Skipped,
    Deferred,
}

// ── Work Marker ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkMarker {
    Done,       // [x]
    InProgress, // [~]
    Pending,    // [ ]
}

impl WorkMarker {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Done => "✓",
            Self::InProgress => "◐",
            Self::Pending => "○",
        }
    }
}

// ── FlowState ──

#[derive(Debug, Clone, Default)]
pub struct CompletedStage {
    pub name: String,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ApprovedStage {
    pub name: String,
    pub depth: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SkippedStage {
    pub name: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DeferredStage {
    pub name: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct WorktreeInfo {
    pub branch: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct FlowState {
    pub phase: Phase,
    pub stage: String,
    pub complexity: Complexity,
    pub selected_approach: Option<String>,
    pub completed_stages: Vec<CompletedStage>,
    pub approved_stages: Vec<ApprovedStage>,
    pub skipped_stages: Vec<SkippedStage>,
    pub deferred_stages: Vec<DeferredStage>,
    pub active_unit: Option<String>,
    pub completed_units: Vec<String>,
    pub worktree: WorktreeInfo,
    pub extra_fields: HashMap<String, String>,
}

// ── SessionSummary ──

#[derive(Debug, Clone)]
pub struct WorkItem {
    pub text: String,
    pub marker: WorkMarker,
}

#[derive(Debug, Clone, Default)]
pub struct SessionSummary {
    pub last_updated: Option<String>,
    pub commit: Option<String>,
    pub phase: Option<Phase>,
    pub stage: Option<String>,
    pub complexity: Option<Complexity>,
    pub key_decisions: Vec<String>,
    pub completed_work: Vec<WorkItem>,
    pub next_steps: Vec<String>,
    pub for_next_session: Vec<String>,
}

// ── AuditEntry ──

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: Option<String>,
    pub stage: Option<String>,
    pub choice: Option<String>,
    pub raw_line: String,
}

// ── ArtifactFile ──

#[derive(Debug, Clone)]
pub struct ArtifactFile {
    pub path: PathBuf,
    pub name: String,
}

// ── Git Models ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitChangeStatus {
    Staged,
    Unstaged,
    Untracked,
    Conflict,
}

#[derive(Debug, Clone)]
pub struct GitChange {
    pub status: GitChangeStatus,
    pub path: String,
    pub additions: Option<u32>,
    pub deletions: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct GitWorktree {
    pub path: String,
    pub branch: Option<String>,
    pub is_current: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DiffStat {
    pub additions: u32,
    pub deletions: u32,
}

#[derive(Debug, Clone, Default)]
pub struct GitSnapshot {
    pub branch: String,
    pub head: String,
    pub changes: Vec<GitChange>,
    pub commits: Vec<GitCommit>,
    pub worktrees: Vec<GitWorktree>,
    pub diff_stat: DiffStat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_state_default() {
        let state = FlowState::default();
        assert_eq!(state.phase, Phase::Inception);
        assert!(state.stage.is_empty());
        assert_eq!(state.complexity, Complexity::Standard);
        assert!(state.completed_stages.is_empty());
        assert!(state.extra_fields.is_empty());
    }

    #[test]
    fn test_phase_display() {
        assert_eq!(Phase::Inception.to_string(), "INCEPTION");
        assert_eq!(Phase::Construction.to_string(), "CONSTRUCTION");
        assert_eq!(Phase::Complete.to_string(), "complete");
        assert_eq!(Phase::Finished.to_string(), "finished");
    }

    #[test]
    fn test_phase_from_str() {
        assert_eq!("INCEPTION".parse::<Phase>(), Ok(Phase::Inception));
        assert_eq!("construction".parse::<Phase>(), Ok(Phase::Construction));
        assert_eq!("Complete".parse::<Phase>(), Ok(Phase::Complete));
        assert!("unknown".parse::<Phase>().is_err());
    }

    #[test]
    fn test_complexity_display() {
        assert_eq!(Complexity::Minimal.to_string(), "Minimal");
        assert_eq!(Complexity::Standard.to_string(), "Standard");
        assert_eq!(Complexity::Comprehensive.to_string(), "Comprehensive");
    }

    #[test]
    fn test_complexity_from_str() {
        assert_eq!("minimal".parse::<Complexity>(), Ok(Complexity::Minimal));
        assert_eq!("Standard".parse::<Complexity>(), Ok(Complexity::Standard));
        assert_eq!("COMPREHENSIVE".parse::<Complexity>(), Ok(Complexity::Comprehensive));
        assert!("unknown".parse::<Complexity>().is_err());
    }

    #[test]
    fn test_git_snapshot_default() {
        let snap = GitSnapshot::default();
        assert!(snap.branch.is_empty());
        assert!(snap.changes.is_empty());
        assert_eq!(snap.diff_stat.additions, 0);
    }

    #[test]
    fn test_stage_status_deferred_variant_exists() {
        let status = StageStatus::Deferred;
        assert_eq!(status, StageStatus::Deferred);
        // Deferred should be distinct from Skipped
        assert_ne!(StageStatus::Deferred, StageStatus::Skipped);
    }

    #[test]
    fn test_deferred_stage_struct() {
        let ds = DeferredStage {
            name: "user-stories".to_string(),
            reason: Some("Deferred to wave 2".to_string()),
        };
        assert_eq!(ds.name, "user-stories");
        assert_eq!(ds.reason.as_deref(), Some("Deferred to wave 2"));
    }

    #[test]
    fn test_flow_state_has_deferred_stages() {
        let state = FlowState::default();
        assert!(state.deferred_stages.is_empty());
    }

    #[test]
    fn test_git_worktree_is_current_field() {
        let wt = GitWorktree {
            path: "/tmp/wt".to_string(),
            branch: Some("main".to_string()),
            is_current: true,
        };
        assert!(wt.is_current);

        let wt_default = GitWorktree {
            path: "/tmp/other".to_string(),
            branch: None,
            is_current: false,
        };
        assert!(!wt_default.is_current);
    }
}
