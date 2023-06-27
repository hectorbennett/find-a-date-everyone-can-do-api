use crate::{
    db::db,
    errors::errors::MyError,
    models::{date::DatePayload, event::NewEvent, user::NewUser},
};
use actix_web::{get, post, web, Error, HttpResponse};

use deadpool_postgres::{Client, Pool};

#[get("/events")]
pub async fn get_events(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let events = db::get_events(&client).await?;
    Ok(HttpResponse::Ok().json(events))
}

#[get("/events/{id}")]
pub async fn get_event(
    db_pool: web::Data<Pool>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let event = db::get_event(&client, &id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(event))
}

#[post("/events")]
pub async fn create_event(
    db_pool: web::Data<Pool>,
    new_event: web::Json<NewEvent>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let events = db::create_event(&client, &new_event.name).await?;
    Ok(HttpResponse::Ok().json(events))
}

#[post("/events/{event_id}/users")]
pub async fn create_user(
    db_pool: web::Data<Pool>,
    event_id: web::Path<String>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let events = db::create_user(&client, &event_id.into_inner(), &new_user.name).await?;
    Ok(HttpResponse::Ok().json(events))
}

#[get("/events/{event_id}/users")]
pub async fn get_event_users(
    db_pool: web::Data<Pool>,
    event_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let events = db::get_event_users(&client, &event_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(events))
}

#[get("/events/{event_id}/dates")]
pub async fn get_event_dates(
    db_pool: web::Data<Pool>,
    event_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let dates = db::get_event_dates(&client, &event_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(dates))
}

#[post("/events/{event_id}/users/{user_id}/dates")]
pub async fn add_user_date(
    db_pool: web::Data<Pool>,
    path: web::Path<(String, String)>,
    date_payload: web::Json<DatePayload>,
) -> Result<HttpResponse, Error> {
    let (event_id, user_id) = path.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let date_entry = db::add_user_date(&client, &event_id, &user_id, &date_payload.date).await?;
    Ok(HttpResponse::Ok().json(date_entry))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(add_user_date)
            .service(create_event)
            .service(create_user)
            .service(get_event_dates)
            .service(get_event_users)
            .service(get_event)
            .service(get_events),
    );
}
