use crate::models::{Client, Exercise, Media, Trainer};
use anyhow::Result;
pub trait Singleton<T> {
    fn get_instance(&self) -> &T
    where
        Self: Sized;
}

pub trait ExternalClient {
    fn new(host: &str, port: u16, db: &str) -> Self
    where
        Self: Sized;
    // async fn connect<T, E>(&mut self) -> Result<T, E>;
    // fn disconnect(&self) -> Result<(), ()>;
    // fn is_connected(&self) -> bool;
}

pub trait CoreRepo {
    // Client
    async fn get_clients(&self, limit: i32) -> Result<Vec<Client>>;
    async fn get_client_by_id(&self, id: &String) -> Result<Client>;
    async fn get_client_by_email(&self, email: &String) -> Result<Client>;
    async fn create_client(&self, client: &Client) -> Result<Client>;
    async fn update_client(&self, client: &Client) -> Result<Client>;
    async fn delete_client(&self, id: &String) -> Result<()>;

    // Trainer
    async fn get_trainers(&self) -> Result<Vec<Trainer>>;
    async fn get_trainer(&self, id: &String) -> Result<Trainer>;
    async fn create_trainer(&self, trainer: Trainer) -> Result<Trainer>;
    async fn update_trainer(&self, trainer: Trainer) -> Result<Trainer>;
    async fn delete_trainer(&self, id: &String) -> Result<()>;

    // Organiser

    // Exercise
    async fn get_exercises(&self) -> Result<Vec<Exercise>>;
    async fn create_exercise(&self, exercise: Exercise) -> Result<Exercise>;

    // Media
    async fn get_media(&self) -> Result<Vec<Media>>;
    async fn create_media(&self, media: Media) -> Result<Media>;
}
