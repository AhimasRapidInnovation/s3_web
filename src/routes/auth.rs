
use askama::Template;
use actix_web::{HttpResponse};


#[derive(Template)]
#[template(path="user_management/login.html")]
pub struct LoginGet;



pub async fn login_get() -> HttpResponse 
{
    HttpResponse::Ok()
        .content_type("text/html")
        .body(LoginGet.render().unwrap())
}
