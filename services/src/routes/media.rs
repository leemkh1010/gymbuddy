use actix_web::web;

use crate::controllers;

pub fn media_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/media").service(controllers::create_remote_upload));
}
