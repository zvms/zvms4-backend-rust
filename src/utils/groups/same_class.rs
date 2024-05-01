use crate::models::{groups::Group, users::User};
use bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use mongodb::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn validate_same_class(
    db: Arc<Mutex<Database>>,
    user: ObjectId,
    target: ObjectId,
) -> Result<bool, String> {
    let db = db.lock().await;
    let collection = db.clone().collection("users");
    let group_collection = db.clone().collection("groups");
    let user = collection.find_one(doc! {"_id": user}, None).await;
    if let Err(_) = user {
        return Err("Base user not found".to_string());
    }
    let user = user.unwrap();
    if let None = user {
        return Err("Target user not found".to_string());
    }
    let user = bson::from_document(user.unwrap());
    if let Err(_) = user {
        return Err("Base user not found".to_string());
    }
    let user: User = user.unwrap();
    let target = collection.find_one(doc! {"_id": target}, None).await;
    if let Err(_) = target {
        return Err("Target user not found".to_string());
    }
    let target = target.unwrap();
    if let None = target {
        return Err("Target user not found".to_string());
    }
    let target = bson::from_document(target.unwrap());
    if let Err(_) = target {
        return Err("Target user not found".to_string());
    }
    let target: User = target.unwrap();
    let groups = group_collection
        .find(doc! {"_id": {"$in": user.group}, "type": "class"}, None)
        .await;
    if let Err(_) = groups {
        return Err("Base user group not found".to_string());
    }
    let groups = groups.unwrap().try_collect().await;
    if let Err(_) = groups {
        return Err("Cannot parse groups".to_string());
    }
    let groups: Vec<Group> = groups.unwrap();
    for group in groups {
        if target.group.contains(&group._id) {
            return Ok(true);
        }
    }
    Ok(false)
}
