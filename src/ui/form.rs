use crate::models::Invoice;
use crate::utils::generate_unique_id;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};
use std::collections::HashSet;
use std::fmt;

#[derive(Clone)]
pub struct InvoiceForm {
    pub number: String,
    pub client: String,
    pub total: String,
    pub status: String,
    pub field_index: usize,
}

impl InvoiceForm {
    pub fn new() -> Self {
        let existing_ids = HashSet::from(["278532".to_string()]);
        Self {
            number: generate_unique_id(&existing_ids),
            client: String::new(),
            total: String::new(),
            status: String::from("Draft"),
            field_index: 0,
        }
    }

    pub fn from_invoice(inv: &Invoice) -> Self {
        Self {
            number: inv.number.clone(),
            client: inv.client.clone(),
            total: format!("{:.2}", inv.total),
            status: inv.status.clone(),
            field_index: 0,
        }
    }

    pub fn next_field(&mut self) {
        self.field_index = (self.field_index + 1) % 4;
    }

    pub fn update_field(&mut self, c: char) {
        match self.field_index {
            0 => self.number.push(c),
            1 => self.client.push(c),
            2 => self.total.push(c),
            3 => self.status.push(c),
            _ => {}
        }
    }

    pub fn backspace(&mut self) {
        match self.field_index {
            0 => {
                self.number.pop();
            }
            1 => {
                self.client.pop();
            }
            2 => {
                self.total.pop();
            }
            3 => {
                self.status.pop();
            }
            _ => {}
        };
    }

    pub fn to_invoice(&self, id: u32) -> Option<Invoice> {
        let total = self.total.parse::<f64>().ok()?;
        Some(Invoice::new(
            id,
            &self.number,
            &self.client,
            total,
            &self.status,
        ))
    }
}

impl fmt::Display for InvoiceForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Number: {}\nClient: {}\nTotal: {}\nStatus: {}",
            self.number, self.client, self.total, self.status
        )
    }
}

pub fn draw_form(frame: &mut Frame, form: &InvoiceForm) {
    let size = frame.area();
    let popup_area = centered_rect(60, 40, size);

    frame.render_widget(Clear, popup_area);

    let title = format!(
        "Editing Field {} of 4 (Tab to switch, Enter to save, Esc to cancel)",
        form.field_index + 1
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().fg(Color::White).bg(Color::Cyan));

    let text = format!("{}", form);
    let paragraph = Paragraph::new(text).alignment(Alignment::Left).block(block);

    frame.render_widget(paragraph, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    vertical[1]
}
