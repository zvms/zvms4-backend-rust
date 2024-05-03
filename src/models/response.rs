use axum::Json;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct SuccessResponse<T, M> {
    pub status: ResponseStatus,
    pub code: u16,
    pub data: T,
    pub metadata: Option<M>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ErrorResponse {
    pub status: ResponseStatus,
    pub code: u16,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Response<T, M> {
    Success(SuccessResponse<T, M>),
    Error(ErrorResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct MetadataSize {
    pub size: u64,
}

pub fn create_error(code: StatusCode, message: String) -> (StatusCode, Json<String>) {
    let resposne = ErrorResponse {
        status: ResponseStatus::Error,
        code: code.as_u16(),
        message,
    };
    let response = serde_json::to_string(&resposne).unwrap();
    (code, Json(response))
}
