use crate::database::database::Database;
use crate::database::database_error::DatabaseError;
use crate::database::utils::BlacklistedToken;
use uuid::Uuid;

pub struct AuthService {
    database: Database,
}

impl AuthService {
    pub fn new(database: Database) -> Self {
        AuthService { database }
    }

    /// Registers a new user with the given email and password.
    ///
    /// # Arguments
    ///
    /// * `email` - The email of the user.
    /// * `password` - The password of the user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the user is successfully registered.
    /// Returns `Err(DatabaseError::UserAlreadyExists)` if the user already exists.
    pub fn register_user(&self, email: String, password: String) -> Result<bool, DatabaseError> {
        self.database.register_user(email, password)
    }

    /// Logs in a user with the given email and password.
    ///
    /// # Arguments
    ///
    /// * `email` - The email of the user.
    /// * `password` - The password of the user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the login is successful.
    /// Returns `Err(DatabaseError::InvalidCredentials)` if the credentials are invalid.
    pub fn login_user(&self, email: String, password: String) -> Result<bool, DatabaseError> {
        self.database.login_user(email, password)
    }

    /// Adds a token to the blacklist with an expiration time and a reason.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to blacklist.
    /// * `expires_at` - The expiration time of the token.
    /// * `reason` - The reason for blacklisting the token.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the token is successfully blacklisted.
    pub fn add_blacklisted_token(&self, token: BlacklistedToken) -> Result<(), DatabaseError> {
        self.database.add_blacklisted_token(&token)
    }

 /*     /// Checks if a token is blacklisted.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to check.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the token is blacklisted.
    /// Returns `Ok(false)` if the token is not blacklisted.
   pub fn is_token_blacklisted(&self, token: &str) -> Result<bool, DatabaseError> {
        let mut conn = self.database.redis.lock().unwrap().get_connection()?;
        let key = format!("blacklist:{}", token);
        let exists: bool = conn.exists(&key)?;
        Ok(exists)
    } */
}