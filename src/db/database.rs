use redis::{Commands, Connection, RedisError};

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

    // Set a value in Redis.
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
}
