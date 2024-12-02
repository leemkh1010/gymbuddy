use actix_web::web;

pub fn exercises_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/exercises"));
}
