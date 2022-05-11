

use dotenv::dotenv;
use actix_web::{
    HttpServer,
    App,
    web,
};




async fn s3_test() -> String{


    "Hello ".into()
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

   HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(s3_test))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
