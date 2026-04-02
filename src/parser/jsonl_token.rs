use serde::Deserialize;

use crate::parser::models::{TimestampedTokens, TokenRecord, TokenSnapshot};

#[derive(Deserialize)]
struct JsonlEntry {
    #[serde(rename = "type")]
    entry_type: Option<String>,
    message: Option<MessageBody>,
    timestamp: Option<String>,
}

#[derive(Deserialize)]
struct MessageBody {
    usage: Option<UsageData>,
}

#[derive(Deserialize)]
struct UsageData {
    #[serde(default)]
    input_tokens: u64,
    #[serde(default)]
    output_tokens: u64,
    #[serde(default)]
    cache_creation_input_tokens: u64,
    #[serde(default)]
    cache_read_input_tokens: u64,
}

/// Parse a single JSONL line and extract token usage + timestamp.
/// Returns None for user messages, malformed JSON, or lines without usage data.
pub fn parse_line(line: &str) -> Option<(TokenRecord, Option<chrono::DateTime<chrono::Utc>>)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    let entry: JsonlEntry = serde_json::from_str(trimmed).ok()?;
    if entry.entry_type.as_deref() != Some("assistant") {
        return None;
    }
    let usage = entry.message?.usage?;
    let timestamp = entry
        .timestamp
        .and_then(|ts| chrono::DateTime::parse_from_rfc3339(&ts).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    Some((
        TokenRecord {
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            cache_creation_input_tokens: usage.cache_creation_input_tokens,
            cache_read_input_tokens: usage.cache_read_input_tokens,
        },
        timestamp,
    ))
}

/// Accumulate a new TokenRecord into an existing TokenSnapshot.
/// Prunes recent_turns entries older than the burn rate window.
pub fn accumulate(
    snapshot: &mut TokenSnapshot,
    record: TokenRecord,
    timestamp: Option<chrono::DateTime<chrono::Utc>>,
) {
    let turn_tokens = record.total();
    snapshot.cumulative.input_tokens = snapshot
        .cumulative
        .input_tokens
        .saturating_add(record.input_tokens);
    snapshot.cumulative.output_tokens = snapshot
        .cumulative
        .output_tokens
        .saturating_add(record.output_tokens);
    snapshot.cumulative.cache_creation_input_tokens = snapshot
        .cumulative
        .cache_creation_input_tokens
        .saturating_add(record.cache_creation_input_tokens);
    snapshot.cumulative.cache_read_input_tokens = snapshot
        .cumulative
        .cache_read_input_tokens
        .saturating_add(record.cache_read_input_tokens);
    snapshot.turn_count = snapshot.turn_count.saturating_add(1);
    if let Some(ts) = timestamp {
        snapshot
            .recent_turns
            .push(TimestampedTokens { timestamp: ts, tokens: turn_tokens });
        prune_old_turns(snapshot);
    }
    snapshot.burn_rate_per_min = calc_burn_rate(snapshot);
}

const BURN_RATE_WINDOW_SECS: i64 = 30;

fn prune_old_turns(snapshot: &mut TokenSnapshot) {
    if let Some(latest) = snapshot.recent_turns.last() {
        let cutoff = latest.timestamp - chrono::Duration::seconds(BURN_RATE_WINDOW_SECS);
        snapshot.recent_turns.retain(|t| t.timestamp >= cutoff);
    }
}

