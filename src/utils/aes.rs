use rand::{RngCore, rngs::OsRng};

pub fn generate_aes256_key() -> String {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    hex::encode(key)
}

pub fn read_aes256_key() -> String {
    let key = std::fs::read_to_string("aes.key");
    if let Err(_) = key {
        let key = generate_aes256_key();
        let _ = std::fs::write("aes.key", key.as_bytes());
        return key;
    }
    key.unwrap()
}
