use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
struct Report {
	pub id: Option<ObjectId>,
	pub initiator: Option<ObjectId>,
	pub target: Option<ObjectId>,
	pub reason: String,	
}
