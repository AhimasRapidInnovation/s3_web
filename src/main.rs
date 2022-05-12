use actix_files::NamedFile;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{web, App, HttpServer};
use actix_web::{HttpRequest, Result};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use cookie::Key;
use dotenv::dotenv;
use rand::RngCore;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // let secret_key = std::env::var(&"SESSION_SECRET_KEY").expect("Set session secret key");
    let mongo_uri = std::env::var("MONGO_URL")?;
    let db = s3_web::Conn::new(mongo_uri).await?;
    let mut random_key = [0u8; 64];
    // make sure your key cryptographically safe
    rand::thread_rng().fill_bytes(&mut random_key);
    let secret_key = Key::from(&random_key);

    let message_store = CookieMessageStore::builder(secret_key.clone()).build();

    let message_framework = FlashMessagesFramework::builder(message_store).build();

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(s3_web::configure_auth())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
    Ok(())
}
