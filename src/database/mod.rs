// src/database.rs

use crate::config;
use mongodb::{options::ClientOptions, Client, Database};
use std::error::Error;

pub async fn create_client() -> Result<Database, Box<dyn Error>> {
    println!("Connecting to MongoDB");
    let mut client_options = ClientOptions::parse(config::MONGO_URL).await?;
    client_options.app_name = Some("zvms".to_string());
    let client = Client::with_options(client_options)?;
    let database = client.database("zvms");
    Ok(database)
}
