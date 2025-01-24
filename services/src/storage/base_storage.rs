use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ObjectStorageReference {
    schema: String,
    bucket: String,
    key: String,
}

impl ObjectStorageReference {
    pub fn new(schema: String, bucket: String, key: String) -> Self {
        ObjectStorageReference {
            schema,
            bucket,
            key,
        }
    }

    pub fn to_location_path(&self) -> String {
        format!("{}/{}", self.bucket, self.key)
    }

    pub fn to_url(&self) -> String {
        format!("{}://{}/{}", self.schema, self.bucket, self.key)
    }
}

pub trait Storage {
    fn to_object_storage_reference(&self, key: String) -> ObjectStorageReference;

    async fn sign_get_public_url(&self, key: &str, expires_in: u64) -> Result<String>;

    async fn sign_put_public_url(&self, key: &str, expires_in: u64) -> Result<String>;

    async fn move_object(&self) -> String;
}
