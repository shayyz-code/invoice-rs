use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub title: String,
    pub price: Option<f64>,
    pub quantity: Option<u32>,
    pub remark: Option<String>,
}

impl Item {
    pub fn new(
        title: &str,
        price: Option<f64>,
        quantity: Option<u32>,
        remark: Option<String>,
    ) -> Self {
        Self {
            id: 1,
            title: title.to_string(),
            price,
            quantity,
            remark,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.id,
            self.title,
            self.price.unwrap_or(0.0),
            self.quantity.unwrap_or(0),
            self.remark.as_deref().unwrap_or("")
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: u32,
    pub code: String,
    pub client: Client,
    pub total: f64,
    pub currency: String,
    pub discount: f64,
    pub tax: f64,
    pub status: String,
    pub date: String,
    pub items: Vec<Item>,
}

impl Invoice {
    pub fn new(
        id: u32,
        code: &str,
        client: &Client,
        currency: &str,
        total: f64,
        discount: f64,
        tax: f64,
        status: &str,
        date: &str,
        items: Vec<Item>,
    ) -> Self {
        Self {
            id,
            code: code.to_string(),
            client: client.to_owned(),
            currency: currency.to_string(),
            total,
            discount,
            tax,
            status: status.to_string(),
            date: date.to_string(),
            items,
        }
    }

    pub fn default() -> Self {
        Self {
            id: 0,
            code: "empty_invoice".to_string(),
            client: Client::default(),
            currency: String::new(),
            total: 0.0,
            discount: 0.0,
            tax: 0.0,
            status: String::new(),
            date: String::new(),
            items: Vec::new(),
        }
    }

    pub fn calculate_net_total(&self) -> f64 {
        self.total - self.discount + self.tax
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: u32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub address: String,
}

impl Client {
    pub fn new(id: u32, name: &str, phone: &str, email: &str, address: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
            address: address.to_string(),
        }
    }

    pub fn default() -> Self {
        Self {
            id: 0,
            name: "empty_client".to_string(),
            phone: String::new(),
            email: String::new(),
            address: String::new(),
        }
    }
}
