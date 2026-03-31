use crate::parser::models::{
    ApprovedStage, Complexity, CompletedStage, FlowState, Phase, SkippedStage, WorktreeInfo,
};
use crate::service::sanitizer::strip_ansi;

/// Parse devflow-state.md content into FlowState.
///
/// Tolerant parsing: unknown sections are stored in extra_fields.
/// Empty or minimal content returns default FlowState.
pub fn parse(content: &str) -> FlowState {
    let content = strip_ansi(content);
    let mut state = FlowState::default();
    let mut current_section: Option<String> = None;
    let mut section_lines: Vec<String> = Vec::new();

    for line in content.lines() {
        if let Some(header) = line.strip_prefix("## ") {
            // Flush previous section
            if let Some(ref section) = current_section {
                apply_section(&mut state, section, &section_lines);
            }
            current_section = Some(header.trim().to_string());
            section_lines.clear();
        } else if line.starts_with("# ") {
            // Top-level header — ignore (e.g., "# DevFlow State")
        } else if current_section.is_some() {
            section_lines.push(line.to_string());
        }
    }

    // Flush last section
    if let Some(ref section) = current_section {
        apply_section(&mut state, section, &section_lines);
    }

    state
}

fn apply_section(state: &mut FlowState, section: &str, lines: &[String]) {
    let value = lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    match section {
        "Current Phase" => {
            match value.parse::<Phase>() {
                Ok(phase) => state.phase = phase,
                Err(e) => {
                    tracing::warn!("devflow-state: {e}, falling back to default");
                    state.phase = Phase::default();
                }
            }
        }
        "Current Stage" => {
            state.stage = value.to_string();
        }
        "Complexity" => {
            match value.parse::<Complexity>() {
                Ok(c) => state.complexity = c,
                Err(e) => {
                    tracing::warn!("devflow-state: {e}, falling back to default");
                    state.complexity = Complexity::default();
                }
            }
        }
        "Selected Approach" => {
            if !value.is_empty() && value != "(pending)" {
                state.selected_approach = Some(value.to_string());
            }
        }
        "Completed Stages" => {
            state.completed_stages = parse_completed_stages(lines);
        }
        "Approved Stages" => {
            state.approved_stages = parse_approved_stages(lines);
        }
        "Skipped Stages" => {
            state.skipped_stages = parse_skipped_stages(lines);
        }
        "Active Unit" => {
            if !value.is_empty() && value != "(pending)" {
                state.active_unit = Some(value.to_string());
            }
        }
        "Completed Units" => {
            state.completed_units = lines
                .iter()
                .filter_map(|l| l.trim().strip_prefix("- ").map(|s| s.trim().to_string()))
                .collect();
        }
        "Worktree" => {
            state.worktree = parse_worktree(lines);
        }
        _ => {
            // Unknown section → store as extra field
            if !value.is_empty() {
                state.extra_fields.insert(section.to_string(), value);
            }
        }
    }
}

fn parse_completed_stages(lines: &[String]) -> Vec<CompletedStage> {
    lines
        .iter()
        .filter_map(|l| {
            let trimmed = l.trim().strip_prefix("- ")?;
            let (name, timestamp) = if let Some((n, t)) = trimmed.rsplit_once('(') {
                (n.trim().to_string(), Some(t.trim_end_matches(')').trim().to_string()))
            } else {
                (trimmed.trim().to_string(), None)
            };
            Some(CompletedStage { name, timestamp })
        })
        .collect()
}

fn parse_approved_stages(lines: &[String]) -> Vec<ApprovedStage> {
    lines
        .iter()
        .filter_map(|l| {
            let trimmed = l.trim().strip_prefix("- ")?;
            let (name, depth) = if let Some((n, d)) = trimmed.split_once("—") {
                let depth_val = d.trim().strip_prefix("depth:").map(|s| s.trim().to_string());
                (n.trim().to_string(), depth_val)
            } else {
                (trimmed.trim().to_string(), None)
            };
            Some(ApprovedStage { name, depth })
        })
        .collect()
}

fn parse_skipped_stages(lines: &[String]) -> Vec<SkippedStage> {
    lines
        .iter()
        .filter_map(|l| {
            let trimmed = l.trim().strip_prefix("- ")?;
            let (name, reason) = if let Some((n, r)) = trimmed.split_once("—") {
                let reason_val = r.trim().strip_prefix("reason:").map(|s| s.trim().to_string());
                (n.trim().to_string(), reason_val)
            } else {
                (trimmed.trim().to_string(), None)
            };
            Some(SkippedStage { name, reason })
        })
        .collect()
}

