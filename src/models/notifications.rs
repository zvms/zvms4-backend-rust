use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum NotificationType {
    Pin,
    Important,
    Normal,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Notification {
    pub _id: ObjectId,
    pub title: String,
    pub content: String,
    pub time: u64,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub publisher: ObjectId,
    pub receivers: Option<Vec<ObjectId>>,
    pub anoymous: bool,
    pub global: bool,
    pub expire: u64,
}
