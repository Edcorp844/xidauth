use crate::database::database_error::DatabaseError;
use crate::database::utils::{BlacklistedToken, EncryptedData, User};
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, OsRng},
    AeadCore, Aes256Gcm, KeyInit,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use redis::{Client, Commands};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use serde_json;

#[derive(Clone)]
pub struct Database {
    redis: Arc<Mutex<Client>>,
    cipher: Arc<Mutex<Aes256Gcm>>,
}

impl Database {
    pub fn new(redis_url: &str, encryption_key: &[u8; 32]) -> Result<Self, DatabaseError> {
        let client = Client::open(redis_url)?;
        let cipher = Aes256Gcm::new(GenericArray::from_slice(encryption_key));

        Ok(Database {
            redis: Arc::new(Mutex::new(client)),
            cipher: Arc::new(Mutex::new(cipher)),
        })
    }

    fn encrypt(&self, data: &[u8]) -> Result<EncryptedData, DatabaseError> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .lock()
            .unwrap()
            .encrypt(&nonce, data)
            .map_err(|_| DatabaseError::Encryption)?;

        Ok(EncryptedData {
            ciphertext,
            nonce: nonce.to_vec(),
        })
    }

    fn decrypt(&self, data: &EncryptedData) -> Result<Vec<u8>, DatabaseError> {
        let nonce = GenericArray::from_slice(&data.nonce);
        self.cipher
            .lock()
            .unwrap()
            .decrypt(nonce, data.ciphertext.as_ref())
            .map_err(|_| DatabaseError::Decryption)
    }

    pub fn add_blacklisted_token(&self, token: &BlacklistedToken) -> Result<(), DatabaseError> {
        let serialized = serde_json::to_vec(token)?;
        let encrypted = self.encrypt(&serialized)?;

        let mut conn = self.redis.lock().unwrap().get_connection()?;
        conn.set_ex(
            format!("blacklist:{}", token.token),
            serde_json::to_vec(&encrypted)?,
            (token.expires_at - Utc::now().timestamp())
                .try_into()
                .unwrap(),
        )?;

        Ok(())
    }

    pub fn register_user(&self, email: String, password: String) -> Result<bool, DatabaseError> {
        let mut conn = self.redis.lock().unwrap().get_connection()?;

        if conn.hexists("users", &email)? {
            return Err(DatabaseError::UserAlreadyExists);
        }

        let password_hash = hash(&password, DEFAULT_COST)?;
        let user = User {
            uid: Uuid::new_v4(),
            email: email.clone(),
            password_hash,
        };

        let serialized_user = serde_json::to_vec(&user)?;
        let encrypted_user = self.encrypt(&serialized_user)?;
        conn.hset("users", &email, serde_json::to_vec(&encrypted_user)?)?;

        Ok(true)
    }

    pub fn login_user(&self, email: String, password: String) -> Result<bool, DatabaseError> {
        let mut conn = self.redis.lock().unwrap().get_connection()?;

        let encrypted_data: Vec<u8> = conn.hget("users", &email).map_err(|_| DatabaseError::InvalidCredentials)?;
        let encrypted: EncryptedData = serde_json::from_slice(&encrypted_data)?;
        let decrypted = self.decrypt(&encrypted)?;
        let user: User = serde_json::from_slice(&decrypted)?;

        if verify(&password, &user.password_hash)? {
            Ok(true)
        } else {
            Err(DatabaseError::InvalidCredentials)
        }
    }
}
