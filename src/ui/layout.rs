use crate::app::{App, Mode};
use crate::ui::invoice_form::draw_form;
use crate::ui::modal::draw_modal;
use ratatui::style::palette::tailwind;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Invoices")]
    TabInvoices,
    #[strum(to_string = "Clients")]
    TabClients,
}

impl SelectedTab {
    pub fn title(self) -> Line<'static> {
        format!("   {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render(self, app: &App, area: Rect, frame: &mut Frame) {
        match self {
            Self::TabInvoices => self.render_tab0(app, area, frame),
            Self::TabClients => self.render_tab1(app, area, frame),
        }
    }

    pub fn render_tab0(self, app: &App, area: Rect, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        let items: Vec<ListItem> = app
            .invoices
            .iter()
            .map(|inv| {
                ListItem::new(format!(
                    "{} - {} ({})",
                    inv.code, inv.client.name, inv.status
                ))
            })
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

        let detail_text = if inv.code.eq("empty_invoice") {
            "No invoice created yet.\nPress 'n' to create a new invoice.\nPress 'h' for help."
                .to_string()
        } else {
            format!(
                "Invoice: {}\nDate: {}\nStatus: {},\nClient:\n- Name: {}\n- Email: {}\n- Phone: {}\n- Address: {}\nTotal: ${:.2}\nDiscount: ${:.2}\nTax: ${:.2}\nNet Total: ${:.2}\nItems:\n{}",
                inv.code,
                inv.date,
                inv.status,
                inv.client.name,
                inv.client.email,
                inv.client.phone,
                inv.client.address,
                inv.total,
                inv.discount,
                inv.tax,
                inv.calculate_net_total(),
                inv.items
                    .iter()
                    .map(|item| format!("- {}", item))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        };

        let paragraph = Paragraph::new(detail_text)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, chunks[1]);
    }

    pub fn render_tab1(self, app: &App, area: Rect, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        let items: Vec<ListItem> = app
            .clients
            .iter()
            .map(|client| ListItem::new(format!("{} - {})", client.id, client.name)))
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

        let detail_text = if inv.code.eq("empty_invoice") {
            "No client added yet.\nPress 'n' to add your very first new client.\nPress 'h' for help."
                .to_string()
        } else {
            format!(
                "Invoice: {}\nDate: {}\nStatus: {},\nClient:\n- Name: {}\n- Email: {}\n- Phone: {}\n- Address: {}\nTotal: ${:.2}\nDiscount: ${:.2}\nTax: ${:.2}\nNet Total: ${:.2}\nItems:\n{}",
                inv.code,
                inv.date,
                inv.status,
                inv.client.name,
                inv.client.email,
                inv.client.phone,
                inv.client.address,
                inv.total,
                inv.discount,
                inv.tax,
                inv.calculate_net_total(),
                inv.items
                    .iter()
                    .map(|item| format!("- {}", item))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        };

        let paragraph = Paragraph::new(detail_text)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, chunks[1]);
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::TabInvoices => tailwind::CYAN,
            Self::TabClients => tailwind::INDIGO,
        }
    }

    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

pub fn render_tabs(app: &App, area: Rect, buf: &mut Buffer) {
    let titles = SelectedTab::iter().map(SelectedTab::title);
    let highlight_style = (Color::default(), app.current_tab.palette().c700);
    let current_tab_index = app.current_tab as usize;
    Tabs::new(titles)
        .highlight_style(highlight_style)
        .select(current_tab_index)
        .padding("", "")
        .divider(" ")
        .render(area, buf)
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "shayyz-code's invoice-rs".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

pub fn draw(frame: &mut Frame, app: &App) {
    use Constraint::{Length, Min};
    let area = frame.area();
    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [header_area, inner_area, footer_area] = vertical.areas(area);

    let horizontal = Layout::horizontal([Min(0), Length(20)]);
    let [tabs_area, title_area] = horizontal.areas(header_area);

    render_title(title_area, frame.buffer_mut());
    render_tabs(app, tabs_area, frame.buffer_mut());
    app.current_tab.render(app, inner_area, frame);
    render_footer(footer_area, frame.buffer_mut());

    if let (Mode::Editing, Some(form)) = (&app.mode, &app.form) {
        draw_form(frame, form);
    }

    if let (Mode::Modal(_), Some(modal)) = (&app.mode, &app.modal) {
        draw_modal(frame, modal);
    }
}
