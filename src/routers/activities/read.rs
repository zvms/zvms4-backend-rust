use crate::{
    models::{
        activities::Activity,
        groups::GroupPermission,
        response::{create_error, MetadataSize, ResponseStatus, SuccessResponse},
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
use mongodb::{options::FindOneOptions, Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadActivityQuery {
    pub page: Option<u32>,
    pub perpage: Option<u32>,
    pub query: Option<String>,
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
        return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
    }
    let page = page.unwrap_or(1);
    let perpage = perpage.unwrap_or(10);
    let query = query.unwrap_or("".to_string());
    let db = db.lock().await;
    let collection: Collection<Activity> = db.collection("activities");
    let target = if user.perms.contains(&GroupPermission::Auditor)
        || user.perms.contains(&GroupPermission::Admin)
    {
        "pending"
    } else {
        ""
    };
    let count = collection
        .count_documents(
            doc! {"name": {"$regex": query.clone(), "$options": "i"}},
            None,
        )
        .await;
    if let Err(e) = count {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read activity: ".to_string() + &e.to_string(),
        );
    }
    let pipeline = vec![
        doc! {"$match": {"name": {"$regex": query, "$options": "i"}}},
        doc! {"$sort": {"_id": -1}},
        doc! {"$project": {
                "name": 1,
                "type": 1,
                "status": 1,
                "date": 1,
                "createdAt": 1,
                "updatedAt": 1,
                "creator": 1,
                "members": {
                    "$filter": {
                        "input": "$members",
                        "as": "member",
                        "cond": {"$eq": ["$$member.status", target]}
                    }
                }
            }
        },
        doc! {"$project": {
            "members.history": 0,
            "members.impression": 0,
            "members.images": 0,
        }},
        doc! {"$skip": (page - 1) * perpage},
        doc! {"$limit": perpage},
    ];
    let cursor = collection.aggregate(pipeline, None).await;
    if let Err(e) = cursor {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read activity: ".to_string() + &e.to_string(),
        );
    }
    let mut cursor = cursor.unwrap();
    let mut activities = Vec::new();
    loop {
        let doc_result = cursor.try_next().await;
        if let Err(e) = doc_result {
            return create_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read activity: ".to_string() + &e.to_string(),
            );
        }
        if let Ok(Some(document)) = doc_result {
            match from_document::<Activity>(document) {
                Ok(activity) => activities.push(activity),
                Err(e) => {
                    return create_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to read activity: ".to_string() + &e.to_string(),
                    )
                }
            }
        } else {
            break;
        }
    }
    let response: SuccessResponse<_, MetadataSize> = SuccessResponse {
        status: ResponseStatus::Success,
        code: 200,
        data: activities,
        metadata: Some(MetadataSize {
            size: count.unwrap(),
        }),
    };
    let response = serde_json::to_string(&response).unwrap();
    (StatusCode::OK, Json(response))
}

pub async fn read_one(
    Extension(client): Extension<Arc<Mutex<Database>>>,
    _: UserData,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = client.lock().await;
    let collection = db.collection("activities");
    let id = ObjectId::parse_str(&id);
    if let Err(_) = id {
        return create_error(StatusCode::BAD_REQUEST, "Invalid ID".to_string());
    }
    let id = id.unwrap();
    let filter = doc! {"_id": id};
    let projection = doc! {
        "members.history": 0,
        "members.impression": 0,
        "members.images": 0,
    };
    let result = collection
        .find_one(
            filter,
            Some(FindOneOptions::builder().projection(projection).build()),
        )
        .await;
    if let Err(e) = result {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read activity: ".to_string() + &e.to_string(),
        );
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
            return create_error(StatusCode::NOT_FOUND, "Activity not found".to_string());
        }
    }
}
