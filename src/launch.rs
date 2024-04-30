use tokio::fs::{try_exists, write};
use crate::utils::{rsa::{generate_keypair, save_keypair}, aes::generate_aes256_key};

pub async fn generate_rsa_keypair() {
    let private_exists = try_exists("private.pem").await.unwrap();
    let public_exists = try_exists("public.pem").await.unwrap();
    if !private_exists || !public_exists {
        let (private_key, public_key) = generate_keypair().await;
        save_keypair(&private_key, &public_key).await;
    }
}

pub async fn generate_aes_key() {
    let exists = try_exists("aes.key").await.unwrap();
    if !exists {
        let key = generate_aes256_key();
        write("aes.key", key).await.unwrap();
    }
}
