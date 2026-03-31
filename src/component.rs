use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::action::Action;
use crate::event::AppEvent;

/// UI 패널이 구현하는 공통 trait.
///
/// nexttui Component trait 기반. `render`에 `focused` 매개변수 추가.
pub trait Component {
    /// 키 입력 처리. Action을 반환하면 App이 처리.
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action>;

    /// 백그라운드 이벤트 처리. 패널 내부 상태 업데이트.
    fn handle_event(&mut self, event: &AppEvent);

    /// 패널 렌더링. focused=true면 Cyan 테두리.
    fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool);
}
