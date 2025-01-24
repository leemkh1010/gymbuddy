use aws_credential_types::Credentials;
use aws_sdk_s3::config::ProvideCredentials;
use core::panic;
use log::warn;
use std::env::var;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ENV {
    db_schema: String,
    db_host: String,
    db_port: u16,
    db_user: String,
    db_password: String,
    db_name: String,

    media_endpoint: Option<String>,
    media_region: String,
    media_access_key: String,
    media_secret_key: String,
    media_bucket: String,
}

impl ENV {
    pub fn new() -> Self {
        let env = dotenv::dotenv();

        if let Some(err) = env.err() {
            panic!("fail env init: {:?}", err);
        }

        ENV {
            db_schema: Self::get_env("DB_SCHEMA", "mongodb"),
            db_host: Self::get_env("DB_HOST", "localhost"),
            db_port: Self::parse_port(Self::get_env("DB_PORT", "9042")),
            db_user: Self::get_env("DB_USER", "admin"),
            db_password: Self::get_env("DB_PASSWORD", "local"),
            db_name: Self::get_env("DB_NAME", "exercise_analyser"),

            media_endpoint: Self::optional_env("MEDIA_ENDPOINT"),
            media_region: Self::get_env("MEDIA_REGION", "ap-southeast-1"),
            media_access_key: Self::must_have_env("MEDIA_ACCESS_KEY"),
            media_secret_key: Self::must_have_env("MEDIA_SECRET_KEY"),
            media_bucket: Self::get_env("MEDIA_BUCKET", "exercise-analyser"),
        }
    }

    pub fn get_db_name(&self) -> String {
        self.db_name.clone()
    }

    pub fn get_db_conn_string(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.db_schema,
            self.db_user,
            self.db_password,
            self.db_host,
            self.db_port,
            self.db_name
        )
    }

    pub fn get_media_endpoint(&self) -> Option<&String> {
        self.media_endpoint.as_ref()
    }

    pub fn get_media_region(&self) -> &String {
        &self.media_region
    }

    pub fn get_media_access_key(&self) -> &String {
        &self.media_access_key
    }

    pub fn get_media_secret_key(&self) -> &String {
        &self.media_secret_key
    }

    pub fn get_media_bucket(&self) -> &String {
        &self.media_bucket
    }

    fn get_env(key: &'static str, default: &'static str) -> String {
        match var(key) {
            Ok(val) => val,
            Err(_) => {
                warn!("{} not found, using default value", key);
                default.to_string()
            }
        }
    }

    fn optional_env(key: &'static str) -> Option<String> {
        match var(key) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    fn must_have_env(key: &'static str) -> String {
        match var(key) {
            Ok(val) => val,
            Err(_) => {
                panic!("{} not found, exiting", key);
            }
        }
    }

    fn parse_port(port: String) -> u16 {
        match port.parse() {
            Ok(port) => port,
            Err(_) => panic!("fail to parse port"),
        }
    }
}

#[derive(Debug)]
pub struct AWSStaticCredentials {
    access_key: String,
    secret_key: String,
}

impl AWSStaticCredentials {
    pub fn new(access_key: String, secret_key: String) -> Self {
        AWSStaticCredentials {
            access_key,
            secret_key,
        }
    }

    async fn load_credentials(&self) -> aws_credential_types::provider::Result {
        Ok(Credentials::new(
            self.access_key.clone(),
            self.secret_key.clone(),
            None,
            None,
            "StaticCredentials",
        ))
    }
}

impl ProvideCredentials for AWSStaticCredentials {
    fn provide_credentials<'a>(
        &'a self,
    ) -> aws_credential_types::provider::future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        aws_credential_types::provider::future::ProvideCredentials::new(self.load_credentials())
    }
}
