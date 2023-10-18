use actix_web::{HttpResponse, Responder, route, web};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
    .service(block)
    .service(unblock);
}

#[route("/u/{id}/block", method = "PUT")]
async fn block(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Blocked user with id {}",
        id,
    ))
}

#[route("/u/{id}/block", method = "DELETE")]
async fn unblock(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Unblocked user with id {}",
        id,
    ))
}