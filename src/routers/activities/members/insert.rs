use crate::{
    models::{
        activities::{Activity, ActivityMember},
        groups::GroupPermission,
        response::{ErrorResponse, ResponseStatus, SuccessResponse},
    },
    utils::jwt::UserData,
};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use bson::{doc, oid::ObjectId};
use mongodb::Database;
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

pub async fn insert_member_into_activity(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Path(id): Path<String>,
    Json(activity_member): Json<ActivityMember>,
) -> impl IntoResponse {
    let db = db.lock().await;
    let collection = db.collection("activities");
    let activity_id = ObjectId::from_str(&id).unwrap();
    let activity = collection.find_one(doc! {"_id": activity_id}, None).await;
    if let Err(_) = activity {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 404,
            message: "Activity not found".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::NOT_FOUND, Json(response));
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
    let activity: Activity = bson::from_document(activity.unwrap()).unwrap();
    let members = activity.members.unwrap_or_default();
    // Check if the activity contains the member
    if members
        .iter()
        .any(|member| member._id == ObjectId::from_str(&activity_member._id.to_hex()).unwrap())
    {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 400,
            message: "Member already exists".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    if user.perms.contains(&GroupPermission::Admin) || user.perms.contains(&GroupPermission::Department) {} else {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 403,
            message: "Permission denied".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::FORBIDDEN, Json(response));
    }
    let member = bson::to_document(&activity_member);
    if let Err(_) = member {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 400,
            message: "Invalid member".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    let result = collection
        .update_one(
            doc! {"_id": activity_id},
            doc! {
                "$push": {
                    "members": member.unwrap()
                }
            },
            None,
        )
        .await;
    if let Ok(_) = result {
        let response: SuccessResponse<_, ()> = SuccessResponse {
            status: ResponseStatus::Success,
            code: 200,
            data: (),
            metadata: None,
        };
        let response = serde_json::to_string(&response).unwrap();
        (StatusCode::OK, Json(response))
    } else {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 500,
            message: "Failed to insert member".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
    }
}
