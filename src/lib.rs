#![allow(unused_attributes)]

pub mod db;
pub mod models;
pub mod routes;

pub use db::Conn;

pub use models::{decode_password_hash, my_password_hash};
pub use routes::configure_auth;

pub const NAME: &str = "NAME";
