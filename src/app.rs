use crate::db::Database;
use crate::ui::layout::SelectedTab;
use crate::ui::{invoice_form::InvoiceForm, modal::Modal, modal::ModalType};
use crate::{
    models::{Client, Invoice},
    pdf::generate_invoice_pdf,
};
use std::sync::{Arc, Mutex};

pub enum Mode {
    Normal,
    Editing,
    Modal(ModalType),
}

pub struct App {
    pub db: Arc<Mutex<Database>>,
    pub invoices: Vec<Invoice>,
    pub clients: Vec<Client>,
    pub selected: usize,
    pub mode: Mode,
    pub current_tab: SelectedTab,
    pub form: Option<InvoiceForm>,
    pub modal: Option<Modal>,
    pub empty_invoice: Invoice,
    pub empty_client: Client,
}

impl App {
    pub fn new() -> Self {
        let db = match Database::new() {
            Ok(db) => Arc::new(Mutex::new(db)),
            Err(_) => {
                // fallback (empty)
                return Self {
                    db: Arc::new(Mutex::new(Database::empty())),
                    invoices: Vec::new(),
                    clients: Vec::new(),
                    selected: 0,
                    mode: Mode::Normal,
                    current_tab: SelectedTab::default(),
                    form: None,
                    modal: None,
                    empty_client: Client::default(),
                    empty_invoice: Invoice::default(),
                };
            }
        };

        let (clients, invoices) = {
            let conn = db.lock().unwrap();
            let clients = conn.get_all_clients().unwrap_or_else(|_| Vec::new());
            let invoices = conn.get_all_invoices().unwrap_or_else(|_| Vec::new());
            (clients, invoices)
        };

        Self {
            db,
            invoices,
            clients,
            selected: 0,
            mode: Mode::Normal,
            current_tab: SelectedTab::default(),
            form: None,
            modal: None,
            empty_client: Client::default(),
            empty_invoice: Invoice::default(),
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = self.current_tab.previous();
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
        &self
            .invoices
            .get(self.selected)
            .unwrap_or(&self.empty_invoice)
    }

    pub fn start_new(&mut self) {
        self.mode = Mode::Editing;
        self.form = Some(InvoiceForm::new(self.clients.clone()));
    }

    pub fn start_edit(&mut self) {
        self.mode = Mode::Editing;
        self.form = Some(InvoiceForm::from_invoice(
            self.selected_invoice(),
            self.clients.clone(),
        ));
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
                    let result = self.db.lock().unwrap().add_invoice(&inv);
                    match result {
                        Ok(_) => (),
                        Err(e) => println!("Error adding invoice: {}", e),
                    }
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
