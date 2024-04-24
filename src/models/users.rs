use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum UserSex {
    Male,
    Female,
    Unknown,
}

impl UserSex {
    pub fn as_str(&self) -> &'static str {
        match *self {
            UserSex::Male => "male",
            UserSex::Female => "female",
            UserSex::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub id: u64,
    pub name: String,
    pub group: Vec<String>,
    password: String,
}
