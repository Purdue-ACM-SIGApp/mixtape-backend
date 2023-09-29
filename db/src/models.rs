use mongodb::bson::oid::ObjectId;
use serde::Serialize;
use serde::Deserialize;


#[derive(Debug, Serialize, Deserialize)]
struct Block {
    pub id: Option<ObjectId>,
    pub initiator: ObjectId,
    pub target: ObjectId,
}