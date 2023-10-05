use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug,Deserialize,Serialize)]
pub struct User {
    id: Option<ObjectId>,
    phonenumber: String,
    name: String,
    username: String,
    sessions: Vec<Session>,
    devices: Vec<UserDevice>,
    last_active: DateTime<Utc>,
    service: Vec<MusicService>
}


#[derive(Debug,Deserialize,Serialize)]
pub struct Session {

}
#[derive(Debug,Deserialize,Serialize)]
pub struct UserDevice {

}
#[derive(Debug,Deserialize,Serialize)]
pub enum MusicService {
    Spotify,
    AppleMusic,
}
