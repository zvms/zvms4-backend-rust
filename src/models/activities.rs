use crate::models::utils::datetime_or_u64;
use mongodb::bson::oid::ObjectId;
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
pub enum SpecialActivityCategory {
    Prize,
    Import,
    Club,
    Deduction,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub _id: ObjectId,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub name: String,
    pub description: Option<String>,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub date: u64,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub created_at: u64,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub updated_at: u64,
    pub creator: ObjectId,
    pub status: ActivityStatus,
    pub members: Vec<ObjectId>,
    pub location: Option<String>,
    pub category: Option<SpecialActivityCategory>,
}
