use actix_web::{http, web::{self, get}, HttpResponse, Responder, Route};

use crate::controllers::{ClientsController, TClientController};

async fn get_clients() -> impl Responder {
    HttpResponse::Ok().body("Get clients")
}

fn create_route(method: http::Method) -> Route {
  web::route()
    .method(method)
}

pub fn clients_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clients")
            .service(
              web::resource("/")
                .route(
                  web::route()
                    .to(get_clients)
                )
            )
            .service(
              web::resource("/{id}")
                .route(
                  web::route()
                    .method(http::Method::GET)
                    .to(get_clients)
                )
                .route(
                  web::route()
                    .method(http::Method::PUT)
                    .to(get_clients)
                )
                .route(
                  web::route()
                    .method(http::Method::DELETE)
                    .to(get_clients)
                )
            ),
    );
}
