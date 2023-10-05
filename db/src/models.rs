use chrono::{DateTime, Utc};
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
