use actix_web::{HttpResponse, Responder, route, web};
use build_time::build_time_utc;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
    .service(test)
    .service(delete_device)
    .service(add_device);
}

#[route("/test", method = "GET")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Mixtape API Server TWO\nBuild Timestamp {}",
        build_time_utc!()
    ))
}

#[route("/u/me/device", method = "PUT")]
async fn add_device(device_info: web::Json<DeviceInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Added device with id {}",
        device_info.id,
    ))
}

#[route("/u/me/device/{id}", method = "DELETE")]
async fn delete_device(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Deleted device with id {}",
        id,
    ))
}