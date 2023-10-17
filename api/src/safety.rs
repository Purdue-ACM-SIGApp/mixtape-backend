use actix_web::{HttpResponse, Responder, route, web};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(report_user);
}

#[route("/u/{id}/report", method = "POST")]
async fn report_user(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Reported user with ID: {}", id
    ))
}