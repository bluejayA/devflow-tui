use std::path::PathBuf;

use crate::error::{AppError, Result};

const DEFAULT_PORT: u16 = 9100;
const DEFAULT_LOG_BUFFER: usize = 1000;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub project_dir: PathBuf,
    pub demo: bool,
    pub regenerate_token: bool,
    pub log_level: String,
    pub log_buffer_size: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: DEFAULT_PORT,
            project_dir: PathBuf::from("."),
            demo: false,
            regenerate_token: false,
            log_level: "info".to_string(),
            log_buffer_size: DEFAULT_LOG_BUFFER,
        }
    }
}

impl AppConfig {
    pub fn from_args(args: &[String]) -> Result<Self> {
        let mut config = Self::default();

        // Apply environment variable overrides first
        if let Ok(port) = std::env::var("DEVFLOW_TUI_PORT") {
            config.port = port.parse::<u16>().map_err(|_| AppError::ConfigRead {
                path: PathBuf::from("DEVFLOW_TUI_PORT"),
                detail: format!("invalid port: {port}"),
            })?;
        }
        if let Ok(log) = std::env::var("DEVFLOW_TUI_LOG") {
            config.log_level = log;
        }
        if let Ok(buf) = std::env::var("DEVFLOW_TUI_LOG_BUFFER") {
            config.log_buffer_size = buf.parse::<usize>().map_err(|_| AppError::ConfigRead {
                path: PathBuf::from("DEVFLOW_TUI_LOG_BUFFER"),
                detail: format!("invalid buffer size: {buf}"),
            })?;
        }

        // Parse CLI args (override env vars)
        let mut i = 1; // skip binary name
        while i < args.len() {
            match args[i].as_str() {
                "--port" => {
                    i += 1;
                    let val = args.get(i).ok_or_else(|| AppError::ConfigRead {
                        path: PathBuf::from("--port"),
                        detail: "missing value".to_string(),
                    })?;
                    config.port = val.parse::<u16>().map_err(|_| AppError::ConfigRead {
                        path: PathBuf::from("--port"),
                        detail: format!("invalid port: {val}"),
                    })?;
                }
                "--project-dir" => {
                    i += 1;
                    let val = args.get(i).ok_or_else(|| AppError::ConfigRead {
                        path: PathBuf::from("--project-dir"),
                        detail: "missing value".to_string(),
                    })?;
                    config.project_dir = PathBuf::from(val);
                }
                "--demo" => {
                    config.demo = true;
                }
                "--regenerate-token" => {
                    config.regenerate_token = true;
                }
                _ => {
                    // Ignore unknown args
                }
            }
            i += 1;
        }

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.port == 0 || self.port < 1024 {
            return Err(AppError::ConfigRead {
                path: PathBuf::from("--port"),
                detail: format!("port must be 1024-65535, got {}", self.port),
            });
        }
        if self.log_buffer_size == 0 {
            return Err(AppError::ConfigRead {
                path: PathBuf::from("DEVFLOW_TUI_LOG_BUFFER"),
                detail: "log buffer size must be > 0".to_string(),
            });
        }
        Ok(())
    }

    pub fn devflow_docs_dir(&self) -> PathBuf {
        self.project_dir.join("devflow-docs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = AppConfig::default();
        assert_eq!(config.port, 9100);
        assert_eq!(config.project_dir, PathBuf::from("."));
        assert!(!config.demo);
        assert!(!config.regenerate_token);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.log_buffer_size, 1000);
    }

    #[test]
    fn test_config_cli_override() {
        let args = vec![
            "devflow-tui".to_string(),
            "--port".to_string(),
            "9200".to_string(),
            "--project-dir".to_string(),
            "/tmp/proj".to_string(),
            "--demo".to_string(),
        ];
        let config = AppConfig::from_args(&args).unwrap();
        assert_eq!(config.port, 9200);
        assert_eq!(config.project_dir, PathBuf::from("/tmp/proj"));
        assert!(config.demo);
    }

    #[test]
    fn test_config_missing_port_value() {
        let args = vec!["devflow-tui".to_string(), "--port".to_string()];
        let result = AppConfig::from_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_invalid_port() {
        let args = vec![
            "devflow-tui".to_string(),
            "--port".to_string(),
            "not_a_number".to_string(),
        ];
        let result = AppConfig::from_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_devflow_docs_dir() {
        let mut config = AppConfig::default();
        config.project_dir = PathBuf::from("/home/user/project");
        assert_eq!(
            config.devflow_docs_dir(),
            PathBuf::from("/home/user/project/devflow-docs")
        );
    }

    #[test]
    fn test_config_regenerate_token() {
        let args = vec![
            "devflow-tui".to_string(),
            "--regenerate-token".to_string(),
        ];
        let config = AppConfig::from_args(&args).unwrap();
        assert!(config.regenerate_token);
    }

    #[test]
    fn test_config_port_zero_rejected() {
        let args = vec![
            "devflow-tui".to_string(),
            "--port".to_string(),
            "0".to_string(),
        ];
        let result = AppConfig::from_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_port_low_rejected() {
        let args = vec![
            "devflow-tui".to_string(),
            "--port".to_string(),
            "80".to_string(),
        ];
        let result = AppConfig::from_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_unknown_args_ignored() {
        let args = vec![
            "devflow-tui".to_string(),
            "--unknown".to_string(),
            "value".to_string(),
        ];
        let config = AppConfig::from_args(&args).unwrap();
        assert_eq!(config.port, 9100); // default preserved
    }
}
