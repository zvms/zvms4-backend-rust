use crate::models::{
    activities::Activity,
    response::{create_error, ResponseStatus, SuccessResponse},
};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::{doc, from_document, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserActivityTime {
    pub on_campus: f64,
    pub off_campus: f64,
    pub social_practice: f64,
    pub total: f64,
}

pub async fn calculate_user_activity_time(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let db = db.lock().await;
    let user_id = ObjectId::from_str(&user_id);
    if let Err(_) = user_id {
        return create_error(StatusCode::BAD_REQUEST, "Invalid user ID".to_string());
    }
    let user_id = user_id.unwrap();
    let collection: Collection<Activity> = db.collection("activities");
    let pipeline = vec![
        doc! {
            "$match": {
                "$or": [
                    { "members._id": user_id },
                    { "members._id": user_id.to_hex() }
                ]
            }
        },
        doc! {
            "$unwind": "$members"
        },
        doc! {
            "$match": {
                "$or": [
                    { "members._id": user_id },
                    { "members._id": user_id.to_hex() }
                ]
            }
        },
        doc! {
            "$group": {
                "_id": "$members.mode",
                "totalDuration": { "$sum": "$members.duration" }
            }
        },
        doc! {
            "$group": {
                "_id": null,
                "on_campus": {
                    "$sum": {
                        "$cond": [{ "$eq": ["$_id", "on-campus"] }, "$totalDuration", 0.0]
                    }
                },
                "off_campus": {
                    "$sum": {
                        "$cond": [{ "$eq": ["$_id", "off-campus"] }, "$totalDuration", 0.0]
                    }
                },
                "social_practice": {
                    "$sum": {
                        "$cond": [{ "$eq": ["$_id", "social-practice"] }, "$totalDuration", 0.0]
                    }
                },
                "total": { "$sum": "$totalDuration" }
            }
        },
        doc! {
            "$project": {
                "_id": 0,
                "on_campus": 1,
                "off_campus": 1,
                "social_practice": 1,
                "total": 1
            }
        },
    ];
    let cursor = collection.aggregate(pipeline, None);
    let mut result = UserActivityTime {
        on_campus: 0.0,
        off_campus: 0.0,
        social_practice: 0.0,
        total: 0.0,
    };
    let cursor = cursor.await;
    if let Err(_) = cursor {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to aggregate documents".to_string(),
        );
    }
    // Only the first document is needed
    let mut cursor = cursor.unwrap();
    if let Some(doc) = cursor.try_next().await.unwrap() {
        result = from_document(doc).unwrap();
    }
    let response: SuccessResponse<UserActivityTime, ()> = SuccessResponse {
        status: ResponseStatus::Success,
        code: StatusCode::OK.as_u16(),
        data: result,
        metadata: None,
    };
    let response = serde_json::to_string(&response).unwrap();
    (StatusCode::OK, Json(response))
}
