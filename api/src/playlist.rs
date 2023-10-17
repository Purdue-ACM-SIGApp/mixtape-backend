use actix_web::{HttpResponse, Responder, route};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(create_playlist)
        .service(get_playlist);
}

#[route("/p/new", method = "POST")]
async fn create_playlist() -> impl Responder {
    HttpResponse::Ok().body("Created playlist.")
}

#[route("/p", method = "GET")]
async fn get_playlist() -> impl Responder {
    HttpResponse::Ok().body("Got some playlists.")
}