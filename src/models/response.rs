use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Error,
}

impl ToString for ResponseStatus {
    fn to_string(&self) -> String {
        match self {
            ResponseStatus::Success => "success".to_string(),
            ResponseStatus::Error => "error".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T, M> {
    pub status: ResponseStatus,
    pub code: i32,
    pub data: Vec<T>,
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
    pub fn success(data: Vec<T>, metadata: Option<M>) -> Self {
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
