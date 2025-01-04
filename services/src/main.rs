mod controllers;
mod databases;
mod models;
mod routes;
mod storage;
mod utils;

use crate::routes::{clients_routes, exercises_routes, organiser_routes, trainers_routes};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use routes::media_routes;

#[get("/health")]
async fn health(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().unwrap();
    env_logger::init();

    const DB_HOST: &str = "localhost";
    const DB_PORT: u16 = 9042;
    const DB_NAME: &str = "exercise_analyser";

    // TODO: Init redis
    // let mut cache = databases:::

    // databases::

    // TODO: Init repo
    let cassandra = databases::Cassandra::new(DB_HOST, DB_PORT, DB_NAME).await;
    let db = web::Data::new(cassandra);

    // TODO: Init storage

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(
                web::scope("/api/v1")
                    .configure(clients_routes)
                    .configure(trainers_routes)
                    .configure(exercises_routes)
                    .configure(organiser_routes)
                    .configure(media_routes),
            )
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