/// Calculate burn rate (tokens per minute) from recent_turns within the 30s window.
pub fn calc_burn_rate(snapshot: &TokenSnapshot) -> f64 {
    if snapshot.recent_turns.len() < 2 {
        return 0.0;
    }
    let earliest = snapshot.recent_turns[0].timestamp;
    let latest = snapshot.recent_turns[snapshot.recent_turns.len() - 1].timestamp;
    let duration_secs = (latest - earliest).num_seconds();
    if duration_secs <= 0 {
        return 0.0;
    }
    let window_tokens: u64 = snapshot.recent_turns.iter().map(|t| t.tokens).sum();
    (window_tokens as f64 / duration_secs as f64) * 60.0
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // ── parse_line tests ──

    #[test]
    fn test_parse_assistant_message_with_usage() {
        let line = r#"{"type":"assistant","message":{"role":"assistant","usage":{"input_tokens":100,"output_tokens":50,"cache_creation_input_tokens":200,"cache_read_input_tokens":300}},"timestamp":"2026-03-31T12:00:00.000Z"}"#;
        let (record, ts) = parse_line(line).expect("should parse assistant usage");
        assert_eq!(record.input_tokens, 100);
        assert_eq!(record.output_tokens, 50);
        assert_eq!(record.cache_creation_input_tokens, 200);
        assert_eq!(record.cache_read_input_tokens, 300);
        assert!(ts.is_some());
    }

    #[test]
    fn test_parse_user_message_returns_none() {
        let line = r#"{"type":"user","message":{"role":"user","content":"hello"},"timestamp":"2026-03-31T12:00:00.000Z"}"#;
        assert!(parse_line(line).is_none());
    }

    #[test]
    fn test_parse_malformed_json_returns_none() {
        assert!(parse_line("{not valid json").is_none());
        assert!(parse_line("").is_none());
    }

    #[test]
    fn test_parse_assistant_without_usage_returns_none() {
        let line = r#"{"type":"assistant","message":{"role":"assistant","content":[]},"timestamp":"2026-03-31T12:00:00.000Z"}"#;
        assert!(parse_line(line).is_none());
    }

    #[test]
    fn test_parse_missing_optional_cache_fields() {
        let line = r#"{"type":"assistant","message":{"role":"assistant","usage":{"input_tokens":100,"output_tokens":50}},"timestamp":"2026-03-31T12:00:00.000Z"}"#;
        let (record, _) = parse_line(line).expect("should parse with missing cache fields");
        assert_eq!(record.input_tokens, 100);
        assert_eq!(record.output_tokens, 50);
        assert_eq!(record.cache_creation_input_tokens, 0);
        assert_eq!(record.cache_read_input_tokens, 0);
    }

    // ── parse_line timestamp tests ──

    #[test]
    fn test_parse_valid_rfc3339_timestamp() {
        let line = r#"{"type":"assistant","message":{"role":"assistant","usage":{"input_tokens":10,"output_tokens":5}},"timestamp":"2026-03-31T12:00:00.000Z"}"#;
        let (_, ts) = parse_line(line).unwrap();
        let ts = ts.unwrap();
        assert_eq!(ts.format("%Y-%m-%d").to_string(), "2026-03-31");
    }

    #[test]
    fn test_parse_invalid_timestamp_returns_none_ts() {
        let line = r#"{"type":"assistant","message":{"role":"assistant","usage":{"input_tokens":10,"output_tokens":5}},"timestamp":"not-a-date"}"#;
        let (record, ts) = parse_line(line).unwrap();
        assert_eq!(record.input_tokens, 10);
        assert!(ts.is_none());
    }

    #[test]
    fn test_parse_missing_timestamp_field() {
        let line = r#"{"type":"assistant","message":{"role":"assistant","usage":{"input_tokens":10,"output_tokens":5}}}"#;
        let (record, ts) = parse_line(line).unwrap();
        assert_eq!(record.input_tokens, 10);
        assert!(ts.is_none());
    }

    // ── accumulate tests ──

    #[test]
    fn test_accumulate_adds_to_cumulative() {
        let mut snap = TokenSnapshot::default();
        let record = TokenRecord {
            input_tokens: 100,
            output_tokens: 50,
            cache_creation_input_tokens: 200,
            cache_read_input_tokens: 300,
        };
        accumulate(&mut snap, record, None);
        assert_eq!(snap.cumulative.input_tokens, 100);
        assert_eq!(snap.cumulative.output_tokens, 50);
        assert_eq!(snap.cumulative.cache_creation_input_tokens, 200);
        assert_eq!(snap.cumulative.cache_read_input_tokens, 300);
        assert_eq!(snap.turn_count, 1);
    }

    #[test]
    fn test_accumulate_multiple_records() {
        let mut snap = TokenSnapshot::default();
        for _ in 0..3 {
            let record = TokenRecord {
                input_tokens: 100,
                output_tokens: 50,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            };
            accumulate(&mut snap, record, None);
        }
        assert_eq!(snap.cumulative.input_tokens, 300);
        assert_eq!(snap.cumulative.output_tokens, 150);
        assert_eq!(snap.turn_count, 3);
    }

    #[test]
    fn test_accumulate_saturating_no_overflow() {
        let mut snap = TokenSnapshot::default();
        let record = TokenRecord {
            input_tokens: u64::MAX,
            output_tokens: 1,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        };
        accumulate(&mut snap, record, None);
        let record2 = TokenRecord {
            input_tokens: 100,
            output_tokens: 0,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        };
        accumulate(&mut snap, record2, None);
        assert_eq!(snap.cumulative.input_tokens, u64::MAX);
    }

    // ── burn rate tests ──

    #[test]
    fn test_burn_rate_empty_returns_zero() {
        let snap = TokenSnapshot::default();
        assert_eq!(calc_burn_rate(&snap), 0.0);
    }

    #[test]
    fn test_burn_rate_single_entry_returns_zero() {
        use chrono::TimeZone;
        let mut snap = TokenSnapshot::default();
        let ts = chrono::Utc.with_ymd_and_hms(2026, 3, 31, 12, 0, 0).unwrap();
        accumulate(
            &mut snap,
            TokenRecord { input_tokens: 100, output_tokens: 0, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
            Some(ts),
        );
        assert_eq!(calc_burn_rate(&snap), 0.0);
    }

    #[test]
    fn test_burn_rate_within_window() {
        use chrono::TimeZone;
        let mut snap = TokenSnapshot::default();
        let base = chrono::Utc.with_ymd_and_hms(2026, 3, 31, 12, 0, 0).unwrap();

        // 3 turns over 30 seconds: 100 billable tokens each
        for i in 0..3 {
            let ts = base + chrono::Duration::seconds(i * 15);
            let record = TokenRecord {
                input_tokens: 50,
                output_tokens: 50,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            };
            accumulate(&mut snap, record, Some(ts));
        }
        let rate = calc_burn_rate(&snap);
        // 300 tokens in 30 seconds = 600 tokens/min
        assert!((rate - 600.0).abs() < 1.0, "expected ~600, got {rate}");
    }

    #[test]
    fn test_burn_rate_long_session_prunes_old() {
        use chrono::TimeZone;
        let mut snap = TokenSnapshot::default();
        let base = chrono::Utc.with_ymd_and_hms(2026, 3, 31, 12, 0, 0).unwrap();

        // Turn 1 at t=0: 1000 tokens (will be pruned)
        accumulate(
            &mut snap,
            TokenRecord { input_tokens: 500, output_tokens: 500, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
            Some(base),
        );

        // Turn 2 at t=60s: 100 tokens (within window of turn 3)
        accumulate(
            &mut snap,
            TokenRecord { input_tokens: 50, output_tokens: 50, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
            Some(base + chrono::Duration::seconds(60)),
        );

        // Turn 3 at t=80s: 100 tokens
        accumulate(
            &mut snap,
            TokenRecord { input_tokens: 50, output_tokens: 50, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
            Some(base + chrono::Duration::seconds(80)),
        );

        // Window: t=50..80 → turns 2+3 = 200 tokens in 20 seconds = 600/min
        let rate = calc_burn_rate(&snap);
        assert!((rate - 600.0).abs() < 1.0, "expected ~600, got {rate}");

        // Turn 1 should have been pruned
        assert_eq!(snap.recent_turns.len(), 2);

        // But cumulative still has all tokens
        assert_eq!(snap.cumulative.input_tokens, 600);
    }

    // ── TokenSnapshot method tests ──

    #[test]
    fn test_total_billable() {
        let snap = TokenSnapshot {
            cumulative: TokenRecord {
                input_tokens: 100,
                output_tokens: 50,
                cache_creation_input_tokens: 200,
                cache_read_input_tokens: 300, // not billable
            },
            ..Default::default()
        };
        assert_eq!(snap.total_billable(), 350); // 100 + 200 + 50
    }

    #[test]
    fn test_total_input_display() {
        let snap = TokenSnapshot {
            cumulative: TokenRecord {
                input_tokens: 100,
                output_tokens: 50,
                cache_creation_input_tokens: 200,
                cache_read_input_tokens: 300,
            },
            ..Default::default()
        };
        assert_eq!(snap.total_input_display(), 600); // 100 + 200 + 300
    }

    #[test]
    fn test_token_record_total() {
        let r = TokenRecord {
            input_tokens: 100,
            output_tokens: 50,
            cache_creation_input_tokens: 200,
            cache_read_input_tokens: 300,
        };
        // total() = input + output + cache_creation (excludes cache_read)
        assert_eq!(r.total(), 350);
    }
}
