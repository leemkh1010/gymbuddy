use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};

use crate::controllers::SuccessResponse;

use crate::databases::Cassandra;

#[get("")]
pub async fn get_exercises(db: web::Data<Cassandra>) -> impl Responder {
    // let exercises = db.get_exercises().await.unwrap();

    HttpResponse::Ok().json(SuccessResponse {
        code: StatusCode::OK.as_u16(),
        data: String::from("exercises"),
    })
}
