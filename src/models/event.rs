use std::collections::HashMap;

// use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewEvent {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub users: HashMap<String, User>,
}
