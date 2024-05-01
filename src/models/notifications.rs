use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::activities::datetime_or_u64;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum NotificationType {
    Pin,
    Important,
    Normal,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Notification {
    pub _id: ObjectId,
    pub title: String,
    pub content: Option<String>,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub time: u64,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub publisher: ObjectId,
    pub receivers: Option<Vec<ObjectId>>,
    pub anoymous: bool,
    pub global: bool,
    #[serde(deserialize_with = "datetime_or_u64")]
    pub expire: u64,
}
