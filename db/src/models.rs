use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
struct Friend {
    id: ObjectId,
    initiales: ObjectId,
    target: ObjectId,
    status: FriendStatus,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
enum FriendStatus {
    InitialesRequested,
    Accepted,
}