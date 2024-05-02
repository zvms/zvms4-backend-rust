use crate::{
    models::{
        activities::Activity,
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
        return create_error(StatusCode::BAD_REQUEST, "Invalid activity id".to_string());
    }
    let id = id.unwrap();
    if user.perms.contains(&GroupPermission::Admin)
        || user.perms.contains(&GroupPermission::Department)
    {
    } else {
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
        if creator != id {
            return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
        }
    }
    let filter = doc! {"_id": id};
    let result = collection.delete_one(filter, None).await;
    if let Err(e) = result {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete activity: ".to_string() + &e.to_string(),
        );
    }
    let result = result.unwrap();
    if result.deleted_count == 0 {
        return create_error(StatusCode::NOT_FOUND, "Activity not found".to_string());
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
