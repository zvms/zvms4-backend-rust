use rsa::{RsaPrivateKey, RsaPublicKey, pkcs1v15};
use rand::rngs::OsRng;

pub async fn generate_keypair() -> (RsaPrivateKey, RsaPublicKey) {
  let mut rng = OsRng;
  let bits = 2048;
  let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
  let public_key = private_key.to_public_key();
  (private_key, public_key)
}

pub async fn encrypt(public_key: &RsaPublicKey, message: &str) -> Vec<u8> {
  let mut rng = OsRng;
  let result = public_key.encrypt(&mut rng, pkcs1v15::Pkcs1v15Encrypt, message.as_bytes());
  if let Ok(encrypted) = result {
    encrypted
  } else {
    Vec::new()
  }
}

pub async fn decrypt(private_key: &RsaPrivateKey, encrypted: &[u8]) -> String {
  let result = private_key.decrypt(pkcs1v15::Pkcs1v15Encrypt, encrypted);
  if let Ok(decrypted) = result {
    String::from_utf8(decrypted).expect("Failed to convert to string")
  } else {
    String::new()
  }
}
