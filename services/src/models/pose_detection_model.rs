use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct DetectionModel {
    name: String,
    // landmarks:
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PoseDetectionModel {
    MediaPipeBlasePoseLite(DetectionModel),
    MediaPipeBlasePoseFull(DetectionModel),
    MediaPipeBlasePoseHeavy(DetectionModel),
}

impl PoseDetectionModel {
    pub fn from_string(s: &str) -> Result<PoseDetectionModel, String> {
        match s {
            "MediaPipeBlasePoseLite" => {
                Ok(PoseDetectionModel::MediaPipeBlasePoseLite(DetectionModel {
                    name: "MediaPipeBlasePoseLite".to_string(),
                }))
            }
            "MediaPipeBlasePoseFull" => {
                Ok(PoseDetectionModel::MediaPipeBlasePoseFull(DetectionModel {
                    name: "MediaPipeBlasePoseFull".to_string(),
                }))
            }
            "MediaPipeBlasePoseHeavy" => Ok(PoseDetectionModel::MediaPipeBlasePoseHeavy(
                DetectionModel {
                    name: "MediaPipeBlasePoseHeavy".to_string(),
                },
            )),
            _ => Err(format!("Invalid PoseDetectionModel: {}", s)),
        }
    }
}
