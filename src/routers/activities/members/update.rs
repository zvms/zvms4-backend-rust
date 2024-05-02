use crate::{
    models::{
        activities::{Activity, ActivityMember, ActivityMemberStatus, ActivityMode},
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
use mongodb::{Collection, Database};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct UpdateActivityMemberMode {
    pub mode: ActivityMode
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateActivityMemberStatus {
    pub status: ActivityMemberStatus,
    pub duration: Option<f64>
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct UpdateActivityMemberImpression {
    pub impression: String
}

pub async fn update_member_status(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Path(id): Path<String>,
    Path(member_id): Path<String>,
    Json(update): Json<UpdateActivityMemberStatus>,
) -> impl IntoResponse {
    let db = db.lock().await;
    let collection: Collection<Activity> = db.collection("activities");
    let activity_id = ObjectId::from_str(&id.as_str());
    if let Err(_) = activity_id {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 400,
            message: "Invalid activity ID".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    let activity_id = activity_id.unwrap();
    let member_id = ObjectId::from_str(&member_id.as_str());
    if let Err(_) = member_id {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 400,
            message: "Invalid member ID".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    let member_id = member_id.unwrap();
    let activity = collection.find_one(doc! {"_id": activity_id, "members._id": member_id}, None).await;
    if let Err(_) = activity {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 404,
            message: "Failed to find activity".to_string(),
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
    let activity: Activity = activity.unwrap();
    let members = activity.members;
    if let None = members {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 404,
            message: "Activity has no members".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::NOT_FOUND, Json(response));
    }
    let mut members = members.unwrap();
    for member in members.iter_mut() {
        if member._id == member_id {
            if member.status == ActivityMemberStatus::Effective || member.status == ActivityMemberStatus::Refused {
                let response = ErrorResponse {
                    status: ResponseStatus::Error,
                    code: 403,
                    message: "Cannot update member duration".to_string(),
                };
                let response = serde_json::to_string(&response).unwrap();
                return (StatusCode::FORBIDDEN, Json(response));
            } else if member.status == ActivityMemberStatus::Pending && !user.perms.contains(&GroupPermission::Auditor) && !user.perms.contains(&GroupPermission::Admin) {
                let response = ErrorResponse {
                    status: ResponseStatus::Error,
                    code: 403,
                    message: "Cannot update member duration".to_string(),
                };
                let response = serde_json::to_string(&response).unwrap();
                return (StatusCode::FORBIDDEN, Json(response));
            } else if member.status == ActivityMemberStatus::Draft || member.status == ActivityMemberStatus::Rejected && user.id != member_id.to_string() {
                let response = ErrorResponse {
                    status: ResponseStatus::Error,
                    code: 403,
                    message: "Cannot update member duration".to_string(),
                };
                let response = serde_json::to_string(&response).unwrap();
                return (StatusCode::FORBIDDEN, Json(response));
            }
            let status = serde_json::to_string(&update.status).unwrap();
            let result = collection.update_one(
                doc! {"_id": activity_id, "members._id": member_id},
                doc! {"$set": {"members.$.status": status, "members.$.duration": update.duration.unwrap_or(member.duration)}},
                None,
            ).await;
            if let Err(_) = result {
                let response = ErrorResponse {
                    status: ResponseStatus::Error,
                    code: 500,
                    message: "Failed to update member status".to_string(),
                };
                let response = serde_json::to_string(&response).unwrap();
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
            }
            let result = result.unwrap();
            if result.modified_count != 1 {
                let response = ErrorResponse {
                    status: ResponseStatus::Error,
                    code: 500,
                    message: "Failed to update member status".to_string(),
                };
                let response = serde_json::to_string(&response).unwrap();
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
            }
            let response: SuccessResponse<Vec<ActivityMember>, ()> = SuccessResponse {
                status: ResponseStatus::Success,
                code: 200,
                data: vec![],
                metadata: None,
            };
            let response = serde_json::to_string(&response).unwrap();
            return (StatusCode::OK, Json(response));
        }
    }
    (StatusCode::OK, Json("".to_string()))
}
