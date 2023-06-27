use std::str::FromStr;

use chrono::NaiveDate;
use deadpool_postgres::Client;
use uuid::Uuid;

use crate::{
    errors::errors::MyError,
    models::{date::DateEntry, event::Event, user::User},
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
    let sql = "SELECT id::TEXT, name FROM users WHERE event_id = $1";
    let query = client.prepare(&sql).await.unwrap();
    let users = client
        .query(&query, &[&Uuid::from_str(event_id).unwrap()])
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

pub async fn get_event_dates(client: &Client, event_id: &str) -> Result<Vec<DateEntry>, MyError> {
    let sql = "SELECT user_id::TEXT, date::TEXT FROM dates WHERE event_id = $1";
    let query = client.prepare(&sql).await?;
    let response = client
        .query(&query, &[&Uuid::from_str(event_id).unwrap()])
        .await
        .unwrap();
    let date_entries = response
        .iter()
        .map(|row| DateEntry::from(row))
        .collect::<Vec<DateEntry>>();
    Ok(date_entries)
}

pub async fn add_user_date(
    client: &Client,
    event_id: &str,
    user_id: &str,
    date: &str,
) -> Result<DateEntry, MyError> {
    let sql = "INSERT INTO dates(event_id, user_id, date) VALUES($1, $2, $3) RETURNING user_id::TEXT, date::TEXT";
    let query = client.prepare(&sql).await?;
    let response = client
        .query(
            &query,
            &[
                &Uuid::from_str(event_id).unwrap(),
                &Uuid::from_str(user_id).unwrap(),
                &NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
            ],
        )
        .await
        .unwrap();
    let date_entry = response
        .iter()
        .map(|row| DateEntry::from(row))
        .collect::<Vec<DateEntry>>()
        .pop()
        .unwrap();
    Ok(date_entry)
}

pub async fn remove_user_date(
    client: &Client,
    event_id: &str,
    user_id: &str,
    date: &str,
) -> Result<DateEntry, MyError> {
    let sql = "DELETE FROM dates WHERE event_id = $1 AND user_id = $2 AND date = $3 RETURNING user_id::TEXT, date::TEXT";
    let query = client.prepare(&sql).await?;
    let response = client
        .query(
            &query,
            &[
                &Uuid::from_str(event_id).unwrap(),
                &Uuid::from_str(user_id).unwrap(),
                &NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
            ],
        )
        .await
        .unwrap();
    let date_entry = response
        .iter()
        .map(|row| DateEntry::from(row))
        .collect::<Vec<DateEntry>>()
        .pop()
        .unwrap();
    Ok(date_entry)
}
