use crate::models::utils::datetime_or_u64;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum VolunteerType {
    Mobile,
    Social,
    Practice,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum VolunteerStatus {
    Pending,
    Effective,
    Refused,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum VolunteerMode {
    OnCampus,
    OffCampus,
    SocialPractice,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Volunteer {
    pub _id: ObjectId,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub volunteer_type: VolunteerType,
    pub status: VolunteerStatus,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub date: u64,
    pub creator: ObjectId,
    pub location: String,
    pub members: Vec<ObjectId>,
}
