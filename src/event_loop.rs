use std::collections::HashSet;
use std::io;
use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyEventKind};
use futures::StreamExt;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tokio::sync::{mpsc, watch};

use crate::adapter::handle::AdapterHandle;
use crate::app::App;
use crate::error::Result;
use crate::event::AppEvent;
use crate::parser::models::{FlowState, GitSnapshot};

const TICK_RATE_MS: u64 = 250;

/// Run the main event loop.
///
/// Integrates keyboard input, adapter events, and periodic ticks
/// into a single tokio::select! loop with conditional rendering.
pub async fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    mut event_rx: mpsc::Receiver<AppEvent>,
    mut flow_state_rx: watch::Receiver<FlowState>,
    mut git_snapshot_rx: watch::Receiver<GitSnapshot>,
    adapter_handles: Vec<AdapterHandle>,
) -> Result<()> {
    let mut key_events = EventStream::new();
    let mut tick = tokio::time::interval(Duration::from_millis(TICK_RATE_MS));
    let mut needs_render = true;
    let mut warned_adapters: HashSet<&str> = HashSet::new();

    loop {
        tokio::select! {
            // Branch 1: Keyboard + resize events
            maybe_event = key_events.next() => {
                if let Some(Ok(event)) = maybe_event {
                    match event {
                        Event::Key(key) if key.kind == KeyEventKind::Press => {
                            needs_render = app.handle_key(key);
                        }
                        Event::Resize(w, h) => {
                            app.on_resize(w, h);
                            needs_render = true;
                        }
                        _ => {}
                    }
                }
            }

            // Branch 2: Periodic tick (250ms)
            _ = tick.tick() => {
                if app.on_tick() {
                    needs_render = true;
                }
            }

            // Branch 3: Flow state changes (watch channel)
            Ok(()) = flow_state_rx.changed() => {
                let state = flow_state_rx.borrow_and_update().clone();
                app.handle_flow_state(state);
                needs_render = true;
            }

            // Branch 4: Git snapshot changes (watch channel)
            Ok(()) = git_snapshot_rx.changed() => {
                let snapshot = git_snapshot_rx.borrow_and_update().clone();
                app.handle_git_snapshot(snapshot);
                needs_render = true;
            }

            // Branch 5: Discrete events (mpsc channel)
            event = event_rx.recv() => {
                match event {
                    Some(ev) => {
                        app.handle_event(ev);
                        needs_render = true;
                    }
                    None => {
                        // All senders dropped — shut down
                        app.should_quit = true;
                    }
                }
            }
        }

        // Adapter supervisor: detect crashed adapters (warn once per adapter)
        for handle in &adapter_handles {
            if handle.is_finished() && warned_adapters.insert(handle.name()) {
                tracing::warn!("Adapter '{}' finished unexpectedly", handle.name());
            }
        }

        // Conditional render
        if needs_render {
            terminal.draw(|frame| app.render(frame))?;
            needs_render = false;
        }

        if app.should_quit {
            break;
        }
    }

    // Graceful shutdown
    for handle in adapter_handles {
        handle.shutdown().await;
    }

    Ok(())
}
