use crate::controllers;
use actix_web::web;

pub fn clients_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clients")
            .service(controllers::get_clients)
            .service(controllers::get_client)
            .service(controllers::create_client)
            .service(controllers::update_client)
            .service(controllers::delete_client),
    );
}
