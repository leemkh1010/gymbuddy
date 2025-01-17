from pymongo import MongoClient
from .Repo import ExternalClient

class MongoDB(ExternalClient):
  _client: MongoClient = None
  
  def __init__(self, conn_str: str, db: str):
    super().__init__(conn_str)
    self.db = db
    
  def connect(self):
    if self._client is not None:
      self.logger.warning("Already connected to MongoDB")
      return
    
    self._client = MongoClient(self.conn_str)
    
  def get_instance(self):
    if self._client is None:
      raise Exception("MongoDB not connected")
    
    return self._client[self.db]

  def health_check(self):
    print("health check")
    
  def is_connected(self) -> bool:
    return True
  
  def close(self):
    self._client.close()
    self._client = None