#![cfg(test)]

use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::Terminal;

use crate::component::Component;

/// Component 트레이트 구현체를 TestBackend에 렌더링하고 Terminal을 반환한다.
#[allow(clippy::unwrap_used)]
pub fn render_component(
    component: &mut dyn Component,
    width: u16,
    height: u16,
    focused: bool,
) -> Terminal<TestBackend> {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            let area = frame.area();
            component.render(frame, area, focused);
        })
        .unwrap();
    terminal
}

/// 자유 함수(header::render 등)를 클로저로 받아 TestBackend에 렌더링한다.
#[allow(clippy::unwrap_used)]
pub fn render_with<F>(width: u16, height: u16, f: F) -> Terminal<TestBackend>
where
    F: FnOnce(&mut ratatui::Frame, Rect),
{
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            let area = frame.area();
            f(frame, area);
        })
        .unwrap();
    terminal
}

/// 버퍼의 전체 텍스트에서 needle 문자열이 포함되어 있는지 확인한다.
pub fn buffer_contains_str(buf: &Buffer, needle: &str) -> bool {
    let full_text = buffer_full_text(buf);
    full_text.contains(needle)
}

/// 특정 행의 텍스트를 추출한다 (trailing 공백 제거, wide char 패딩 스킵).
pub fn buffer_row_text(buf: &Buffer, row: u16) -> String {
    let area = buf.area();
    let mut text = String::new();
    let mut col = area.x;
    while col < area.x + area.width {
        let cell = &buf[(col, row)];
        let sym = cell.symbol();
        text.push_str(sym);
        col += display_width_of_first_char(sym);
    }
    text.trim_end().to_string()
}

/// 첫 번째 문자의 display width를 반환한다 (CJK/한글 = 2, 나머지 = 1).
fn display_width_of_first_char(s: &str) -> u16 {
    s.chars().next().map_or(1, |c| {
        if ('\u{1100}'..='\u{115F}').contains(&c)
            || ('\u{2E80}'..='\u{A4CF}').contains(&c)
            || ('\u{AC00}'..='\u{D7A3}').contains(&c)
            || ('\u{F900}'..='\u{FAFF}').contains(&c)
            || ('\u{FE10}'..='\u{FE6F}').contains(&c)
            || ('\u{FF01}'..='\u{FF60}').contains(&c)
            || ('\u{FFE0}'..='\u{FFE6}').contains(&c)
            || c > '\u{1F000}'
        {
            2
        } else {
            1
        }
    })
}

/// 버퍼 전체 텍스트를 한 문자열로 합친다.
/// Wide character 뒤의 패딩 셀을 건너뛰어 연속된 텍스트로 만든다.
fn buffer_full_text(buf: &Buffer) -> String {
    let area = buf.area();
    let mut text = String::new();
    for row in area.y..area.y + area.height {
        let mut col = area.x;
        while col < area.x + area.width {
            let cell = &buf[(col, row)];
            let sym = cell.symbol();
            text.push_str(sym);
            col += display_width_of_first_char(sym);
        }
    }
    text
}
