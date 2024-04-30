use crate::models::groups::{Group, GroupPermission};
use crate::utils::jwt::{generate_token, TokenType};
use bcrypt::{hash, verify};
use bson::doc;
use bson::oid::ObjectId;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserSex {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub id: String,
    pub name: String,
    pub group: Vec<String>,
    password: String,
}

pub trait UserTrait {
    async fn valid_password(self, password: String) -> bool;
    async fn set_password(&mut self, password: String) -> ();
    async fn generate_token(
        &self,
        users: &Collection<User>,
        groups: &Collection<Group>,
        term: TokenType,
    ) -> Result<String, String>;
}

impl UserTrait for User {
    async fn valid_password(self, password: String) -> bool {
        let result = verify(password, self.password.as_str());
        if let Ok(valid) = result {
            valid
        } else {
            false
        }
    }
    async fn set_password(&mut self, password: String) -> () {
        let result = hash(password, 12);
        if let Ok(hashed) = result {
            self.password = hashed;
        }
    }
    async fn generate_token(
        &self,
        users: &Collection<User>,
        groups_collection: &Collection<Group>,
        term: TokenType,
    ) -> Result<String, String> {
        let user: Result<Option<User>, mongodb::error::Error> = users.find_one(None, None).await;
        match user {
            Ok(Some(user)) => {
                let groups = user.group;
                let mut permissions: Vec<GroupPermission> = vec![];
                for group in groups {
                    let id = ObjectId::from_str(group.as_str());
                    if let Err(_) = id {
                        return Err("Invalid group".to_string());
                    }
                    let id: ObjectId = id.unwrap();
                    let group = groups_collection
                        .find_one(Some(doc! {"_id": id}), None)
                        .await;
                    match group {
                        Ok(Some(group)) => {
                            permissions.extend(group.permissions);
                        }
                        Ok(None) => {
                            return Err("Group not found".to_string());
                        }
                        _ => {
                            return Err("Invalid group".to_string());
                        }
                    }
                }
                let token =
                    generate_token(&user._id.to_string(), term, permissions);
                Ok(token)
            }
            Ok(None) => Err("User not found".to_string()),
            _ => Err("Invalid user".to_string()),
        }
    }
}
