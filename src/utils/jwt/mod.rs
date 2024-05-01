use crate::models::groups::GroupPermission;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::utils::aes::read_aes256_key;
use std::time::{SystemTime, UNIX_EPOCH};
pub mod valid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum TokenType {
    LongTerm,
    ShortTerm,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub term: TokenType,
    pub perms: Vec<GroupPermission>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserData {
    pub id: String,
    pub perms: Vec<GroupPermission>,
    pub term: TokenType,
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
    let encoding_key = EncodingKey::from_secret(read_aes256_key().as_bytes());
    encode(&Header::default(), &token, &encoding_key).unwrap()
}

pub fn verify_token(token: String) -> Result<Token, String> {
    let token = token.replace("Bearer ", "");
    let token = token.as_str();
    let decoding_key = DecodingKey::from_secret(read_aes256_key().as_bytes());
    let validation = Validation::default();
    match decode::<Token>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err("Invalid token".to_string()),
    }
}
