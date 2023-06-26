use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewEvent {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Event {
    pub id: String,
    pub name: String,
    pub creation_date: String,
    pub modification_date: String,
}

impl From<&tokio_postgres::Row> for Event {
    fn from(row: &tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            creation_date: row.get("creation_date"),
            modification_date: row.get("modification_date"),
        }
    }
}
