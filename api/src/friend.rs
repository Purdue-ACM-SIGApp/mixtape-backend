use actix_web::{HttpResponse, Responder, route, web};
use serde::Deserialize;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(change_friend)
        .service(delete_friend)
        .service(find_friends);
}

#[route("/u/{id}/friend", method = "PUT")]
async fn change_friend(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Changed friend status of user with ID {}", id
    ))
}

#[route("/u/{id}/friend", method = "DELETE")]
async fn delete_friend(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Removed friend with user ID: {}", id
    ))
}

#[derive(Deserialize)]
struct FriendData {
    phone_numbers: Vec<String>,
}

#[route("/f/find", method = "POST")]
async fn find_friends(payload: web::Json<FriendData>) -> impl Responder {
    let phone_numbers = payload.phone_numbers.clone();

    HttpResponse::Ok().body(format!(
        "Found friends based upon the following phone numbers: {:?}", 
        phone_numbers
    ))
}