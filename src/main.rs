extern crate chrono;
mod database;
mod models;
mod routers;
mod utils;
mod launch;
use axum::{
    routing::{delete, get, post},
    Extension, Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::Value;
use launch::{generate_aes_key, generate_rsa_keypair};
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    socket.emit("auth", data).ok();

    socket.on(
        "message",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            socket.bin(bin).emit("message-back", data).ok();
        },
    );

    socket.on(
        "message-with-ack",
        |Data::<Value>(data), ack: AckSender, Bin(bin)| {
            ack.bin(bin).send(data).ok();
        },
    );
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = database::create_client()
        .await
        .expect("Failed to create client");

    let shared_client = Arc::new(Mutex::new(client));

    let (_, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    // Generate RSA keypair
    generate_rsa_keypair().await;

    // Generate AES key
    generate_aes_key().await;

    // Set up the router
    let app = Router::new()
        .route("/activity/:id", get(routers::activities::read::read_one))
        .route("/activity/:id", delete(routers::activities::remove::remove_activity))
        .route("/activity/", get(routers::activities::read::read_all))
        .route("/activity/", post(routers::activities::insert::insert_activity))
        .route("/user/auth", post(routers::auth::login))
        .layer(Extension(shared_client.clone()));

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
