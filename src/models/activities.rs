use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Specified,
    Social,
    Scale,
    Special,
}

impl ToString for ActivityType {
    fn to_string(&self) -> String {
        match *self {
            ActivityType::Specified => "specified".to_string(),
            ActivityType::Social => "social".to_string(),
            ActivityType::Scale => "scale".to_string(),
            ActivityType::Special => "special".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityStatus {
    Effective,
    Pending,
    Refused,
}

impl ToString for ActivityStatus {
    fn to_string(&self) -> String {
        match *self {
            ActivityStatus::Effective => "effective".to_string(),
            ActivityStatus::Pending => "pending".to_string(),
            ActivityStatus::Refused => "refused".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityMemberStatus {
    Effective,
    Pending,
    Refused,
    Rejected,
    Draft,
}

impl ToString for ActivityMemberStatus {
    fn to_string(&self) -> String {
        match *self {
            ActivityMemberStatus::Effective => "effective".to_string(),
            ActivityMemberStatus::Pending => "pending".to_string(),
            ActivityMemberStatus::Refused => "refused".to_string(),
            ActivityMemberStatus::Rejected => "rejected".to_string(),
            ActivityMemberStatus::Draft => "draft".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityMode {
    OnCampus,
    OffCampus,
    SocialPractice,
}

impl ToString for ActivityMode {
    fn to_string(&self) -> String {
        match *self {
            ActivityMode::OnCampus => "on-campus".to_string(),
            ActivityMode::OffCampus => "off-campus".to_string(),
            ActivityMode::SocialPractice => "social-practice".to_string(),
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
    pub members: Vec<ActivityMember>,
}
