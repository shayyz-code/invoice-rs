use crate::models::{Client, Invoice, Item};
use rusqlite::{Connection, Result, params};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("invoice-rs.db")?;
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS clients (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                phone TEXT NOT NULL,
                email TEXT NOT NULL,
                address TEXT NOT NULL,
                UNIQUE (id)
            );

            CREATE TABLE IF NOT EXISTS invoices (
                id TEXT PRIMARY KEY,
                code TEXT NOT NULL,
                client_id TEXT NOT NULL,
                total REAL NOT NULL,
                currency TEXT NOT NULL DEFAULT 'USD',
                date TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                status TEXT NOT NULL DEFAULT 'draft',
                items JSON NOT NULL DEFAULT '[]',
                FOREIGN KEY (client_id) REFERENCES clients(id),
                UNIQUE (code),
                UNIQUE (id)
            );
            ",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn add_invoice(&self, invoice: &Invoice) -> Result<()> {
        let json_items = serde_json::to_string(&invoice.items).unwrap();
        self.conn.execute(
            "INSERT INTO invoices (code, client_id, total, currency, discount, tax, status, date, items)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                invoice.code,
                invoice.client.id,
                invoice.total,
                invoice.currency,
                invoice.discount,
                invoice.tax,
                invoice.status,
                invoice.date.to_string(),
                json_items
            ],
        )?;
        Ok(())
    }

    pub fn add_client(&self, client: &Client) -> Result<()> {
        self.conn.execute(
            "INSERT INTO clients (id, name, phone, email, address)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                client.id,
                client.name,
                client.phone,
                client.email,
                client.address,
            ],
        )?;
        Ok(())
    }

    pub fn get_all_invoices(&self) -> Result<Vec<Invoice>> {
        let mut stmt = self
            .conn
            .prepare("
                SELECT id, code, total, currency, discount, tax, status, date, items, client_id, c.name AS client_name, c.phone AS client_phone, c.email AS client_email, c.address AS client_address
                FROM invoices
                JOIN clients c ON invoices.client_id = c.id GROUP BY invoices.id")?;
        let rows = stmt.query_map([], |row| {
            let items_json: String = row.get(8)?;
            let items: Vec<Item> = serde_json::from_str(&items_json).unwrap();

            Ok(Invoice {
                id: row.get(0)?,
                code: row.get(1)?,
                total: row.get(2)?,
                currency: row.get(3)?,
                discount: row.get(4)?,
                tax: row.get(5)?,
                status: row.get(6)?,
                date: row.get(7)?,
                items,
                client: Client {
                    id: row.get(9)?,
                    name: row.get(10)?,
                    phone: row.get(11)?,
                    email: row.get(12)?,
                    address: row.get(13)?,
                },
            })
        })?;

        Ok(rows.filter_map(Result::ok).collect())
    }
}
