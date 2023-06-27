use crate::config::config::ExampleConfig;
use ::config::Config;
use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use serde::Serialize;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let mut builder =
        SslConnector::builder(SslMethod::tls()).expect("unable to create sslconnector builder");
    builder
        .set_ca_file("/Users/hectorbennett/dev/find-a-date-everyone-can-do-api/ca-certificate.crt")
        .expect("unable to load ca.cert");
    builder.set_verify(SslVerifyMode::NONE);

    let connector = MakeTlsConnector::new(builder.build());

    let pool = config.pg.create_pool(None, connector).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::handlers::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
}
