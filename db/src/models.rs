use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug,Deserialize,Serialize)]
pub struct User {
    pub id: Option<ObjectId>,
    pub phonenumber: String,
    pub name: String,
    pub username: String,
    pub sessions: Vec<Session>,
    pub devices: Vec<UserDevice>,
    pub last_active: DateTime<Utc>,
    pub service: Vec<MusicService>
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

#[derive(Debug, Deserialize, Serialize)]
pub struct TempNumber {
    pub phone_number: String,
    pub verification: VerificationStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VerificationStatus {
    NotVerified {
        code: String,
        expiration: DateTime<Utc>,
    },
    Verified,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Friend {
    pub id: ObjectId,
    pub initiales: ObjectId,
    pub target: ObjectId,
    pub status: FriendStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FriendStatus {
    InitialesRequested,
    Accepted,
}
