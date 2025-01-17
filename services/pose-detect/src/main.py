from abc import ABC, abstractmethod
import os
import logging
from typing import Dict, Generator
import mediapipe as mp
from mediapipe import Image
from mediapipe.tasks.python.vision import (
    PoseLandmarker,
    PoseLandmarkerOptions,
    PoseLandmarksConnections,
    PoseLandmarkerResult,
    RunningMode,
)
from pyparsing import C
from exercises.squat import KeyInterestPoint2D
from repo import CoreRepo, MongoCoreRepo
from exercises import ExerciseType, Squat
import numpy as np
import cv2 as cv
from mediapipe import solutions
from mediapipe.framework.formats import landmark_pb2
from utils import Video
from utils.estimator import EstimatorOutput
from utils.video import CameraView

BaseOptions = mp.tasks.BaseOptions


class Estimator(ABC):
    _connections: frozenset[(int, int)]

    exercise_types = {ExerciseType.SQUAT: Squat()}

    def calculate_angle(self, a, b, c, outer=False):
        """
        Calculate the angle between three points
        """
        a = np.array(a)
        b = np.array(b)
        c = np.array(c)

        radians = np.arctan2(c[1] - b[1], c[0] - b[0]) - np.arctan2(
            a[1] - b[1], a[0] - b[0]
        )
        angle = np.abs(radians * 180.0 / np.pi)

        if outer or angle > 180:
            angle = 360 - angle

        return angle

    def draw_angle(
        self,
        image,
        angle,
        center=None,
        rotation_angle=None,
        label_text=None,
        label_colour=None,
    ):
        if center and rotation_angle:
            cv.ellipse(
                image, center, (30, 30), rotation_angle, 0, angle, (0, 255, 0), 2
            )

        cv.putText(
            image,
            format(angle, ".1f"),
            tuple(np.add(center, [-100, 10])),
            cv.FONT_HERSHEY_SIMPLEX,
            0.6,
            (0, 255, 0),
            2,
        )

        if label_text and label_colour:
            cv.putText(
                image,
                label_text,
                tuple(np.add(center, [-100, 30])),
                cv.FONT_HERSHEY_SIMPLEX,
                0.6,
                label_colour,
                2,
            )

    def draw_landmark(
        self,
        image: np.ndarray,
        landmarks,
        angles: Dict[str, KeyInterestPoint2D],
        vis_threshold=0.6,
        presence_threshold=0.6,
    ):
        annotated_image = np.copy(image)

        # If no landmarks, return the original image
        if not landmarks:
            return annotated_image

        # Get image height and width
        height, width, _ = annotated_image.shape

        # Convert landmarks to coordinates / pixel values (x, y) where x and y are equal to math.floor
        landmark_to_coordinates = lambda landmark: (
            int(landmark.x * width),
            int(landmark.y * height),
        )
        # Filter out landmarks with low confidence and return a dictionary of idx to coordinates / pixel
        # idx 0 to 10 are related to the face, not needed here
        idx_to_coordinates = {
            idx: landmark_to_coordinates(landmark)
            for idx, landmark in enumerate(landmarks)
        }

        # Draw landmarks
        for landmark in idx_to_coordinates.values():
            cv.circle(annotated_image, landmark, 7, (40, 116, 107), -1)

        # Draw connections
        for idx1, idx2 in self._connections:
            cv.line(
                annotated_image,
                idx_to_coordinates[idx1],
                idx_to_coordinates[idx2],
                (255, 255, 255),
                2,
            )

        for key_interest_point in angles.values():
            _, center, _ = key_interest_point.idx_to_coordinates.values()
            self.draw_angle(
                annotated_image,
                key_interest_point.angle,
                (int(center[0]), int(center[1])),
                key_interest_point.rotation_angle,
                key_interest_point.comment,
                key_interest_point.colour,
            )

        return annotated_image

    @abstractmethod
    def execute(
        self, path: str, is_video: bool
    ) -> Generator[EstimatorOutput, None, None]:
        pass


