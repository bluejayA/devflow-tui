use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};

const TOKEN_LEN: usize = 32; // 32 bytes → 64 hex chars
const TOKEN_FILE: &str = ".tui-token";

/// Get or create a CSPRNG token for the given project directory.
///
/// If a token file exists and `regenerate` is false, returns the existing token.
/// Otherwise generates a new cryptographically random token.
pub fn get_or_create_token(project_dir: &Path, regenerate: bool) -> Result<String> {
    let token_path = project_dir.join("devflow-docs").join(TOKEN_FILE);

    if !regenerate
        && let Ok(existing) = std::fs::read_to_string(&token_path)
    {
        let trimmed = existing.trim().to_string();
        if !trimmed.is_empty() {
            return Ok(trimmed);
        }
    }

    let token = generate_token()?;
    write_token(&token_path, &token)?;

    if regenerate {
        tracing::warn!(
            "Token regenerated — hooks in .claude/settings.json must be updated with the new token"
        );
    }

    Ok(token)
}

/// Generate a cryptographically random token.
///
/// Uses the OS CSPRNG via `getrandom` for unpredictable tokens.
/// Returns a 64-character hex string (32 random bytes).
pub fn generate_token() -> Result<String> {
    let mut buf = [0u8; TOKEN_LEN];
    getrandom::fill(&mut buf).map_err(|e| AppError::TokenGeneration(e.to_string()))?;

    let mut hex = String::with_capacity(TOKEN_LEN * 2);
    for byte in &buf {
        use std::fmt::Write;
        let _ = write!(hex, "{byte:02x}");
    }
    Ok(hex)
}

fn write_token(path: &PathBuf, token: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| AppError::TokenGeneration(e.to_string()))?;
    }

    // On Unix, create the file with 0600 permissions atomically to avoid
    // a TOCTOU window where the file is briefly world-readable.
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)
            .map_err(|e| AppError::TokenGeneration(e.to_string()))?;
        file.write_all(token.as_bytes())
            .map_err(|e| AppError::TokenGeneration(e.to_string()))?;
    }

    #[cfg(not(unix))]
    {
        std::fs::write(path, token).map_err(|e| AppError::TokenGeneration(e.to_string()))?;
    }

    Ok(())
}

/// Validate a token against the expected value.
pub fn validate_token(expected: &str, provided: &str) -> bool {
    // Constant-time comparison to prevent timing attacks
    if expected.len() != provided.len() {
        return false;
    }
    expected
        .bytes()
        .zip(provided.bytes())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b))
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_generate_token_length() {
        let token = generate_token().unwrap();
        assert_eq!(token.len(), 64); // 32 bytes → 64 hex chars
    }

    #[test]
    fn test_generate_token_is_hex() {
        let token = generate_token().unwrap();
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_token_is_random() {
        let token1 = generate_token().unwrap();
        let token2 = generate_token().unwrap();
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_get_or_create_persists() {
        let dir = TempDir::new().unwrap();
        let devflow_dir = dir.path().join("devflow-docs");
        std::fs::create_dir_all(&devflow_dir).unwrap();

        let token1 = get_or_create_token(dir.path(), false).unwrap();
        let token2 = get_or_create_token(dir.path(), false).unwrap();
        assert_eq!(token1, token2); // Reads from file
    }

    #[test]
    fn test_get_or_create_regenerate_produces_new_token() {
        let dir = TempDir::new().unwrap();
        let devflow_dir = dir.path().join("devflow-docs");
        std::fs::create_dir_all(&devflow_dir).unwrap();

        let token1 = get_or_create_token(dir.path(), false).unwrap();
        let token2 = get_or_create_token(dir.path(), true).unwrap();
        assert_ne!(token1, token2);
    }

    #[cfg(unix)]
    #[test]
    fn test_token_file_permissions_0600() {
        use std::os::unix::fs::PermissionsExt;

        let dir = TempDir::new().unwrap();
        let devflow_dir = dir.path().join("devflow-docs");
        std::fs::create_dir_all(&devflow_dir).unwrap();

        get_or_create_token(dir.path(), false).unwrap();

        let token_path = devflow_dir.join(".tui-token");
        let metadata = std::fs::metadata(&token_path).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }

    #[test]
    fn test_validate_token() {
        assert!(validate_token("abc123", "abc123"));
        assert!(!validate_token("abc123", "abc124"));
        assert!(!validate_token("abc123", "abc12"));
    }
}
