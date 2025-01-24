use std::collections::HashMap;

use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::controllers::{ErrorResponse, SuccessResponse};

use crate::databases::{CoreRepo, MongoDB};
use crate::models::{Exercise, Media, MediaStep, PoseDetectionModel};
use crate::storage::{Storage, S3};

#[get("")]
pub async fn get_exercises(db: web::Data<MongoDB>) -> impl Responder {
    // let exercises = db.get_exercises().await.unwrap();

    HttpResponse::Ok().json(SuccessResponse {
        code: StatusCode::OK.as_u16(),
        data: String::from("exercises"),
    })
}

#[get("/{id}")]
pub async fn get_exercise_by_id(db: web::Data<MongoDB>, path: web::Path<String>) -> impl Responder {
    // let exercise = db.get_exercise_by_id(id).await.unwrap();

    HttpResponse::Ok().json(SuccessResponse {
        code: StatusCode::OK.as_u16(),
        data: String::from("exercise"),
    })
}

struct MQTTMessage {
    exercise: Exercise,
    media: Media,
}

#[derive(Debug, Deserialize)]
struct MediaReference {
    object_storage_bucket: String,
    object_storage_key: String,
    name: String,
    size: i64,
    r#type: String,
    camera_view: String,
}

#[derive(Debug, Deserialize)]
struct CreateExerciseRequest {
    name: String,
    description: String,
    r#type: String,
    client_id: ObjectId,
    trainer_id: ObjectId,
    date: Option<i64>,
    remote_storage_reference: Vec<MediaReference>,
}

#[derive(Debug, Serialize)]
struct CreateExerciseResponse {
    exercise: Exercise,
    media: Vec<Media>,
}

#[post("")]
pub async fn create_exercise(
    db: web::Data<MongoDB>,
    storage: web::Data<S3>,
    body: web::Json<CreateExerciseRequest>,
) -> impl Responder {
    // let exercise = db.create_exercise(body.into_inner()).await.unwrap();
    // TODO: Validate request body

    // TODO: Create exercise
    let exercise = Exercise {
        id: None,
        name: body.name.clone(),
        description: body.description.clone(),
        r#type: body.r#type.clone(), // TODO: Convert to ExerciseType
        client_id: Some(body.client_id),
        trainer_id: Some(body.trainer_id),
        comment: String::from(""),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let exercise = match db.create_exercise(exercise).await {
        Ok(exercise) => exercise,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: e.to_string(),
            });
        }
    };

    let mut media_list: Vec<Media> = vec![];
    // TODO: For each remote_storage_reference, create a new media object
    for reference in body.remote_storage_reference.iter() {
        let media = Media {
            id: None,
            exercise_id: exercise.id.clone().unwrap(),
            original_video_location: storage
                .to_object_storage_reference(reference.object_storage_key.clone())
                .to_location_path(),
            processed_video_location: None,
            pose_detection_model_name: PoseDetectionModel::MediaPipeBlasePoseFull
                .as_str()
                .to_string(),
            step: MediaStep::Queuing.as_str().to_string(),
            camera_view: reference.camera_view.clone(),
            metadata: HashMap::new(),
            errors: HashMap::new(),
            angles_of_interest_enum: HashMap::new(),
            angles_of_interest: HashMap::new(),
            landmark_results_2d: HashMap::new(),
            landmark_results_3d: HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            completed_at: None,
        };

        match db.create_media(media).await {
            Ok(media) => {
                let message = MQTTMessage {
                    exercise: exercise.clone(),
                    media: media.clone(),
                };

                // mqtt_client.send(message).await;
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: e.to_string(),
                });
            }
        }
    }

    HttpResponse::Ok().json(SuccessResponse {
        code: StatusCode::OK.as_u16(),
        data: CreateExerciseResponse {
            exercise,
            media: media_list,
        },
    })
}
