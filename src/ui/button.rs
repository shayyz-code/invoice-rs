use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::StatefulWidget};

use ratatui::style::Color;

pub struct Button {
    pub label: String,
    pub is_focused: bool,
    pub style: Style,
    pub focused_style: Option<Style>,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            is_focused: false,
            style: Style::default().fg(Color::White).bg(Color::Black),
            focused_style: Some(Style::default().fg(Color::White).bg(Color::Cyan)),
        }
    }

    pub fn focus(&mut self) {
        self.is_focused = true;
    }

    pub fn unfocus(&mut self) {
        self.is_focused = false;
    }
}

impl StatefulWidget for Button {
    type State = String;

    fn render(self, area: Rect, buf: &mut Buffer, _: &mut Self::State) {
        let style = if self.is_focused {
            self.focused_style
                .unwrap_or_else(|| Style::default().fg(Color::Blue))
        } else {
            self.style
        };

        let text = self.label.clone();

        // Compute centered text position
        let text_x = area.x + (area.width.saturating_sub(text.len() as u16)) / 2;
        let text_y = area.y + area.height / 2;

        // Fill background first (optional, but looks clean)
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_style(style);
                }
            }
        }

        // Draw label centered
        buf.set_string(text_x, text_y, text, style);
    }
}
