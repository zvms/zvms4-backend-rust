use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserSex {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub id: String,
    pub name: String,
    pub group: Vec<String>,
    password: String,
}
