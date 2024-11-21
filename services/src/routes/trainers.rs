use actix_web::web;

pub fn trainers_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::resource("/trainers")
  );
}