use actix_web::web;

pub struct ClientsController {

}

pub trait TClientController {
  fn new() -> Self;
}

impl TClientController for ClientsController {
  fn new() -> Self {
    ClientsController {}
  }
}