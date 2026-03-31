use std::path::PathBuf;
use std::time::Duration;

use tokio::process::Command;
use tokio::sync::{mpsc, watch};
use tokio_util::sync::CancellationToken;

use crate::error::Result;
use crate::event::AppEvent;
use crate::parser::models::{DiffStat, GitChange, GitChangeStatus, GitCommit, GitSnapshot, GitWorktree};
use crate::service::sanitizer::strip_ansi;

const POLL_INTERVAL_SECS: u64 = 2;
const GIT_TIMEOUT_SECS: u64 = 5;

/// Run the git poller adapter.
///
/// Polls git status every 2 seconds via CLI subprocess.
/// Sends GitSnapshot updates via watch channel.
pub async fn run(
    cancel: CancellationToken,
    project_dir: PathBuf,
    git_snapshot_tx: watch::Sender<GitSnapshot>,
    event_tx: mpsc::Sender<AppEvent>,
) -> Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(POLL_INTERVAL_SECS));

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                tracing::info!("git_poller: shutting down");
                return Ok(());
            }
            _ = interval.tick() => {
                match poll_git(&project_dir).await {
                    Ok(snapshot) => {
                        let _ = git_snapshot_tx.send(snapshot);
                    }
                    Err(e) => {
                        tracing::warn!("git_poller: {e}");
                        let _ = event_tx.send(AppEvent::GitPollError {
                            error: e.to_string(),
                        }).await;
                    }
                }
            }
        }
    }
}

async fn poll_git(dir: &PathBuf) -> Result<GitSnapshot> {
    // Run essential commands, propagate first error for branch/head
    let branch = git_cmd(dir, &["rev-parse", "--abbrev-ref", "HEAD"]).await;
    let head = git_cmd(dir, &["rev-parse", "--short", "HEAD"]).await;

    // If we can't even get branch/head, this isn't a git repo — propagate error
    let branch = branch.map_err(|e| crate::error::AppError::GitCommand {
        command: "git rev-parse --abbrev-ref HEAD".to_string(),
        stderr: e,
    })?;
    let head = head.map_err(|e| crate::error::AppError::GitCommand {
        command: "git rev-parse --short HEAD".to_string(),
        stderr: e,
    })?;

    // Non-critical commands: log warnings on failure, use defaults
    let status_out = git_cmd(dir, &["status", "--porcelain=v2"]).await.unwrap_or_else(|e| {
        tracing::warn!("git status failed: {e}");
        String::new()
    });
    let log_out = git_cmd(dir, &["log", "--oneline", "-n", "10"]).await.unwrap_or_else(|e| {
        tracing::warn!("git log failed: {e}");
        String::new()
    });
    let worktree_out = git_cmd(dir, &["worktree", "list", "--porcelain"]).await.unwrap_or_else(|e| {
        tracing::warn!("git worktree list failed: {e}");
        String::new()
    });
    let diff_out = git_cmd(dir, &["diff", "--stat"]).await.unwrap_or_else(|e| {
        tracing::warn!("git diff --stat failed: {e}");
        String::new()
    });

    Ok(GitSnapshot {
        branch: strip_ansi(branch.trim()),
        head: strip_ansi(head.trim()),
        changes: parse_status_porcelain_v2(&status_out),
        commits: parse_log_oneline(&log_out),
        worktrees: parse_worktree_porcelain(&worktree_out),
        diff_stat: parse_diff_stat(&diff_out),
    })
}

async fn git_cmd(dir: &PathBuf, args: &[&str]) -> std::result::Result<String, String> {
    let child = Command::new("git")
        .args(args)
        .current_dir(dir)
        .kill_on_drop(true) // Prevent orphaned git processes
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("git spawn error: {e}"))?;

    let result = tokio::time::timeout(
        Duration::from_secs(GIT_TIMEOUT_SECS),
        child.wait_with_output(),
    )
    .await;

    match result {
        Ok(Ok(output)) => {
            if output.status.success() {
                Ok(strip_ansi(&String::from_utf8_lossy(&output.stdout)))
            } else {
                Err(strip_ansi(String::from_utf8_lossy(&output.stderr).trim()))
            }
        }
        Ok(Err(e)) => Err(format!("git exec error: {e}")),
        Err(_) => {
            // Timeout — child is killed on drop
            Err(format!("git command timed out: git {}", args.join(" ")))
        }
    }
}

