use actix_web::{HttpResponse, Responder, route};
use build_time::build_time_utc;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(test);
}

#[route("/test", method = "GET", method = "HEAD")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Mixtape API Server TWO\nBuild Timestamp {}",
        build_time_utc!()
    ))
}