fn parse_worktree(lines: &[String]) -> WorktreeInfo {
    let mut info = WorktreeInfo::default();
    for line in lines {
        let trimmed = line.trim();
        if let Some((key, val)) = trimmed.split_once(':') {
            match key.trim().to_lowercase().as_str() {
                "branch" => info.branch = Some(val.trim().to_string()),
                "path" => info.path = Some(val.trim().to_string()),
                _ => {}
            }
        }
    }
    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_minimal() {
        let content = r#"# DevFlow State

## Current Phase
INCEPTION

## Current Stage
(pending)

## Complexity
(pending)

## Selected Approach
(pending)
"#;
        let state = parse(content);
        assert_eq!(state.phase, Phase::Inception);
        assert_eq!(state.stage, "(pending)");
        assert_eq!(state.complexity, Complexity::Standard); // (pending) → default
        assert!(state.selected_approach.is_none());
    }

    #[test]
    fn test_golden_standard() {
        let content = r#"# DevFlow State

## Current Phase
INCEPTION

## Current Stage
requirements-analysis

## Complexity
Standard

## Selected Approach
B (계층별 유닛 분해)

## Completed Stages
- workspace-detection (2026-03-30T14:18)
- complexity-declaration (2026-03-30T14:19)

## Approved Stages
- code-generation — depth: Standard
- build-and-test — depth: Standard

## Skipped Stages
- user-stories — reason: Minimal complexity
"#;
        let state = parse(content);
        assert_eq!(state.phase, Phase::Inception);
        assert_eq!(state.stage, "requirements-analysis");
        assert_eq!(state.complexity, Complexity::Standard);
        assert_eq!(state.selected_approach.as_deref(), Some("B (계층별 유닛 분해)"));
        assert_eq!(state.completed_stages.len(), 2);
        assert_eq!(state.completed_stages[0].name, "workspace-detection");
        assert_eq!(
            state.completed_stages[0].timestamp.as_deref(),
            Some("2026-03-30T14:18")
        );
        assert_eq!(state.approved_stages.len(), 2);
        assert_eq!(state.approved_stages[0].depth.as_deref(), Some("Standard"));
        assert_eq!(state.skipped_stages.len(), 1);
        assert_eq!(
            state.skipped_stages[0].reason.as_deref(),
            Some("Minimal complexity")
        );
    }

    #[test]
    fn test_golden_construction() {
        let content = r#"# DevFlow State

## Current Phase
CONSTRUCTION

## Current Stage
code-generation

## Complexity
Standard

## Active Unit
unit-03-file-watcher

## Completed Units
- unit-01-foundation
- unit-02-parser

## Worktree
branch: feature/devflow-tui
path: /Users/jay/projects/backend/devflow-tui
"#;
        let state = parse(content);
        assert_eq!(state.phase, Phase::Construction);
        assert_eq!(state.stage, "code-generation");
        assert_eq!(state.active_unit.as_deref(), Some("unit-03-file-watcher"));
        assert_eq!(state.completed_units.len(), 2);
        assert_eq!(state.completed_units[0], "unit-01-foundation");
        assert_eq!(state.worktree.branch.as_deref(), Some("feature/devflow-tui"));
    }

    #[test]
    fn test_golden_nonstandard() {
        let content = r#"# DevFlow State

## Current Phase
INCEPTION

## Current Stage
workflow-planning

## Complexity
Standard

## Project Root
/Users/jay/projects/backend

## Finishing Choice
B (PR pending)

## PR URL
https://github.com/user/repo/pull/42
"#;
        let state = parse(content);
        assert_eq!(state.phase, Phase::Inception);
        assert_eq!(
            state.extra_fields.get("Project Root").map(|s| s.as_str()),
            Some("/Users/jay/projects/backend")
        );
        assert_eq!(
            state.extra_fields.get("Finishing Choice").map(|s| s.as_str()),
            Some("B (PR pending)")
        );
        assert_eq!(
            state.extra_fields.get("PR URL").map(|s| s.as_str()),
            Some("https://github.com/user/repo/pull/42")
        );
    }

    #[test]
    fn test_golden_empty() {
        let state = parse("");
        assert_eq!(state.phase, Phase::Inception);
        assert!(state.stage.is_empty());
        assert_eq!(state.complexity, Complexity::Standard);
    }

    #[test]
    fn test_ansi_stripped() {
        let content = "# DevFlow State\n\n## Current Phase\n\x1b[32mCONSTRUCTION\x1b[0m\n";
        let state = parse(content);
        assert_eq!(state.phase, Phase::Construction);
    }
}
