use anyhow::Result;

struct BaseStorage {}

pub trait Storage {
    async fn sign_get_public_url(&self, key: &str, expires_in: u64) -> Result<String>;

    async fn sign_put_public_url(&self, key: &str, expires_in: u64) -> Result<String>;

    async fn move_object(&self) -> String;
}
