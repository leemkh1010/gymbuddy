use std::{collections::HashMap, str::FromStr};

use super::PoseDetectionModel;
use anyhow::{Error, Result};
use scylla::{
    frame::value::CqlTimestamp,
    macros::{DeserializeValue, SerializeValue},
    DeserializeRow,
};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

pub enum MediaStep {
    Queuing(String),
    VideoFetching(String),
    Preprocessing(String),
    PoseDetecting(String),
    Postprocessing(String),
    Finalizing(String),
    Completed(String),
}

impl MediaStep {
    pub fn from_string(s: &String) -> Result<MediaStep> {
        match s.as_str() {
            "QUEUING" => Ok(MediaStep::Queuing("QUEUING".to_string())),
            "VIDEO_FETCHING" => Ok(MediaStep::VideoFetching("VIDEO_FETCHING".to_string())),
            "PRE_PROCESSING" => Ok(MediaStep::Preprocessing("PRE_PROCESSING".to_string())),
            "POSE_DETECTING" => Ok(MediaStep::PoseDetecting("POSE_DETECTING".to_string())),
            "POST_PROCESSING" => Ok(MediaStep::Postprocessing("POST_PROCESSING".to_string())),
            "FINALIZING" => Ok(MediaStep::Finalizing("FINALISING".to_string())),
            "COMPLETED" => Ok(MediaStep::Completed("COMPLETED".to_string())),
            _ => Err(Error::msg(format!("Invalid MediaStep: {}", s))),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Media {
    pub id: Ulid,
    pub exercise_id: Ulid,
    pub original_video_location: String,
    pub processed_video_location: String,
    pub step: String,
    pub camera_view: String,
    pub pose_detection_model_name: String,
    pub metadata: HashMap<String, String>,
    pub errors: HashMap<String, String>,
    angles_of_interest_enum: HashMap<i16, String>,
    angles_of_interest: HashMap<i16, Vec<AngleOfInterest>>,
    landmark_results_2d: HashMap<i16, LandmarkResult2D>,
    landmark_results_3d: HashMap<i16, LandmarkResult3D>,
    pub created_at: i64,
    pub updated_at: i64,
    pub completed_at: i64,
}

impl Media {
    pub fn new() -> Media {
        Media {
            id: Ulid::new(),
            exercise_id: Ulid::new(),
            original_video_location: "f".to_string(),
            processed_video_location: "w".to_string(),
            step: "f".to_string(),
            camera_view: "f".to_string(),
            pose_detection_model_name: "f".to_string(),
            metadata: HashMap::new(),
            errors: HashMap::new(),
            angles_of_interest_enum: HashMap::new(),
            angles_of_interest: HashMap::new(),
            landmark_results_2d: HashMap::new(),
            landmark_results_3d: HashMap::new(),
            created_at: 0,
            updated_at: 0,
            completed_at: 0,
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

#[derive(SerializeValue, DeserializeValue, Serialize, Deserialize, Clone, Debug)]
pub struct AngleOfInterest {
    idx: i8,
    degree: i16,
}

#[derive(SerializeValue, DeserializeValue, Serialize, Deserialize, Clone, Debug)]
pub struct LandmarkResult2D {
    landmark_index: i8,
    x: f32,
    y: f32,
    x_score: f32,
    y_score: f32,
}

#[derive(SerializeValue, DeserializeValue, Serialize, Deserialize, Clone, Debug)]
pub struct LandmarkResult3D {
    landmark_index: i8,
    score: f32,
    x: f32,
    y: f32,
    z: f32,
}

#[derive(SerializeValue, DeserializeValue, DeserializeRow, Clone, Debug)]
pub struct MediaByIdRow {
    media_id: String,
    exercise_id: String,
    original_video_location: String,
    processed_video_location: String,
    step: String,
    camera_view: String,
    pose_detection_model_name: String,
    metadata: HashMap<String, String>,
    errors: HashMap<String, String>,
    angles_of_interest_enum: HashMap<i16, String>,
    angles_of_interest: HashMap<i16, Vec<AngleOfInterest>>,
    landmark_results_2d: HashMap<i16, LandmarkResult2D>,
    landmark_results_3d: HashMap<i16, LandmarkResult3D>,
    created_at: CqlTimestamp,
    updated_at: CqlTimestamp,
    completed_at: CqlTimestamp,
}

impl MediaByIdRow {
    pub fn to_media(self) -> Media {
        Media {
            id: Ulid::from_string(&self.media_id).unwrap(),
            exercise_id: Ulid::from_string(&self.exercise_id).unwrap(),
            original_video_location: self.original_video_location,
            processed_video_location: self.processed_video_location,
            step: self.step,
            camera_view: self.camera_view,
            pose_detection_model_name: "MediaPipeBlasePoseLite".to_string(),
            metadata: self.metadata,
            errors: self.errors,
            angles_of_interest_enum: self.angles_of_interest_enum,
            angles_of_interest: self.angles_of_interest,
            landmark_results_2d: self.landmark_results_2d,
            landmark_results_3d: self.landmark_results_3d,
            created_at: self.created_at.0,
            updated_at: self.updated_at.0,
            completed_at: self.completed_at.0,
        }
    }
}

#[derive(DeserializeRow, Clone, Debug)]
pub struct MediaByExerciseIdRow {
    media_id: String,
    exercise_id: String,
}
