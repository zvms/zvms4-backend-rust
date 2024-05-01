use crate::{models::{
    activities::Activity, groups::GroupPermission, response::{ErrorResponse, ResponseStatus, SuccessResponse}
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

pub async fn remove_activity(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Path(id): Path<String>,
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
    if user.perms.contains(&GroupPermission::Admin) || user.perms.contains(&GroupPermission::Department) {
    } else {
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
        if creator != user.id {
            let response = ErrorResponse {
                status: ResponseStatus::Error,
                code: 403,
                message: "Permission denied".to_string(),
            };
            let response = serde_json::to_string(&response).unwrap();
            return (StatusCode::FORBIDDEN, Json(response));
        }
    }
    let filter = doc! {"_id": id};
    let result = collection.delete_one(filter, None).await;
    if let Err(e) = result {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 500,
            message: "Failed to delete activity: ".to_string() + &e.to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
    }
    let result = result.unwrap();
    if result.deleted_count == 0 {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 404,
            message: "Activity not found".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::NOT_FOUND, Json(response));
    }
    let response: SuccessResponse<_, ()> = SuccessResponse {
        status: ResponseStatus::Success,
        code: 200,
        data: (),
        metadata: None,
    }
    .into();
    let response = serde_json::to_string(&response).unwrap();
    (StatusCode::OK, Json(response))
}
