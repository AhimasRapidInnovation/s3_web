#![allow(unused_attributes)]

use aws_sdk_s3::Client as S3Client;
use aws_types::credentials::{Credentials, SharedCredentialsProvider};
use aws_types::{region::Region, sdk_config};
use actix_web::{web};
use std::{collections::HashMap, hash::Hash};
use tokio::sync::Mutex;
pub mod db;
pub mod models;
pub mod routes;
pub use db::Conn;
use std::ops::{Deref, DerefMut};

pub use routes::{configure_auth, configure_s3_service};

pub const NAME: &str = "NAME";

pub struct Client(Mutex<HashMap<String, S3Client>>);

impl Deref for Client {
    type Target = Mutex<HashMap<String, S3Client>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Client {
    pub fn new() -> Mutex<HashMap<String, S3Client>> {
        Mutex::new(HashMap::new())
    }

    async fn create_new(&mut self, 
                        session_id: impl Into<String>, 
                        region: impl Into<String>,
                        access_key_id : impl Into<String>,
                        secret_access_key : impl Into<String>,
    ) 
    {
        //  take access_key_id and secret access key from database
        let creds = Credentials::from_keys(
            access_key_id,
            secret_access_key,
            None,
        );
        let sc = SharedCredentialsProvider::new(creds);
        let sdk = sdk_config::SdkConfig::builder()
            .credentials_provider(sc)
            .region(Region::new(region.into()))
            .build();
        let client = aws_sdk_s3::Client::new(&sdk);
        let mut lock = self.0.lock().await;
        (*lock).insert(session_id.into(), client);
    }
}
