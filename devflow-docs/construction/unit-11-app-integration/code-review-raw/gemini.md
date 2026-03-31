Loaded cached credentials.
I have analyzed the provided code for Unit 11 (App Integration) of `devflow-tui`. To provide a comprehensive security and architectural review, I will first examine the underlying implementations of the token service and the hooks server, as these are critical areas for security and resource management.

### Intent
I will investigate the token management, hooks server implementation, and configuration handling to identify potential security vulnerabilities, race conditions, or resource leaks.

---

I will now check the `strip_ansi` implementation, the `file_watcher` adapter for resource management, and verify if the token file is excluded from version control.

---

I will examine `src/service/hook_config.rs` to verify how the token is included in the generated snippets and used in the configuration check, ensuring there are no accidental exposures or mismanagements of sensitive data.

---

This is a security and edge-case review for the **Unit 11: App Integration** phase of `devflow-tui`.

The review covers the integration of the HTTP hooks server, file watcher, git poller, and the main event loop.

---

### **Summary of Findings**
The overall architecture is robust, utilizing `tokio` for safe concurrency and `axum` for a secure-by-default local HTTP server. However, there are critical concerns regarding **token predictability** and **file system permissions** that could lead to unauthorized local access in multi-user environments.

---

### **1. Security Vulnerabilities**

#### **[CRITICAL] Deterministic Token Predictability**
*   **Location:** `src/service/token.rs` (`generate_token`)
*   **Risk:** The authentication token is generated using `SHA-256(absolute_project_path + salt)`. Since the salt is hardcoded in the source and absolute paths are often predictable (e.g., `/Users/username/projects/repo`), an attacker with local user access can pre-calculate the token.
*   **Impact:** A malicious process on the same machine could spoof hooks and inject fake agent statuses or turn completions into the TUI.
*   **Recommendation:** Use a cryptographically secure random number generator (CSPRNG) to generate a unique token on the first run, rather than a deterministic one.

#### **[HIGH] Insecure Token File Permissions**
*   **Location:** `src/service/token.rs` (`write_token`)
*   **Risk:** The token is written to `devflow-docs/.tui-token` using `std::fs::write`, which typically defaults to `644` (readable by everyone) on Unix systems.
*   **Impact:** Any user on a shared machine can read the token and interact with the `hooks_server`.
*   **Recommendation:** Use `std::os::unix::fs::PermissionsExt` to set file permissions to `600` immediately after creation.

#### **[MEDIUM] Sensitive Data in Clipboard**
*   **Location:** `src/app.rs` (`copy_hooks_snippet`)
*   **Risk:** Pressing the `c` key copies a JSON snippet containing the authentication token to the system clipboard.
*   **Impact:** The secret token may be persisted in clipboard history managers or leaked to other applications monitoring the clipboard.
*   **Recommendation:** Provide a clear warning in the UI that the clipboard now contains a secret token.

---

### **2. Race Conditions & Concurrency**

#### **[MEDIUM] Potential for Event Coalescing/Loss**
*   **Location:** `src/adapter/file_watcher.rs`
*   **Risk:** The `notify` callback uses `try_send` to an MPSC channel with a buffer of 64. If many files change simultaneously (e.g., a large `git checkout`), the channel may fill up.
*   **Impact:** While the code logs a warning, some file change events might be dropped. However, the subsequent debounce-triggered `parse_and_send` scans the entire directory, which mitigates the risk of stale state but adds disk I/O pressure.
*   **Recommendation:** Increase the buffer size or ensure `parse_and_send` is strictly called after any dropped event.

---

### **3. Resource Management & Performance**

#### **[MEDIUM] Recursive Artifact Scanning Scalability**
*   **Location:** `src/adapter/file_watcher.rs` (`scan_artifacts`)
*   **Risk:** `scan_artifacts` performs a recursive directory walk of `inception/` and `construction/` on every file change (after debounce).
*   **Impact:** If the project accumulates thousands of markdown files (artifacts), the UI may stutter during the re-scan.
*   **Recommendation:** Limit the depth of the scan or use a more efficient incremental update strategy if the number of files grows large.

---

### **4. Panic Paths**

#### **[MEDIUM] Potential Panic in Focus Logic**
*   **Location:** `src/app.rs` (`ensure_valid_focus`)
*   **Risk:** `self.focus = available[0];`
*   **Impact:** If `available_panels()` ever returns an empty vector (e.g., during a future refactor or conditional UI), the app will panic immediately.
*   **Recommendation:** Use `.get(0).copied().unwrap_or(FocusPane::WorkflowMap)` to provide a safe fallback.

---

### **5. Input Validation & Edge Cases**

#### **[LOW] Terminal Resize Scrolling**
*   **Location:** `src/app.rs` (`on_resize`)
*   **Observation:** Resizing the terminal forcefully resets `workflow_map.scroll_offset` to 0.
*   **Impact:** This results in a poor UX if a user is inspecting the bottom of a long list and accidentally resizes their window.
*   **Recommendation:** Attempt to clamp the existing scroll offset instead of resetting it to zero.

#### **[GOOD] Input Sanitization**
*   **Location:** `src/service/sanitizer.rs`
*   **Comment:** The `strip_ansi` implementation is excellent. It correctly handles both CSI and OSC sequences, preventing ANSI injection attacks that could mess up terminal state or hide malicious content.

#### **[GOOD] Network Isolation**
*   **Location:** `src/adapter/hooks_server.rs`
*   **Comment:** Binding to `127.0.0.1` and enforcing a 64KB body limit are strong security defaults for a local-only tool.
