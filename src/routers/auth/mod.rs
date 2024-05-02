use crate::{
    models::{
        response::{create_error, ResponseStatus, SuccessResponse},
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
use bson::{doc, oid::ObjectId};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginRequest {
    pub credentials: String,
    pub userid: String,
    pub term: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    let id = ObjectId::from_str(&body.userid.as_str());
    if let Err(_) = id {
        return create_error(StatusCode::BAD_REQUEST, "Invalid user id".to_string());
    }
    let id = id.unwrap();
    let user = collection.find_one(Some(doc! {"_id": id}), None).await;
    if let Err(_) = user {
        return create_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to find user".to_string(),
        );
    }
    let user: Option<User> = user.unwrap();
    if let Some(user) = user {
        let keypair = load_keypair().await;
        let credentials = hex::decode(&body.credentials);
        if let Err(_) = credentials {
            return create_error(StatusCode::BAD_REQUEST, "Invalid credentials".to_string());
        }
        let credentials = credentials.unwrap();
        let credentials = decrypt(&keypair.0, &credentials).await;
        let credentials = serde_json::from_str(&credentials);
        if let Err(_) = credentials {
            return create_error(StatusCode::BAD_REQUEST, "Invalid credentials".to_string());
        }
        let credentials: LoginCredentials = credentials.unwrap();
        if user.clone().valid_password(credentials.password).await {
            let groups = client.collection("groups");
            let token = user.generate_token(&collection, &groups, body.term).await;
            if let Err(_) = token {
                return create_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to generate token".to_string(),
                );
            }
            let token = token.unwrap();
            println!("{:?}", token.clone());
            let response: SuccessResponse<String, ()> = SuccessResponse {
                status: ResponseStatus::Success,
                code: 200,
                data: token,
                metadata: None,
            };
            let response = json!(response).to_string();
            (StatusCode::OK, Json(response))
        } else {
            return create_error(StatusCode::UNAUTHORIZED, "Invalid credentials".to_string());
        }
    } else {
        return create_error(StatusCode::NOT_FOUND, "User not found".to_string());
    }
}
