// src/database.rs

use crate::utils::config::load_or_init_config;
use mongodb::{options::ClientOptions, Client, Database};
use std::error::Error;

pub async fn create_client() -> Result<Database, Box<dyn Error>> {
    println!("Connecting to MongoDB");
    let config = load_or_init_config().await?;
    let mut client_options = ClientOptions::parse(config.server).await?;
    let name = config.database;
    client_options.app_name = Some(name.clone());
    let client = Client::with_options(client_options)?;
    let database = client.database(&name);
    Ok(database)
}
