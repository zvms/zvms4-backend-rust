use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Specified,
    Social,
    Scale,
    Special,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityStatus {
    Effective,
    Pending,
    Refused,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityMemberStatus {
    Effective,
    Pending,
    Refused,
    Rejected,
    Draft,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityMode {
    OnCampus,
    OffCampus,
    SocialPractice,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActivityMember {
    pub _id: String, // ObjectId
    pub status: ActivityMemberStatus,
    pub impression: Option<String>,
    pub duration: f64,
    pub mode: ActivityMode,
    pub history: Option<Vec<ActivityMemberHistory>>,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActivityMemberHistory {
    pub impression: String,
    pub duration: f64,
    pub time: String,
    pub actioner: String, // ObjectId
    pub action: ActivityMemberStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Activity {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub name: String,
    pub description: Option<String>,
    pub duration: Option<f64>,
    pub date: u128,
    pub created_at: u128,
    pub updated_at: u128,
    pub creator: ObjectId, // ObjectId
    pub status: ActivityStatus,
    pub members: Option<Vec<ActivityMember>>,
}
