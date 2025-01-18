use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

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

#[get("/{id}")]
pub async fn get_exercise_by_id(
    db: web::Data<Cassandra>,
    path: web::Path<String>,
) -> impl Responder {
    // let exercise = db.get_exercise_by_id(id).await.unwrap();

    HttpResponse::Ok().json(SuccessResponse {
        code: StatusCode::OK.as_u16(),
        data: String::from("exercise"),
    })
}

#[derive(Debug, Deserialize)]
struct CreateExerciseRequest {}

#[derive(Debug, Serialize)]
struct CreateExerciseResponse {}

#[post("")]
pub async fn create_exercise(
    db: web::Data<Cassandra>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    // let exercise = db.create_exercise(body.into_inner()).await.unwrap();

    HttpResponse::Ok().json(SuccessResponse {
        code: StatusCode::OK.as_u16(),
        data: CreateExerciseResponse {},
    })
}
