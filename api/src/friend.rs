use actix_web::{HttpResponse, Responder, route, web};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(delete_friend)
        .service(find_friend);
}

#[derive(Debug, Deserialize, Serialize)]
struct FindFriendRequest {
    phone_numbers: Vec<String>,
}

#[route("/u/{id}/friend", method= "DELETE")]
async fn delete_friend(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Deleted friend with id {}", id
    ))
}

#[route("/f/find", method= "POST")]
async fn find_friend(request: web::Json<FindFriendRequest>) -> impl Responder {
    let phone_numbers: Vec<String> = request.phone_numbers.clone();

    HttpResponse::Ok().body(format!(
        "Found friends based upon these phone numbers: {:?}", phone_numbers
    ))
}