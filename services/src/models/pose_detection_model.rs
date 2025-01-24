use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct DetectionModel {
    name: String,
    // landmarks:
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PoseDetectionModel {
    MediaPipeBlasePoseLite,
    MediaPipeBlasePoseFull,
    MediaPipeBlasePoseHeavy,
}

impl PoseDetectionModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            PoseDetectionModel::MediaPipeBlasePoseLite => "MEDIAPIPE_BLASE_POSE_LITE",
            PoseDetectionModel::MediaPipeBlasePoseFull => "MEDIAPIPE_BLASE_POSE_FULL",
            PoseDetectionModel::MediaPipeBlasePoseHeavy => "MEDIAPIPE_BLASE_POSE_HEAVY",
        }
    }
}
