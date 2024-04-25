use crate::models::{
    activities::Activity,
    response::{ErrorResponse, Response as ZVMSResponse, ResponseStatus, SuccessResponse},
};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::{doc, from_document, oid::ObjectId};
use futures::stream::StreamExt;
use mongodb::{Collection, Database};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn read_all(Extension(db): Extension<Arc<Mutex<Database>>>) -> impl IntoResponse {
    let db = db.lock().await;
    let collection = db.collection("activities");
    let cursor = collection.find(None, None).await.unwrap();
    let activities: Vec<Activity> = cursor
        .filter_map(|item| async move {
            match item {
                Ok(activity) => Some(activity),
                Err(_) => None,
            }
        })
        .collect()
        .await;
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

pub async fn read_with_filter(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    page: u32,
    perpage: u32,
    query: String,
) -> Json<Vec<Activity>> {
    let db = db.lock().await;
    let collection = db.collection("activities") as Collection<Activity>;
    let pipeline = vec![
        doc! {"$match": {"name": {"$regex": query, "$options": "i"}}},
        doc! {"$skip": (page - 1) * perpage as u32},
        doc! {"$limit": perpage as u32},
    ];

    let mut cursor = match collection.aggregate(pipeline, None).await {
        Ok(c) => c,
        Err(_) => return Json(vec![].into()),
    };

    let mut activities = Vec::new();
    while let Some(doc_result) = cursor.next().await {
        match doc_result {
            Ok(document) => match from_document::<Activity>(document) {
                Ok(activity) => activities.push(activity),
                Err(_) => return Json(vec![].into()),
            },
            Err(_) => return Json(vec![].into()),
        }
    }

    Json(activities)
}

pub async fn read_one(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    id: String,
) -> Result<Json<Activity>, String> {
    let db = db.lock().await;
    let collection = db.collection("activities");
    let filter = doc! {"_id": ObjectId::parse_str(&id).unwrap()};
    let result = collection.find_one(filter, None).await.unwrap();
    match result {
        Some(document) => {
            let activity = from_document(document).unwrap();
            Ok(Json(activity))
        }
        None => Err("Activity not found".into()),
    }
}
