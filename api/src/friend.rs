use actix_web::{HttpResponse, Responder, route, web};
use serde::{Deserialize, Serialize};
use build_time::build_time_utc;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
  cfg
    .service(delete_friend)
    .service(find_friend)
    .service(get_friends)
    .service(share);
}

#[derive(Debug, Deserialize, Serialize)]
struct FindFriendRequest {
    phone_numbers: Vec<String>,
}

struct FriendRet {
    id: string,
    requester: string,
    receiver: string,
    status: FriendStatus
}

mod internal {
  async fn get_friends(db: &Database, user:ID) -> anyhow::Result<Vec<FriendRet>> {

    // let client = Client::with_uri_str("mongodb://localhost:27017/your_database").await?;
    // let db = client.database("your_database");

    // let collection_name = "friends"; // Change this to the actual collection name

    // let collection: Collection<FriendRet> = db.collection(&collection_name);

    // let user_id: String = user.into(); // Assuming user is convertible to String

    let filter = doc! {"_id": user};

    let cursor = collection.find(filter, None).await?;

    let mut friends: Vec<FriendRet> = vec![];
    for result in cursor {
        let friend: FriendRet = result?;
        friends.push(friend);
    }

    Ok(friends);
  }

  async fn delete_friend(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!(
        "Deleted friend with id {}", id
    ))
  }

  async fn find_friend(request: web::Json<FindFriendRequest>) -> impl Responder {
    let phone_numbers: Vec<String> = request.phone_numbers.clone();

    HttpResponse::Ok().body(format!(
        "Found friends based upon these phone numbers: {:?}", phone_numbers
    ))
  }

  async fn share(db: &Database, user:ID, phone:&impl Phone, numbers: Vec<String>) -> impl Responder {

    let phone_numbers: Vec<String> = numbers.clone();

    // Shares Mixtape with friends via SMS (currently it's just a HttpResponse,
    // but there will be functionality to send text messages)

    let message = "🎶 Introducing Mixtape: The Ultimate Playlist Collaboration App! 🎶\n
        Hey there, music lovers! 🎧\n
        I've just stumbled upon something incredible called Mixtape, and it's a game-changer for music enthusiasts like us:\n
        🤝 Collaborate with friends on the perfect playlist.\n
        🎵 Syncs seamlessly with Spotify and Apple Music.\n
        💬 Share your amazing playlists effortlessly.\n
        Get started today! Download Mixtape now:\n
        App Store: [Insert App Store Link]\n
        Google Play: [Insert Google Play Link]\n";
    
    HttpResponse::Ok().body(message)

  }
}

#[route("/f", method = "GET")]
async fn get_friends(data:Data<State>, user:AuthedUser) -> anyhow::Result<Vec<FriendRet>> {

    let filter = doc! {"_id": user};

    let cursor = collection.find(filter, None).await?;

    let mut friends: Vec<FriendRet> = vec![];
    for result in cursor {
        let friend: FriendRet = result?;
        friends.push(friend);
    }

    Ok(friends);
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

#[route("/f/share", method = "POST")]
async fn share(data:Data<State>, user:AuthedUser) -> impl Responder {

    let phone_numbers: Vec<String> = numbers.clone();

    // Shares Mixtape with friends via SMS (currently it's just a HttpResponse,
    // but there will be functionality to send text messages)

    let message = "🎶 Introducing Mixtape: The Ultimate Playlist Collaboration App! 🎶\n
        Hey there, music lovers! 🎧\n
        I've just stumbled upon something incredible called Mixtape, and it's a game-changer for music enthusiasts like us:\n
        🤝 Collaborate with friends on the perfect playlist.\n
        🎵 Syncs seamlessly with Spotify and Apple Music.\n
        💬 Share your amazing playlists effortlessly.\n
        Get started today! Download Mixtape now:\n
        App Store: [Insert App Store Link]\n
        Google Play: [Insert Google Play Link]\n";
    
    HttpResponse::Ok().body(message)

}