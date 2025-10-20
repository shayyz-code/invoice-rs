use crate::app::{App, Mode};
use crate::ui::form::draw_form;
use crate::ui::modal::draw_modal;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(size);

    let items: Vec<ListItem> = app
        .invoices
        .iter()
        .map(|inv| ListItem::new(format!("{} - {} ({})", inv.number, inv.client, inv.status)))
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Invoices"))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, chunks[0], &mut list_state);

    let inv = app.selected_invoice();
    let detail_text = format!(
        "Invoice No: {}\nClient: {}\nTotal: ${:.2}\nStatus: {}",
        inv.number, inv.client, inv.total, inv.status
    );

    let paragraph = Paragraph::new(detail_text)
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, chunks[1]);

    if let (Mode::Editing, Some(form)) = (&app.mode, &app.form) {
        draw_form(frame, form);
    }

    if let (Mode::Modal(_), Some(modal)) = (&app.mode, &app.modal) {
        draw_modal(frame, modal);
    }
}
