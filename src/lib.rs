#![allow(unused_attributes)]


pub mod models;
pub mod db;

pub use db::Conn;


pub use models::{my_password_hash,decode_password_hash};


pub const NAME : &str = "NAME";



