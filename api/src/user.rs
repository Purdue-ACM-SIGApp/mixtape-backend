use actix_web::{HttpResponse, Responder, route, web};
use build_time::build_time_utc;
use serde::Deserialize;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(test)
        .service(me)
        .service(search);
}

#[route("/test", method = "GET", method = "HEAD")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Mixtape API Server TWO\nBuild Timestamp {}",
        build_time_utc!()
    ))
}

#[route("/me", method = "PUT")]
async fn me() -> impl Responder {
    HttpResponse::Ok().body(format!{
        "/u/me"
    })
}

#[derive(Deserialize)]
struct SearchInfo {
    query: String,
}

#[route("/search", method = "GET")]
async fn search(search_info: web::Json<SearchInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Success!\n/u/search with a query of: \"{}\"",
        search_info.query,
    ))
}