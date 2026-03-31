use ratatui::layout::{Constraint, Layout, Rect};

/// Layout mode determined by terminal size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    TooSmall,              // < 80x24
    Compact,               // 80x24 ~ 119x29
    Standard,              // 120x30 ~ 199x49
    Wide,                  // 200x50+
}

/// Computed layout areas for each UI section.
#[derive(Debug, Clone)]
pub struct LayoutAreas {
    pub header: Rect,
    pub body: Rect,
    pub status_bar: Rect,
}

/// Panel areas within the body, depending on layout mode.
#[derive(Debug, Clone)]
pub enum PanelAreas {
    TooSmall {
        message: Rect,
    },
    Compact {
        /// Single panel fills the body
        panel: Rect,
    },
    Standard {
        workflow_map: Rect,
        git_status: Rect,
        agent_status: Rect,
        audit_log: Rect,
    },
    Wide {
        workflow_map: Rect,
        git_status: Rect,
        artifacts: Rect,
        agent_status: Rect,
        audit_log: Rect,
        gate_alert: Rect,
    },
}

/// Manages layout calculations based on terminal size.
pub struct LayoutManager {
    width: u16,
    height: u16,
}

impl LayoutManager {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn on_resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn mode(&self) -> LayoutMode {
        if self.width < 80 || self.height < 24 {
            LayoutMode::TooSmall
        } else if self.width < 120 || self.height < 30 {
            LayoutMode::Compact
        } else if self.width < 200 || self.height < 50 {
            LayoutMode::Standard
        } else {
            LayoutMode::Wide
        }
    }

    /// Calculate the top-level layout areas (header, body, status bar).
    pub fn areas(&self, frame_size: Rect) -> LayoutAreas {
        let [header, body, status_bar] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame_size);

        LayoutAreas {
            header,
            body,
            status_bar,
        }
    }

    /// Calculate panel areas within the body based on current layout mode.
    pub fn panel_areas(&self, body: Rect) -> PanelAreas {
        match self.mode() {
            LayoutMode::TooSmall => PanelAreas::TooSmall { message: body },
            LayoutMode::Compact => PanelAreas::Compact { panel: body },
            LayoutMode::Standard => self.standard_panels(body),
            LayoutMode::Wide => self.wide_panels(body),
        }
    }

    fn standard_panels(&self, body: Rect) -> PanelAreas {
        // Left(40%) | Right(fill)
        let [left, right] = Layout::horizontal([
            Constraint::Percentage(40),
            Constraint::Fill(1),
        ])
        .areas(body);

        // Right: Top(50%) | Bottom(fill)
        let [right_top, right_bottom] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ])
        .areas(right);

        // Right bottom: Agent(40%) | Audit(fill)
        let [agent, audit] = Layout::horizontal([
            Constraint::Percentage(40),
            Constraint::Fill(1),
        ])
        .areas(right_bottom);

        PanelAreas::Standard {
            workflow_map: left,
            git_status: right_top,
            agent_status: agent,
            audit_log: audit,
        }
    }

    fn wide_panels(&self, body: Rect) -> PanelAreas {
        // Top row | Bottom row
        let [top_row, bottom_row] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ])
        .areas(body);

        // Top: 3 columns
        let [wf, git, art] = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Fill(1),
        ])
        .areas(top_row);

        // Bottom: 3 columns
        let [agent, audit, gate] = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Fill(1),
        ])
        .areas(bottom_row);

        PanelAreas::Wide {
            workflow_map: wf,
            git_status: git,
            artifacts: art,
            agent_status: agent,
            audit_log: audit,
            gate_alert: gate,
        }
    }
}