// ── Parsers ──

/// Parse `git status --porcelain=v2` output.
pub fn parse_status_porcelain_v2(output: &str) -> Vec<GitChange> {
    let mut changes = Vec::new();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_prefix("1 ") {
            // Changed entry after "1 ": XY sub mH mI mW hH hI path
            let parts: Vec<&str> = rest.splitn(8, ' ').collect();
            if parts.len() >= 8 {
                let xy = parts[0];
                let path = parts[7].to_string();
                let x = xy.as_bytes().first().copied().unwrap_or(b'.');
                let y = xy.as_bytes().get(1).copied().unwrap_or(b'.');

                if x != b'.' && x != b'?' {
                    changes.push(GitChange {
                        status: GitChangeStatus::Staged,
                        path: path.clone(),
                        additions: None,
                        deletions: None,
                    });
                }
                if y != b'.' && y != b'?' {
                    changes.push(GitChange {
                        status: GitChangeStatus::Unstaged,
                        path,
                        additions: None,
                        deletions: None,
                    });
                }
            }
        } else if let Some(rest) = line.strip_prefix("2 ") {
            // Renamed/copied after "2 ": XY sub mH mI mW hH hI Xscore path\torigPath
            let parts: Vec<&str> = rest.splitn(9, ' ').collect();
            if parts.len() >= 9 {
                let path = parts[8].split('\t').next().unwrap_or("").to_string();
                changes.push(GitChange {
                    status: GitChangeStatus::Staged,
                    path,
                    additions: None,
                    deletions: None,
                });
            }
        } else if let Some(rest) = line.strip_prefix("u ") {
            // Unmerged after "u ": XY sub m1 m2 m3 mW h1 h2 h3 path
            let parts: Vec<&str> = rest.splitn(10, ' ').collect();
            if parts.len() >= 10 {
                changes.push(GitChange {
                    status: GitChangeStatus::Conflict,
                    path: parts[9].to_string(),
                    additions: None,
                    deletions: None,
                });
            }
        } else if let Some(rest) = line.strip_prefix("? ") {
            // Untracked
            changes.push(GitChange {
                status: GitChangeStatus::Untracked,
                path: rest.to_string(),
                additions: None,
                deletions: None,
            });
        }
    }

    changes
}

/// Parse `git log --oneline -n 10` output.
pub fn parse_log_oneline(output: &str) -> Vec<GitCommit> {
    output
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let (hash, message) = line.split_once(' ').unwrap_or((line, ""));
            Some(GitCommit {
                hash: hash.to_string(),
                message: message.to_string(),
            })
        })
        .collect()
}

/// Parse `git worktree list --porcelain` output.
pub fn parse_worktree_porcelain(output: &str) -> Vec<GitWorktree> {
    let mut worktrees = Vec::new();
    let mut current_path: Option<String> = None;
    let mut current_branch: Option<String> = None;

    for line in output.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            // Flush previous
            if let Some(p) = current_path.take() {
                worktrees.push(GitWorktree {
                    path: p,
                    branch: current_branch.take(),
                });
            }
            current_path = Some(path.to_string());
            current_branch = None;
        } else if let Some(branch) = line.strip_prefix("branch refs/heads/") {
            current_branch = Some(branch.to_string());
        } else if line.is_empty() {
            // Blank line separates entries
            if let Some(p) = current_path.take() {
                worktrees.push(GitWorktree {
                    path: p,
                    branch: current_branch.take(),
                });
            }
        }
    }

    // Flush last
    if let Some(p) = current_path.take() {
        worktrees.push(GitWorktree {
            path: p,
            branch: current_branch.take(),
        });
    }

    worktrees
}

