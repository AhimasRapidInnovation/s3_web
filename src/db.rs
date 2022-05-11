use std::ops::Deref;

use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig},
    Client, Database,
};


const DB_NAME : &str = "s3_web";


#[derive(Clone, Debug)]
pub struct Conn(pub Database);

impl Conn {
    pub async fn new(uri: String) -> Result<Self, Error> {
        let options =
            ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;
        let client = Client::with_options(options)?;
        let db = client.database(DB_NAME);
        Ok(Self(db))
    }
}

impl Deref for Conn {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

