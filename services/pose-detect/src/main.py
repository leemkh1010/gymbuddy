from abc import ABC, abstractmethod
import os
import logging
import mediapipe as mp
from mediapipe import Image
from mediapipe.tasks.python.vision import PoseLandmarker, PoseLandmarkerOptions, PoseLandmarksConnections, PoseLandmarkerResult, RunningMode
from repo.cassandra import CassandraRepo, Repo
from exercises import ExerciseType, Squat
import numpy as np
import cv2 as cv
from mediapipe import solutions
from mediapipe.framework.formats import landmark_pb2

BaseOptions = mp.tasks.BaseOptions

class Video():
  def __init__(self, video_path: str):
    self.video_path = video_path
    self.video = cv.VideoCapture(video_path)
    
    assert self.video.isOpened(), "Cannot open video"
    
    self.fps = self.video.get(cv.CAP_PROP_FPS)
    self.shape = (int(self.video.get(cv.CAP_PROP_FRAME_WIDTH)), int(self.video.get(cv.CAP_PROP_FRAME_HEIGHT)))
    self.total_frames = self.video.get(cv.CAP_PROP_FRAME_COUNT)
  
  def get_frames(self):
    has_next, frame = self.video.read()
    
    while has_next:
      yield frame
      has_next, frame = self.video.read()

    self.video.release()


class Estimator(ABC):
  exercise_types = {
    ExerciseType.SQUAT: Squat()
  }
  
  def calculate_angle(self, a, b, c, outer = False):
    """
    Calculate the angle between three points
    """
    a = np.array(a)
    b = np.array(b)
    c = np.array(c)
    
    radians = np.arctan2(c[1] - b[1], c[0] - b[0]) - np.arctan2(a[1] - b[1], a[0] - b[0])
    angle = np.abs(radians * 180.0 / np.pi)
    
    if outer or angle > 180:
      angle = 360 - angle
      
    return angle
  
  def draw_landmarks_on_image(self, rgb_image, detection_result) -> np.ndarray:
    pose_landmarks_list = detection_result.pose_landmarks
    annotated_image = np.copy(rgb_image)

    # Loop through the detected poses to visualize.
    for idx in range(len(pose_landmarks_list)):
      pose_landmarks = pose_landmarks_list[idx]

      # Draw the pose landmarks.
      pose_landmarks_proto = landmark_pb2.NormalizedLandmarkList()
      pose_landmarks_proto.landmark.extend([
        landmark_pb2.NormalizedLandmark(x=landmark.x, y=landmark.y, z=landmark.z) for landmark in pose_landmarks
      ])
      solutions.drawing_utils.draw_landmarks(
        annotated_image,
        pose_landmarks_proto,
        solutions.pose.POSE_CONNECTIONS,
        solutions.drawing_styles.get_default_pose_landmarks_style())
    return annotated_image
  
  def draw_angle(self, image, angle, center = None, rotation_angle = None, label_text = None, label_colour = None):  
    if center and rotation_angle:
      cv.ellipse(image, center, (30, 30), rotation_angle, 0, angle, (0, 255, 0), 2)
      
    cv.putText(image, format(angle, '.1f'), tuple(np.add(center, [-100, 10])), cv.FONT_HERSHEY_SIMPLEX, 0.6, (0, 255, 0), 2)
    
    if label_text and label_colour:
      cv.putText(image, label_text, tuple(np.add(center, [-100, 30])), cv.FONT_HERSHEY_SIMPLEX, 0.6, label_colour, 2)
      
  def draw_landmark(self, image: np.ndarray, landmarks, vis_threshold = 0.6, presence_threshold = 0.6):
    POSE_CONNECTIONS = [(c.start, c.end) for c in PoseLandmarksConnections.POSE_LANDMARKS]
    annotated_image = np.copy(image)
    
    # If no landmarks, return the original image
    if not landmarks:
      return annotated_image
    
    # Get image height and width
    height, width, _ = annotated_image.shape
    
    # Convert landmarks to coordinates / pixel values (x, y) where x and y are equal to math.floor
    landmark_to_coordinates = lambda landmark: (int(landmark.x * width), int(landmark.y * height))
    # Filter out landmarks with low confidence and return a dictionary of idx to coordinates / pixel
    # idx 0 to 10 are related to the face, not needed here
    idx_to_coordinates = {idx: landmark_to_coordinates(landmark) for idx, landmark in enumerate(landmarks) if idx > 10 and landmark.visibility > vis_threshold and landmark.presence > presence_threshold}
    
    indexes = idx_to_coordinates.keys()
    
    # idx not in idx_to_coordinates are filtered out
    connections = [(idx1, idx2) for idx1, idx2 in POSE_CONNECTIONS if idx1 in indexes and idx2 in indexes]
    
    # Draw connections
    for idx1, idx2 in connections:
      cv.line(annotated_image, idx_to_coordinates[idx1], idx_to_coordinates[idx2], (255, 255, 255), 2)
      
    # Draw horizontal line along the hips (may be useful for other calculations, need input from Charlie)
    # if 23 in indexes and 24 in indexes:
    #   right_hip = idx_to_coordinates[23]
    #   right_hip_x, right_hip_y = right_hip
    #   left_hip = idx_to_coordinates[24]
    #   left_hip_x, left_hip_y = left_hip
      
    #   # Front line by extending the x axis by 100 pixels
    #   cv.line(annotated_image, right_hip, (right_hip_x + 100, right_hip_y), (0, 0, 255), 2)
    #   cv.line(annotated_image, left_hip, (left_hip_x + 100, left_hip_y), (0, 0, 255), 2)
    #   # Back line by subtracting the x axis by 100 pixels
    #   cv.line(annotated_image, right_hip, (right_hip_x - 100, right_hip_y), (0, 0, 255), 2)
    #   cv.line(annotated_image, left_hip, (left_hip_x - 100, left_hip_y), (0, 0, 255), 2)
    
    # Calculate angle between (right inner knee angle) idx 24, 26, 28 or (left inner knee angle) idx 23, 25, 27
    if 24 in indexes and 26 in indexes and 28 in indexes:
      # right hip = 24, right knee = 26, right ankle = 28
      right_hip = idx_to_coordinates[24]
      right_knee = idx_to_coordinates[26]
      right_knee_x, right_knee_y = right_knee
      right_ankle = idx_to_coordinates[28]
      
      rotation_angle = self.calculate_angle((right_knee_x + 90, right_knee_y), right_knee, right_ankle)
      angle = self.calculate_angle(right_hip, right_knee, right_ankle)
      # cv.line(annotated_image, right_knee, (right_knee_x + 90, right_knee_y), (0, 0, 255), 2)
      
      print('right inner knee angle', angle)
      check, colour = None, None
      
      if int(angle) in range(90, 120):
        print('right inner knee angle is good')
        check = "GOOD"
        colour = (0, 255, 0)
      elif int(angle) <= 90:
        print('right inner knee angle is too LOW')
        check = "TOO LOW"
        colour = (255, 0, 0)
      elif int(angle) in range(120, 150):
        print('right inner knee angle can be lower')
        check = "LOWER"
        colour = (0, 255, 255)
      
      self.draw_angle(annotated_image, angle, right_knee, rotation_angle, check, colour)
    
    return annotated_image

  @abstractmethod
  def postprocessing(self, result):
    pass
  
  @abstractmethod
  def detect(self):
    pass
  
  @abstractmethod
  def image_detect(self, image_path: str):
    pass
  
  @abstractmethod
  def video_detect(self, video_path: str):
    pass

