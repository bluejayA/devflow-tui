use std::io::{self, stdout};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::{mpsc, watch};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use devflow_tui::adapter::file_watcher;
use devflow_tui::adapter::git_poller;
use devflow_tui::adapter::handle::AdapterHandle;
use devflow_tui::adapter::hooks_server;
use devflow_tui::app::App;
use devflow_tui::config::AppConfig;
use devflow_tui::event::AppEvent;
use devflow_tui::parser::models::{FlowState, GitSnapshot};
use devflow_tui::service::token;

/// RAII guard that restores terminal state on drop.
struct TerminalGuard;

impl TerminalGuard {
    fn new() -> devflow_tui::error::Result<(Self, Terminal<CrosstermBackend<io::Stdout>>)> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok((Self, terminal))
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

#[tokio::main]
async fn main() -> devflow_tui::error::Result<()> {
    // Parse config
    let args: Vec<String> = std::env::args().collect();
    let config = AppConfig::from_args(&args)?;

    // Initialize tracing — keep guard alive for log flushing on exit
    let _log_guard = init_tracing(&config);

    // Install panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Setup terminal with RAII guard — cleanup guaranteed even on early error
    let (_guard, mut terminal) = TerminalGuard::new()?;
    tracing::info!(
        "devflow-tui started (port: {}, demo: {})",
        config.port,
        config.demo
    );

    // Generate/load token
    let tok = token::get_or_create_token(&config.project_dir, config.regenerate_token)?;

    // Create channels
    let (event_tx, event_rx) = mpsc::channel::<AppEvent>(256);
    let (flow_state_tx, flow_state_rx) = watch::channel(FlowState::default());
    let (git_snapshot_tx, git_snapshot_rx) = watch::channel(GitSnapshot::default());

    // Get terminal size for App init
    let size = terminal.size()?;

    // Create App
    let mut app = App::new(
        size.width,
        size.height,
        event_tx.clone(),
        tok.clone(),
        config.project_dir.clone(),
    );

    if config.demo {
        devflow_tui::demo::populate_demo_data(&mut app);
    }

    // Spawn adapters
    let mut adapters = Vec::new();

    if !config.demo {
        let project_dir = config.project_dir.clone();
        let fw_flow_tx = flow_state_tx;
        let fw_event_tx = event_tx.clone();
        adapters.push(AdapterHandle::spawn("file_watcher", |cancel| async move {
            file_watcher::run(cancel, project_dir, fw_flow_tx, fw_event_tx).await
        }));

        let project_dir = config.project_dir.clone();
        let gp_git_tx = git_snapshot_tx;
        let gp_event_tx = event_tx.clone();
        adapters.push(AdapterHandle::spawn("git_poller", |cancel| async move {
            git_poller::run(cancel, project_dir, gp_git_tx, gp_event_tx).await
        }));

        let hs_event_tx = event_tx.clone();
        let hs_token = tok;
        let hs_port = config.port;
        adapters.push(AdapterHandle::spawn("hooks_server", |cancel| async move {
            hooks_server::run(cancel, hs_port, hs_token, hs_event_tx).await
        }));
    }

    // Run event loop
    devflow_tui::event_loop::run_event_loop(
        &mut terminal,
        &mut app,
        event_rx,
        flow_state_rx,
        git_snapshot_rx,
        adapters,
    )
    .await?;

    tracing::info!("devflow-tui exited cleanly");
    // _guard drops here → terminal restored
    // _log_guard drops here → logs flushed

    Ok(())
}

fn init_tracing(config: &AppConfig) -> WorkerGuard {
    let log_dir = dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("devflow-tui");

    let file_appender = rolling::daily(&log_dir, "devflow-tui.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_env("DEVFLOW_TUI_LOG")
        .unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    guard
}
