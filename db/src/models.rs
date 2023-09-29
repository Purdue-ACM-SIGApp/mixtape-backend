use mongodb::bson::Timestamp;
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
        expiration: Timestamp
    },
    Verified,
}
