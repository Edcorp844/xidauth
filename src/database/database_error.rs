use bcrypt::BcryptError;
use redis::RedisError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError), // Converts RedisError into DatabaseError::Redis

    #[error("Serialization error: {0}")]
    Serialization(#[from] SerdeError), // Converts SerdeError into DatabaseError::Serialization

    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] BcryptError), // Converts BcryptError into DatabaseError::Bcrypt

    #[error("Encryption error")]
    Encryption, // No underlying error type to convert

    #[error("Decryption error")]
    Decryption, // No underlying error type to convert

    #[error("User already exists")]
    UserAlreadyExists, // Custom error with no underlying type

    #[error("Invalid credentials")]
    InvalidCredentials, // Custom error with no underlying type

    #[error("UTF-8 conversion error")]
    Utf8ConversionError,
}

impl From<std::string::FromUtf8Error> for DatabaseError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        DatabaseError::Utf8ConversionError
    }
}