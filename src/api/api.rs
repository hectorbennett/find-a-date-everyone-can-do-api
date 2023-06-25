use crate::{
    database::database::Database,
    models::{
        event::NewEvent,
        user::{DatePayload, NewUser},
    },
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[post("/events")]
pub async fn create_event(db: web::Data<Database>, new_event: web::Json<NewEvent>) -> HttpResponse {
    println!("{}", "Hello world 1");
    let event = db.create_event(new_event.into_inner());
    match event {
        Ok(event) => HttpResponse::Ok().json(event),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/events/{id}")]
pub async fn get_event(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    println!("{}", "Hello world 2");
    let event = db.get_event(&id);
    match event {
        Some(event) => HttpResponse::Ok().json(event),
        None => HttpResponse::NotFound().body("Event not found"),
    }
}

#[get("/events")]
pub async fn get_events(db: web::Data<Database>) -> HttpResponse {
    println!("{}", "Hello world 3");
    let events = db.get_events();
    HttpResponse::Ok().json(events)
}

#[post("/events/{event_id}/users")]
pub async fn create_user(
    db: web::Data<Database>,
    event_id: web::Path<String>,
    new_user: web::Json<NewUser>,
) -> HttpResponse {
    println!("{}", "Hello world 4");
    let user = db.create_user(&event_id, new_user.into_inner());
    match user {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/events/{event_id}/users/{user_id}/dates")]
pub async fn add_user_date(
    db: web::Data<Database>,
    path: web::Path<(String, String)>,
    date_payload: web::Json<DatePayload>,
) -> HttpResponse {
    let (event_id, user_id) = path.into_inner();
    println!("{}", "Hello world 5");
    let date = NaiveDate::parse_from_str(&date_payload.date, "%Y-%m-%d").unwrap();
    let t = db.add_user_date(&event_id, &user_id, date);
    match t {
        Ok(thing) => HttpResponse::Ok().json(thing),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/events/{event_id}/users/{user_id}/dates")]
pub async fn remove_user_date(
    db: web::Data<Database>,
    path: web::Path<(String, String)>,
    date_payload: web::Json<DatePayload>,
) -> HttpResponse {
    let (event_id, user_id) = path.into_inner();
    let date = NaiveDate::parse_from_str(&date_payload.date, "%Y-%m-%d").unwrap();
    let t = db.remove_user_date(&event_id, &user_id, date);
    match t {
        Ok(thing) => HttpResponse::Ok().json(thing),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(create_event)
            .service(get_events)
            .service(get_event)
            .service(create_user)
            .service(add_user_date)
            .service(remove_user_date),
    );
}
