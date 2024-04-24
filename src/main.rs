mod config;
mod database;
mod models;
mod utils;
mod routers;
use axum::{routing::get, Extension, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let client = database::create_client()
        .await
        .expect("Failed to create client");
    let shared_client = Arc::new(Mutex::new(client));

    println!("Server running on port 3000");

    // Set up the router
    let app = Router::new().route("/activity/", get(routers::activities::read::read_one)).layer(Extension(shared_client));

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
