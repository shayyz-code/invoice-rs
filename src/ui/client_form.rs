use crate::models::Client;
use chrono::prelude::*;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};
use std::fmt;

#[derive(Clone)]
pub struct ClientForm {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub field_index: usize,
}

impl ClientForm {
    pub fn new(clients: Vec<Client>) -> Self {
        let now = Local::now();
        let today = now.date_naive();
        Self {
            name: String::new(),
            email: String::new(),
            phone: String::new(),
            address: String::new(),
            field_index: 0,
        }
    }

    pub fn from_client(client: &Client) -> Self {
        Self {
            name: client.name.clone(),
            email: client.email.clone(),
            phone: client.phone.clone(),
            address: client.address.clone(),
            field_index: 0,
        }
    }

    pub fn next_field(&mut self) {
        self.field_index = (self.field_index + 1) % 4;
    }

    pub fn update_field(&mut self, c: char) {
        match self.field_index {
            0 => self.name.push(c),
            1 => self.email.push(c),
            2 => self.phone.push(c),
            3 => self.address.push(c),
            _ => {}
        }
    }

    pub fn backspace(&mut self) {
        match self.field_index {
            0 => {
                self.name.pop();
            }
            1 => {
                self.email.pop();
            }
            2 => {
                self.phone.pop();
            }
            3 => {
                self.address.pop();
            }
            _ => {}
        };
    }

    pub fn to_client(&self, id: u32) -> Option<Client> {
        Some(Client::new(
            id,
            &self.name,
            &self.email,
            &self.phone,
            &self.address,
        ))
    }
}

impl fmt::Display for ClientForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nEmail: {}\nPhone: {}\nAddress: {}",
            self.name, self.email, self.phone, self.address
        )
    }
}

pub fn draw_form(frame: &mut Frame, form: &ClientForm) {
    let size = frame.area();
    let popup_area = centered_rect(70, 60, size);

    frame.render_widget(Clear, popup_area);

    let title = format!(
        "Editing Field {} of 4 (Tab to switch, Enter to save, Esc to cancel)",
        form.field_index + 1
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().fg(Color::White).bg(Color::Black));

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
