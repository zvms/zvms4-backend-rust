use crate::{
    models::{
        activities::{Activity, ActivityMember},
        groups::GroupPermission,
        response::{create_error, ResponseStatus, SuccessResponse},
    },
    utils::{groups::same_class, jwt::UserData},
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

pub async fn read_member(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Path(id): Path<String>,
    Path(member_id): Path<String>,
) -> impl IntoResponse {
    let db_clone = db.clone();
    let db = db.lock().await;
    let collection = db.collection("activities");
    let activity_id = ObjectId::from_str(id.as_str());
    if let Err(_) = activity_id {
        return create_error(StatusCode::BAD_REQUEST, "Invalid activity ID".to_string());
    }
    let activity_id = activity_id.unwrap();
    let member_id = ObjectId::from_str(member_id.as_str());
    if let Err(_) = member_id {
        return create_error(StatusCode::BAD_REQUEST, "Invalid member ID".to_string());
    }
    let member_id = member_id.unwrap();
    let activity = collection
        .find_one(doc! {"_id": activity_id, "members._id": member_id}, None)
        .await;
    if let Err(_) = activity {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to find activity".to_string(),
        );
    }
    let activity = activity.unwrap();
    if let None = activity {
        return create_error(StatusCode::NOT_FOUND, "Activity not found".to_string());
    }
    let activity: Activity = bson::from_document(activity.unwrap()).unwrap();
    if user.perms.contains(&GroupPermission::Admin)
        || user.perms.contains(&GroupPermission::Department)
        || user.perms.contains(&GroupPermission::Auditor)
        || user.id == member_id.clone().to_string()
    {
    } else if user.perms.contains(&GroupPermission::Secretary) {
        let user_id = ObjectId::from_str(&user.id).unwrap();
        let same = same_class::validate_same_class(db_clone, member_id, user_id).await;
        if let Err(e) = same {
            return create_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Cannot validate user".to_string() + &e,
            );
        }
        if !same.unwrap() {
            return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
        }
    } else {
        return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
    }
    let member = activity
        .members
        .unwrap_or_default()
        .into_iter()
        .find(|member| member._id == member_id);
    if let None = member {
        return create_error(StatusCode::NOT_FOUND, "Member not found".to_string());
    }
    let member = member.unwrap();
    let response: SuccessResponse<ActivityMember, ()> = SuccessResponse {
        status: ResponseStatus::Success,
        code: 200,
        data: member,
        metadata: None,
    };
    let response = serde_json::to_string(&response).unwrap();
    (StatusCode::OK, Json(response))
}
