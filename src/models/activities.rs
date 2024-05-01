use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Specified,
    Social,
    Scale,
    Special,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityStatus {
    Effective,
    Pending,
    Refused,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityMemberStatus {
    Effective,
    Pending,
    Refused,
    Rejected,
    Draft,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    pub description: String,
    pub duration: i32,
    pub date: String,
    pub created_at: String,
    pub updated_at: String,
    pub creator: String, // ObjectId
    pub status: ActivityStatus,
    pub members: Option<Vec<ActivityMember>>,
}
