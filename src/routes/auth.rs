use crate::{
    db::Conn,
    models::users::{SessionModel, SignInUser, User, SESSION_TABLE, USER_TABLE},
};
use actix_session::Session;
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
pub(crate) async fn login(
    conn: web::Data<Conn>,
    user: web::Form<LoginUser>,
    session: Session,
) -> impl Responder {
    let LoginUser {
        user_name,
        password,
    } = user.into_inner();

    let cur = conn
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
            println!("Password matched");
            let new_session = SessionModel::new(user.id.unwrap().to_hex());
            match conn
                .collection(SESSION_TABLE)
                .insert_one(new_session, None)
                .await
            {
                Ok(inserted) => {
                    let _ = session.insert("session_id", inserted.inserted_id).unwrap();
                    println!("Session inserted successfully");
                }
                Err(e) => {
                    println!("Session error while inserting");
                }
            }
        }
        false => {
            println!("password is not matched");
        }
    }
    HttpResponse::SeeOther()
            .insert_header((LOCATION, "/s3"))
            .finish()
}
