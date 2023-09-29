use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TempNumber {
    phone_number: String,
    verification: VerificationStatus,
}

#[derive(Debug, Deserialize, Serialize)]
enum VerificationStatus {
    NotVerified {
        code: String,
        expiration: DateTime<Utc>,
    },
    Verified,
}

#[derive(Debug, Deserialize, Serialize)]
struct Friend {
    id: ObjectId,
    initiales: ObjectId,
    target: ObjectId,
    status: FriendStatus,
}

#[derive(Debug, Deserialize, Serialize)]
enum FriendStatus {
    InitialesRequested,
    Accepted,
}