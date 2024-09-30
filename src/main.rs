pub mod db;
use db::database::DataBase;
use redis::RedisError;

fn main() -> Result<(), RedisError> {
    // Initialize the database connection (using "redis://127.0.0.1/")
    let mut db = DataBase::new("redis://127.0.0.1:6379")?;

    // Set a value in Redis
    db.set_value("username", "rustacean")?;

    // Get the value back
    if let Some(value) = db.get_value("username")? {
        println!("Value: {}", value);
    }

    // Check if the key exists
    if db.key_exists("username")? {
        println!("Key exists!");
    }

    // Expire the key in 10 seconds
    db.expire_key("username", 10)?;

    // Delete the key
    db.delete_key("username")?;

    Ok(())
}
