use rand::{RngCore, rngs::OsRng};

pub fn generate_aes256_key() -> String {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    hex::encode(key)
}
