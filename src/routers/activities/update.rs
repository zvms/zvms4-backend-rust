use crate::{models::{
    activities::{Activity, ActivityStatus}, groups::GroupPermission, response::{ErrorResponse, ResponseStatus, SuccessResponse}
}, utils::jwt::UserData};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::{doc, oid::ObjectId};
use mongodb::{Database, Collection};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

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
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 400,
            message: "Invalid activity id".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    let id = id.unwrap();
    let activity = collection.find_one(doc! {"_id": id}, None).await;
    if let Err(e) = activity {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 500,
            message: "Failed to find activity: ".to_string() + &e.to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
    }
    let activity = activity.unwrap();
    if let None = activity {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 404,
            message: "Activity not found".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::NOT_FOUND, Json(response));
    }
    let activity = activity.unwrap();
    let creator = activity.creator;
    if user.perms.contains(&GroupPermission::Admin) || user.perms.contains(&GroupPermission::Department) || id == creator {
        let result = collection.update_one(doc! {"_id": id}, doc! {"$set": {"name": data.name}}, None).await;
        if let Err(e) = result {
            let response = ErrorResponse {
                status: ResponseStatus::Error,
                code: 500,
                message: "Failed to update activity: ".to_string() + &e.to_string(),
            };
            let response = serde_json::to_string(&response).unwrap();
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
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
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 403,
            message: "Permission denied".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::FORBIDDEN, Json(response));
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
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 400,
            message: "Invalid activity id".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    let id = id.unwrap();
    let activity = collection.find_one(doc! {"_id": id}, None).await;
    if let Err(e) = activity {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 500,
            message: "Failed to find activity: ".to_string() + &e.to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
    }
    let activity = activity.unwrap();
    if let None = activity {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 404,
            message: "Activity not found".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::NOT_FOUND, Json(response));
    }
    let activity = activity.unwrap();
    let creator = activity.creator;
    if user.perms.contains(&GroupPermission::Admin) || user.perms.contains(&GroupPermission::Department) || id == creator {
        let result = collection.update_one(doc! {"_id": id}, doc! {"$set": {"description": data.description}}, None).await;
        if let Err(e) = result {
            let response = ErrorResponse {
                status: ResponseStatus::Error,
                code: 500,
                message: "Failed to update activity: ".to_string() + &e.to_string(),
            };
            let response = serde_json::to_string(&response).unwrap();
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
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
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 403,
            message: "Permission denied".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::FORBIDDEN, Json(response));
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct UpdateActivityStatus {
    status: ActivityStatus,
}
