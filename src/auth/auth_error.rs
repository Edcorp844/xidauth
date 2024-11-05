use serde_json::Error as SerdeError;
use std::fmt;

// Define the AuthError enum with variants for different error types.
#[derive(Debug)]
pub enum AuthError {
    UserAlreadyExists,
    Json(SerdeError),
}

// Implement the From trait to convert SerdeError into AuthError::Json.
impl From<SerdeError> for AuthError {
    fn from(err: SerdeError) -> AuthError {
        AuthError::Json(err)
    }
}

// Implement fmt::Display for AuthError to provide custom error messages.
impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::UserAlreadyExists => write!(f, "User already exists."),
            AuthError::Json(err) => write!(f, "JSON error: {}", err),
        }
    }
}

// Optional: Implement the std::error::Error trait for AuthError to allow it to be used in error handling more effectively.
impl std::error::Error for AuthError {}
