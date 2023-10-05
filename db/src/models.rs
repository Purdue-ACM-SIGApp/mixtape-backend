use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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