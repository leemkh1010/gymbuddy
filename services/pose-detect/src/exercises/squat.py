from typing import Dict
from .base import KeyInterestPoint

class Squat(KeyInterestPoint):
  indexes_angle_of_interest = [
    (24, 26, 28), # Left hip, knee, ankle
    (12, 24, 26), # Left shoulder, hip, knee
  ]
  
  def calculate_2d_angle(self, key_points: dict) -> dict:
    """
    Calculate the angle between the hip, knee and ankle
    """
    hip = key_points["hip"]
    knee = key_points["knee"]
    ankle = key_points["ankle"]
    
    return self.calculate_angle(hip, knee, ankle)
  
  def calculate_2d_angles(self, key_points: dict) -> dict:
    pass
  
  def calculate_3d_angles(self, key_points: dict) -> dict:
    pass
  
  def get_2d_key_points(self, result, image_width, image_height) -> Dict[str, int]:
    key_points = []
    # Convert landmarks to coordinates / pixel values (x, y) where x and y are equal to math.floor
    landmark_to_coordinates = lambda landmark: (int(landmark.x * image_width), int(landmark.y * image_height))
    
    idx_to_coordinates = {idx: landmark_to_coordinates(landmark) for idx, landmark in enumerate(result)}
    
    indexes = idx_to_coordinates.keys()
    
    print(idx_to_coordinates)

    
    return key_points
  
  def get_3d_key_points(self, result) -> dict:
    key_points = []
    
    return key_points