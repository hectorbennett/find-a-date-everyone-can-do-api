use chrono::{DateTime, NaiveDateTime, Utc};
// use std::collections::HashMap;
use tokio_pg_mapper_derive::PostgresMapper;

// use postgres::rows::Row;
// use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use uuid::Uuid;
// use tokio_postgres::Row;

// extern crate postgres;

// use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewEvent {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Event {
    pub id: String,
    pub name: String,
    // pub users: HashMap<String, User>,
    pub creation_date: String,
    pub modification_date: String,
}

impl From<&tokio_postgres::Row> for Event {
    fn from(row: &tokio_postgres::Row) -> Self {
        // println!("{:?}", row.get::<&str, DateTime<Utc>>("creation_date"));

        Self {
            id: row.get("id"),
            name: row.get("name"),
            // creation_date: Utc::now(),
            creation_date: row.get("creation_date"),
            modification_date: row.get("modification_date"),
        }
    }
}

// #[derive(Serialize, Deserialize, PostgresMapper, Debug, Clone)]
// #[pg_mapper(table = "events")]
// pub struct EventTable {
//     pub id: String,
//     pub name: String,
// }
