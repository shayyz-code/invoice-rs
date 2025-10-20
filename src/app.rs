use crate::{form::InvoiceForm, models::Invoice, pdf};
pub enum Mode {
    Normal,
    Editing,
}

pub struct App {
    pub invoices: Vec<Invoice>,
    pub selected: usize,
    pub mode: Mode,
    pub form: Option<InvoiceForm>,
}

impl App {
    pub fn new() -> Self {
        Self {
            invoices: vec![
                Invoice::new(1, "INV-001", "Alice Co.", 123.45, "Paid"),
                Invoice::new(2, "INV-002", "Bob Ltd.", 456.78, "Unpaid"),
            ],
            selected: 0,
            mode: Mode::Normal,
            form: None,
        }
    }

    pub fn next(&mut self) {
        if self.selected + 1 < self.invoices.len() {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn selected_invoice(&self) -> &Invoice {
        &self.invoices[self.selected]
    }

    pub fn start_new(&mut self) {
        self.mode = Mode::Editing;
        self.form = Some(InvoiceForm::new());
    }

    pub fn start_edit(&mut self) {
        self.mode = Mode::Editing;
        self.form = Some(InvoiceForm::from_invoice(self.selected_invoice()));
    }

    pub fn save_form(&mut self) {
        if let Some(form) = &self.form {
            let id = if self.selected < self.invoices.len() {
                self.selected_invoice().id
            } else {
                self.invoices.len() as u32 + 1
            };
            if let Some(inv) = form.to_invoice(id) {
                if self.selected < self.invoices.len() {
                    self.invoices[self.selected] = inv;
                } else {
                    self.invoices.push(inv);
                }
            }
        }
        self.mode = Mode::Normal;
        self.form = None;
    }

    pub fn cancel_form(&mut self) {
        self.mode = Mode::Normal;
        self.form = None;
    }

    pub fn export_pdf(&mut self) {
        pdf::generate_invoice_html(&self.selected_invoice()).unwrap();
    }
}
