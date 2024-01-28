#![allow(dead_code)]

use std::{env, sync::Arc};

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    modules::chat::service::Service, utils::infrastructure::postgres_repository::PostgresRepository,
};
mod errors;
mod modules;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var(
        "RUST_LOG",
        "info,debug,sqlx::query=off,handlebars::render=off,hyper=off,html5ever=off,selectors::matching=off,llm_rust::chat_models::openai::chat_llm=off,appinsights::channel::state=off",
    );

    env_logger::init();
    log::info!("Logger initialized.");
    let pg_repository = Arc::new(PostgresRepository::new().await);

    let chat_service = web::Data::new(Service::new(pg_repository.clone(), pg_repository.clone()));

    log::info!("Starting HTTP server on 0.0.0.0:80...");
    // Start the HTTP server on 0.0.0.0:8080
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(chat_service.clone())
            .configure(routes::configure)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
