use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize,Deserialize)]
pub struct Playlist {
    pub id: Option<ObjectId>,
    pub name: String,
    pub owner: Option<ObjectId>,
    pub service_id: String,
    pub songs: Vec<Song>,
    pub collaborators: Vec<Option<ObjectId>>,
    pub privacy: PlaylistPrivacy,
    pub description: String, 
}

#[derive(Debug, Serialize,Deserialize)]
pub struct Song {
    pub service_id: String,
    pub name: String,
    pub artist: Vec<String>,
    pub album: String,
}


#[derive(Debug, Serialize,Deserialize)]
pub struct QueuedSong {
    pub song: Song,
    pub contributor: QueueContributor,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum PlaylistPrivacy {
    Locked,

    Open {
        queue: Vec<QueuedSong>,
        requires_account: bool,
    }
    
}


#[derive(Debug, Serialize,Deserialize)]
pub enum QueueContributor {
    User(Option<ObjectId>),
    NonUser(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub id: Option<ObjectId>,
    pub initiator: ObjectId,
    pub target: ObjectId,
}

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
