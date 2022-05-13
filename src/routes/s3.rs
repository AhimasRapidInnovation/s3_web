use std::path::Path;
// use std::fs;
use tokio::{fs::File, io::AsyncWriteExt};

use actix_multipart::{Field, Multipart};
use actix_session::Session;
use actix_web::{http::header::DispositionParam, web, HttpResponse};
use askama::Template;
use futures::StreamExt;

#[derive(Template)]
#[template(path = "s3/home.html")]
pub(crate) struct S3Home;

#[derive(serde::Deserialize, Debug)]
pub(crate) struct UploadForm {
    file_name: String,
}

pub(crate) async fn s3_home(session: Session) -> HttpResponse {
    let home = S3Home {};
    HttpResponse::Ok()
        .content_type("text/html")
        .body(home.render().unwrap())
}

async fn save_file(mut field: Field) {
    let file_name = field.content_disposition().get_filename().unwrap();
    let path = Path::new("files").join(Path::new(file_name));
    let mut file = File::create(path).await.unwrap();
    while let Some(chunk) = field.next().await {
        let _ = file.write_all(&chunk.unwrap()).await;
    }
}

pub(crate) async fn upload_file(mut payload: Multipart) -> HttpResponse {
    while let Some(item) = payload.next().await {
        let field = item.unwrap();
        let name = field.content_disposition().get_name();
        match name {
            Some("s3-file") => {
                save_file(field).await;
            }
            Some(_) => {}
            _ => {}
        };
    }
    HttpResponse::Ok().finish()
}