class BlazePoseEstimator(Estimator, Video):
  _model_path: str
  _connections: frozenset[(int, int)]

  def __init__(self, model_path: str):
    self._model_path = model_path

    self._options = PoseLandmarkerOptions(
      base_options=BaseOptions(model_asset_path=self._model_path),
      running_mode=RunningMode.IMAGE
    )
    
    self._connections = frozenset([(c.start, c.end) for c in PoseLandmarksConnections.POSE_LANDMARKS])

  def postprocessing(self, result):
    return result.pose_landmarks[0]
  
  def image_detect(self, image_path: str) -> Image:
    return mp.Image.create_from_file(image_path)
  
  def video_detect(self, video_path: str) -> Image:
    video = cv.VideoCapture(video_path)

    assert video.isOpened(), "Cannot open video"

    has_next, frame = video.read()
    
    while has_next:
      mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=frame)
      yield mp_image
      has_next, frame = video.read()
      
    video.release()

  def detect(self, path: str, is_video: bool = False):
    with PoseLandmarker.create_from_options(self._options) as landmarker:
      
      if is_video == False:
        image = self.image_detect(path)
        result = landmarker.detect(image)
        return image, result
        
      for frame in self.video_detect(path):
        result = landmarker.detect(frame)
        yield frame, result

class PoseEstimationWorker():
  _repo: Repo
  _estimator: Estimator
  _logger: logging.Logger
  
  def __init__(self, repo: Repo, estimator: Estimator) -> None:
    self._repo = repo
    self._estimator = estimator
    self._logger = logging.getLogger(__name__)
    
  def execute(self) -> None:
    # TODO: Implement processing
    while True:
      # TODO: iterate on the steps
      break
    
  def run(self):
    # TODO: Pull job from queue
    
    # TODO: Parse the string
    
    # TODO: Validate the schema
    
    video_path = "/Volumes/BACKUP/projects/test-projects/exercise-analyser/services/pose-detect/src/test.mp4"
    
    video = Video(video_path)
    
    type = self._estimator.exercise_types[ExerciseType.SQUAT]
    
    fourcc = cv.VideoWriter_fourcc(*"X264")
    new_video = cv.VideoWriter("processed.mp4", fourcc=fourcc, fps=video.fps, frameSize=video.shape, isColor=True)
    
    for frame, result in self._estimator.detect(video_path, is_video=True):
      result = self._estimator.postprocessing(result)
      # TODO: Calculate the angles
      key_interest_points_2d = type.get_2d_key_points(result, video.shape[1], video.shape[0])
      # key_interest_points_3d = type.get_3d_key_points(result.pose_world_landmarks[0])

      # TODO: draw th default landmarks on the image
      # annotated_image = self._estimator.draw_landmarks_on_image(frame.numpy_view(), result)
      annotated_image = self._estimator.draw_landmark(frame.numpy_view(), result)

      # TODO: Process the landmarks so it can be saved to the database
      
      cv.imwrite("test1.jpg", annotated_image)
      
      # TODO: save the image to video writer
      new_video.write(annotated_image)
      break
    
    new_video.release()

def main() -> None:
  logger = logging.getLogger(__name__)
  root_path = os.path.dirname(os.path.abspath(__file__))

  try:
    repo = CassandraRepo()
  except Exception as e:
    logger.fatal("cassandra repo init failed")
    
  try:
    estimator = BlazePoseEstimator(
      model_path=os.path.join(root_path, "models", "pose_landmarker_full.task")
    )
  except Exception as e:
    logger.fatal("pose estimator init failed")
    
  try:
    worker = PoseEstimationWorker(
      repo=repo,
      estimator=estimator
    )
  except Exception as e:
    print(e)
    logger.fatal("pose detection worker init failed: ", e)

  try:
    worker.run()
  except Exception as e:
    logger.fatal("worker run fatal error: ", e)
  
if __name__ == "__main__":
  main()