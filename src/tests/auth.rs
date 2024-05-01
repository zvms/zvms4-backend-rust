#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::utils::rsa::{generate_keypair, encrypt, decrypt};
    use crate::routers::auth::LoginCredentials;
    #[tokio::test]
    async fn rsa_validation() {
        let payload = "Hello, world!".to_string();
        let (private_key, public_key) = generate_keypair().await;
        let encrypted = encrypt(&public_key, payload.as_str());
        println!("Encrypted: {:?}", &encrypted);
        let decrypted = decrypt(&private_key, &encrypted).await;
        println!("Decrypted: {:?}", &decrypted);
        assert_eq!(payload, decrypted);
    }
    #[tokio::test]
    async fn rsa_validation_claims() {
        // Now timestamp is a u64 generated from SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let payload = LoginCredentials {
            password: "password".to_string(),
            timestamp: timestamp as u64,
        };
        let credential = serde_json::to_string(&payload);
        if let Err(_) = credential {
            assert!(false);
        }
        let credential = credential.unwrap();
        let (private_key, public_key) = generate_keypair().await;
        let encrypted = encrypt(&public_key, credential.as_str());
        println!("Encrypted: {:?}", &encrypted);
        let decrypted = decrypt(&private_key, &encrypted).await;
        println!("Decrypted: {:?}", &decrypted);
        let decrypted = serde_json::from_str(&decrypted.as_str());
        if let Err(_) = decrypted {
            assert!(false);
        }
        let decrypted = decrypted.unwrap();
        assert_eq!(payload, decrypted);
    }
}
