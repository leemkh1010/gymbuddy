use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum ExerciseType {
    Squat(),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Exercise {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer_id: Option<ObjectId>,
    pub comment: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
