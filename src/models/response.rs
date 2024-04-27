use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T, M> {
    pub status: ResponseStatus,
    pub code: i32,
    pub data: T,
    pub metadata: Option<M>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: ResponseStatus,
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response<T, M> {
    Success(SuccessResponse<T, M>),
    Error(ErrorResponse),
}

impl<T, M> Response<T, M> {
    pub fn success(data: T, metadata: Option<M>) -> Self {
        Response::Success(SuccessResponse {
            status: ResponseStatus::Success,
            code: 200,
            data,
            metadata,
        })
    }

    pub fn error(code: i32, message: String) -> Self {
        Response::Error(ErrorResponse {
            status: ResponseStatus::Error,
            code,
            message,
        })
    }
}
