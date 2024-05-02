use crate::{
    models::{
        activities::{Activity, ActivityStatus},
        groups::GroupPermission,
        response::{create_error, ResponseStatus, SuccessResponse},
    },
    utils::jwt::UserData,
};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::{doc, oid::ObjectId};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct UpdateActivityName {
    pub name: String,
}

pub async fn update_activity_name(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Path(id): Path<String>,
    Json(data): Json<UpdateActivityName>,
) -> impl IntoResponse {
    let db = db.lock().await;
    let collection: Collection<Activity> = db.collection("activities");
    let id = ObjectId::parse_str(&id);
    if let Err(_) = id {
        return create_error(StatusCode::BAD_REQUEST, "Invalid activity id".to_string());
    }
    let id = id.unwrap();
    let activity = collection.find_one(doc! {"_id": id}, None).await;
    if let Err(e) = activity {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to find activity: ".to_string() + &e.to_string(),
        );
    }
    let activity = activity.unwrap();
    if let None = activity {
        return create_error(StatusCode::NOT_FOUND, "Activity not found".to_string());
    }
    let activity = activity.unwrap();
    let creator = activity.creator;
    if user.perms.contains(&GroupPermission::Admin)
        || user.perms.contains(&GroupPermission::Department)
        || id == creator
    {
        let result = collection
            .update_one(doc! {"_id": id}, doc! {"$set": {"name": data.name}}, None)
            .await;
        if let Err(e) = result {
            return create_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update activity: ".to_string() + &e.to_string(),
            );
        }
        let response: SuccessResponse<Vec<Activity>, ()> = SuccessResponse {
            status: ResponseStatus::Success,
            code: 200,
            data: vec![],
            metadata: None,
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::OK, Json(response));
    } else {
        return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct UpdateActivityDescription {
    pub description: String,
}

pub async fn update_activity_description(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Path(id): Path<String>,
    Json(data): Json<UpdateActivityDescription>,
) -> impl IntoResponse {
    let db = db.lock().await;
    let collection: Collection<Activity> = db.collection("activities");
    let id = ObjectId::parse_str(&id);
    if let Err(_) = id {
        return create_error(StatusCode::BAD_REQUEST, "Invalid activity id".to_string());
    }
    let id = id.unwrap();
    let activity = collection.find_one(doc! {"_id": id}, None).await;
    if let Err(e) = activity {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to find activity: ".to_string() + &e.to_string(),
        );
    }
    let activity = activity.unwrap();
    if let None = activity {
        return create_error(StatusCode::NOT_FOUND, "Activity not found".to_string());
    }
    let activity = activity.unwrap();
    let creator = activity.creator;
    if user.perms.contains(&GroupPermission::Admin)
        || user.perms.contains(&GroupPermission::Department)
        || id == creator
    {
        let result = collection
            .update_one(
                doc! {"_id": id},
                doc! {"$set": {"description": data.description}},
                None,
            )
            .await;
        if let Err(e) = result {
            return create_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update activity: ".to_string() + &e.to_string(),
            );
        }
        let response: SuccessResponse<Vec<Activity>, ()> = SuccessResponse {
            status: ResponseStatus::Success,
            code: 200,
            data: vec![],
            metadata: None,
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::OK, Json(response));
    } else {
        return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct UpdateActivityStatus {
    status: ActivityStatus,
}