/// Parse `git diff --stat` output. Extract total additions/deletions from summary line.
pub fn parse_diff_stat(output: &str) -> DiffStat {
    let mut stat = DiffStat::default();

    // Last line format: " N files changed, N insertions(+), N deletions(-)"
    if let Some(last_line) = output.lines().last() {
        for part in last_line.split(',') {
            let part = part.trim();
            if part.contains("insertion") {
                if let Some(num) = part.split_whitespace().next() {
                    stat.additions = num.parse().unwrap_or(0);
                }
            } else if part.contains("deletion")
                && let Some(num) = part.split_whitespace().next()
            {
                stat.deletions = num.parse().unwrap_or(0);
            }
        }
    }

    stat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_porcelain_v2_staged_and_unstaged() {
        // Real porcelain v2 format: 1 <XY> <sub> <mH> <mI> <mW> <hH> <hI> <path>
        let output = "\
1 M. N... 100644 100644 100644 abc1234567890123456789012345678901234567890 def4567890123456789012345678901234567890 src/main.rs
1 .M N... 100644 100644 100644 abc1234567890123456789012345678901234567890 def4567890123456789012345678901234567890 src/lib.rs
";
        let changes = parse_status_porcelain_v2(output);
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].status, GitChangeStatus::Staged);
        assert_eq!(changes[0].path, "src/main.rs");
        assert_eq!(changes[1].status, GitChangeStatus::Unstaged);
        assert_eq!(changes[1].path, "src/lib.rs");
    }

    #[test]
    fn test_parse_status_untracked() {
        let output = "? new_file.rs\n? another.txt\n";
        let changes = parse_status_porcelain_v2(output);
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].status, GitChangeStatus::Untracked);
        assert_eq!(changes[0].path, "new_file.rs");
    }

    #[test]
    fn test_parse_status_conflict() {
        // Real porcelain v2 unmerged: u <XY> <sub> <m1> <m2> <m3> <mW> <h1> <h2> <h3> <path>
        let output = "\
u UU N... 100644 100644 100644 100644 abc1234567890123456789012345678901234567890 def4567890123456789012345678901234567890 ghi4567890123456789012345678901234567890 src/conflict.rs
";
        let changes = parse_status_porcelain_v2(output);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].status, GitChangeStatus::Conflict);
        assert_eq!(changes[0].path, "src/conflict.rs");
    }

    #[test]
    fn test_parse_status_empty() {
        let changes = parse_status_porcelain_v2("");
        assert!(changes.is_empty());
    }

    #[test]
    fn test_parse_log_oneline() {
        let output = "a1b2c3d feat: add parser module\nd4e5f6a init: project setup\n";
        let commits = parse_log_oneline(output);
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].hash, "a1b2c3d");
        assert_eq!(commits[0].message, "feat: add parser module");
        assert_eq!(commits[1].hash, "d4e5f6a");
    }

    #[test]
    fn test_parse_log_empty() {
        let commits = parse_log_oneline("");
        assert!(commits.is_empty());
    }

    #[test]
    fn test_parse_worktree_porcelain() {
        let output = "worktree /Users/jay/project\nHEAD abc123\nbranch refs/heads/main\n\n\
                       worktree /Users/jay/project-feat\nHEAD def456\nbranch refs/heads/feature/tui\n\n";
        let wts = parse_worktree_porcelain(output);
        assert_eq!(wts.len(), 2);
        assert_eq!(wts[0].path, "/Users/jay/project");
        assert_eq!(wts[0].branch.as_deref(), Some("main"));
        assert_eq!(wts[1].branch.as_deref(), Some("feature/tui"));
    }

    #[test]
    fn test_parse_worktree_empty() {
        let wts = parse_worktree_porcelain("");
        assert!(wts.is_empty());
    }

    #[test]
    fn test_parse_diff_stat() {
        let output = " src/main.rs | 42 +++++++-\n src/lib.rs  | 18 +++\n 2 files changed, 57 insertions(+), 3 deletions(-)\n";
        let stat = parse_diff_stat(output);
        assert_eq!(stat.additions, 57);
        assert_eq!(stat.deletions, 3);
    }

    #[test]
    fn test_parse_diff_stat_empty() {
        let stat = parse_diff_stat("");
        assert_eq!(stat.additions, 0);
        assert_eq!(stat.deletions, 0);
    }

    #[test]
    fn test_parse_diff_stat_insertions_only() {
        let output = " 1 file changed, 10 insertions(+)\n";
        let stat = parse_diff_stat(output);
        assert_eq!(stat.additions, 10);
        assert_eq!(stat.deletions, 0);
    }
}
