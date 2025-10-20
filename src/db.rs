use chrono::NaiveDate;
use rusqlite::{Connection, Result, params};

#[derive(Debug, Clone)]
pub struct Invoice {
    pub id: String,
    pub customer: String,
    pub amount: f64,
    pub date: NaiveDate,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("invoice-rs.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS invoices (
                id TEXT PRIMARY KEY,
                customer TEXT NOT NULL,
                amount REAL NOT NULL,
                date TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn add_invoice(&self, invoice: &Invoice) -> Result<()> {
        self.conn.execute(
            "INSERT INTO invoices (id, customer, amount, date)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                invoice.id,
                invoice.customer,
                invoice.amount,
                invoice.date.to_string()
            ],
        )?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Invoice>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, customer, amount, date FROM invoices")?;
        let rows = stmt.query_map([], |row| {
            Ok(Invoice {
                id: row.get(0)?,
                customer: row.get(1)?,
                amount: row.get(2)?,
                date: NaiveDate::parse_from_str(&row.get::<_, String>(3)?, "%Y-%m-%d").unwrap(),
            })
        })?;

        Ok(rows.filter_map(Result::ok).collect())
    }
}
