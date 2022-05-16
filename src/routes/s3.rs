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
pub(crate) struct S3Home{
    buckets : Vec<String>
}

#[derive(Template)]
#[template(path = "s3/list_objects.html")]
pub(crate) struct S3Objects{
    objects : Vec<String>,
}


#[derive(serde::Deserialize, Debug)]
pub(crate) struct UploadForm {
    file_name: String,
}

pub(crate) async fn s3_home(session: Session, client : web::Data<crate::Client>) -> HttpResponse {
    
    let session_id = session.get::<String>("session_id").unwrap().unwrap();
    let client_guard = client.lock().await;
    let cl = client_guard.inner.get(&session_id).unwrap();
    let resp = cl.list_buckets().send().await.unwrap();
    let buckets = resp
                                .buckets()
                                .unwrap_or_default()
                                .into_iter()
                                .map(|buck| buck.name().unwrap().to_string())
                                // .flatten()
                                .collect::<Vec<String>>();
    let num_buckets = buckets.len();
    let home = S3Home {buckets : buckets}; 
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


pub(crate) async fn list_objects(query : web::Path<String>, session: Session, client : web::Data<crate::Client>) -> HttpResponse {

    let bucket = query.into_inner();
    let session_id = session.get::<String>("session_id").unwrap().unwrap();
    let client_guard = client.lock().await;
    let cl = client_guard.inner.get(&session_id).unwrap();
    let objects = cl.list_objects_v2().bucket(bucket).send().await.unwrap();
    let objs = objects
                .contents()
                .unwrap_or_default()
                .iter()
                .map(|obj|obj.key().unwrap().to_string())
                .collect::<Vec<_>>();
    let s3_objs = S3Objects{objects: objs};
    HttpResponse::Ok()
        .content_type("text/html")
        .body(s3_objs.render().unwrap())
}