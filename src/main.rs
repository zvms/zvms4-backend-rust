mod config;
mod database;
mod models;
mod routers;
use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let client = database::create_client()
        .await
        .expect("Failed to create client");
    let shared_client = Arc::new(Mutex::new(client));

    // Set up the router
    let app = Router::new();

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
