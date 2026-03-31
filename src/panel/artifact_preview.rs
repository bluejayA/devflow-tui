use std::fs::File;
use std::io::{BufRead, BufReader};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

use crate::action::Action;
use crate::component::Component;
use crate::event::AppEvent;
use crate::parser::models::ArtifactFile;
use crate::service::sanitizer;
use crate::ui::theme::{self, Theme};

const MAX_PREVIEW_LINES: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePane {
    FileList,
    Preview,
}

pub struct ArtifactPreviewPanel {
    files: Vec<ArtifactFile>,
    selected: usize,
    preview: Option<String>,
    active_pane: ActivePane,
    scroll_offset: usize,
}

impl Default for ArtifactPreviewPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactPreviewPanel {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            selected: 0,
            preview: None,
            active_pane: ActivePane::FileList,
            scroll_offset: 0,
        }
    }

    pub fn files(&self) -> &[ArtifactFile] {
        &self.files
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn preview_content(&self) -> Option<&str> {
        self.preview.as_deref()
    }

    pub fn active_pane(&self) -> ActivePane {
        self.active_pane
    }

    pub fn preview_scroll(&self) -> usize {
        self.scroll_offset
    }

    /// Load selected file content using BufReader to avoid OOM on large files.
    fn load_selected_file(&mut self) {
        let Some(file) = self.files.get(self.selected) else {
            return;
        };
        match File::open(&file.path) {
            Ok(f) => {
                let reader = BufReader::new(f);
                let lines: Vec<String> = reader
                    .lines()
                    .take(MAX_PREVIEW_LINES)
                    .filter_map(|l| l.ok())
                    .collect();
                let joined = lines.join("\n");
                self.preview = Some(sanitizer::strip_ansi(&joined));
            }
            Err(e) => {
                self.preview = Some(format!("Error: {e}"));
            }
        }
    }
}

