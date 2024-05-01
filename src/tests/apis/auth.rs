/*
Use user: 105
password: 105
ObjectId: 65e6fa210edc81d012ec483a
*/

#[cfg(test)]
mod tests {
    use crate::database::create_client;
    use crate::launch::generate_rsa_keypair;
    use crate::routers::auth::{login, LoginCredentials, LoginRequest};
    use crate::utils::jwt::TokenType;
    use crate::utils::rsa::{encrypt, load_keypair};
    use axum::extract::Extension;
    use axum::response::IntoResponse;
    use axum::Json;
    use std::sync::Arc;
    use std::time::SystemTime;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_login() {
        generate_rsa_keypair().await;
        let client = create_client().await.unwrap();
        let client = Arc::new(Mutex::new(client));
        let client = Extension(client);
        let credential = LoginCredentials {
            password: "105".to_string(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };
        let credential = serde_json::to_string(&credential).unwrap();
        let (_, public_key) = load_keypair().await;
        let credential = encrypt(&public_key, credential.as_str());
        let credential = hex::encode(credential);
        let request = LoginRequest {
            credentials: credential,
            userid: "65e6fa210edc81d012ec483a".to_string(),
            term: TokenType::LongTerm
        };
        let result = login(client, Json(request)).await;
        let result = result.into_response();
        assert!(result.status().is_success())
    }
}
