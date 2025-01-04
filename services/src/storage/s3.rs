use std::time::Duration;

use crate::utils::{AWSStaticCredentials, ENV};

use anyhow::{anyhow, Result};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    config::SharedCredentialsProvider,
    operation::{get_object::GetObjectError, put_object::PutObjectError},
    presigning::PresigningConfig,
    Client,
};

use super::Storage;

pub struct S3 {
    bucket: String,
    client: Box<Client>,
}

impl S3 {
    pub async fn new(env: &ENV) -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;

        let mut new_config = config.into_builder();

        if let Some(endpoint) = env.get_media_endpoint() {
            new_config = new_config.endpoint_url(endpoint);
        }

        new_config = new_config.region(Region::new(env.get_media_region().clone()));

        new_config = new_config.credentials_provider(SharedCredentialsProvider::new(
            AWSStaticCredentials::new(
                env.get_media_access_key().clone(),
                env.get_media_secret_key().clone(),
            ),
        ));

        let client = Client::new(&new_config.build());

        let buckets = client.list_buckets().send().await;

        match buckets {
            Ok(_) => {}
            Err(e) => {
                panic!("s3 init error: {:#?}", e);
            }
        }

        S3 {
            bucket: env.get_media_bucket().clone(),
            client: Box::new(client),
        }
    }

    fn get_client(&self) -> &Client {
        &self.client
    }
}

impl Storage for S3 {
    async fn sign_get_public_url(&self, key: &str, expires_in: u64) -> Result<String> {
        let client = self.get_client();

        let expires_in = Duration::from_secs(expires_in);

        let presigned = client
            .get_object()
            .bucket(self.bucket.clone())
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in).expect("presigning config"))
            .await;

        match presigned {
            Ok(url) => Ok(url.uri().to_string()),
            Err(e) => match e.into_service_error() {
                GetObjectError::InvalidObjectState(_) => Err(anyhow!("invalid object state")),
                GetObjectError::NoSuchKey(_) => Err(anyhow!("key not found")),
                _ => Err(anyhow!("error signing get object url")),
            },
        }
    }

    async fn sign_put_public_url(&self, key: &str, expires_in: u64) -> Result<String> {
        let client = self.get_client();

        let expires_in = Duration::from_secs(expires_in);

        let presigned = client
            .put_object()
            .bucket(self.bucket.clone())
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in).expect("presigning config"))
            .await;

        match presigned {
            Ok(url) => Ok(url.uri().to_string()),
            Err(e) => match e.into_service_error() {
                PutObjectError::InvalidRequest(_) => Err(anyhow!("invalid request")),
                _ => Err(anyhow!("error signing put object url")),
            },
        }
    }

    async fn move_object(&self) -> String {
        todo!()
    }
}
