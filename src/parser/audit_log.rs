use crate::parser::models::AuditEntry;
use crate::service::sanitizer::strip_ansi;

/// Parse audit.md / devflow-audit.md content into Vec<AuditEntry>.
///
/// Supports two formats:
/// - Brief: `[timestamp] stage — choice`
/// - Detailed: `## Stage` + `**Timestamp**:` + metadata
///
/// Unrecognized lines are preserved as raw_line.
pub fn parse(content: &str) -> Vec<AuditEntry> {
    let content = strip_ansi(content);
    let mut entries = Vec::new();
    let mut current_stage: Option<String> = None;
    let mut current_timestamp: Option<String> = None;
    let mut current_choice: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Detailed format: ## Stage header
        if let Some(stage) = trimmed.strip_prefix("## ") {
            // Flush previous detailed entry
            flush_detailed(
                &mut entries,
                &mut current_stage,
                &mut current_timestamp,
                &mut current_choice,
            );
            current_stage = Some(stage.trim().to_string());
            continue;
        }

        // Inside detailed entry — check if we've left the block
        if current_stage.is_some() {
            if let Some(val) = trimmed.strip_prefix("**Timestamp**:") {
                current_timestamp = Some(val.trim().to_string());
                continue;
            }
            if let Some(val) = trimmed.strip_prefix("**User Input**:") {
                current_choice = Some(val.trim().to_string());
                continue;
            }
            if trimmed.starts_with("**") {
                // Other metadata fields — skip
                continue;
            }
            // Non-metadata line inside block: this means the detailed block has ended.
            // Flush the current detailed entry, then process this line normally.
            flush_detailed(
                &mut entries,
                &mut current_stage,
                &mut current_timestamp,
                &mut current_choice,
            );
            // Fall through to brief/unrecognized parsing below
        }

        // Brief format: [timestamp] stage — choice
        if let Some(entry) = try_parse_brief(trimmed) {
            entries.push(entry);
            continue;
        }

        // Unrecognized line — preserve as raw
        entries.push(AuditEntry {
            timestamp: None,
            stage: None,
            choice: None,
            raw_line: trimmed.to_string(),
        });
    }

    // Flush last detailed entry
    flush_detailed(
        &mut entries,
        &mut current_stage,
        &mut current_timestamp,
        &mut current_choice,
    );

    entries
}

fn flush_detailed(
    entries: &mut Vec<AuditEntry>,
    stage: &mut Option<String>,
    timestamp: &mut Option<String>,
    choice: &mut Option<String>,
) {
    if let Some(s) = stage.take() {
        entries.push(AuditEntry {
            timestamp: timestamp.take(),
            stage: Some(s),
            choice: choice.take(),
            raw_line: String::new(),
        });
    }
    *timestamp = None;
    *choice = None;
}

fn try_parse_brief(line: &str) -> Option<AuditEntry> {
    // Format: [timestamp] stage — choice
    // or: [timestamp] stage → choice
    let rest = line.strip_prefix('[')?;
    let (ts, rest) = rest.split_once(']')?;
    let rest = rest.trim();

    let (stage, choice) = if let Some((s, c)) = rest.split_once('—') {
        (Some(s.trim().to_string()), Some(c.trim().to_string()))
    } else if let Some((s, c)) = rest.split_once('→') {
        (Some(s.trim().to_string()), Some(c.trim().to_string()))
    } else {
        (Some(rest.to_string()), None)
    };

    Some(AuditEntry {
        timestamp: Some(ts.trim().to_string()),
        stage,
        choice,
        raw_line: line.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_brief_audit() {
        let content = r#"[2026-03-30T14:18] workspace-detection → B (Approve)
[2026-03-30T14:19] complexity-declaration → Standard
[2026-03-30T14:25] requirements-analysis — B (Approve, assumptions noted)
"#;
        let entries = parse(content);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].timestamp.as_deref(), Some("2026-03-30T14:18"));
        assert_eq!(entries[0].stage.as_deref(), Some("workspace-detection"));
        assert_eq!(entries[0].choice.as_deref(), Some("B (Approve)"));
        assert_eq!(entries[2].choice.as_deref(), Some("B (Approve, assumptions noted)"));
    }

    #[test]
    fn test_golden_detailed_audit() {
        let content = r#"## workspace-detection
**Timestamp**: 2026-03-30T14:18:00+09:00
**User Input**: B (Approve)
**AI Response**: Greenfield detected
**Context**: Initial workspace analysis

## complexity-declaration
**Timestamp**: 2026-03-30T14:19:00+09:00
**User Input**: Standard
"#;
        let entries = parse(content);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].stage.as_deref(), Some("workspace-detection"));
        assert_eq!(
            entries[0].timestamp.as_deref(),
            Some("2026-03-30T14:18:00+09:00")
        );
        assert_eq!(entries[0].choice.as_deref(), Some("B (Approve)"));
        assert_eq!(entries[1].stage.as_deref(), Some("complexity-declaration"));
    }

    #[test]
    fn test_golden_mixed_audit() {
        // Note: "Some unrecognized line here" is inside the detailed block
        // (after **User Input** but before the next brief entry),
        // so it gets absorbed by the detailed parser (skipped as other content).
        // To test unrecognized lines, they must appear outside any ## block.
        let content = r#"[2026-03-30T14:18] workspace-detection → B (Approve)

## requirements-analysis
**Timestamp**: 2026-03-30T14:25
**User Input**: B (Approve)

[2026-03-30T14:30] workflow-planning — B (Approve)
Some unrecognized line outside blocks
"#;
        let entries = parse(content);
        assert_eq!(entries.len(), 4);

        // Brief entry
        assert_eq!(entries[0].stage.as_deref(), Some("workspace-detection"));

        // Detailed entry (flushed when next non-## line encountered outside block)
        assert_eq!(entries[1].stage.as_deref(), Some("requirements-analysis"));
        assert_eq!(entries[1].timestamp.as_deref(), Some("2026-03-30T14:25"));

        // Brief entry
        assert_eq!(entries[2].stage.as_deref(), Some("workflow-planning"));

        // Unrecognized line (outside any block)
        assert!(entries[3].stage.is_none());
        assert_eq!(entries[3].raw_line, "Some unrecognized line outside blocks");
    }

    #[test]
    fn test_empty_audit() {
        let entries = parse("");
        assert!(entries.is_empty());
    }

    #[test]
    fn test_ansi_stripped_in_audit() {
        let content = "[\x1b[36m2026-03-30\x1b[0m] \x1b[32mworkspace\x1b[0m → B";
        let entries = parse(content);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].timestamp.as_deref(), Some("2026-03-30"));
        assert_eq!(entries[0].stage.as_deref(), Some("workspace"));
    }
}
