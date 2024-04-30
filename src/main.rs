mod config;
mod database;
mod models;
mod routers;
mod utils;
mod launch;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use launch::{generate_aes_key, generate_rsa_keypair};

#[tokio::main]
async fn main() {
    let client = database::create_client()
        .await
        .expect("Failed to create client");

    let shared_client = Arc::new(Mutex::new(client));

    // Generate RSA keypair
    generate_rsa_keypair().await;

    // Generate AES key
    generate_aes_key().await;

    // Set up the router
    let app = Router::new()
        .route("/activity/:id", get(routers::activities::read::read_one))
        .route("/activity/", get(routers::activities::read::read_all))
        .route("/user/auth", post(routers::auth::login))
        .layer(Extension(shared_client.clone()));

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
