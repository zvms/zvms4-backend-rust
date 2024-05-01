#[cfg(test)]
mod tests {
    use crate::{
        models::groups::GroupPermission,
        routers::auth::LoginCredentials,
        utils::{
            jwt::{generate_token, verify_token, TokenType},
            rsa::{decrypt, encrypt, generate_keypair},
        },
    };
    use bson::oid::ObjectId;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn rsa_validation_claims() {
        // Now timestamp is a u64 generated from SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
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
        let decrypted = decrypt(&private_key, &encrypted).await;
        let decrypted = serde_json::from_str(&decrypted.as_str());
        if let Err(_) = decrypted {
            assert!(false);
        }
        let decrypted = decrypted.unwrap();
        println!("Decrypted: {:#?}", &decrypted);
        assert_eq!(payload, decrypted);
    }
    #[tokio::test]
    async fn token_validation() {
        let sub = ObjectId::new().to_hex();
        let perms = vec![GroupPermission::Student, GroupPermission::Admin];
        let token = generate_token(sub.as_str(), TokenType::LongTerm, perms.clone());
        let token = token.as_str();
        println!("Token: {:?}", token);
        let result = verify_token(token.to_string());
        if let Err(_) = result {
            assert!(false);
        }
        let result = result.unwrap();
        assert_eq!(result.sub, sub);
        assert_eq!(result.perms, perms);
        assert_eq!(result.term, TokenType::LongTerm);
    }
}
