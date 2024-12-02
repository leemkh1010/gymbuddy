use actix_web::web;

pub fn organiser_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/organiser"));
}
