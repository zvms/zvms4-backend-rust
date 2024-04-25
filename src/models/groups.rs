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

impl ToString for GroupPermission {
    fn to_string(&self) -> String {
        match *self {
            GroupPermission::Student => "student".to_string(),
            GroupPermission::Secretary => "secretary".to_string(),
            GroupPermission::Department => "department".to_string(),
            GroupPermission::Auditor => "auditor".to_string(),
            GroupPermission::Inspector => "inspector".to_string(),
            GroupPermission::Admin => "admin".to_string(),
            GroupPermission::System => "system".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GroupType {
    Class,
    Permission,
    Group,
}

impl ToString for GroupType {
    fn to_string(&self) -> String {
        match *self {
            GroupType::Class => "class".to_string(),
            GroupType::Permission => "permission".to_string(),
            GroupType::Group => "group".to_string(),
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
