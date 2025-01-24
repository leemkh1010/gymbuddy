use super::CoreRepo;
use crate::models::{Client, Exercise, Media, Trainer};
use anyhow::{Error, Result};
use bson::doc;
use mongodb::{Client as MongoClient, Collection, Database};

enum Tables {
    Clients,
    Trainers,
    Organisers,
    Exercises,
    Media,
}

pub struct MongoDB {
    conn_str: String,
    db: String,
    client: Option<Database>,
}

impl MongoDB {
    pub fn new(conn_str: &str, db: &str) -> MongoDB {
        MongoDB {
            conn_str: conn_str.to_owned(),
            db: db.to_owned(),
            client: None,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let client = MongoClient::with_uri_str(&self.conn_str).await;

        match client {
            Ok(client) => {
                self.client = Some(client.database(&self.db));
                Ok(())
            }
            Err(e) => Err(anyhow::Error::msg(e.to_string())),
        }
    }
}

impl CoreRepo for MongoDB {
    // Client
    async fn get_clients(&self, limit: i32) -> Result<Vec<Client>> {
        let col: Collection<Client> = self
            .client
            .as_ref()
            .expect("db should be connected")
            .collection("clients");
        Ok(vec![])
    }

    async fn get_client_by_id(&self, id: &String) -> Result<Client> {
        let col: Collection<Client> = self
            .client
            .as_ref()
            .expect("db should be connected")
            .collection("clients");

        let result = col.find_one(doc! {"_id": id}).await;

        match result {
            Ok(client) => {
                if let Some(client) = client {
                    Ok(client)
                } else {
                    Err(Error::msg("Client not found"))
                }
            }
            Err(e) => Err(Error::msg(e.to_string())),
        }
    }

    async fn get_client_by_email(&self, email: &String) -> Result<Client> {
        let col: Collection<Client> = self
            .client
            .as_ref()
            .expect("db should be connected")
            .collection("clients");

        let result = col.find_one(doc! {"email": email}).await;

        match result {
            Ok(client) => {
                if let Some(client) = client {
                    Ok(client)
                } else {
                    Err(Error::msg("Client not found"))
                }
            }
            Err(e) => Err(Error::msg(e.to_string())),
        }
    }

    async fn create_client(&self, client: &Client) -> Result<Client> {
        if client.id.is_some() {
            return Err(Error::msg("client should not have id when creating"));
        }

        let col: Collection<Client> = self
            .client
            .as_ref()
            .expect("db should be connected")
            .collection("clients");

        let result = col.insert_one(client.clone()).await;

        if result.is_err() {
            return Err(anyhow::Error::msg(result.err().unwrap().to_string()));
        }

        let id = result.unwrap().inserted_id.as_object_id().unwrap();
        let mut c = client.clone();
        c.id = Some(id);
        Ok(c)
    }

    async fn update_client(&self, client: &Client) -> Result<Client> {
        if client.id.is_none() {
            return Err(Error::msg("Client id is required"));
        }

        let col: Collection<Client> = self
            .client
            .as_ref()
            .expect("db should be connected")
            .collection("clients");

        let result = col
            .replace_one(doc! {"_id": client.id}, client.clone())
            .await;

        if result.is_err() {
            return Err(anyhow::Error::msg(result.err().unwrap().to_string()));
        }

        Ok(client.clone())
    }

    async fn delete_client(&self, id: &String) -> Result<()> {
        let col: Collection<Client> = self
            .client
            .as_ref()
            .expect("db should be connected")
            .collection("clients");

        let result = col.delete_one(doc! {"_id": id}).await;

        if result.is_err() {
            return Err(anyhow::Error::msg(result.err().unwrap().to_string()));
        }

        Ok(())
    }

    // Trainer
    async fn get_trainers(&self) -> Result<Vec<Trainer>> {
        unimplemented!()
    }

    async fn get_trainer(&self, id: &String) -> Result<Trainer> {
        unimplemented!()
    }

    async fn create_trainer(&self, trainer: Trainer) -> Result<Trainer> {
        unimplemented!()
    }

    async fn update_trainer(&self, trainer: Trainer) -> Result<Trainer> {
        unimplemented!()
    }

    async fn delete_trainer(&self, id: &String) -> Result<()> {
        unimplemented!()
    }

    async fn get_exercises(&self) -> Result<Vec<Exercise>> {
        unimplemented!()
    }

    async fn create_exercise(&self, mut exercise: Exercise) -> Result<Exercise> {
        if let Some(id) = exercise.id {
            return Err(anyhow::Error::msg(format!(
                "Exercise already exists with id: {}",
                id
            )));
        }

        let exercise_col: Collection<Exercise> = self
            .client
            .as_ref()
            .expect("db should be connected")
            .collection("exercises");

        let result = exercise_col.insert_one(exercise.clone()).await;

        if result.is_err() {
            return Err(anyhow::Error::msg(result.err().unwrap().to_string()));
        }

        let id = result.unwrap().inserted_id.as_object_id().unwrap();
        exercise.id = Some(id);
        Ok(exercise)
    }

    async fn get_media(&self) -> Result<Vec<Media>> {
        unimplemented!()
    }

    async fn create_media(&self, mut media: Media) -> Result<Media> {
        if let Some(id) = media.id {
            return Err(anyhow::Error::msg(format!(
                "Media already exists with id: {}",
                id
            )));
        }

        let media_col: Collection<Media> = self
            .client
            .as_ref()
            .expect("db should be connected")
            .collection("media");

        let result = media_col.insert_one(media.clone()).await;

        if result.is_err() {
            return Err(anyhow::Error::msg(result.err().unwrap().to_string()));
        }

        let id = result.unwrap().inserted_id.as_object_id().unwrap();
        media.id = Some(id);
        Ok(media)
    }
}
