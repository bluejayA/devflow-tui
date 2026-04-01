use std::path::{Path, PathBuf};

/// Result of checking Claude Code hooks configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookConfigStatus {
    /// Hooks are configured and endpoint matches.
    Configured,
    /// Hooks exist but endpoint doesn't match TUI port.
    EndpointMismatch { configured_url: String },
    /// No hooks configuration found.
    NotConfigured,
}

/// Check if Claude Code hooks are configured for devflow-tui.
///
/// Checks two locations:
/// 1. Project-level: .claude/settings.json
/// 2. User-level: ~/.claude/settings.json
pub fn check_hooks_config(project_dir: &Path, expected_port: u16, token: &str) -> HookConfigStatus {
    let project_settings = project_dir.join(".claude").join("settings.json");
    let user_settings = dirs::home_dir()
        .map(|h| h.join(".claude").join("settings.json"))
        .unwrap_or_else(|| PathBuf::from(""));

    for path in &[project_settings, user_settings] {
        if let Ok(content) = std::fs::read_to_string(path)
            && let Some(status) = check_content(&content, expected_port, token)
        {
            return status;
        }
    }

    HookConfigStatus::NotConfigured
}

fn check_content(content: &str, expected_port: u16, token: &str) -> Option<HookConfigStatus> {
    let json: serde_json::Value = serde_json::from_str(content).ok()?;
    let hooks = json.get("hooks")?.as_object()?;

    let expected_url_base = format!("http://localhost:{expected_port}/hook?token={token}");
    let expected_url_127 = format!("http://127.0.0.1:{expected_port}/hook?token={token}");
    let port_base = format!("localhost:{expected_port}");
    let port_127 = format!("127.0.0.1:{expected_port}");

    // Collect all hook URLs from the structured JSON
    let mut found_urls: Vec<String> = Vec::new();

    for (_event, matchers) in hooks {
        let Some(matchers) = matchers.as_array() else {
            continue;
        };
        for matcher in matchers {
            let Some(hook_list) = matcher.get("hooks").and_then(|h| h.as_array()) else {
                continue;
            };
            for hook in hook_list {
                if let Some(url) = hook.get("url").and_then(|u| u.as_str()) {
                    found_urls.push(url.to_string());
                }
            }
        }
    }

    if found_urls.is_empty() {
        return None;
    }

    // Check for exact match (port + token)
    for url in &found_urls {
        if url == &expected_url_base || url == &expected_url_127 {
            return Some(HookConfigStatus::Configured);
        }
    }

    // Check for port match but token mismatch
    for url in &found_urls {
        if url.contains(&port_base) || url.contains(&port_127) {
            return Some(HookConfigStatus::EndpointMismatch {
                configured_url: format!("port {expected_port} matched but token mismatch"),
            });
        }
    }

    // Different port entirely
    if let Some(url) = found_urls.first() {
        return Some(HookConfigStatus::EndpointMismatch {
            configured_url: url.clone(),
        });
    }

    None
}

/// Generate a hooks configuration JSON snippet for the user to add to settings.json.
pub fn generate_hooks_snippet(port: u16, token: &str) -> String {
    format!(
        r#"{{
  "hooks": {{
    "SubagentStart": [
      {{
        "matcher": ".*",
        "hooks": [
          {{
            "type": "http",
            "url": "http://localhost:{port}/hook?token={token}"
          }}
        ]
      }}
    ],
    "SubagentStop": [
      {{
        "matcher": ".*",
        "hooks": [
          {{
            "type": "http",
            "url": "http://localhost:{port}/hook?token={token}"
          }}
        ]
      }}
    ],
    "PreToolUse": [
      {{
        "matcher": ".*",
        "hooks": [
          {{
            "type": "http",
            "url": "http://localhost:{port}/hook?token={token}"
          }}
        ]
      }}
    ],
    "PostToolUse": [
      {{
        "matcher": ".*",
        "hooks": [
          {{
            "type": "http",
            "url": "http://localhost:{port}/hook?token={token}"
          }}
        ]
      }}
    ],
    "Stop": [
      {{
        "matcher": ".*",
        "hooks": [
          {{
            "type": "http",
            "url": "http://localhost:{port}/hook?token={token}"
          }}
        ]
      }}
    ]
  }}
}}"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_hooks_not_configured() {
        let dir = tempfile::TempDir::new().unwrap();
        let status = check_hooks_config(dir.path(), 9100, "testtoken");
        assert_eq!(status, HookConfigStatus::NotConfigured);
    }

    #[test]
    fn test_check_hooks_configured() {
        let dir = tempfile::TempDir::new().unwrap();
        let claude_dir = dir.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();
        let settings = claude_dir.join("settings.json");
        std::fs::write(
            &settings,
            r#"{"hooks":{"SubagentStart":[{"matcher":".*","hooks":[{"type":"http","url":"http://localhost:9100/hook?token=abc"}]}]}}"#,
        ).unwrap();

        let status = check_hooks_config(dir.path(), 9100, "abc");
        assert_eq!(status, HookConfigStatus::Configured);
    }

    #[test]
    fn test_check_hooks_token_mismatch() {
        let dir = tempfile::TempDir::new().unwrap();
        let claude_dir = dir.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();
        let settings = claude_dir.join("settings.json");
        std::fs::write(
            &settings,
            r#"{"hooks":{"SubagentStart":[{"matcher":".*","hooks":[{"type":"http","url":"http://localhost:9100/hook?token=oldtoken"}]}]}}"#,
        ).unwrap();

        let status = check_hooks_config(dir.path(), 9100, "newtoken");
        assert!(matches!(status, HookConfigStatus::EndpointMismatch { .. }));
    }

    #[test]
    fn test_check_hooks_endpoint_mismatch() {
        let dir = tempfile::TempDir::new().unwrap();
        let claude_dir = dir.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();
        let settings = claude_dir.join("settings.json");
        std::fs::write(
            &settings,
            r#"{"hooks":{"SubagentStart":[{"matcher":".*","hooks":[{"type":"http","url":"http://localhost:9200/hook?token=abc"}]}]}}"#,
        ).unwrap();

        let status = check_hooks_config(dir.path(), 9100, "abc");
        assert!(matches!(status, HookConfigStatus::EndpointMismatch { .. }));
    }

    #[test]
    fn test_check_content_invalid_json_returns_none() {
        let result = check_content("not json at all", 9100, "abc");
        assert!(result.is_none());
    }

    #[test]
    fn test_check_content_no_hooks_key_returns_none() {
        let result = check_content(r#"{"other": "value"}"#, 9100, "abc");
        assert!(result.is_none());
    }

    #[test]
    fn test_check_content_127_address() {
        let content = r#"{"hooks":{"SubagentStart":[{"matcher":".*","hooks":[{"type":"http","url":"http://127.0.0.1:9100/hook?token=abc"}]}]}}"#;
        let result = check_content(content, 9100, "abc");
        assert_eq!(result, Some(HookConfigStatus::Configured));
    }

    #[test]
    fn test_generate_hooks_snippet() {
        let snippet = generate_hooks_snippet(9100, "mytoken123");
        assert!(snippet.contains("localhost:9100"));
        assert!(snippet.contains("mytoken123"));
        assert!(snippet.contains("SubagentStart"));
        assert!(snippet.contains("SubagentStop"));
        assert!(snippet.contains("Stop"));
    }
}
