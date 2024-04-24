use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GroupPermission {
    Student,
    Secretary,
    Department,
    Auditor,
    Inspector,
    Admin,
    System,
}

impl GroupPermission {
    pub fn as_str(&self) -> &'static str {
        match *self {
            GroupPermission::Student => "student",
            GroupPermission::Secretary => "secretary",
            GroupPermission::Department => "department",
            GroupPermission::Auditor => "auditor",
            GroupPermission::Inspector => "inspector",
            GroupPermission::Admin => "admin",
            GroupPermission::System => "system",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GroupType {
    Class,
    Permission,
    Group,
}

impl GroupType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            GroupType::Class => "class",
            GroupType::Permission => "permission",
            GroupType::Group => "group",
        }
    }
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
