use crate::models::groups::GroupPermission;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TokenType {
    LongTerm,
    ShortTerm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub term: TokenType,
    pub perms: Vec<GroupPermission>,
}

pub trait TokenTrait {
    fn verify_token(&self, token: &str) -> Result<Token, String>;
}

pub fn generate_token(sub: &str, term: TokenType, perms: Vec<GroupPermission>) -> String {
    let now = SystemTime::now();
    let iat = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let exp = iat
        + match term {
            TokenType::LongTerm => 60 * 60 * 24 * 30,
            TokenType::ShortTerm => 60 * 60,
        };
    let token = Token {
        sub: sub.to_string(),
        exp,
        iat,
        term,
        perms,
    };
    let encoding_key = EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());
    encode(&Header::default(), &token, &encoding_key).unwrap()
}

impl TokenTrait for Token {
    fn verify_token(&self, token: &str) -> Result<Token, String> {
        let decoding_key = DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());
        let validation = Validation::default();
        match decode::<Token>(token, &decoding_key, &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(_) => Err("Invalid token".to_string()),
        }
    }
}
