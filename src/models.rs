#[derive(Clone)]
pub struct Invoice {
    pub id: u32,
    pub number: String,
    pub client: String,
    pub total: f64,
    pub status: String,
}

impl Invoice {
    pub fn new(id: u32, number: &str, client: &str, total: f64, status: &str) -> Self {
        Self {
            id,
            number: number.to_string(),
            client: client.to_string(),
            total,
            status: status.to_string(),
        }
    }
}
