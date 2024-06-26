use std::env;

use serde::{Deserialize, Serialize};
use tokio::fs::{read, write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: String,
    pub database: String,
    pub timezone: String,
    pub port: u16,
}

pub async fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config = read("config.json").await?;
    let config = serde_json::from_slice(&config)?;
    Ok(config)
}

pub async fn save_config(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let config = serde_json::to_vec(&config)?;
    write("config.json", config).await?;
    Ok(())
}

pub async fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("DATABASE_URL").unwrap_or("mongodb://localhost:27017".to_string());
    let config = Config {
        server: url,
        database: "zvms".to_string(),
        timezone: "0".to_string(),
        port: 8080,
    };
    save_config(config).await?;
    Ok(())
}

pub async fn load_or_init_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config = load_config().await;
    match config {
        Ok(config) => Ok(config),
        Err(_) => {
            init_config().await?;
            load_config().await
        }
    }
}

pub fn load_config_sync() -> Result<Config, Box<dyn std::error::Error>> {
    let config = std::fs::read("config.json");
    if let Err(_) = config {
        return Err("Failed to read config file".into());
    }
    let config = config.unwrap();
    let config: Config = serde_json::from_slice(&config)?;
    Ok(config)
}
