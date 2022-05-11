#![allow(unused_attributes)]


pub mod models;
pub mod db;
pub mod routes;

pub use db::Conn;


pub use models::{my_password_hash,decode_password_hash};
pub use routes::auth::login_get;


pub const NAME : &str = "NAME";



