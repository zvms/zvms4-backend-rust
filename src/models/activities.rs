use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityType {
    Specified,
    Social,
    Scale,
    Special,
}

impl ActivityType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ActivityType::Specified => "specified",
            ActivityType::Social => "social",
            ActivityType::Scale => "scale",
            ActivityType::Special => "special",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityStatus {
    Effective,
    Pending,
    Refused,
}

impl ActivityStatus {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ActivityStatus::Effective => "effective",
            ActivityStatus::Pending => "pending",
            ActivityStatus::Refused => "refused",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityMemberStatus {
    Effective,
    Pending,
    Refused,
    Rejected,
    Draft,
}

impl ActivityMemberStatus {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ActivityMemberStatus::Effective => "effective",
            ActivityMemberStatus::Pending => "pending",
            ActivityMemberStatus::Refused => "refused",
            ActivityMemberStatus::Rejected => "rejected",
            ActivityMemberStatus::Draft => "draft",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityMode {
    OnCampus,
    OffCampus,
    SocialPractice,
}

impl ActivityMode {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ActivityMode::OnCampus => "on-campus",
            ActivityMode::OffCampus => "off-campus",
            ActivityMode::SocialPractice => "social-practice",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityMember {
    pub _id: String, // ObjectId
    pub name: String,
    pub status: ActivityMemberStatus,
    pub impression: String,
    pub duration: f64,
    pub mode: ActivityMode,
    pub history: Vec<ActivityMemberHistory>,
    pub images: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityMemberHistory {
    pub impression: String,
    pub duration: f64,
    pub time: String,
    pub actioner: String, // ObjectId
    pub action: ActivityMemberStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub duration: i32,
    pub date: String,
    pub created_at: String,
    pub updated_at: String,
    pub creator: String, // ObjectId
    pub status: ActivityStatus,
    pub members: Vec<ActivityMember>,
}
