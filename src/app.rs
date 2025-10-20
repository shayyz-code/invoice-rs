use crate::ui::{form::InvoiceForm, modal::Modal, modal::ModalType};
use crate::{models::Invoice, pdf::generate_invoice_pdf};

pub enum Mode {
    Normal,
    Editing,
    Modal(ModalType),
}

pub struct App {
    pub invoices: Vec<Invoice>,
    pub selected: usize,
    pub mode: Mode,
    pub form: Option<InvoiceForm>,
    pub modal: Option<Modal>,
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
            modal: None,
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

    pub fn open_save_modal(&mut self) {
        let invoice_id = self.selected_invoice().id;
        let content = "Are you sure you want to save this invoice?".to_string();

        // take a mutable pointer to self to pass into closure
        let app_ptr: *mut App = self;

        self.mode = Mode::Modal(ModalType::Confirm);
        self.modal = Some(Modal::new(
            Some(invoice_id),
            content,
            ModalType::Confirm,
            Some(Box::new(move |_: &mut App| {
                // cast pointer back to mutable reference
                let app: &mut App = unsafe { &mut *app_ptr };
                app.save_form();
            })),
        ));
    }

    pub fn close_modals(&mut self) {
        self.mode = Mode::Normal;
        self.modal = None;
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
        generate_invoice_pdf(&self.selected_invoice()).unwrap();
    }
}
