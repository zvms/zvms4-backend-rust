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
    pub code: i32,
    pub data: T,
    pub metadata: Option<M>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ErrorResponse {
    pub status: ResponseStatus,
    pub code: i32,
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
