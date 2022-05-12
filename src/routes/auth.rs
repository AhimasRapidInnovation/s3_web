use crate::{db::Conn, models::users::{User, SignInUser}};
use actix_web::{web, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "user_management/login.html")]
pub struct LoginGet;

pub(crate) async fn auth_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(LoginGet.render().unwrap())
}

pub(crate) async fn signin(_conn: web::Data<Conn>, user: web::Form<SignInUser>) -> impl Responder {
    println!("{:?}", user);
    "Ok"
}
pub(crate) async fn login(_conn: web::Data<Conn>) -> impl Responder {
    "hello from login"
}
