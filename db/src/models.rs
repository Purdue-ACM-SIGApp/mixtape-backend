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
