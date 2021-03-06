pub mod auth;
pub mod s3;

use actix_web::{web, Scope};

pub fn configure_auth() -> Scope {
    web::scope("/auth")
        .route("", web::get().to(auth::auth_index))
        .route("/signin", web::post().to(auth::signin))
        .route("/login", web::post().to(auth::login))
}

pub fn configure_s3_service() -> Scope {
    web::scope("/s3")
        .route("", web::get().to(s3::s3_home))
        .route("/upload", web::post().to(s3::upload_file))
        .route("/list_ojects/{bucket}", web::get().to(s3::list_objects))
        .route("/uploads3", web::post().to(s3::upload_s3))
        .route("/create_bucket", web::post().to(s3::create_bucket))
        .route("/download_object", web::get().to(s3::download_object))
        .route("/presigned_uri", web::get().to(s3::presigned_uri))
}
