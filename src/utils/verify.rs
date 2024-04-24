use bcrypt::{hash, verify};

pub async fn hash_password(password: &str) -> String {
  let result = hash(password, 12);
  if let Ok(hashed) = result {
    hashed
  } else {
    String::new()
  }
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
  let result = verify(password, hash);
  if let Ok(valid) = result {
    valid
  } else {
    false
  }
}
