use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatePayload {
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub name: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct User {
//     pub id: String,
//     pub name: String,
//     pub dates: Vec<NaiveDate>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct User {
    pub id: String,
    pub name: String,
    // pub users: HashMap<String, User>,
    // pub creation_date: String,
    // pub modification_date: String,
}

impl From<&tokio_postgres::Row> for User {
    fn from(row: &tokio_postgres::Row) -> Self {
        // println!("{:?}", row.get::<&str, DateTime<Utc>>("creation_date"));

        Self {
            id: row.get("id"),
            name: row.get("name"),
            // creation_date: Utc::now(),
            // creation_date: row.get("creation_date"),
            // modification_date: row.get("modification_date"),
        }
    }
}
