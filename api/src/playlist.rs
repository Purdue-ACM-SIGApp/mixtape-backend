use actix_web::{HttpResponse, Responder, route, web};
use futures_util::TryStreamExt;
use mongodb::bson::Document;

use crate::State;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(create_playlist)
        .service(get_playlists)
        .service(get_playlist)
        .service(update_playlist)
        .service(delete_playlist);
}

#[route("/p/new", method = "POST")]
async fn create_playlist(data: web::Data<State>) -> impl Responder {
    let db = data.database();
    let collection = db.collection::<Document>("fakecollection");
    let mut cursor = collection.find(None, None).await.unwrap();

    while let Some(doc) = cursor.try_next().await.unwrap() {
        // Print each document
        println!("{:#?}", doc);
    }

    HttpResponse::Ok().body(format!(
        "Created the playlist"
    ))
}

#[route("/p", method = "GET")]
async fn get_playlists() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Got some playlists."
    ))
}

#[route("/p/{id}", method = "GET")]
async fn get_playlist(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Found playlist with ID: {}", id
    ))
}

#[route("/p/{id}", method = "PUT")]
async fn update_playlist(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Updated playlist with ID: {}", id
    ))
}

#[route("/p/{id}", method = "DELETE")]
async fn delete_playlist(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Deleted playlist with ID: {}", id
    ))
}