class BlazePoseEstimator(Estimator, Video):
    _model_path: str
    _connections: frozenset[(int, int)]

    def __init__(self, model_path: str):
        self._model_path = model_path

        self._options = PoseLandmarkerOptions(
            base_options=BaseOptions(model_asset_path=self._model_path),
            running_mode=RunningMode.IMAGE,
        )

        self._connections = frozenset(
            [(c.start, c.end) for c in PoseLandmarksConnections.POSE_LANDMARKS]
        )

    def execute(
        self, type: ExerciseType, video: Video
    ) -> Generator[EstimatorOutput, None, None]:
        type_processor = self.exercise_types[type]

        with PoseLandmarker.create_from_options(self._options) as landmarker:
            for idx, frame in video.get_frames():
                result = landmarker.detect(
                    mp.Image(image_format=mp.ImageFormat.SRGB, data=frame)
                )
                raw_landmark_2d = result.pose_landmarks[0]
                key_interest_points_2d = type_processor.get_2d_key_points(
                    raw_landmark_2d, video.camera_view
                )
                annotated_image = self.draw_landmark(
                    frame, raw_landmark_2d, key_interest_points_2d
                )
                yield EstimatorOutput(
                    idx, annotated_image, raw_landmark_2d, key_interest_points_2d
                )


class PoseEstimationWorker:
    _repo: CoreRepo
    _estimator: Estimator
    _logger: logging.Logger

    def __init__(self, repo: CoreRepo, estimator: Estimator) -> None:
        self._repo = repo
        self._estimator = estimator
        self._logger = logging.getLogger(self.__class__.__name__)

    def postprocessing(self, result):
        formatted_landmark_2d = [
            {
                "landmark_index": idx,
                "x": each.x,
                "y": each.y,
                "x_score": each.visibility,
                "y_score": each.visibility,
            }
            for idx, each in enumerate(result)
        ]

        return formatted_landmark_2d

    def handle_task(self) -> None:
        # TODO: Implement processing
        while True:
            # TODO: iterate on the steps
            break

    def run(self):
        # TODO: Pull job from queue

        # TODO: Parse the string

        # TODO: Validate the schema

        video_path = "/Volumes/BACKUP/projects/test-projects/exercise-analyser/services/pose-detect/src/media/test.mp4"

        video = Video(video_path, CameraView.RIGHT)

        fourcc = cv.VideoWriter_fourcc(*"X264")
        new_video = cv.VideoWriter(
            "processed.mp4",
            fourcc=fourcc,
            fps=video.fps,
            frameSize=video.shape,
            isColor=True,
        )

        for result in self._estimator.execute(ExerciseType.SQUAT, video):
            frame_count, annotated_image, raw_landmarks, key_interest_point_2d = result
            formatted_landmarks = self.postprocessing(raw_landmarks)
            new_video.write(annotated_image)

        new_video.release()


def main() -> None:
    logger = logging.getLogger(__name__)
    root_path = os.path.dirname(os.path.abspath(__file__))

    try:
        db = "exercise_analyser"
        conn_str = f"mongodb://admin:local@localhost:27017/{db}"
        repo = MongoCoreRepo(conn_str, db)
    except Exception as e:
        logger.fatal("cassandra repo init failed")
        exit(1)

    try:
        estimator = BlazePoseEstimator(
            model_path=os.path.join(root_path, "models", "pose_landmarker_full.task")
        )
    except Exception as e:
        logger.fatal("pose estimator init failed", e)
        exit(1)

    try:
        worker = PoseEstimationWorker(repo=repo, estimator=estimator)
    except Exception as e:
        logger.fatal("pose detection worker init failed: ", e)
        exit(1)

    try:
        worker.run()
    except Exception as e:
        logger.fatal("worker run fatal error: ", e)
        exit(1)


if __name__ == "__main__":
    main()
