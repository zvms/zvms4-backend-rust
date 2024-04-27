use crate::utils::{rsa::decrypt, verify::verify_password};
use axum::{extract::Extension, response::{IntoResponse, Json}};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use mongodb::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub credentials: String,
    pub userid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub password: String,
    pub timestamp: u64,
}


pub async fn login(Extension(client): Extension<Arc<Mutex<Database>>>, body: Json<LoginRequest>) -> impl IntoResponse {

}
