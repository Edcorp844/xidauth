use crate::db::{database::DataBase, db_error::DBError};
use redis::RedisError;
use serde_json::json;
use std::collections::HashMap;

pub struct AuthService {
    db: DataBase, // Use DataBase for Redis operations
}

impl AuthService {
    // Initialize the AuthService with a DataBase instance.
    pub fn new(redis_url: &str) -> Result<Self, RedisError> {
        let db = DataBase::new(redis_url)?;
        Ok(AuthService { db })
    }

    // Register a new user in Redis with JSON data.
    pub fn register_user(
        &mut self,
        user_id: &str,
        user_info: &HashMap<&str, &str>,
    ) -> Result<(), RedisError> {
        // Convert user_info HashMap to JSON
        let json_data = json!(user_info).to_string();
        self.db.set_json_value(user_id, &json_data)?;
        Ok(())
    }

    // Authenticate user by checking the stored password.
    pub fn authenticate_user(&mut self, user_id: &str, password: &str) -> Result<bool, DBError> {
        // Retrieve user data from Redis as JSON
        let user_data: Option<HashMap<String, String>> = Some(self.db.get_json_value(user_id)?);

        // Check if user data exists
        if let Some(user_info) = user_data {
            if let Some(stored_password) = user_info.get("password") {
                return Ok(stored_password == password); // Password matches
            }
        }

        Ok(false) // Authentication failed
    }

    // Store an OAuth token for a user (with expiration).
    pub fn store_oauth_token(
        &mut self,
        user_id: &str,
        token: &str,
        expiry_secs: usize,
    ) -> Result<(), RedisError> {
        self.db
            .set_value(&format!("oauth_token:{}", user_id), token)?;
        self.db
            .expire_key(&format!("oauth_token:{}", user_id), expiry_secs)?;
        Ok(())
    }

    // Retrieve the OAuth token for a user.
    pub fn get_oauth_token(&mut self, user_id: &str) -> Result<Option<String>, RedisError> {
        self.db.get_value(&format!("oauth_token:{}", user_id))
    }

    // Store a session with expiration in Redis.
    pub fn set_session(
        &mut self,
        session_id: &str,
        user_id: &str,
        expiry_secs: usize,
    ) -> Result<(), RedisError> {
        self.db.set_value(session_id, user_id)?;
        self.db.expire_key(session_id, expiry_secs)?;
        Ok(())
    }

    // Validate a session by checking Redis.
    pub fn validate_session(&mut self, session_id: &str) -> Result<Option<String>, RedisError> {
        self.db.get_value(session_id)
    }

    // Delete a session (Logout)
    pub fn delete_session(&mut self, session_id: &str) -> Result<(), RedisError> {
        self.db.delete_key(session_id)?;
        Ok(())
    }

    pub fn get_user_info(&mut self, user_id: &str) -> Result<HashMap<String, String>, DBError> {
        let user_info = self.db.get_json_value(user_id)?;
        Ok(user_info)
    }
}
