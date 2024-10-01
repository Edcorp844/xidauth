use redis::RedisError;
use serde_json::Error as SerdeError;
use std::fmt;

#[derive(Debug)]
pub enum DBError {
    Redis(RedisError),
    Json(SerdeError),
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBError::Redis(err) => write!(f, "Redis error: {}", err),
            DBError::Json(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl From<RedisError> for DBError {
    fn from(err: RedisError) -> DBError {
        DBError::Redis(err)
    }
}

impl From<SerdeError> for DBError {
    fn from(err: SerdeError) -> DBError {
        DBError::Json(err)
    }
}
