use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize,Deserialize)]
pub struct Playlist {
    id: Option<ObjectId>,
    name: String,
    owner: Option<ObjectId>,
    service_id: String,
    songs: Vec<Song>,
    collaborators: Vec<Option<ObjectId>>,
    privacy: PlaylistPrivacy,
    description: String, 
}

#[derive(Debug, Serialize,Deserialize)]
pub struct Song {
    service_id: String,
    name: String,
    artist: Vec<String>,
    album: String,
}


#[derive(Debug, Serialize,Deserialize)]
pub struct QueuedSong {
    song: Song,
    contributor: QueueContributor,
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
