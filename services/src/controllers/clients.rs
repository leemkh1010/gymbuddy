use std::time::SystemTime;

use super::{ErrorResponse, PaginatedResponse, SuccessResponse};
use crate::{
    databases::{CoreRepo, MongoDB},
    models::Client,
};
use actix_web::{delete, get, http::StatusCode, post, put, web, HttpResponse, Responder};
use aws_sdk_s3::types::Object;
use bson::oid::ObjectId;
use serde::Deserialize;

#[get("")]
pub async fn get_clients(db: web::Data<MongoDB>) -> impl Responder {
    let clients = db.get_clients(100).await.unwrap();

    let response = PaginatedResponse::<Client> {
        data: clients,
        total: 0,
        page: 0,
        limit: 0,
    };

    HttpResponse::Ok().json(response)
}

#[get["/{id}"]]
pub async fn get_client(path: web::Path<String>, db: web::Data<MongoDB>) -> impl Responder {
    let id = path.into_inner();

    let client = db.get_client_by_id(&id).await;

    match client {
        Ok(client) => HttpResponse::Ok().json(SuccessResponse {
            code: StatusCode::OK.as_u16(),
            data: client,
        }),
        Err(_) => HttpResponse::NotFound().json(ErrorResponse {
            code: StatusCode::NOT_FOUND.as_u16(),
            message: "Client not found".to_string(),
        }),
    }
}

#[derive(Debug, Deserialize)]
struct CreateClientRequest {
    first_name: String,
    last_name: String,
    email: String,
}

#[post("")]
pub async fn create_client(
    db: web::Data<MongoDB>,
    req: web::Json<CreateClientRequest>,
) -> impl Responder {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let client = db.get_client_by_email(&req.email).await;

    if client.is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: client.err().unwrap().to_string(),
        });
    }

    let client = client.unwrap();

    if client.email == req.email {
        return HttpResponse::Conflict().json(ErrorResponse {
            code: StatusCode::CONFLICT.as_u16(),
            message: "Email already exists".to_string(),
        });
    }

    let client = Client {
        id: Some(ObjectId::new()),
        first_name: req.first_name.clone(),
        last_name: req.last_name.clone(),
        email: req.email.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    db.create_client(&client).await.unwrap();

    HttpResponse::Ok().json(SuccessResponse {
        code: StatusCode::OK.as_u16(),
        data: client,
    })
}

#[derive(Deserialize)]
struct UpdateClientRequest {
    first_name: String,
    last_name: String,
    email: String,
}

#[put["/{id}"]]
pub async fn update_client(
    path: web::Path<String>,
    db: web::Data<MongoDB>,
    req: web::Json<UpdateClientRequest>,
) -> impl Responder {
    let id = path.into_inner();

    let old_client = db.get_client_by_id(&id).await;

    if old_client.is_err() {
        return HttpResponse::NotFound().json(ErrorResponse {
            code: StatusCode::NOT_FOUND.as_u16(),
            message: "Client not found".to_string(),
        });
    }

    let old_client = old_client.unwrap();

    let new_client = Client {
        id: old_client.id,
        first_name: req.first_name.clone(),
        last_name: req.last_name.clone(),
        email: req.email.clone(),
        created_at: old_client.created_at,
        updated_at: chrono::Utc::now(),
    };

    db.update_client(&new_client).await.unwrap();

    HttpResponse::Ok().json(SuccessResponse {
        code: StatusCode::OK.as_u16(),
        data: new_client,
    })
}

#[delete["/{id}"]]
pub async fn delete_client() -> impl Responder {
    HttpResponse::Ok().body("Delete clients")
}
