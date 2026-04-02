use std::path::{Path, PathBuf};
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::error::Result;
use crate::event::AppEvent;
use crate::parser::jsonl_token;
use crate::parser::models::TokenSnapshot;

const POLL_INTERVAL_SECS: u64 = 2;
const MAX_READ_CHUNK: u64 = 10 * 1024 * 1024; // 10 MB per poll cycle
const MAX_SCAN_DEPTH: u32 = 3;

/// Run the JSONL token watcher adapter.
///
/// Discovers the current session's JSONL file under `~/.claude/projects/`,
/// then incrementally reads new lines and accumulates token usage.
pub async fn run(
    cancel: CancellationToken,
    project_dir: PathBuf,
    event_tx: mpsc::Sender<AppEvent>,
) -> Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(POLL_INTERVAL_SECS));
    let mut snapshot = TokenSnapshot::default();
    let mut bytes_read: u64 = 0;
    let mut current_jsonl: Option<PathBuf> = None;

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                tracing::info!("jsonl_watcher: shutting down");
                return Ok(());
            }
            _ = interval.tick() => {
                // Re-discover JSONL file periodically (session may start later)
                if current_jsonl.as_ref().is_none_or(|p| !p.exists()) {
                    current_jsonl = find_session_jsonl(&project_dir);
                    bytes_read = 0;
                    if current_jsonl.is_some() {
                        tracing::info!("jsonl_watcher: found {:?}", current_jsonl);
                    }
                }

                if let Some(ref path) = current_jsonl {
                    match read_incremental(path, &mut bytes_read, &mut snapshot).await {
                        Ok(true) => {
                            let _ = event_tx
                                .send(AppEvent::TokenUsageUpdated(snapshot.clone()))
                                .await;
                        }
                        Ok(false) => {} // no new data
                        Err(e) => {
                            tracing::warn!("jsonl_watcher: read error: {e}");
                        }
                    }
                }
            }
        }
    }
}

/// Read new lines from JSONL file starting at `bytes_read` offset using seek.
/// Returns true if new token records were found.
async fn read_incremental(
    path: &Path,
    bytes_read: &mut u64,
    snapshot: &mut TokenSnapshot,
) -> std::result::Result<bool, String> {
    let metadata = tokio::fs::metadata(path)
        .await
        .map_err(|e| e.to_string())?;
    let file_size = metadata.len();

    if file_size <= *bytes_read {
        return Ok(false);
    }

    let to_read = (file_size - *bytes_read).min(MAX_READ_CHUNK);

    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|e| e.to_string())?;
    file.seek(std::io::SeekFrom::Start(*bytes_read))
        .await
        .map_err(|e| e.to_string())?;

    let mut buf = vec![0u8; to_read as usize];
    let bytes_actually_read = file.read(&mut buf).await.map_err(|e| e.to_string())?;
    buf.truncate(bytes_actually_read);

    let raw = String::from_utf8_lossy(&buf);

    // Only advance to last complete newline to avoid partial line loss
    let usable_len = match raw.rfind('\n') {
        Some(pos) => pos + 1,
        None => return Ok(false), // no complete line yet
    };
    let usable = &raw[..usable_len];
    *bytes_read += usable_len as u64;

    let mut found = false;
    for line in usable.lines() {
        if let Some((record, timestamp)) = jsonl_token::parse_line(line) {
            jsonl_token::accumulate(snapshot, record, timestamp);
            found = true;
        }
    }

    Ok(found)
}

/// Find the most recently modified `.jsonl` file for the given project directory
/// under `~/.claude/projects/`.
fn find_session_jsonl(project_dir: &Path) -> Option<PathBuf> {
    let claude_dir = dirs::home_dir()?.join(".claude").join("projects");
    if !claude_dir.exists() {
        return None;
    }

    // Claude Code encodes project path as directory name:
    // /Users/jay/projects/backend/devflow-tui → -Users-jay-projects-backend-devflow-tui
    let project_key = project_dir
        .to_string_lossy()
        .replace('/', "-");

    let project_claude_dir = claude_dir.join(&project_key);
    if !project_claude_dir.exists() {
        return None;
    }

    find_newest_jsonl(&project_claude_dir, 0)
}

