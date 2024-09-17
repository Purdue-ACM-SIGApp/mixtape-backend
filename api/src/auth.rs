use actix_web::{HttpResponse, Responder, route, web};
use build_time::build_time_utc;
use serde::Deserialize;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(test)
        .service(auth)
        .service(register);
}

#[derive(Deserialize)]
struct LoginInfo {
    phone_number : String,
    code : String,

}

#[route("/auth", method = "POST")]
async fn auth(login_info: web::Json<LoginInfo>) -> impl Responder {
    if (login_info.code.is_empty()) {
        HttpResponse::Ok().body(format!(
            "User with phone number {} exists",
            login_info.phone_number,
        ))
    } else {
        HttpResponse::Ok().body(format!(
            "User with phone number {} has been authenticated",
            login_info.phone_number,
        ))
    }
}

#[route("/register-phone", method = "POST")]
async fn register(login_info: web::Json<LoginInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "User with phone number {} has been registered",
        login_info.phone_number,
    ))
}

#[route("/test", method = "GET", method = "HEAD")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Mixtape API Server TWO\nBuild Timestamp {}",
        build_time_utc!()
    ))
}









