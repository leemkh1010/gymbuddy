use std::collections::HashMap;

use super::PoseDetectionModel;
use anyhow::{Error, Result};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
pub enum MediaStep {
    Queuing,
    VideoFetching,
    Preprocessing,
    PoseDetecting,
    Postprocessing,
    Finalizing,
    Completed,
}

impl MediaStep {
    // pub fn from_string(s: &String) -> Result<MediaStep> {
    //     match s.as_str() {
    //         "QUEUING" => Ok(MediaStep::Queuing(s.to_owned())),
    //         "VIDEO_FETCHING" => Ok(MediaStep::VideoFetching(s.to_owned())),
    //         "PRE_PROCESSING" => Ok(MediaStep::Preprocessing(s.to_owned())),
    //         "POSE_DETECTING" => Ok(MediaStep::PoseDetecting(s.to_owned())),
    //         "POST_PROCESSING" => Ok(MediaStep::Postprocessing(s.to_owned())),
    //         "FINALIZING" => Ok(MediaStep::Finalizing(s.to_owned())),
    //         "COMPLETED" => Ok(MediaStep::Completed(s.to_owned())),
    //         _ => Err(Error::msg(format!("Invalid MediaStep: {}", s))),
    //     }
    // }

    pub fn as_str(&self) -> &str {
        match self {
            MediaStep::Queuing => "QUEUING",
            MediaStep::VideoFetching => "VIDEO_FETCHING",
            MediaStep::Preprocessing => "PRE_PROCESSING",
            MediaStep::PoseDetecting => "POSE_DETECTING",
            MediaStep::Postprocessing => "POST_PROCESSING",
            MediaStep::Finalizing => "FINALIZING",
            MediaStep::Completed => "COMPLETED",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Media {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub exercise_id: ObjectId,
    pub original_video_location: String,
    pub processed_video_location: Option<String>,
    pub step: String,
    pub camera_view: String,
    pub pose_detection_model_name: String,
    pub metadata: HashMap<String, String>,
    pub errors: HashMap<String, String>,
    pub angles_of_interest_enum: HashMap<i16, String>,
    pub angles_of_interest: HashMap<i16, Vec<AngleOfInterest>>,
    pub landmark_results_2d: HashMap<i16, LandmarkResult2D>,
    pub landmark_results_3d: HashMap<i16, LandmarkResult3D>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Media {
    pub fn new() -> Media {
        Media {
            id: Some(ObjectId::new()),
            exercise_id: ObjectId::new(),
            original_video_location: "f".to_string(),
            processed_video_location: Some("w".to_string()),
            step: "f".to_string(),
            camera_view: "f".to_string(),
            pose_detection_model_name: "f".to_string(),
            metadata: HashMap::new(),
            errors: HashMap::new(),
            angles_of_interest_enum: HashMap::new(),
            angles_of_interest: HashMap::new(),
            landmark_results_2d: HashMap::new(),
            landmark_results_3d: HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            completed_at: Some(chrono::Utc::now()),
        }
    }

    pub fn get_angles_of_interest_enum(&self) -> HashMap<i16, String> {
        self.angles_of_interest_enum.clone()
    }

    pub fn get_angles_of_interest(&self) -> HashMap<i16, Vec<AngleOfInterest>> {
        self.angles_of_interest.clone()
    }

    pub fn get_landmark_results_2d(&self) -> HashMap<i16, LandmarkResult2D> {
        self.landmark_results_2d.clone()
    }

    pub fn get_landmark_results_3d(&self) -> HashMap<i16, LandmarkResult3D> {
        self.landmark_results_3d.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AngleOfInterest {
    idx: i8,
    degree: i16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LandmarkResult2D {
    landmark_index: i8,
    x: f32,
    y: f32,
    x_score: f32,
    y_score: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LandmarkResult3D {
    landmark_index: i8,
    score: f32,
    x: f32,
    y: f32,
    z: f32,
}
