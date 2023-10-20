use actix_web::{HttpResponse, Responder, route, web};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(create_playlist)
        .service(get_playlists)
        .service(get_playlist);
}

#[route("/p/new", method = "POST")]
async fn create_playlist() -> impl Responder {
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