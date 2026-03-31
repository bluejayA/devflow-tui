#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum Action {
    // Navigation
    FocusNextPanel,
    FocusPrevPanel,
    FocusDirection(Direction),
    ExpandPanel,
    CollapsePanel,
    OpenArtifactModal,

    // Panel interaction
    ScrollUp,
    ScrollDown,
    Select,

    // Async (delegated to CommandRunner)
    Refresh,
    CopyToClipboard(String),

    // System
    Quit,
}

impl Action {
    pub fn name(&self) -> &'static str {
        match self {
            Self::FocusNextPanel => "focus_next_panel",
            Self::FocusPrevPanel => "focus_prev_panel",
            Self::FocusDirection(_) => "focus_direction",
            Self::ExpandPanel => "expand_panel",
            Self::CollapsePanel => "collapse_panel",
            Self::OpenArtifactModal => "open_artifact_modal",
            Self::ScrollUp => "scroll_up",
            Self::ScrollDown => "scroll_down",
            Self::Select => "select",
            Self::Refresh => "refresh",
            Self::CopyToClipboard(_) => "copy_to_clipboard",
            Self::Quit => "quit",
        }
    }

    pub fn is_async(&self) -> bool {
        matches!(self, Self::Refresh | Self::CopyToClipboard(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_name() {
        assert_eq!(Action::FocusNextPanel.name(), "focus_next_panel");
        assert_eq!(Action::Quit.name(), "quit");
        assert_eq!(Action::CopyToClipboard("x".into()).name(), "copy_to_clipboard");
        assert_eq!(Action::FocusDirection(Direction::Up).name(), "focus_direction");
    }

    #[test]
    fn test_action_is_async() {
        assert!(Action::Refresh.is_async());
        assert!(Action::CopyToClipboard("x".into()).is_async());
        assert!(!Action::Quit.is_async());
        assert!(!Action::ScrollUp.is_async());
    }
}
