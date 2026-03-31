use crate::parser::models::{Complexity, Phase, SessionSummary, WorkItem, WorkMarker};
use crate::service::sanitizer::strip_ansi;

/// Parse session-summary.md content into SessionSummary.
pub fn parse(content: &str) -> SessionSummary {
    let content = strip_ansi(content);
    let mut summary = SessionSummary::default();
    let mut current_section: Option<String> = None;
    let mut section_lines: Vec<String> = Vec::new();

    for line in content.lines() {
        if let Some(header) = line.strip_prefix("## ") {
            if let Some(ref section) = current_section {
                apply_section(&mut summary, section, &section_lines);
            }
            current_section = Some(header.trim().to_string());
            section_lines.clear();
        } else if line.starts_with("# ") {
            // Top-level header — ignore
        } else if line.starts_with("**") && current_section.is_none() {
            // Metadata lines before first section (e.g., **Last Updated**)
            parse_metadata_line(&mut summary, line);
        } else if let Some(ref _section) = current_section {
            section_lines.push(line.to_string());
        }
    }

    if let Some(ref section) = current_section {
        apply_section(&mut summary, section, &section_lines);
    }

    summary
}

fn parse_metadata_line(summary: &mut SessionSummary, line: &str) {
    if let Some(rest) = line.strip_prefix("**Last Updated**:") {
        summary.last_updated = Some(rest.trim().to_string());
    } else if let Some(rest) = line.strip_prefix("**Commit**:") {
        summary.commit = Some(rest.trim().to_string());
    }
}

fn apply_section(summary: &mut SessionSummary, section: &str, lines: &[String]) {
    match section {
        "Current State" => {
            for line in lines {
                let trimmed = line.trim().strip_prefix("- ").unwrap_or(line.trim());
                if let Some(val) = trimmed.strip_prefix("Phase:") {
                    match val.trim().parse::<Phase>() {
                        Ok(phase) => summary.phase = Some(phase),
                        Err(e) => tracing::warn!("session-summary: {e}"),
                    }
                } else if let Some(val) = trimmed.strip_prefix("Stage:") {
                    summary.stage = Some(val.trim().to_string());
                } else if let Some(val) = trimmed.strip_prefix("Complexity:") {
                    match val.trim().parse::<Complexity>() {
                        Ok(c) => summary.complexity = Some(c),
                        Err(e) => tracing::warn!("session-summary: {e}"),
                    }
                } else if let Some(val) = trimmed.strip_prefix("Commit:") {
                    summary.commit = Some(val.trim().to_string());
                }
            }
        }
        "Key Decisions" => {
            summary.key_decisions = lines
                .iter()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .map(|l| l.strip_prefix("- ").unwrap_or(l).to_string())
                .collect();
        }
        "Completed Work" | "INCEPTION" | "CONSTRUCTION" => {
            let items = parse_work_items(lines);
            summary.completed_work.extend(items);
        }
        "Next Steps" => {
            summary.next_steps = lines
                .iter()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .map(|l| l.strip_prefix("- ").unwrap_or(l).to_string())
                .collect();
        }
        "For Next Session" => {
            summary.for_next_session = lines
                .iter()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .map(|l| l.strip_prefix("- ").unwrap_or(l).to_string())
                .collect();
        }
        _ => {
            // Unknown section — ignore
        }
    }
}

fn parse_work_items(lines: &[String]) -> Vec<WorkItem> {
    lines
        .iter()
        .filter_map(|line| {
            let trimmed = line.trim();
            let trimmed = trimmed.strip_prefix("- ").unwrap_or(trimmed);
            parse_single_work_item(trimmed)
        })
        .collect()
}

fn parse_single_work_item(trimmed: &str) -> Option<WorkItem> {
    let markers: &[(&str, WorkMarker)] = &[
        ("[x] ", WorkMarker::Done),
        ("[~] ", WorkMarker::InProgress),
        ("[ ] ", WorkMarker::Pending),
    ];

    for (prefix, marker) in markers {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return Some(WorkItem {
                text: rest.to_string(),
                marker: *marker,
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_inception_session() {
        let content = r#"# Session Summary

**Last Updated**: 2026-03-30T14:40:00+09:00
**Commit**: 2a3cbee

## Current State
- Phase: INCEPTION
- Stage: requirements-analysis
- Complexity: Standard

## Key Decisions
- 2026-03-30T14:18 workspace-detection B — Greenfield
- 2026-03-30T14:19 complexity A→Standard

## Completed Work
### INCEPTION
- [x] workspace-detection — Greenfield detected
- [~] requirements-analysis — in progress
- [ ] workflow-planning — pending

## Next Steps
- Complete requirements-analysis

## For Next Session
- Review open questions in requirements
"#;
        let summary = parse(content);
        assert_eq!(summary.last_updated.as_deref(), Some("2026-03-30T14:40:00+09:00"));
        assert_eq!(summary.commit.as_deref(), Some("2a3cbee"));
        assert_eq!(summary.phase, Some(Phase::Inception));
        assert_eq!(summary.stage.as_deref(), Some("requirements-analysis"));
        assert_eq!(summary.complexity, Some(Complexity::Standard));
        assert_eq!(summary.key_decisions.len(), 2);
        assert_eq!(summary.completed_work.len(), 3);
        assert_eq!(summary.completed_work[0].marker, WorkMarker::Done);
        assert_eq!(summary.completed_work[1].marker, WorkMarker::InProgress);
        assert_eq!(summary.completed_work[2].marker, WorkMarker::Pending);
        assert_eq!(summary.next_steps.len(), 1);
        assert_eq!(summary.for_next_session.len(), 1);
    }

    #[test]
    fn test_golden_construction_session() {
        let content = r#"# Session Summary

**Last Updated**: 2026-03-30

## Current State
- Phase: CONSTRUCTION
- Stage: code-generation

## Key Decisions
- Approach B selected

## Completed Work
### INCEPTION
- [x] workspace-detection — done
- [x] requirements-analysis — done
### CONSTRUCTION
- [x] code-generation unit-01 — done
- [~] code-generation unit-02 — in progress

## Next Steps
- Complete unit-02 parser
"#;
        let summary = parse(content);
        assert_eq!(summary.phase, Some(Phase::Construction));
        assert_eq!(summary.completed_work.len(), 4);
        assert_eq!(summary.completed_work[3].marker, WorkMarker::InProgress);
        assert!(summary.completed_work[3].text.contains("unit-02"));
    }

    #[test]
    fn test_golden_empty_session() {
        let summary = parse("");
        assert!(summary.phase.is_none());
        assert!(summary.key_decisions.is_empty());
        assert!(summary.completed_work.is_empty());
    }
}
