use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::StatefulWidget,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};
use std::fmt;

use crate::{app::App, ui::button::Button};

#[derive(Clone, Copy)]
pub enum ModalType {
    Alert,
    Confirm,
}

impl fmt::Display for ModalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModalType::Alert => write!(f, "Alert"),
            ModalType::Confirm => write!(f, "Confirm"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ActiveFieldForConfirm {
    Confirm,
    Cancel,
}

pub struct Modal {
    pub invoice_id: Option<u32>,
    pub kind: ModalType,
    pub content: String,
    pub active: ActiveFieldForConfirm,
    pub action: Option<Box<dyn FnMut(&mut App)>>,
}

impl Modal {
    pub fn new(
        invoice_id: Option<u32>,
        content: String,
        kind: ModalType,
        action: Option<Box<dyn FnMut(&mut App)>>,
    ) -> Self {
        Self {
            invoice_id,
            kind,
            content,
            active: ActiveFieldForConfirm::Cancel,
            action,
        }
    }

    pub fn next_field(&mut self) {
        self.active = match self.active {
            ActiveFieldForConfirm::Confirm => ActiveFieldForConfirm::Cancel,
            ActiveFieldForConfirm::Cancel => ActiveFieldForConfirm::Confirm,
        };
    }

    pub fn press_field<F>(&mut self, mut callback_on_close: F, app: &mut App)
    where
        F: FnMut(&mut App),
    {
        match self.active {
            ActiveFieldForConfirm::Confirm => {
                if let Some(action) = self.action.as_deref_mut() {
                    action(app);
                }
            }
            _ => {}
        }
        callback_on_close(app);
    }
}

impl fmt::Display for Modal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(invoice_id) = &self.invoice_id {
            write!(
                f,
                "Invoice ID: {}\nKind: {}\nContent: {}",
                invoice_id, self.kind, self.content
            )
        } else {
            write!(f, "Kind: {}\nContent: {}", self.kind, self.content)
        }
    }
}

pub fn draw_modal(frame: &mut Frame, modal: &Modal) {
    let size = frame.area();
    let popup_area = centered_rect(50, 26, size);

    frame.render_widget(Clear, popup_area);

    let title = format!("{}", modal.kind);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().fg(Color::White).bg(Color::Black));

    frame.render_widget(block, popup_area);

    // Inner layout inside the popup
    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Min(3),    // space for content
            Constraint::Length(3), // space for buttons
        ])
        .split(popup_area);

    let text_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // top padding
            Constraint::Length(3),      // height for text
            Constraint::Percentage(40), // bottom padding
        ])
        .split(inner_layout[0]);

    let text = Paragraph::new(modal.content.clone()).alignment(Alignment::Center);

    frame.render_widget(text, text_area[1]);

    match modal.kind {
        ModalType::Alert => {
            let btn_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)])
                .split(inner_layout[1]);

            let ok_btn = Block::default()
                .borders(Borders::ALL)
                .title("OK")
                .style(Style::default().fg(Color::White).bg(Color::Green));

            frame.render_widget(ok_btn, btn_layout[0]);
        }

        ModalType::Confirm => {
            let btn_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(inner_layout[1]);

            let mut cancel_btn = Button::new("Cancel");

            let mut confirm_btn = Button::new("Confirm");

            // Focus toggle example (you can tie this to modal.field_index)
            if modal.active == ActiveFieldForConfirm::Confirm {
                confirm_btn.focus();
                cancel_btn.unfocus();
            } else {
                confirm_btn.unfocus();
                cancel_btn.focus();
            }

            // Draw buttons
            cancel_btn.render(btn_layout[0], frame.buffer_mut(), &mut String::new());
            confirm_btn.render(btn_layout[1], frame.buffer_mut(), &mut String::new());
        }
    }
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
