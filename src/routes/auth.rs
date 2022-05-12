use crate::{
    db::Conn,
    models::users::{SignInUser, User, USER_TABLE},
};
use actix_web::{http::header::LOCATION, web, HttpResponse, Responder};
use askama::Template;
use futures::StreamExt;
use mongodb::bson::doc;

#[derive(Template)]
#[template(path = "user_management/login.html")]
pub struct LoginGet;

#[derive(serde:: Deserialize)]
pub(crate) struct QueryParams {
    error: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct LoginUser {
    user_name: String,
    password: String,
}

pub(crate) async fn auth_index(error: web::Query<QueryParams>) -> HttpResponse {
    let e = error.into_inner();
    if e.error.is_none() {
        // add flash message
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(LoginGet.render().unwrap())
}

pub(crate) async fn signin(conn: web::Data<Conn>, user: web::Form<SignInUser>) -> HttpResponse {
    println!("{:?}", user);
    if !user.validate() {
        return HttpResponse::SeeOther()
            .insert_header((LOCATION, "/auth"))
            .finish();
    }

    let new_user: User = user.into_inner().into();

    match conn
        .collection(crate::models::users::USER_TABLE)
        .insert_one(new_user, None)
        .await
    {
        Ok(inserted) => {
            println!("Inserted new record");
        }
        Err(e) => {
            println!("Got Error");
        }
    }
    HttpResponse::Ok().finish()
}
pub(crate) async fn login(conn: web::Data<Conn>, user: web::Form<LoginUser>) -> impl Responder {
    let LoginUser {
        user_name,
        password,
    } = user.into_inner();

    let mut cur = conn
        .collection::<User>(USER_TABLE)
        .find(doc! {"name":&user_name }, None)
        .await
        .unwrap();
    let db_user = cur.collect::<Vec<_>>().await;

    if db_user.is_empty() {
        //  TODO return invalid username or password
        return HttpResponse::Conflict().finish();
    }
    if db_user.len() != 1 {
        return HttpResponse::InternalServerError().finish();
    }

    let user = db_user[0].as_ref().unwrap();
    match user.verify_password(&password) {
        true => {
            // TODO
            // Create a session and add cookie
            //Add cookie
            //
            println!("Password matched");
        }
        false => {
            println!("password is not matched");
        }
    }
    HttpResponse::Ok().finish()
}
