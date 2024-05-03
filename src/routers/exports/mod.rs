use crate::{
    models::{groups::GroupPermission, response::create_error},
    utils::jwt::UserData,
};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ExportFormat {
    CSV,
    JSON,
    Excel,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportActivityTimesOptions {
    pub start: u64, // Unix timestamp
    pub end: u64,   // Unix timestamp
    pub format: ExportFormat,
}

pub async fn export_activity_times(
    Extension(db): Extension<Arc<Mutex<Database>>>,
    user: UserData,
    Json(options): Json<ExportActivityTimesOptions>,
) -> impl IntoResponse {
    if !user.perms.contains(&GroupPermission::Admin)
        && !user.perms.contains(&GroupPermission::Inspector)
    {
        return create_error(StatusCode::FORBIDDEN, "Permission denied".to_string());
    }

    (StatusCode::OK, Json("".to_string()))
}
