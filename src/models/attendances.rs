use crate::models::utils::{object_id_to_string, string_to_object_id};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum AttendanceStatus {
    Effective,
    Pending,
    Refused,
    Rejected,
    Draft,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum AttendanceMode {
    OnCampus,
    OffCampus,
    SocialPractice,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AttendanceHistory {
    pub impression: String,
    pub duration: f64,
    pub time: String,
    #[serde(
        serialize_with = "object_id_to_string",
        deserialize_with = "string_to_object_id"
    )]
    pub actor: ObjectId, // ObjectId
    pub result: AttendanceStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActivityMember {
    pub _id: ObjectId,
    pub status: AttendanceStatus,
    pub impression: Option<String>,
    pub duration: f64,
    pub mode: AttendanceMode,
    pub history: Option<Vec<AttendanceHistory>>,
    pub images: Option<Vec<String>>,
}
