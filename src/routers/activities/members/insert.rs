use crate::{
    models::{
        activities::{Activity, ActivityMember},
        groups::GroupPermission,
        response::{create_error, ResponseStatus, SuccessResponse},
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
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to find activity".to_string(),
        );
    }
    let activity = activity.unwrap();
    if let None = activity {
        return create_error(
            StatusCode::NOT_FOUND,
            "Activity not found".to_string(),
        );
    }
    let activity: Activity = bson::from_document(activity.unwrap()).unwrap();
    let members = activity.members.unwrap_or_default();
    // Check if the activity contains the member
    if members
        .iter()
        .any(|member| member._id == ObjectId::from_str(&activity_member._id.to_hex()).unwrap())
    {
        return create_error(
            StatusCode::BAD_REQUEST,
            "Member already exists".to_string(),
        );
    }
    if user.perms.contains(&GroupPermission::Admin) || user.perms.contains(&GroupPermission::Department) {} else {
        return create_error(
            StatusCode::FORBIDDEN,
            "Permission denied".to_string(),
        );
    }
    let member = bson::to_document(&activity_member);
    if let Err(_) = member {
        return create_error(
            StatusCode::BAD_REQUEST,
            "Invalid member".to_string(),
        );
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
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert member".to_string(),
        );
    }
}
