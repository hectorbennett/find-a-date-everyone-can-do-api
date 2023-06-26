use std::str::FromStr;

use deadpool_postgres::Client;
use uuid::Uuid;

use crate::{
    errors::errors::MyError,
    models::{event::Event, user::User},
};

pub async fn get_events(client: &Client) -> Result<Vec<Event>, MyError> {
    let sql = "SELECT CAST(id AS TEXT), name, CAST(creation_date AS TEXT), CAST(modification_date AS TEXT) FROM events";
    let query = client.prepare(&sql).await.unwrap();
    let events = client
        .query(&query, &[])
        .await?
        .iter()
        .map(|row| Event::from(row))
        .collect::<Vec<Event>>();

    Ok(events)
}

pub async fn get_event(client: &Client, event_id: &str) -> Result<Event, MyError> {
    let sql = "SELECT id::TEXT, name, creation_date::TEXT, modification_date::TEXT FROM events WHERE id::TEXT = $1";
    let query = client.prepare(&sql).await.unwrap();
    let thing = client.query(&query, &[&event_id]).await?;
    let event = thing
        .iter()
        .map(|row| Event::from(row))
        .collect::<Vec<Event>>()
        .pop()
        .unwrap();
    Ok(event)
}

pub async fn create_event(client: &Client, event_name: &str) -> Result<Event, MyError> {
    let sql = "INSERT INTO events(name) VALUES($1) RETURNING CAST(id AS TEXT), name, CAST(creation_date AS TEXT), CAST(modification_date AS TEXT)";
    let query = client.prepare(&sql).await.unwrap();
    let event = client
        .query(&query, &[&event_name])
        .await?
        .iter()
        .map(|row| Event::from(row))
        .collect::<Vec<Event>>()
        .pop()
        .unwrap();
    Ok(event)
}

pub async fn get_event_users(client: &Client, event_id: &str) -> Result<Vec<User>, MyError> {
    let sql = "SELECT id::TEXT, name FROM users";
    let query = client.prepare(&sql).await.unwrap();
    let users = client
        .query(&query, &[])
        .await?
        .iter()
        .map(|row| User::from(row))
        .collect::<Vec<User>>();

    Ok(users)
}

pub async fn create_user(
    client: &Client,
    event_id: &str,
    event_name: &str,
) -> Result<User, MyError> {
    let sql = "INSERT INTO users(event_id, name) VALUES($1, $2) RETURNING id::TEXT, name";
    let query = client.prepare(&sql).await?;
    let id_as_uuid = Uuid::from_str(event_id).unwrap();
    let response = client
        .query(&query, &[&id_as_uuid, &event_name])
        .await
        .unwrap();
    let user = response
        .iter()
        .map(|row| User::from(row))
        .collect::<Vec<User>>()
        .pop()
        .unwrap();
    Ok(user)
}

// pub async fn get_event_date_selections(client: &Client, event_id: &str) -> Result<Vec<User>, MyError> {
//     let sql = "SELECT id::TEXT, name FROM users";
//     let query = client.prepare(&sql).await.unwrap();
//     let users = client
//         .query(&query, &[])
//         .await?
//         .iter()
//         .map(|row| User::from(row))
//         .collect::<Vec<User>>();

//     Ok(users)
// }


// use deadpool_postgres::Client;
// use tokio_pg_mapper::FromTokioPostgresRow;

// use crate::models::event::{Event, NewEvent};
// // use crate::models::user::{NewUser, User};
// // use chrono::prelude::*;
// // use std::collections::HashMap;
// use std::fmt::Error;
// use std::sync::{Arc, Mutex};
// use tinyid::TinyId;

// fn new_id() -> String {
//     TinyId::random().to_string()
// }

// pub struct Database {
//     pub events: Arc<Mutex<Vec<Event>>>,
// }

// impl Database {
//     pub fn new() -> Self {
//         let events = Arc::new(Mutex::new(vec![]));
//         Database { events }
//     }

//     pub fn get_events(&self) -> Vec<Event> {
//         let events = self.events.lock().unwrap();
//         events.clone()
//     }

//     pub fn get_event(&self, id: &str) -> Option<Event> {
//         let events = self.events.lock().unwrap();
//         events
//             .iter()
//             .find(|event| event.id == id.to_string())
//             .cloned()
//     }

//     pub fn create_event(&self, event: NewEvent) -> Result<Event, Error> {
//         let mut events = self.events.lock().unwrap();
//         let id = new_id();
//         let event = Event {
//             id,
//             name: event.name,
//             users: HashMap::new(),
//             creation_date: Utc::now(),
//             modification_date: Utc::now(),
//         };
//         events.push(event.clone());
//         Ok(event)
//     }

//     pub fn create_user(&self, event_id: &str, user: NewUser) -> Result<User, Error> {
//         let mut events = self.events.lock().unwrap();
//         let event: &mut Event = events
//             .iter_mut()
//             .find(|event| event.id == event_id.to_string())
//             .unwrap();
//         let id = new_id();
//         let user = User {
//             id: id.clone(),
//             name: user.name,
//             dates: vec![],
//         };
//         event.users.insert(id, user.clone());
//         event.modification_date = Utc::now();
//         Ok(user)
//     }

//     pub fn add_user_date(
//         &self,
//         event_id: &str,
//         user_id: &str,
//         date: NaiveDate,
//     ) -> Result<User, Error> {
//         let mut events = self.events.lock().unwrap();
//         let event: &mut Event = events
//             .iter_mut()
//             .find(|event| event.id == event_id.to_string())
//             .unwrap();
//         let user = event.users.get_mut(user_id).unwrap();
//         user.dates.push(date);
//         event.modification_date = Utc::now();
//         Ok(user.clone())
//     }

//     pub fn remove_user_date(
//         &self,
//         event_id: &str,
//         user_id: &str,
//         date: NaiveDate,
//     ) -> Result<User, Error> {
//         let mut events = self.events.lock().unwrap();
//         let event: &mut Event = events
//             .iter_mut()
//             .find(|event| event.id == event_id.to_string())
//             .unwrap();
//         let user = event.users.get_mut(user_id).unwrap();
//         user.dates.retain(|&d| d != date);
//         event.modification_date = Utc::now();
//         Ok(user.clone())
//     }
// }

// use deadpool_postgres::Client;

// use crate::{errors::errors::MyError, models::event::Event};

// pub async fn get_events(client: &Client) -> Result<Vec<Event>, MyError> {
//     let sql = "SELECT CAST(id AS TEXT), name, CAST(creation_date AS TEXT), CAST(modification_date AS TEXT) FROM events";
//     let query = client.prepare(&sql).await.unwrap();
//     let events = client
//         .query(&query, &[])
//         .await?
//         .iter()
//         .map(|row| Event::from(row))
//         .collect::<Vec<Event>>();

//     Ok(events)
// }
