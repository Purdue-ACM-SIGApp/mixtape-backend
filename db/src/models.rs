<<<<<<< HEAD
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
=======
use chrono::{DateTime, Utc};
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
>>>>>>> c3a856eade390b33176467d633352809b197bcb3
