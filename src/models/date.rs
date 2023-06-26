use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatePayload {
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateEntry {
    pub user_id: String,
    pub date: String,
}

impl From<&tokio_postgres::Row> for DateEntry {
    fn from(row: &tokio_postgres::Row) -> Self {
        Self {
            user_id: row.get("user_id"),
            date: row.get("date"),
        }
    }
}
