use crate::{
    models::{
        activities::Activity,
        groups::GroupPermission,
        response::{ErrorResponse, Response as ZVMSResponse, ResponseStatus, SuccessResponse},
    },
    utils::jwt::UserData,
};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::{doc, from_document, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadActivityQuery {
    page: Option<u32>,
    perpage: Option<u32>,
    query: Option<String>,
}

pub async fn read_all(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Query(ReadActivityQuery {
        page,
        perpage,
        query,
    }): Query<ReadActivityQuery>,
) -> impl IntoResponse {
    if user.perms.contains(&GroupPermission::Department)
        || user.perms.contains(&GroupPermission::Admin)
        || user.perms.contains(&GroupPermission::Auditor)
    {
    } else {
        let response: ErrorResponse = ErrorResponse {
            status: ResponseStatus::Error,
            code: 403,
            message: "Permission denied".to_string(),
        }
        .into();
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::FORBIDDEN, Json(response));
    }
    let page = page.unwrap_or(1);
    let perpage = perpage.unwrap_or(10);
    let query = query.unwrap_or("".to_string());
    let db = db.lock().await;
    let collection: Collection<Activity> = db.collection("activities");
    let pipeline = vec![
        doc! {"$match": {"name": {"$regex": query, "$options": "i"}}},
        doc! {"$skip": (page - 1) * perpage},
        doc! {"$limit": perpage},
    ];
    let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
    let mut activities = Vec::new();
    loop {
        let doc_result = cursor.try_next().await;
        if let Ok(Some(document)) = doc_result {
            if let Ok(activity) = from_document::<Activity>(document) {
                activities.push(activity);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if activities.is_empty() {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 404,
            message: "No activities found".to_string(),
        }
        .into();
        let response = ZVMSResponse::<(), ()>::Error(response);
        let response = serde_json::to_string(&response).unwrap();
        (StatusCode::NOT_FOUND, Json(response))
    } else {
        let response: SuccessResponse<_, ()> = SuccessResponse {
            status: ResponseStatus::Success,
            code: 200,
            data: activities,
            metadata: None,
        }
        .into();
        let response = ZVMSResponse::Success(response);
        let response = serde_json::to_string(&response).unwrap();
        (StatusCode::OK, Json(response))
    }
}

pub async fn read_one(
    Extension(client): Extension<Arc<Mutex<Database>>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    println!("ID: {:?}", id);
    let db = client.lock().await;
    let collection = db.collection("activities");
    let id = ObjectId::parse_str(&id);
    if let Err(_) = id {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 400,
            message: "Invalid ID".to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::BAD_REQUEST, Json(response));
    }
    let id = id.unwrap();
    let filter = doc! {"_id": id};
    let result = collection.find_one(filter, None).await;
    if let Err(e) = result {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 500,
            message: "Failed to read activity: ".to_string() + &e.to_string(),
        };
        let response = serde_json::to_string(&response).unwrap();
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
    }
    let result: Option<Activity> = result.unwrap();
    match result {
        Some(document) => {
            let response: SuccessResponse<Activity, ()> = SuccessResponse {
                status: ResponseStatus::Success,
                code: 200,
                data: document,
                metadata: None,
            };
            let response = serde_json::to_string(&response).unwrap();
            (StatusCode::OK, Json(response))
        }
        None => {
            let response = ErrorResponse {
                status: ResponseStatus::Error,
                code: 404,
                message: "Activity not found".to_string(),
            };
            let response = serde_json::to_string(&response).unwrap();
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}
