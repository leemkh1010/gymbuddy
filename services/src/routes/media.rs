use actix_web::web;

pub fn media_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/media"));
}
