from abc import abstractmethod, ABC
import numpy as np
from enum import Enum

class ExerciseType(Enum):
  SQUAT = 1
  PUSH_UP = 2

class KeyInterestPoint(ABC):
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

  @abstractmethod
  def calculate_2d_angles(self):
    pass

  @abstractmethod
  def calculate_3d_angles(self):
    pass
  
  @abstractmethod
  def get_2d_key_points(self, result):
    pass
  
  @abstractmethod
  def get_3d_key_points(self, result):
    pass