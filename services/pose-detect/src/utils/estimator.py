from typing import Dict, NamedTuple, Tuple
import numpy as np
from mediapipe.tasks.python.vision.pose_landmarker import PoseLandmarkerResult

from exercises import KeyInterestPoint2D


class EstimatorOutput(NamedTuple):
    frame_count: int
    annotated_image: np.ndarray
    raw_landmarks: PoseLandmarkerResult
    key_interest_points_2d: Dict[str, KeyInterestPoint2D]
