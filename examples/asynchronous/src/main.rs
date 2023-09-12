use replit_db::{self, Asynchronous};
use tokio;

#[tokio::main]
async fn main() -> Result<(), replit_db::Error> {

    let db = replit_db::Database::new(replit_db::Config::new().unwrap());
    db.get("Hello").await?; // Get a value from key's name.
    db.set("Hello", "World").await?; // Set a value to that key
    db.delete("Hello").await?; // Delete a key
    db.list(None::<&str>).await?; // List all keys
    db.list(Some("H")).await?; // List keys with "H" prefix
    return Ok(())
}