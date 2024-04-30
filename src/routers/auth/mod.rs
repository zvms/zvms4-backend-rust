use crate::{
    models::{
        response::{ErrorResponse, ResponseStatus, SuccessResponse},
        users::{User, UserTrait},
    },
    utils::{
        jwt::TokenType,
        rsa::{decrypt, load_keypair},
    },
};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub credentials: String,
    pub userid: String,
    pub term: TokenType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub password: String,
    pub timestamp: u64,
}

pub async fn login(
    Extension(client): Extension<Arc<Mutex<Database>>>,
    Json(body): Json<LoginRequest>,

) -> impl IntoResponse {
    let client = client.lock().await;
    let collection = client.collection("users");
    let user: Option<User> = collection
        .find_one(Some(doc! {"id": body.userid}), None)
        .await
        .unwrap();
    if let Some(user) = user {
        let keypair = load_keypair().await;
        let credentials = decrypt(&keypair.0, &body.credentials.as_bytes()).await;
        let credentials = serde_json::from_str(&credentials);
        if let Err(_) = credentials {
            let response = ErrorResponse {
                status: ResponseStatus::Error,
                code: 400,
                message: "Invalid credentials".to_string(),
            };
            let response = json!(response);
            return (StatusCode::BAD_REQUEST, Json(response));
        }
        let credentials: LoginCredentials = credentials.unwrap();
        if user.clone().valid_password(credentials.password).await {
            let groups = client.collection("groups");
            let token = user.generate_token(&collection, &groups, body.term).await.unwrap();
            let response: SuccessResponse<String, ()> = SuccessResponse {
                status: ResponseStatus::Success,
                code: 200,
                data: token,
                metadata: None,
            };
            let response = json!(response);
            (StatusCode::OK, Json(response))
        } else {
            let response = ErrorResponse {
                status: ResponseStatus::Error,
                code: 401,
                message: "Invalid credentials".to_string(),
            };
            let response = json!(response);
            (StatusCode::UNAUTHORIZED, Json(response))
        }
    } else {
        let response = ErrorResponse {
            status: ResponseStatus::Error,
            code: 404,
            message: "User not found".to_string(),
        };
        let response = json!(response);
        (StatusCode::NOT_FOUND, Json(response))
    }
}
