use crate::models::activities::Activity;
use axum::{extract::Extension, response::Json, Router};
use futures::stream::StreamExt;
use mongodb::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn read_all(Extension(db): Extension<Arc<Mutex<Database>>>) -> Json<Vec<Activity>> {
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
    Json(activities)
}
