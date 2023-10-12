use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


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
