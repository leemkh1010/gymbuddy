use anyhow::Result;

struct BaseStorage {}

pub struct RemoteStorageReference {
    
}

pub struct ObjectStorageReference {
    schema: String,
    bucket: String,
    key: String,
}

impl ObjectStorageReference {
    pub fn to_url(&self) -> String {
        format!("{}://{}/{}", self.schema, self.bucket, self.key)
    }
}

pub trait Storage {
    async fn sign_get_public_url(&self, key: &str, expires_in: u64) -> Result<String>;

    async fn sign_put_public_url(&self, key: &str, expires_in: u64) -> Result<String>;

    async fn move_object(&self) -> String;
}
