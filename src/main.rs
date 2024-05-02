extern crate chrono;
mod calc;
mod database;
mod launch;
mod models;
mod routers;
mod tests;
mod utils;
use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};
use launch::{generate_aes_key, generate_rsa_keypair};
use serde_json::Value;
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
use std::sync::Arc;
use tokio::sync::Mutex;

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
    tracing_subscriber::fmt().init();

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
        .route("/activity/", get(routers::activities::read::read_all))
        .route(
            "/activity/",
            post(routers::activities::insert::insert_activity),
        )
        .route("/activity/:id", get(routers::activities::read::read_one))
        .route(
            "/activity/:id",
            delete(routers::activities::remove::remove_activity),
        )
        .route(
            "/activity/:id/name",
            put(routers::activities::update::update_activity_name),
        )
        .route(
            "/activity/:id/description",
            put(routers::activities::update::update_activity_description),
        )
        .route(
            "/activity/:id/member/:member_id",
            get(routers::activities::members::read::read_member),
        )
        .route(
            "/activity/:id/member",
            post(routers::activities::members::insert::insert_member_into_activity),
        )
        .route(
            "/activity/:id/member/:member_id/status",
            put(routers::activities::members::update::update_member_status),
        )
        .route("/user/auth", post(routers::auth::login))
        .layer(Extension(shared_client.clone()));

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
