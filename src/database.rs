// src/database.rs

use crate::config;
use mongodb::{options::ClientOptions, Client};
use std::error::Error;

pub async fn create_client() -> Result<Client, Box<dyn Error>> {
    let mut client_options = ClientOptions::parse(config::MONGO_URL).await?;
    client_options.app_name = Some("zvms".to_string());
    Ok(Client::with_options(client_options)?)
}
