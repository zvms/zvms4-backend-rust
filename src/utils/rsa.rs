use rand::rngs::OsRng;
use rsa::{
    pkcs1::EncodeRsaPrivateKey, pkcs1::EncodeRsaPublicKey, pkcs1v15, RsaPrivateKey, RsaPublicKey,
};
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey};
use serde::{Deserialize, Serialize};
use tokio::fs::{read, write};
use tokio::fs::File;
use pem::parse;
use tokio::io::AsyncReadExt;

pub async fn generate_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let public_key = private_key.to_public_key();
    (private_key, public_key)
}

pub async fn save_keypair(private_key: &RsaPrivateKey, public_key: &RsaPublicKey) {
    let private_key_bytes = private_key.to_pkcs1_pem(Default::default());
    if let Ok(private_key_bytes) = private_key_bytes {
        write("private.pem", private_key_bytes).await.expect("Failed to save private key");
    }
    let public_key_bytes = public_key.to_pkcs1_pem(Default::default());
    if let Ok(public_key_bytes) = public_key_bytes {
        write("public.pem", public_key_bytes).await.expect("Failed to save public key");
    }
}

pub async fn load_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    // Asynchronously read the private key PEM file
    let mut private_key_file = File::open("private.pem").await.unwrap();
    let mut private_key_pem = String::new();
    private_key_file.read_to_string(&mut private_key_pem).await.unwrap();
    let private_key_pem = parse(&private_key_pem).unwrap();

    // Decode the private key from PKCS#1 DER
    let private_key = RsaPrivateKey::from_pkcs1_der(&private_key_pem.contents()).unwrap();

    // Asynchronously read the public key PEM file
    let mut public_key_file = File::open("public.pem").await.unwrap();
    let mut public_key_pem = String::new();
    public_key_file.read_to_string(&mut public_key_pem).await.unwrap();
    let public_key_pem = parse(&public_key_pem).unwrap();

    // Decode the public key from PKCS#1 DER
    let public_key = RsaPublicKey::from_pkcs1_der(&public_key_pem.contents()).unwrap();

    println!("RSA private key and public key loaded successfully.");

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
