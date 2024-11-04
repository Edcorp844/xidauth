use redis::{Commands, Connection, RedisError, RedisResult};
use std::collections::HashMap;

use super::db_error::DBError;

pub struct DataBase {
    connection: Connection,
}

impl DataBase {
    // Initialize a new instance of the database with a Redis connection.
    pub fn new(redis_url: &str) -> Result<Self, RedisError> {
        let client = redis::Client::open(redis_url)?;
        let connection = client.get_connection()?;
        Ok(DataBase { connection })
    }

    // Set a key-value pair in Redis.
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<(), RedisError> {
        let _: () = self.connection.set(key, value)?;
        Ok(())
    }

    // Get a value from Redis.
    pub fn get_value(&mut self, key: &str) -> Result<Option<String>, RedisError> {
        let value: Option<String> = self.connection.get(key)?;
        Ok(value)
    }

    // Delete a key from Redis.
    pub fn delete_key(&mut self, key: &str) -> Result<(), RedisError> {
        let _: () = self.connection.del(key)?;
        Ok(())
    }

    // Check if a key exists in Redis.
    pub fn key_exists(&mut self, key: &str) -> Result<bool, RedisError> {
        let exists: bool = self.connection.exists(key)?;
        Ok(exists)
    }

    // Expire a key after a certain number of seconds.
    pub fn expire_key(&mut self, key: &str, seconds: usize) -> Result<(), RedisError> {
        let _: () = self.connection.expire(key, seconds.try_into().unwrap())?;
        Ok(())
    }

    // Store JSON data in Redis using RedisJSON
    pub fn set_json_value(&mut self, key: &str, json_data: &str) -> RedisResult<()> {
        let _: () = redis::cmd("JSON.SET")
            .arg(key)
            .arg(".") // Root path for the JSON
            .arg(json_data)
            .query(&mut self.connection)?;
        Ok(())
    }

    pub fn get_json_value(&mut self, key: &str) -> Result<HashMap<String, String>, DBError> {
        let json_data: String = redis::cmd("JSON.GET")
            .arg(key)
            .arg(".") // Root path for the JSON
            .query(&mut self.connection)
            .map_err(DBError::from)?; // Convert RedisError to MyError

        // Deserialize JSON data
        let user_info: HashMap<String, String> =
            serde_json::from_str(&json_data).map_err(DBError::from)?; // Convert SerdeError to MyError

        Ok(user_info)
    }

    // Check which Redis modules are loaded
    pub fn check_redis_modules(&mut self) -> Result<String, RedisError> {
        let result: String = redis::cmd("MODULE")
            .arg("LIST")
            .query(&mut self.connection)?;

        Ok(result)
    }
}