fn find_newest_jsonl(dir: &Path, depth: u32) -> Option<PathBuf> {
    if depth > MAX_SCAN_DEPTH {
        return None;
    }

    let mut newest: Option<(PathBuf, std::time::SystemTime)> = None;

    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        // Use entry.file_type() which does NOT follow symlinks
        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        let path = entry.path();
        if file_type.is_dir() {
            if let Some(found) = find_newest_jsonl(&path, depth + 1) {
                let modified = std::fs::metadata(&found)
                    .ok()
                    .and_then(|m| m.modified().ok());
                if let Some(mod_time) = modified {
                    if newest.as_ref().is_none_or(|(_, t)| mod_time > *t) {
                        newest = Some((found, mod_time));
                    }
                }
            }
        } else if file_type.is_file() && path.extension().is_some_and(|ext| ext == "jsonl") {
            let modified = entry.metadata().ok().and_then(|m| m.modified().ok());
            if let Some(mod_time) = modified {
                if newest.as_ref().is_none_or(|(_, t)| mod_time > *t) {
                    newest = Some((path, mod_time));
                }
            }
        }
    }

    newest.map(|(p, _)| p)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_newest_jsonl_selects_most_recent() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        // Create two JSONL files with different modification times
        let old = dir.join("old.jsonl");
        let new = dir.join("new.jsonl");
        fs::write(&old, r#"{"type":"user"}"#).unwrap();
        // Ensure different mtime
        std::thread::sleep(Duration::from_millis(50));
        fs::write(&new, r#"{"type":"assistant"}"#).unwrap();

        let result = find_newest_jsonl(dir, 0).unwrap();
        assert_eq!(result, new);
    }

    #[test]
    fn test_find_newest_jsonl_recurses_into_subdirs() {
        let tmp = TempDir::new().unwrap();
        let subdir = tmp.path().join("session-abc");
        fs::create_dir_all(&subdir).unwrap();
        let jsonl = subdir.join("log.jsonl");
        fs::write(&jsonl, r#"{"type":"assistant"}"#).unwrap();

        let result = find_newest_jsonl(tmp.path(), 0).unwrap();
        assert_eq!(result, jsonl);
    }

    #[test]
    fn test_find_newest_jsonl_empty_dir() {
        let tmp = TempDir::new().unwrap();
        assert!(find_newest_jsonl(tmp.path(), 0).is_none());
    }

    #[test]
    fn test_find_newest_jsonl_ignores_non_jsonl() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("notes.txt"), "hello").unwrap();
        fs::write(tmp.path().join("data.json"), "{}").unwrap();
        assert!(find_newest_jsonl(tmp.path(), 0).is_none());
    }

    #[test]
    fn test_find_newest_jsonl_depth_limit() {
        let tmp = TempDir::new().unwrap();
        // Create deeply nested directory beyond MAX_SCAN_DEPTH (3)
        let mut dir = tmp.path().to_path_buf();
        for i in 0..5 {
            dir = dir.join(format!("level{i}"));
        }
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("deep.jsonl"), r#"{"type":"assistant"}"#).unwrap();

        // Also a reachable file at depth 1
        let shallow = tmp.path().join("session");
        fs::create_dir_all(&shallow).unwrap();
        fs::write(shallow.join("log.jsonl"), r#"{"type":"assistant"}"#).unwrap();

        let result = find_newest_jsonl(tmp.path(), 0).unwrap();
        // Should find the shallow one, not the deep one
        assert!(result.to_string_lossy().contains("log.jsonl"));
    }

    #[tokio::test]
    async fn test_read_incremental_new_lines() {
        let tmp = TempDir::new().unwrap();
        let jsonl_path = tmp.path().join("session.jsonl");

        // Write initial content
        let line1 = r#"{"type":"assistant","message":{"role":"assistant","usage":{"input_tokens":100,"output_tokens":50}},"timestamp":"2026-03-31T12:00:00.000Z"}"#;
        fs::write(&jsonl_path, format!("{line1}\n")).unwrap();

        let mut snapshot = TokenSnapshot::default();
        let mut bytes_read = 0u64;

        // First read
        let changed = read_incremental(&jsonl_path, &mut bytes_read, &mut snapshot)
            .await
            .unwrap();
        assert!(changed);
        assert_eq!(snapshot.cumulative.input_tokens, 100);
        assert_eq!(snapshot.cumulative.output_tokens, 50);
        assert_eq!(snapshot.turn_count, 1);

        // No change
        let changed = read_incremental(&jsonl_path, &mut bytes_read, &mut snapshot)
            .await
            .unwrap();
        assert!(!changed);
        assert_eq!(snapshot.turn_count, 1);

        // Append new line
        use std::io::Write;
        let mut file = fs::OpenOptions::new().append(true).open(&jsonl_path).unwrap();
        let line2 = r#"{"type":"assistant","message":{"role":"assistant","usage":{"input_tokens":200,"output_tokens":100}},"timestamp":"2026-03-31T12:01:00.000Z"}"#;
        writeln!(file, "{line2}").unwrap();

        let changed = read_incremental(&jsonl_path, &mut bytes_read, &mut snapshot)
            .await
            .unwrap();
        assert!(changed);
        assert_eq!(snapshot.cumulative.input_tokens, 300);
        assert_eq!(snapshot.turn_count, 2);
    }

    #[tokio::test]
    async fn test_read_incremental_skips_user_messages() {
        let tmp = TempDir::new().unwrap();
        let jsonl_path = tmp.path().join("session.jsonl");

        let content = format!(
            "{}\n{}\n",
            r#"{"type":"user","message":{"role":"user","content":"hi"},"timestamp":"2026-03-31T12:00:00.000Z"}"#,
            r#"{"type":"assistant","message":{"role":"assistant","usage":{"input_tokens":50,"output_tokens":25}},"timestamp":"2026-03-31T12:00:01.000Z"}"#,
        );
        fs::write(&jsonl_path, content).unwrap();

        let mut snapshot = TokenSnapshot::default();
        let mut bytes_read = 0u64;

        let changed = read_incremental(&jsonl_path, &mut bytes_read, &mut snapshot)
            .await
            .unwrap();
        assert!(changed);
        assert_eq!(snapshot.turn_count, 1); // Only assistant message counted
        assert_eq!(snapshot.cumulative.input_tokens, 50);
    }
}
