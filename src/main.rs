
extern crate s3_web;

use dotenv::dotenv;
use actix_web::{
    HttpServer,
    App,
    web,
};
use actix_session::{SessionMiddleware, storage::CookieSessionStore, Session};
use rand::RngCore;
use cookie::Key;


async fn s3_test(session: Session) -> String{

    let s = session.get::<i32>("counter");
    println!("counter {:?}",s);
    "Hello ".into()
}


async fn index(session: Session) -> Result<&'static str, Box<dyn std::error::Error>> {
    // access the session state
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        // modify the session state
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    Ok("Welcome!")
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    dotenv().ok();

    // let secret_key = std::env::var(&"SESSION_SECRET_KEY").expect("Set session secret key");
    let mongo_uri = std::env::var("MONGO_URL")?;
    let db = s3_web::Conn::new(mongo_uri).await?;
    let mut random_key = [0u8;64];
    // make sure your key cryptographically safe
    rand::thread_rng().fill_bytes(&mut random_key);
    let secret_key = Key::from(&random_key);

   let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
            .route("/hello", web::get().to(s3_test))
            .route("/", web::get().to(index))
            .route("/login", web::get().to(s3_web::login_get))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
  Ok(())
}
