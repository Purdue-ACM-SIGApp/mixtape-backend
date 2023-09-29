use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize,Deserialize)]
struct Playlist {
    id: ObjectId,
    name: String,
    owner: ObjectId,
    service_id: String,
    songs: Vec<Song>,
    collaborators: Vec<ObjectId>,
    privacy: PlaylistPrivacy,
    description: String, 
}

#[derive(Debug, Serialize,Deserialize)]
struct Song {
    service_id: String,
    name: String,
    artist: Vec<String>,
    album: String,
}


#[derive(Debug, Serialize,Deserialize)]
struct QueuedSong {
    song: Song,
    contributor: QueueContributor,
}


#[derive(Debug, Serialize, Deserialize)]
enum PlaylistPrivacy {
    Locked,

    Open {
        queue: Vec<QueuedSong>,
        requires_account: bool,
    }
    
}


#[derive(Debug, Serialize,Deserialize)]
enum QueueContributor {
    User(ObjectId),
    NonUser(String),
}
