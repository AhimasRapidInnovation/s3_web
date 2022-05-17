use std::path::Path;
// use std::fs;
use tokio::{fs::File, io::AsyncWriteExt};

use actix_multipart::{Field, Multipart};
use actix_session::Session;
use actix_web::{http::header::{DispositionParam, LOCATION}, web, HttpResponse};
use askama::Template;
use futures::StreamExt;
use aws_sdk_s3::model::{
    BucketLocationConstraint,
    CreateBucketConfiguration
};



#[derive(Template)]
#[template(path = "s3/home.html")]
pub(crate) struct S3Home{
    buckets : Vec<String>
}

#[derive(Template)]
#[template(path = "s3/list_objects.html")]
pub(crate) struct S3Objects{
    objects : Vec<String>,
    bucket_name : String,
}


#[derive(serde::Deserialize, Debug)]
pub(crate) struct UploadForm {
    file_name: String,
}

#[derive(serde:: Deserialize, Debug)]
pub(crate) struct CreateBucketForm{
    name : String,
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
    println!("list_obj session_id {:?}" , session_id);
    let client_guard = client.lock().await;
    let cl = client_guard.inner.get(&session_id).unwrap();
    let objects = cl.list_objects_v2().bucket(&bucket).send().await.unwrap();
    let objs = objects
                .contents()
                .unwrap_or_default()
                .iter()
                .map(|obj|obj.key().unwrap().to_string())
                .collect::<Vec<_>>();
    let s3_objs = S3Objects{objects: objs, bucket_name: bucket};
    println!("client after list obj {:?}", client_guard);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(s3_objs.render().unwrap())
}

async fn read_string(mut field: Field) -> String
{
    let mut rv= String::new();
    while let Some(chunk) = field.next().await {
        let bytes = chunk.unwrap();
        let new_str = String::from_utf8_lossy(&bytes);
        rv.push_str(&new_str);
    }
    rv
}

async fn read_to_bytes(mut field: Field) -> (String, Vec<u8>) {
    let file_name = field.content_disposition().get_filename().unwrap().to_string();
    let mut rv = vec![];
    while let Some(chunk) = field.next().await {
        let _ = rv.extend(&chunk.unwrap());
    }
    (file_name, rv)
}


pub(crate) async fn upload_s3(mut payload: Multipart,session: Session, client : web::Data<crate::Client>) -> HttpResponse {
    println!("calling upload_s3");
    let mut  bucket_name = None;
    let mut field_item = None;
    let session_id = session.get::<String>("session_id").unwrap().unwrap();
    println!("Session ID {:?} ", session_id);
    let client_guard = client.lock().await;
    println!("Client guard {:?}", client_guard);
    let cl = client_guard.inner.get(&session_id).unwrap();
    while let Some(item) = payload.next().await {
        let field = item.unwrap();
        let name = field.content_disposition().get_name();
        println!("at 112 {:?}", name);
        match name {
            Some("s3-file") => {
                println!("reading s3 file");
                field_item = Some(read_to_bytes(field).await);
                println!("reading is done");
            },
            Some("bucket_name") => {
                println!("reading bucket_name");
                let name = read_string(field).await;
                println!("done with reading bucket_name");
                bucket_name = Some(name);
            },
            Some(_) => {},
            _ => {}
        };
    }
    if field_item.is_some() && bucket_name.is_some(){
            let (file_name, body) = field_item.unwrap();
            let uploaded_res = cl
                                        .put_object()
                                        .bucket(bucket_name.unwrap())
                                        .key(file_name)
                                        .body(body.into())
                                        .send()
                                        .await;
            println!("uploaded !!");
    }
    HttpResponse::Ok().finish()
}

pub(crate) async fn create_bucket(session: Session, form : web::Form<CreateBucketForm>, client: web::Data<crate::Client>) -> HttpResponse {
    let form = form.into_inner();
    let session_id = session.get::<String>("session_id").unwrap().unwrap();
    let client_guard = client.lock().await;
    let cl = client_guard.inner.get(&session_id).unwrap();
    match cl
            .create_bucket()
            // .create_bucket_configuration(cfg)
            .bucket(form.name.as_str())
            .send()
            .await
        {
            Ok(res) => {
                println!("bucket created successfully !");
                HttpResponse::SeeOther()
                    .insert_header((LOCATION, format!("/s3/list_ojects/{}", form.name)))
                    .finish()
            },
            Err(err) => {
                println!("bucket creation error {:?}", err);
                HttpResponse::Conflict().finish()
            }
        }
    
}