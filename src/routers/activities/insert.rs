use crate::{models::{
    activities::{Activity, ActivityStatus, ActivityType}, groups::GroupPermission, response::{ResponseStatus, SuccessResponse, create_error}
}, utils::jwt::UserData};
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::oid::ObjectId;
use mongodb::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn insert_activity(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Query(renew_object_id): Query<Option<bool>>,
    Json(mut activity): Json<Activity>,
) -> impl IntoResponse {
    let renew_object_id = renew_object_id.unwrap_or(true);
    let db = db.lock().await;
    let collection = db.collection("activities");
    if user.perms.contains(&GroupPermission::Admin) {
        activity.status = ActivityStatus::Effective;
    } else if user.perms.contains(&GroupPermission::Department) {
        if activity.activity_type == ActivityType::Special {
            activity.status = ActivityStatus::Pending;
        } else {
            activity.status = ActivityStatus::Effective;
        }
    } else if user.perms.contains(&GroupPermission::Secretary) {
        if activity.activity_type == ActivityType::Specified {
            activity.status = ActivityStatus::Pending;
        } else if activity.activity_type != ActivityType::Special {
            activity.status = ActivityStatus::Effective;
        } else {
            return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
        }
    } else {
        if activity.activity_type == ActivityType::Social || activity.activity_type == ActivityType::Scale {
            activity.status = ActivityStatus::Pending;
        } else {
            return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
        }
    }
    // Remove the _id field if it exists
    let mut activity = activity;
    if renew_object_id {
        activity._id = ObjectId::new();
    }
    let activity = bson::to_document(&activity);
    if let Ok(activity) = activity {
        if let Ok(_) = collection.insert_one(activity, None).await {
            let response: SuccessResponse<_, ()> = SuccessResponse {
                status: ResponseStatus::Success,
                code: 200,
                data: (),
                metadata: None,
            }
            .into();
            let response = serde_json::to_string(&response).unwrap();
            (StatusCode::OK, Json(response))
        } else {
            return create_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert activity".to_string(),
            );
        }
    } else {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert activity".to_string(),
        );
    }
}
