
use argon2::{Argon2, Algorithm, Version,Params, PasswordHasher};
use serde::{Serialize, Deserialize};
use secrecy::{ExposeSecret, Secret};

use argon2::password_hash::{SaltString,PasswordVerifier, PasswordHash};



pub fn my_password_hash(password: String)-> Result<String, Box<dyn std::error::Error>>{
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hasher = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000,2, 1, None).or_else(|_| Err("hello".to_string()))?
    );
    let password_hasher = hasher
                        .hash_password(password.as_bytes(), &salt).unwrap();
    
    println!("{:?}",password_hasher.encoding());
    let op = password_hasher.to_string();
    Ok(
        op
    )
}

pub fn decode_password_hash<'a>(password: &'a str) -> bool {
    let ph = PasswordHash::new(password).unwrap();
    Argon2::default()
        .verify_password(
            password.as_bytes(), &ph
        ).is_ok()
    
}

#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct Signin {
    name : String,
    access_key_id: String,
    secret_key: String,
    password: String,
}



