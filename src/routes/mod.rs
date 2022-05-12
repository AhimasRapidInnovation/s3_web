pub mod auth;
use actix_web::{web, Scope};

pub fn configure_auth() -> Scope {
    web::scope("/auth")
        .route("", web::get().to(auth::auth_index))
        .route("/signin", web::post().to(auth::signin))
        .route("/login", web::post().to(auth::login))
}
