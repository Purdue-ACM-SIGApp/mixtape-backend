use mongodb::bson::oid::ObjectId;
use serde::Serialize;
use serde::Deserialize;


#[derive(Debug, Serialize, Deserialize)]
struct Block {
    id: ObjectId,
    initiator: ObjectId,
    target: ObjectId,
}