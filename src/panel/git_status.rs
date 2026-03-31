use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::action::Action;
use crate::component::Component;
use crate::event::AppEvent;
use crate::parser::models::{GitChangeStatus, GitSnapshot};
use crate::ui::theme::{self, Icons, Theme};

pub struct GitStatusPanel {
    snapshot: GitSnapshot,
    changes_state: ListState,
}

impl Default for GitStatusPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl GitStatusPanel {
    pub fn new() -> Self {
        Self {
            snapshot: GitSnapshot::default(),
            changes_state: ListState::default(),
        }
    }

    pub fn set_snapshot(&mut self, snapshot: GitSnapshot) {
        self.snapshot = snapshot;
        self.clamp_selection();
    }

    pub fn clamp_scroll(&mut self) {
        self.clamp_selection();
    }

    fn clamp_selection(&mut self) {
        let len = self.snapshot.changes.len();
        if let Some(selected) = self.changes_state.selected()
            && selected >= len
            && len > 0
        {
            self.changes_state.select(Some(len - 1));
        }
    }
}

impl Component for GitStatusPanel {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        let len = self.snapshot.changes.len();
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if len > 0 {
                    let current = self.changes_state.selected().unwrap_or(0);
                    self.changes_state.select(Some((current + 1).min(len - 1)));
                }
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if len > 0 {
                    let current = self.changes_state.selected().unwrap_or(0);
                    self.changes_state.select(Some(current.saturating_sub(1)));
                }
                None
            }
            KeyCode::Enter => Some(Action::Select),
            _ => None,
        }
    }

    fn handle_event(&mut self, event: &AppEvent) {
        if let AppEvent::GitStatusUpdated(snapshot) = event {
            self.set_snapshot(snapshot.clone());
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let border_style = if focused {
            Theme::focus_border()
        } else {
            Theme::unfocus_border()
        };

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(theme::panel_title("Git Status", focused))
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.height < 3 {
            return;
        }

        // Split inner: header(3) | changes(fill) | bottom(commits + worktrees)
        let header_h = 3u16;
        let worktrees_h = if self.snapshot.worktrees.is_empty() {
            0
        } else {
            self.snapshot.worktrees.len() as u16 + 2 // blank line + "Worktrees:" + entries
        };
        let commits_h = self.snapshot.commits.len().min(5) as u16 + 2 + worktrees_h;
        let changes_h = inner.height.saturating_sub(header_h + commits_h);

        let [header_area, changes_area, commits_area] = Layout::vertical([
            Constraint::Length(header_h),
            Constraint::Length(changes_h),
            Constraint::Length(commits_h),
        ])
        .areas(inner);

        // Header: branch + HEAD + diff stat
        let header_lines = vec![
            Line::from(vec![
                Span::from("  Branch: ").dim(),
                Span::from(self.snapshot.branch.as_str()).cyan().bold(),
            ]),
            Line::from(vec![
                Span::from("  HEAD:   ").dim(),
                Span::from(self.snapshot.head.as_str()).dim(),
            ]),
            Line::from(vec![
                Span::from("  Diff:   ").dim(),
                Span::from(format!("+{}", self.snapshot.diff_stat.additions)).green(),
                Span::from(" / "),
                Span::from(format!("-{}", self.snapshot.diff_stat.deletions)).red(),
            ]),
        ];
        frame.render_widget(Paragraph::new(header_lines), header_area);

        // Changes list
        let change_items: Vec<ListItem> = self
            .snapshot
            .changes
            .iter()
            .map(|c| {
                let (icon, style) = match c.status {
                    GitChangeStatus::Staged => (Icons::staged(), Theme::staged()),
                    GitChangeStatus::Unstaged => (Icons::unstaged(), Theme::unstaged()),
                    GitChangeStatus::Untracked => (Icons::untracked(), Theme::untracked()),
                    GitChangeStatus::Conflict => (Icons::conflict(), Theme::conflict()),
                };
                let diff_str = match (c.additions, c.deletions) {
                    (Some(a), Some(d)) => format!(" +{a} -{d}"),
                    _ => String::new(),
                };
                ListItem::new(Line::from(vec![
                    Span::from(format!("  {icon} ")).style(style),
                    Span::from(c.path.as_str()).style(style),
                    Span::from(diff_str).dim(),
                ]))
            })
            .collect();

        let changes_list = List::new(change_items)
            .highlight_style(Theme::highlight());
        frame.render_stateful_widget(changes_list, changes_area, &mut self.changes_state);

        // Recent commits + worktrees
        let mut bottom_lines: Vec<Line> = Vec::new();

        bottom_lines.push(Line::from("  Commits:").dim());
        for c in &self.snapshot.commits {
            bottom_lines.push(Line::from(vec![
                Span::from(format!("    {} ", c.hash)).yellow(),
                Span::from(c.message.as_str()).dim(),
            ]));
        }

        if !self.snapshot.worktrees.is_empty() {
            bottom_lines.push(Line::from(""));
            bottom_lines.push(Line::from("  Worktrees:").dim());
            for wt in &self.snapshot.worktrees {
                let branch_str = wt
                    .branch
                    .as_deref()
                    .unwrap_or("(detached)");
                bottom_lines.push(Line::from(vec![
                    Span::from("    ").dim(),
                    Span::from(branch_str).cyan(),
                    Span::from(format!("  {}", wt.path)).dim(),
                ]));
            }
        }

        frame.render_widget(Paragraph::new(bottom_lines), commits_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::GitChange;

    #[test]
    fn test_clamp_scroll_empty() {
        let mut panel = GitStatusPanel::new();
        panel.changes_state.select(Some(5));
        panel.clamp_scroll();
        // No changes → selection stays (len=0 guard)
    }

    #[test]
    fn test_set_snapshot_clamps() {
        let mut panel = GitStatusPanel::new();
        panel.changes_state.select(Some(10));
        let snapshot = GitSnapshot {
            changes: vec![GitChange {
                status: GitChangeStatus::Staged,
                path: "file.rs".to_string(),
                additions: None,
                deletions: None,
            }],
            ..Default::default()
        };
        panel.set_snapshot(snapshot);
        assert_eq!(panel.changes_state.selected(), Some(0));
    }

    // ── Render tests ──

    use crate::test_helpers::{buffer_contains_str, render_component};
    use crate::parser::models::{DiffStat, GitCommit, GitWorktree};

    #[test]
    fn render_empty_snapshot_no_panic() {
        let mut panel = GitStatusPanel::new();
        let _terminal = render_component(&mut panel, 60, 20, true);
    }

    #[test]
    fn render_branch_and_head() {
        let mut panel = GitStatusPanel::new();
        panel.set_snapshot(GitSnapshot {
            branch: "feature/test".to_string(),
            head: "abc1234".to_string(),
            ..Default::default()
        });
        let terminal = render_component(&mut panel, 60, 20, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "feature/test"));
        assert!(buffer_contains_str(buf, "abc1234"));
    }

    #[test]
    fn render_diff_stat() {
        let mut panel = GitStatusPanel::new();
        panel.set_snapshot(GitSnapshot {
            diff_stat: DiffStat { additions: 10, deletions: 5 },
            ..Default::default()
        });
        let terminal = render_component(&mut panel, 60, 20, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "+10"));
        assert!(buffer_contains_str(buf, "-5"));
    }

    #[test]
    fn render_staged_change() {
        let mut panel = GitStatusPanel::new();
        panel.set_snapshot(GitSnapshot {
            changes: vec![GitChange {
                status: GitChangeStatus::Staged,
                path: "src/main.rs".to_string(),
                additions: Some(3),
                deletions: Some(1),
            }],
            ..Default::default()
        });
        let terminal = render_component(&mut panel, 60, 20, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "S"));
        assert!(buffer_contains_str(buf, "src/main.rs"));
    }

    #[test]
    fn render_unstaged_change() {
        let mut panel = GitStatusPanel::new();
        panel.set_snapshot(GitSnapshot {
            changes: vec![GitChange {
                status: GitChangeStatus::Unstaged,
                path: "lib.rs".to_string(),
                additions: None,
                deletions: None,
            }],
            ..Default::default()
        });
        let terminal = render_component(&mut panel, 60, 20, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "M"));
        assert!(buffer_contains_str(buf, "lib.rs"));
    }

    #[test]
    fn render_commits() {
        let mut panel = GitStatusPanel::new();
        panel.set_snapshot(GitSnapshot {
            commits: vec![GitCommit {
                hash: "deadbeef".to_string(),
                message: "fix: resolve crash".to_string(),
            }],
            ..Default::default()
        });
        let terminal = render_component(&mut panel, 60, 20, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "deadbeef"));
        assert!(buffer_contains_str(buf, "fix: resolve crash"));
    }

    #[test]
    fn render_worktrees_visible() {
        let mut panel = GitStatusPanel::new();
        panel.set_snapshot(GitSnapshot {
            commits: vec![GitCommit {
                hash: "abc1234".to_string(),
                message: "init".to_string(),
            }],
            worktrees: vec![GitWorktree {
                path: "/tmp/wt".to_string(),
                branch: Some("feature/wt".to_string()),
            }],
            ..Default::default()
        });
        let terminal = render_component(&mut panel, 60, 20, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Worktrees:"));
        assert!(buffer_contains_str(buf, "feature/wt"));
    }
}
