use crate::models::{Client, Invoice, Item};
use crate::utils::generate_unique_id;
use chrono::prelude::*;
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
    pub code: String,
    pub client_email: String,
    pub total: String,
    pub currency: String,
    pub discount: String,
    pub tax: String,
    pub status: String,
    pub date: String,
    pub items: Vec<Item>,
    pub field_index: usize,
    pub clients: Vec<Client>,
}

impl InvoiceForm {
    pub fn new(clients: Vec<Client>) -> Self {
        let existing_ids = HashSet::from(["278532".to_string()]);
        let now = Local::now();
        let today = now.date_naive();
        Self {
            code: format!("INV-{}", generate_unique_id(&existing_ids)),
            client_email: String::new(),
            currency: String::from("USD"),
            total: String::new(),
            discount: String::from("0"),
            tax: String::from("0"),
            status: String::from("draft"),
            date: String::from(today.to_string()),
            items: Vec::new(),
            field_index: 0,
            clients,
        }
    }

    pub fn from_invoice(inv: &Invoice, clients: Vec<Client>) -> Self {
        Self {
            code: inv.code.clone(),
            client_email: inv.client.email.to_string(),
            currency: inv.currency.clone(),
            total: format!("{:.2}", inv.total),
            discount: format!("{:.2}", inv.discount),
            tax: format!("{:.2}", inv.tax),
            status: inv.status.clone(),
            date: inv.date.clone(),
            items: inv.items.clone(),
            field_index: 0,
            clients,
        }
    }

    pub fn next_field(&mut self) {
        self.field_index = (self.field_index + 1) % 8;
    }

    pub fn update_field(&mut self, c: char) {
        match self.field_index {
            0 => self.code.push(c),
            1 => self.client_email.push(c),
            2 => self.currency.push(c),
            3 => self.total.push(c),
            4 => self.discount.push(c),
            5 => self.tax.push(c),
            6 => self.status.push(c),
            7 => self.date.push(c),
            8 => {}
            _ => {}
        }
    }

    pub fn backspace(&mut self) {
        match self.field_index {
            0 => {
                self.code.pop();
            }
            1 => {
                self.client_email.pop();
            }
            2 => {
                self.currency.pop();
            }
            3 => {
                self.total.pop();
            }
            4 => {
                self.discount.pop();
            }
            5 => {
                self.tax.pop();
            }
            6 => {
                self.status.pop();
            }
            7 => {
                self.date.pop();
            }
            8 => {}
            _ => {}
        };
    }

    pub fn to_invoice(&self, id: u32) -> Option<Invoice> {
        let total = self.total.parse::<f64>().ok()?;
        let discount = self.discount.parse::<f64>().ok()?;
        let tax = self.tax.parse::<f64>().ok()?;
        let client_opt = self.clients.iter().find(|c| c.email == self.client_email);

        if let Some(client) = client_opt {
            Some(Invoice::new(
                id,
                &self.code,
                client,
                &self.currency,
                total,
                discount,
                tax,
                &self.status,
                &self.date,
                self.items.clone(),
            ))
        } else {
            None
        }
    }
}

impl fmt::Display for InvoiceForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Code: {}\nClient Email: {}\nCurrency: {}\nTotal: {}\nDiscount: {}\nTax: {}\nStatus: {}\nDate: {}\nItems:\n{}",
            self.code,
            self.client_email,
            self.currency,
            self.total,
            self.discount,
            self.tax,
            self.status,
            self.date,
            self.items
                .iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub fn draw_form(frame: &mut Frame, form: &InvoiceForm) {
    let size = frame.area();
    let popup_area = centered_rect(70, 60, size);

    frame.render_widget(Clear, popup_area);

    let title = format!(
        "Editing Field {} of 8 (Tab to switch, Enter to save, Esc to cancel)",
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
