use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum UserSex {
    Male,
    Female,
    Unknown,
}

impl ToString for UserSex {
    fn to_string(&self) -> String {
        match *self {
            UserSex::Male => "male".to_string(),
            UserSex::Female => "female".to_string(),
            UserSex::Unknown => "unknown".to_string(),
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
