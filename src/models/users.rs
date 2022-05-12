use argon2::password_hash::{PasswordHash, PasswordVerifier, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use mongodb::bson::oid::ObjectId;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

// use actix_web::{FromRequest};

pub(crate) const USER_TABLE: &str = "users";
pub(crate) const SESSION_TABLE: &str = "session";

fn password_to_phc(password: String) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hasher = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).or_else(|_| Err("hello".to_string()))?,
    );
    let password_hasher = hasher.hash_password(password.as_bytes(), &salt).unwrap();

    let op = password_hasher.to_string();
    Ok(op)
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    name: String,
    access_key_id: String,
    secret_key: String,
    password: String,
}

impl User {
    pub(crate) fn new(
        name: String,
        access_key_id: String,
        secret_key: String,
        password: String,
    ) -> Self {
        let password = password_to_phc(password).unwrap();
        Self {
            id: Some(ObjectId::new()),
            name,
            access_key_id,
            secret_key,
            password,
        }
    }

    pub(crate) fn verify_password<'a>(&self, password: &'a str) -> bool {
        let ph = PasswordHash::new(&self.password).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &ph)
            .is_ok()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SignInUser {
    user_name: String,
    user_password: String,
    confirm_password: String,
    access_key_id: String,
    secret_key: String,
}

impl SignInUser {
    pub(crate) fn validate(&self) -> bool {
        self.user_password == self.confirm_password
    }
}

impl From<SignInUser> for User {
    fn from(user: SignInUser) -> Self {
        Self::new(
            user.user_name,
            user.access_key_id,
            user.secret_key,
            user.user_password,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SessionModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    user_id: String,
}

impl SessionModel {
    pub(crate) fn new(user_id: String) -> Self {
        Self { id: None, user_id }
    }
}
