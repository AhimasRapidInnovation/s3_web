#![allow(unused_attributes)]

pub mod db;
pub mod models;
pub mod routes;

pub use db::Conn;

pub use routes::{
    configure_auth,
    configure_s3_service
};

pub const NAME: &str = "NAME";
