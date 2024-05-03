use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::oid::ObjectId;
use mongodb::Database;
use std::{fs, str::FromStr, sync::Arc, time::SystemTime};
use tempfile::NamedTempFile;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    models::{
        exports::{ExportActivityTimesOptions, ExportState, Task, TaskStatus},
        groups::GroupPermission,
        response::create_error,
    },
    utils::{
        exports::{csv_to_excel, export_csv},
        jwt::UserData,
    },
};

pub async fn export_activity_times(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    Extension(exporters): Extension<Arc<ExportState>>,
    user: UserData,
    Json(options): Json<ExportActivityTimesOptions>,
) -> impl IntoResponse {
    if !user.perms.contains(&GroupPermission::Admin)
        && !user.perms.contains(&GroupPermission::Inspector)
    {
        return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string())
            .into_response();
    }

    let task_id = Uuid::new_v4();
    println!(
        "Received task to export activity times, job ID: {}",
        task_id
    );

    if let Err(e) = ObjectId::from_str(&user.id) {
        return create_error(StatusCode::BAD_REQUEST, format!("Invalid user ID: {}", e))
            .into_response();
    }
    let user_id = ObjectId::from_str(&user.id).unwrap();

    println!("Starting to export excel by user {}", user_id);

    let mut tasks = exporters.lock().await;
    let task = create_task(user_id, &options);
    tasks.insert(task_id, task);
    // Release the lock
    drop(tasks);

    let _ = spawn_task(task_id, Arc::clone(&exporters), db).await;

    (axum::http::StatusCode::OK, Json(task_id.to_string())).into_response()
}

pub async fn query_export_status(
    Extension(exporters): Extension<Arc<ExportState>>,
    Path(task_id): Path<String>,
) -> impl IntoResponse {
    let task_id = Uuid::parse_str(&task_id);
    if let Err(_) = task_id {
        return create_error(StatusCode::BAD_REQUEST, "Invalid task ID".to_string())
            .into_response();
    }
    let task_id = task_id.unwrap();
    let tasks = exporters.lock().await;
    let task = tasks.get(&task_id);
    if let None = task {
        return create_error(StatusCode::NOT_FOUND, "Task not found".to_string()).into_response();
    }
    let task = task.unwrap();
    (axum::http::StatusCode::OK, Json(task)).into_response()
}

fn create_task(user_id: ObjectId, options: &ExportActivityTimesOptions) -> Task {
    Task {
        time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        status: TaskStatus::Pending,
        options: options.clone(),
        actioner: user_id,
        result: None,
        percent: Some(0.0),
    }
}

async fn spawn_task(task_id: Uuid, exporters: Arc<ExportState>, db: Arc<Mutex<Database>>) {
    println!("Start to spawn task {}", task_id);
    let _ = tokio::spawn(async move {
        process_task(task_id, Arc::clone(&exporters), Arc::clone(&db)).await;
    })
    .await;
}

async fn process_task(task_id: Uuid, exporters: Arc<ExportState>, db: Arc<Mutex<Database>>) {
    println!("Start to process task {}", task_id);
    let mut tasks = exporters.lock().await;
    let task = tasks.get_mut(&task_id).unwrap();
    task.status = TaskStatus::Processing;
    println!("Task {} is processing", task_id);
    let result = export_csv::export_to_dataframe(db).await;
    if let Err(_) = result {
        task.status = TaskStatus::Error;
        task.result = None;
        return;
    }
    let result = result.unwrap();
    let temp_csv = NamedTempFile::new();
    if let Err(_) = temp_csv {
        task.status = TaskStatus::Error;
        task.result = None;
        return;
    }
    let temp_csv = temp_csv.unwrap();
    let temp_csv_name = temp_csv.path().to_str();
    if let None = temp_csv_name {
        task.status = TaskStatus::Error;
        task.result = None;
        return;
    }
    let temp_csv_name = String::from(temp_csv_name.unwrap());
    let temp_csv = temp_csv.as_file();
    println!("Start to save to csv");
    let result = export_csv::save_to_csv(result, temp_csv).await;
    if let Err(_) = result {
        task.status = TaskStatus::Error;
        task.result = None;
        return;
    }
    println!("Start to convert to excel {}", temp_csv_name);
    let temp_excel = NamedTempFile::new();
    if let Err(_) = temp_excel {
        task.status = TaskStatus::Error;
        task.result = None;
        return;
    }
    let temp_excel = temp_excel.unwrap();
    let temp_excel_name = temp_excel.path().to_str();
    if let None = temp_excel_name {
        task.status = TaskStatus::Error;
        task.result = None;
        return;
    }
    let temp_excel_name = String::from(temp_excel_name.unwrap());
    let result = csv_to_excel::to_excel(temp_csv_name.clone(), temp_excel_name.clone());
    if let Err(_) = result {
        task.status = TaskStatus::Error;
        task.result = None;
        return;
    }
    task.status = TaskStatus::Done;
    task.result = Some(temp_excel_name.clone());
    fs::copy(
        temp_excel_name.clone().as_str(),
        format!("public/exports/{}.xlsx", task_id),
    )
    .unwrap();
}