/// Centered rectangle for modals/overlays.
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let px = percent_x.min(100);
    let py = percent_y.min(100);

    let [_, center_v, _] = Layout::vertical([
        Constraint::Percentage((100 - py) / 2),
        Constraint::Percentage(py),
        Constraint::Percentage((100 - py) / 2),
    ])
    .areas(area);

    let [_, center, _] = Layout::horizontal([
        Constraint::Percentage((100 - px) / 2),
        Constraint::Percentage(px),
        Constraint::Percentage((100 - px) / 2),
    ])
    .areas(center_v);

    center
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_mode_too_small() {
        let lm = LayoutManager::new(79, 23);
        assert_eq!(lm.mode(), LayoutMode::TooSmall);
    }

    #[test]
    fn test_layout_mode_compact() {
        let lm = LayoutManager::new(100, 25);
        assert_eq!(lm.mode(), LayoutMode::Compact);
    }

    #[test]
    fn test_layout_mode_standard() {
        let lm = LayoutManager::new(150, 40);
        assert_eq!(lm.mode(), LayoutMode::Standard);
    }

    #[test]
    fn test_layout_mode_wide() {
        let lm = LayoutManager::new(220, 60);
        assert_eq!(lm.mode(), LayoutMode::Wide);
    }

    #[test]
    fn test_areas_has_three_sections() {
        let lm = LayoutManager::new(120, 30);
        let areas = lm.areas(Rect::new(0, 0, 120, 30));
        assert_eq!(areas.header.height, 1);
        assert_eq!(areas.status_bar.height, 1);
        assert!(areas.body.height > 0);
    }

    #[test]
    fn test_standard_panels() {
        let lm = LayoutManager::new(120, 30);
        let areas = lm.areas(Rect::new(0, 0, 120, 30));
        let panels = lm.panel_areas(areas.body);
        assert!(matches!(panels, PanelAreas::Standard { .. }));
    }

    #[test]
    fn test_wide_panels() {
        let lm = LayoutManager::new(200, 50);
        let areas = lm.areas(Rect::new(0, 0, 200, 50));
        let panels = lm.panel_areas(areas.body);
        assert!(matches!(panels, PanelAreas::Wide { .. }));
    }

    #[test]
    fn test_centered_rect() {
        let area = Rect::new(0, 0, 100, 50);
        let center = centered_rect(80, 60, area);
        assert!(center.x > 0);
        assert!(center.y > 0);
        assert!(center.width < 100);
        assert!(center.height < 50);
    }

    #[test]
    fn test_resize() {
        let mut lm = LayoutManager::new(80, 24);
        assert_eq!(lm.mode(), LayoutMode::Compact);
        lm.on_resize(200, 50);
        assert_eq!(lm.mode(), LayoutMode::Wide);
    }

    // ── Render tests ──

    use crate::test_helpers::render_with;

    #[test]
    fn render_areas_non_overlapping() {
        let terminal = render_with(120, 30, |frame, area| {
            let lm = LayoutManager::new(area.width, area.height);
            let areas = lm.areas(area);

            // Header and status_bar should not overlap body
            assert!(areas.header.y + areas.header.height <= areas.body.y);
            assert!(areas.body.y + areas.body.height <= areas.status_bar.y);

            // Render something to confirm no panic
            let block = ratatui::widgets::Block::bordered();
            frame.render_widget(block, areas.body);
        });
        let _buf = terminal.backend().buffer();
    }

    #[test]
    fn render_centered_rect_position() {
        let terminal = render_with(100, 50, |frame, area| {
            let center = centered_rect(60, 60, area);
            // Center should be within the area
            assert!(center.x >= area.x);
            assert!(center.y >= area.y);
            assert!(center.x + center.width <= area.x + area.width);
            assert!(center.y + center.height <= area.y + area.height);

            let block = ratatui::widgets::Block::bordered()
                .title("Modal");
            frame.render_widget(ratatui::widgets::Clear, center);
            frame.render_widget(block, center);
        });
        let buf = terminal.backend().buffer();
        assert!(crate::test_helpers::buffer_contains_str(buf, "Modal"));
    }

    #[test]
    fn render_too_small_mode_uses_full_body() {
        let lm = LayoutManager::new(60, 20);
        let areas = lm.areas(Rect::new(0, 0, 60, 20));
        let panels = lm.panel_areas(areas.body);
        match panels {
            PanelAreas::TooSmall { message } => {
                assert_eq!(message, areas.body);
            }
            _ => panic!("Expected TooSmall layout"),
        }
    }
}
