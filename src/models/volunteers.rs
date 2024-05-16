use crate::models::activities::datetime_or_u64;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum VolunteerMemberStatus {
    Effective,
    Pending,
    Refused,
    Rejected,
    Draft,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VolunteerMember {
    pub _id: ObjectId,
    pub user: ObjectId,
    pub status: VolunteerMemberStatus,
    pub impression: Option<String>,
    pub duration: f64,
    pub mode: VolunteerMode,
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
