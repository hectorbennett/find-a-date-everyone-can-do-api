use chrono::prelude::*;
use std::collections::HashMap;
use std::fmt::Error;
use std::sync::{Arc, Mutex};

use crate::models::event::{Event, NewEvent};
use crate::models::user::{NewUser, User};

pub struct Database {
    pub events: Arc<Mutex<Vec<Event>>>,
}

impl Database {
    pub fn new() -> Self {
        let events = Arc::new(Mutex::new(vec![]));
        Database { events }
    }

    pub fn get_events(&self) -> Vec<Event> {
        let events = self.events.lock().unwrap();
        events.clone()
    }

    pub fn get_event(&self, id: &str) -> Option<Event> {
        let events = self.events.lock().unwrap();
        events
            .iter()
            .find(|event| event.id == id.to_string())
            .cloned()
    }

    pub fn create_event(&self, event: NewEvent) -> Result<Event, Error> {
        let mut events = self.events.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        // let created_at = Utc::now();
        // let updated_at = Utc::now();
        let event = Event {
            id,
            name: event.name,
            users: HashMap::new(),
        };
        events.push(event.clone());
        Ok(event)
    }

    pub fn create_user(&self, event_id: &str, user: NewUser) -> Result<User, Error> {
        let mut events = self.events.lock().unwrap();
        let event: &mut Event = events
            .iter_mut()
            .find(|event| event.id == event_id.to_string())
            .unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let user = User {
            id: id.clone(),
            name: user.name,
            dates: vec![],
        };
        event.users.insert(id, user.clone());
        Ok(user)
    }

    pub fn add_user_date(
        &self,
        event_id: &str,
        user_id: &str,
        date: NaiveDate,
    ) -> Result<User, Error> {
        let mut events = self.events.lock().unwrap();
        let event: &mut Event = events
            .iter_mut()
            .find(|event| event.id == event_id.to_string())
            .unwrap();
        let user = event.users.get_mut(user_id).unwrap();
        user.dates.push(date);
        Ok(user.clone())
    }

    pub fn remove_user_date(
        &self,
        event_id: &str,
        user_id: &str,
        date: NaiveDate,
    ) -> Result<User, Error> {
        let mut events = self.events.lock().unwrap();
        let event: &mut Event = events
            .iter_mut()
            .find(|event| event.id == event_id.to_string())
            .unwrap();
        let user = event.users.get_mut(user_id).unwrap();
        user.dates.retain(|&d| d != date);
        Ok(user.clone())
    }
}