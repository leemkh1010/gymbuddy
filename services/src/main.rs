mod controllers;
mod databases;
mod models;
mod routes;
mod storage;
mod utils;

use std::process::exit;

use crate::routes::{clients_routes, exercises_routes, organiser_routes, trainers_routes};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use bson::oid::ObjectId;
use databases::{CoreRepo, MongoDB};
use models::{Exercise, Media};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use routes::media_routes;
use storage::S3;
use utils::ENV;

#[get("/health")]
async fn health(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().unwrap();
    env_logger::init();

    let env = ENV::new();

    // TODO: Init redis
    // let mut cache = databases:::

    // TODO: Init repo
    let mut mongodb = MongoDB::new(&env.get_db_conn_string(), &env.get_db_name());

    let db = match mongodb.connect().await {
        Ok(_) => web::Data::new(mongodb),
        Err(e) => {
            log::error!("Failed to connect to MongoDB: {}", e);
            exit(1);
        }
    };

    // TODO: Init storage
    let s3 = S3::new(&env).await;
    let storage = web::Data::new(s3);

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .app_data(storage.clone())
            .service(
                web::scope("/api/v1")
                    .configure(clients_routes)
                    .configure(trainers_routes)
                    .configure(exercises_routes)
                    .configure(organiser_routes)
                    .configure(media_routes),
            )
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     std::env::set_var("RUST_LOG", "debug");
//     dotenv::dotenv().unwrap();
//     env_logger::init();

//     const DB_NAME: &'static str = "exercise_analyser";
//     const DB_CONNECTION: &'static str =
//         "mongodb://app_local:local@localhost:27017/exercise_analyser";
//     // TODO: Connect to MongoDB
//     let client = Client::with_uri_str(DB_CONNECTION).await.unwrap();
//     let db = client.database(DB_NAME);

//     let mut mongodb = MongoDB::new(DB_CONNECTION, DB_NAME);

//     mongodb.connect().await.unwrap();

//     // TODO: Declare Exercise type
//     let exercise = Exercise {
//         id: Some(ObjectId::new()),
//         name: "Push Up".to_string(),
//         description: "Push up exercise".to_string(),
//         r#type: "Strength".to_string(),
//         client_id: Some(ObjectId::new()),
//         trainer_id: Some(ObjectId::new()),
//         comment: "Do it slowly".to_string(),
//         created_at: chrono::Utc::now(),
//         updated_at: chrono::Utc::now(),
//     };

//     let result = mongodb.create_exercise(exercise).await;

//     match result {
//         Ok(exercise) => println!("{:#?}", exercise),
//         Err(e) => println!("{:#?}", e),
//     }

//     // TODO: Declare Media type
//     let media_col: Collection<Media> = db.collection("media");
//     let media = Media::new();

//     let result = media_col.insert_one(media).await.unwrap();

//     println!("{:#?}", result);

//     // TODO: Implement create function for Exercise and Media

//     // TODO: Implement get function for Exercise and Media

//     // TODO: Implement update function for Exercise and Media

//     // TODO: Rearrange code to class MongoDB

//     // TODO: Add logic to APIs

//     Ok(())
// }
