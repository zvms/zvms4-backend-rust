use crate::models::response::{ErrorResponse, ResponseStatus};
use crate::utils::jwt::{verify_token, UserData};
use axum::{
    async_trait,
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuthenticationError {
    InvalidToken,
    MissingToken,
    ExpiredToken,
}

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> Response<Body> {
        let (status, response) = match self {
            AuthenticationError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
            }
            AuthenticationError::MissingToken => {
                (StatusCode::BAD_REQUEST, "Missing token".to_string())
            }
            AuthenticationError::ExpiredToken => {
                (StatusCode::UNAUTHORIZED, "Expired token".to_string())
            }
        };
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: status.as_u16(),
            message: response,
        };
        let response = serde_json::to_string(&response).unwrap();
        (StatusCode::UNAUTHORIZED, response).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserData
where
    S: Send + Sync + 'static,
{
    type Rejection = AuthenticationError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let token = parts.extract::<TypedHeader<Authorization<Bearer>>>().await;
        if let Ok(token) = token {
            let token = token.0.token().to_string();
            let token = verify_token(token);
            if let Ok(token) = token {
                let data = UserData {
                    id: token.sub,
                    perms: token.perms,
                    term: token.term,
                };
                Ok(data)
            } else {
                return Err(AuthenticationError::InvalidToken);
            }
        } else {
            Err(AuthenticationError::MissingToken)
        }
    }
}
