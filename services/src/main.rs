mod routes;
mod controllers;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::routes::{clients_routes, trainers_routes, exercises_routes};

#[get("/health")]
async fn health(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        

        App::new()
            .service(
                web::scope("/v1")
                    .configure(clients_routes)
                    .configure(trainers_routes)
                    .configure(exercises_routes)
            )
            .service(health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}