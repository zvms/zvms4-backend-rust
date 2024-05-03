use crate::{
    models::{
        activities::Activity,
        groups::GroupPermission,
        response::{create_error, MetadataSize, ResponseStatus, SuccessResponse},
    },
    routers::activities::read::ReadActivityQuery,
    utils::{groups::same_class::validate_same_class, jwt::UserData},
};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::{doc, from_document, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

pub async fn read_user_activities(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Query(ReadActivityQuery {
        page,
        perpage,
        query,
    }): Query<ReadActivityQuery>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let page = page.unwrap_or(1);
    let perpage = perpage.unwrap_or(10);
    let query = query.unwrap_or("".to_string());
    let user_id = ObjectId::from_str(&user_id);
    if let Err(_) = user_id {
        return create_error(StatusCode::BAD_REQUEST, "Invalid user ID".to_string());
    }
    let user_id = user_id.unwrap();
    let is_same_class = validate_same_class(
        db.clone(),
        ObjectId::from_str(user.id.as_str()).unwrap(),
        user_id.clone(),
    )
    .await;
    let db = db.lock().await;
    let collection: Collection<Activity> = db.collection("activities");
    if user.perms.contains(&GroupPermission::Admin)
        || user.perms.contains(&GroupPermission::Auditor)
        || user.perms.contains(&GroupPermission::Department)
    {
    } else if user.perms.contains(&GroupPermission::Secretary) {
        if let Ok(true) = is_same_class {
        } else {
            return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
        }
    } else if user.id != user_id.clone().to_hex() {
        return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
    }
    let counts = collection
        .count_documents(
            doc! {
                "_id": user_id
            },
            None,
        )
        .await;
    if let Err(_) = counts {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to count documents".to_string(),
        );
    }
    let counts = counts.unwrap();
    let pipeline = [
        doc! {
            "$match": {
                "user": user_id
            }
        },
        doc! {
            "$match": {
                "$or": [
                    {
                        "name": {
                            "$regex": &query,
                            "$options": "i"
                        }
                    },
                ]
            }
        },
        doc! {
            "$project": {
                // Only display `members._id == user_id` for `members` array
                "members": {
                    "$filter": {
                        "input": "$members",
                        "as": "member",
                        "cond": {
                            "$eq": [
                                "$$member._id",
                                user_id
                            ]
                        }
                    }
                },
            }
        },
        doc! {
            "$project": {
                "members": 1,
                "name": 1,
                "description": 1,
                "start": 1,
                "end": 1,
                "created_at": 1,
                "updated_at": 1,
                "deleted_at": 1,
            }
        },
        doc! {
            "$sort": {
                "_id": -1
            }
        },
        doc! {
            "$skip": (page - 1) * perpage
        },
        doc! {
            "$limit": perpage
        },
    ];
    let cursor = collection.aggregate(pipeline, None).await;
    if let Err(_) = cursor {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch documents".to_string(),
        );
    }
    let mut cursor = cursor.unwrap();
    let mut activities: Vec<Activity> = Vec::new();
    let metadata = MetadataSize { size: counts };
    while let Some(activity) = cursor.try_next().await.unwrap() {
        activities.push(from_document(activity).unwrap());
    }
    let response = SuccessResponse {
        status: ResponseStatus::Success,
        code: 200,
        data: activities,
        metadata: Some(metadata),
    };
    let response = serde_json::to_string(&response).unwrap();
    (StatusCode::OK, Json(response))
}
