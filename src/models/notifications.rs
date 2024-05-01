use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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
    pub content: String,
    pub time: u128,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub publisher: ObjectId,
    pub receivers: Option<Vec<ObjectId>>,
    pub anoymous: bool,
    pub global: bool,
    pub expire: u128,
}
