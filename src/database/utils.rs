use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub uid: Uuid,
    pub email: String, // Encrypted email
    pub password_hash: String,
    //   pub nonce: Vec<u8>, // Nonce used for encryption
}

#[derive(Serialize, Deserialize)]
pub struct BlacklistedToken {
    pub token: String,
    pub expires_at: i64,
    pub reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}
