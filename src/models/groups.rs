use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GroupPermission {
    Student,
    Secretary,
    Department,
    Auditor,
    Inspector,
    Admin,
    System,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GroupType {
    Class,
    Permission,
    Group,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub permissions: Vec<GroupPermission>,
    #[serde(rename = "type")]
    pub group_type: GroupType,
}