impl Component for ArtifactPreviewPanel {
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Tab => {
                self.active_pane = match self.active_pane {
                    ActivePane::FileList => ActivePane::Preview,
                    ActivePane::Preview => ActivePane::FileList,
                };
                None
            }
            KeyCode::Char('j') | KeyCode::Down => {
                match self.active_pane {
                    ActivePane::FileList => {
                        if !self.files.is_empty() && self.selected < self.files.len() - 1 {
                            self.selected += 1;
                        }
                    }
                    ActivePane::Preview => {
                        self.scroll_offset = self.scroll_offset.saturating_add(1);
                    }
                }
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                match self.active_pane {
                    ActivePane::FileList => {
                        self.selected = self.selected.saturating_sub(1);
                    }
                    ActivePane::Preview => {
                        self.scroll_offset = self.scroll_offset.saturating_sub(1);
                    }
                }
                None
            }
            KeyCode::Enter => {
                self.load_selected_file();
                self.scroll_offset = 0;
                self.active_pane = ActivePane::Preview;
                None
            }
            _ => None,
        }
    }

    fn handle_event(&mut self, event: &AppEvent) {
        if let AppEvent::ArtifactListChanged(files) = event {
            self.files = files.clone();
            if self.files.is_empty() {
                self.selected = 0;
            } else if self.selected >= self.files.len() {
                self.selected = self.files.len() - 1;
            }
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(theme::panel_title("Artifacts", focused))
            .border_style(if focused {
                Theme::focus_border()
            } else {
                Theme::unfocus_border()
            });

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.preview.is_some() && inner.width >= 40 {
            // Split: file list (left) + preview (right)
            let chunks = Layout::horizontal([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(inner);

            self.render_file_list(frame, chunks[0]);
            self.render_preview(frame, chunks[1]);
        } else {
            self.render_file_list(frame, inner);
        }
    }
}

impl ArtifactPreviewPanel {
    fn render_file_list(&self, frame: &mut Frame, area: Rect) {
        let lines: Vec<ratatui::text::Line> = self
            .files
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let prefix = if i == self.selected { "▸ " } else { "  " };
                let style = if i == self.selected {
                    Theme::highlight()
                } else {
                    ratatui::style::Style::default()
                };
                ratatui::text::Line::styled(format!("{prefix}{}", f.name), style)
            })
            .collect();

        let list = Paragraph::new(lines);
        frame.render_widget(list, area);
    }

    fn render_preview(&self, frame: &mut Frame, area: Rect) {
        let content = self.preview.as_deref().unwrap_or("");
        let paragraph = Paragraph::new(content)
            .scroll((self.scroll_offset as u16, 0));
        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::test_helpers::{buffer_contains_str, render_component};

    #[test]
    fn test_new_panel_empty() {
        let panel = ArtifactPreviewPanel::new();
        assert!(panel.files().is_empty());
    }

    #[test]
    fn test_handle_artifact_list_event() {
        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![
            ArtifactFile {
                path: PathBuf::from("/tmp/a.md"),
                name: "a.md".to_string(),
            },
            ArtifactFile {
                path: PathBuf::from("/tmp/b.md"),
                name: "b.md".to_string(),
            },
        ];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        assert_eq!(panel.files().len(), 2);
        assert_eq!(panel.files()[0].name, "a.md");
    }

    #[test]
    fn test_render_file_list() {
        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![
            ArtifactFile {
                path: PathBuf::from("/tmp/requirements.md"),
                name: "requirements.md".to_string(),
            },
        ];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        let terminal = render_component(&mut panel, 40, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "requirements.md"));
    }

    #[test]
    fn test_scroll_file_list() {
        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![
            ArtifactFile { path: PathBuf::from("/tmp/a.md"), name: "a.md".to_string() },
            ArtifactFile { path: PathBuf::from("/tmp/b.md"), name: "b.md".to_string() },
            ArtifactFile { path: PathBuf::from("/tmp/c.md"), name: "c.md".to_string() },
        ];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        assert_eq!(panel.selected_index(), 0);

        panel.handle_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(panel.selected_index(), 1);

        panel.handle_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(panel.selected_index(), 2);

        // Should not go beyond last item
        panel.handle_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(panel.selected_index(), 2);

        panel.handle_key(KeyEvent::from(KeyCode::Char('k')));
        assert_eq!(panel.selected_index(), 1);
    }

    #[test]
    fn test_render_selection_indicator() {
        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![
            ArtifactFile { path: PathBuf::from("/tmp/a.md"), name: "a.md".to_string() },
            ArtifactFile { path: PathBuf::from("/tmp/b.md"), name: "b.md".to_string() },
        ];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));

        // First item selected — should show ▸ on a.md
        let terminal = render_component(&mut panel, 40, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "▸ a.md"));
        assert!(!buffer_contains_str(buf, "▸ b.md"));

        // Move selection down
        panel.handle_key(KeyEvent::from(KeyCode::Char('j')));
        let terminal = render_component(&mut panel, 40, 10, true);
        let buf = terminal.backend().buffer();
        assert!(!buffer_contains_str(buf, "▸ a.md"));
        assert!(buffer_contains_str(buf, "▸ b.md"));
    }

    #[test]
    fn test_select_file_loads_content() {
        use std::fs;
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("test.md");
        fs::write(&file_path, "# Hello\nWorld").unwrap();

        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![ArtifactFile {
            path: file_path,
            name: "test.md".to_string(),
        }];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        panel.handle_key(KeyEvent::from(KeyCode::Enter));

        assert_eq!(panel.preview_content(), Some("# Hello\nWorld"));
        // M1: Enter should auto-switch to Preview pane
        assert_eq!(panel.active_pane(), ActivePane::Preview);
    }

    #[test]
    fn test_select_file_read_error() {
        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![ArtifactFile {
            path: PathBuf::from("/nonexistent/file.md"),
            name: "file.md".to_string(),
        }];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        panel.handle_key(KeyEvent::from(KeyCode::Enter));

        let content = panel.preview_content().unwrap();
        assert!(content.contains("Error"));
    }

    #[test]
    fn test_render_error_in_preview() {
        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![ArtifactFile {
            path: PathBuf::from("/nonexistent/file.md"),
            name: "file.md".to_string(),
        }];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        panel.handle_key(KeyEvent::from(KeyCode::Enter));

        let terminal = render_component(&mut panel, 80, 15, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Error"));
    }

    #[test]
    fn test_render_preview_pane() {
        use std::fs;
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("doc.md");
        fs::write(&file_path, "# Preview Content").unwrap();

        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![ArtifactFile {
            path: file_path,
            name: "doc.md".to_string(),
        }];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        panel.handle_key(KeyEvent::from(KeyCode::Enter));

        // Wide enough for split view
        let terminal = render_component(&mut panel, 80, 15, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "doc.md"));
        assert!(buffer_contains_str(buf, "Preview Content"));
    }

    #[test]
    fn test_render_preview_scroll_applied() {
        use std::fs;
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("scroll.md");
        let content = (0..20).map(|i| format!("Line {i}")).collect::<Vec<_>>().join("\n");
        fs::write(&file_path, &content).unwrap();

        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![ArtifactFile {
            path: file_path,
            name: "scroll.md".to_string(),
        }];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        panel.handle_key(KeyEvent::from(KeyCode::Enter));

        // Scroll down 5 lines in preview pane (already in Preview after Enter)
        for _ in 0..5 {
            panel.handle_key(KeyEvent::from(KeyCode::Char('j')));
        }

        let terminal = render_component(&mut panel, 80, 15, true);
        let buf = terminal.backend().buffer();
        // After scrolling 5 lines, "Line 0" through "Line 4" should not be visible
        // but "Line 5" should be visible
        assert!(buffer_contains_str(buf, "Line 5"));
        assert!(!buffer_contains_str(buf, "Line 0"));
    }

    #[test]
    fn test_tab_switches_pane() {
        let mut panel = ArtifactPreviewPanel::new();
        assert_eq!(panel.active_pane(), ActivePane::FileList);

        panel.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(panel.active_pane(), ActivePane::Preview);

        panel.handle_key(KeyEvent::from(KeyCode::Tab));
        assert_eq!(panel.active_pane(), ActivePane::FileList);
    }

    #[test]
    fn test_preview_scroll() {
        use std::fs;
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("long.md");
        let content = (0..50).map(|i| format!("Line {i}")).collect::<Vec<_>>().join("\n");
        fs::write(&file_path, &content).unwrap();

        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![ArtifactFile {
            path: file_path,
            name: "long.md".to_string(),
        }];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        panel.handle_key(KeyEvent::from(KeyCode::Enter));

        // Enter auto-switches to Preview pane now
        assert_eq!(panel.active_pane(), ActivePane::Preview);
        assert_eq!(panel.preview_scroll(), 0);

        panel.handle_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(panel.preview_scroll(), 1);

        panel.handle_key(KeyEvent::from(KeyCode::Char('k')));
        assert_eq!(panel.preview_scroll(), 0);

        // Should not go below 0
        panel.handle_key(KeyEvent::from(KeyCode::Char('k')));
        assert_eq!(panel.preview_scroll(), 0);
    }

    #[test]
    fn test_artifact_list_update_clamps_selection() {
        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![
            ArtifactFile { path: PathBuf::from("/tmp/a.md"), name: "a.md".to_string() },
            ArtifactFile { path: PathBuf::from("/tmp/b.md"), name: "b.md".to_string() },
            ArtifactFile { path: PathBuf::from("/tmp/c.md"), name: "c.md".to_string() },
        ];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));

        // Select last item
        panel.handle_key(KeyEvent::from(KeyCode::Char('j')));
        panel.handle_key(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(panel.selected_index(), 2);

        // Update with fewer files — selection should clamp
        let fewer = vec![
            ArtifactFile { path: PathBuf::from("/tmp/a.md"), name: "a.md".to_string() },
        ];
        panel.handle_event(&AppEvent::ArtifactListChanged(fewer));
        assert_eq!(panel.selected_index(), 0);

        // Render should show ▸ on first (only) item
        let terminal = render_component(&mut panel, 40, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "▸ a.md"));
    }

    #[test]
    fn test_render_focused_vs_unfocused_border() {
        let mut panel = ArtifactPreviewPanel::new();

        // Focused: should show [ Artifacts ]
        let terminal = render_component(&mut panel, 40, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "[ Artifacts ]"));

        // Unfocused: should show   Artifacts
        let terminal = render_component(&mut panel, 40, 10, false);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "Artifacts"));
        assert!(!buffer_contains_str(buf, "[ Artifacts ]"));
    }

    #[test]
    fn test_render_narrow_shows_list_only() {
        use std::fs;
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("test.md");
        fs::write(&file_path, "# Content here").unwrap();

        let mut panel = ArtifactPreviewPanel::new();
        let files = vec![ArtifactFile {
            path: file_path,
            name: "test.md".to_string(),
        }];
        panel.handle_event(&AppEvent::ArtifactListChanged(files));
        panel.handle_key(KeyEvent::from(KeyCode::Enter));

        // Narrow width (inner < 40) — should show file list only, no split
        let terminal = render_component(&mut panel, 30, 10, true);
        let buf = terminal.backend().buffer();
        assert!(buffer_contains_str(buf, "test.md"));
        // Content should NOT appear in narrow mode (no split view)
        assert!(!buffer_contains_str(buf, "Content here"));
    }
}
