use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::error::{AppError, Result};

const SALT: &str = "devflow-tui-v1";
const TOKEN_FILE: &str = ".tui-token";

/// Get or create a stable token for the given project directory.
///
/// Token is deterministic: SHA-256(project_dir + salt).
/// Stored in devflow-docs/.tui-token for persistence.
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

    let token = generate_token(project_dir);
    write_token(&token_path, &token)?;
    Ok(token)
}

/// Generate a deterministic token from project directory path.
///
/// Uses absolute path (not canonicalize) to avoid flakiness with symlinks,
/// Docker mounts, or network filesystems where canonicalize may fail or
/// return inconsistent results.
pub fn generate_token(project_dir: &Path) -> String {
    // Prefer absolute path over canonicalize for stability
    let abs_path = if project_dir.is_absolute() {
        project_dir.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(project_dir))
            .unwrap_or_else(|_| project_dir.to_path_buf())
    };

    let mut hasher = Sha256::new();
    hasher.update(abs_path.to_string_lossy().as_bytes());
    hasher.update(SALT.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn write_token(path: &PathBuf, token: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| AppError::TokenGeneration(e.to_string()))?;
    }
    std::fs::write(path, token).map_err(|e| AppError::TokenGeneration(e.to_string()))?;
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
    fn test_same_dir_same_token() {
        let dir = TempDir::new().unwrap();
        let token1 = generate_token(dir.path());
        let token2 = generate_token(dir.path());
        assert_eq!(token1, token2);
        assert_eq!(token1.len(), 64); // SHA-256 hex = 64 chars
    }

    #[test]
    fn test_different_dir_different_token() {
        let dir1 = TempDir::new().unwrap();
        let dir2 = TempDir::new().unwrap();
        let token1 = generate_token(dir1.path());
        let token2 = generate_token(dir2.path());
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_get_or_create_and_persist() {
        let dir = TempDir::new().unwrap();
        let devflow_dir = dir.path().join("devflow-docs");
        std::fs::create_dir_all(&devflow_dir).unwrap();

        let token1 = get_or_create_token(dir.path(), false).unwrap();
        let token2 = get_or_create_token(dir.path(), false).unwrap();
        assert_eq!(token1, token2); // Reads from file

        // Regenerate
        let token3 = get_or_create_token(dir.path(), true).unwrap();
        assert_eq!(token1, token3); // Same dir → same hash
    }

    #[test]
    fn test_validate_token() {
        assert!(validate_token("abc123", "abc123"));
        assert!(!validate_token("abc123", "abc124"));
        assert!(!validate_token("abc123", "abc12"));
    }
}
