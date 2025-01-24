use actix_web::{
    http::StatusCode,
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::storage::{Storage, S3};

use super::{ErrorResponse, SuccessResponse};

#[derive(Debug, Deserialize)]
struct CreateRemoteUploadRequest {
    file_name: String,
    content_type: String,
    size: i64,
}

#[derive(Debug, Serialize)]
struct CreateRemoteUploadResponse {
    url: String,
}

#[post("/create-remote-upload")]
pub async fn create_remote_upload(
    body: Json<CreateRemoteUploadRequest>,
    object_storage: Data<S3>,
) -> impl Responder {
    let req = object_storage
        .sign_put_public_url(&body.file_name, 600)
        .await;

    match req {
        Ok(url) => HttpResponse::Ok().json(SuccessResponse {
            code: StatusCode::OK.as_u16(),
            data: CreateRemoteUploadResponse { url },
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }),
    }
}
