use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SuccessResponse<T, M> {
    pub status: ResponseStatus,
    pub code: i32,
    pub data: T,
    pub metadata: Option<M>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorResponse {
    pub status: ResponseStatus,
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Response<T, M> {
    Success(SuccessResponse<T, M>),
    Error(ErrorResponse),
}
