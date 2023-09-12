use replit_db::{self, Synchronous};

fn main() -> Result<(), replit_db::Error> {

    let db = replit_db::Database::new(replit_db::Config::new().unwrap());
    db.get("Hello")?; // Get a value from key's name.
    db.set("Hello", "World")?; // Set a value to that key
    db.delete("Hello")?; // Delete a key
    db.list(replit_db::NONE)?; // List all keys
    db.list(Some("H"))?; // List keys with "H" prefix
    return Ok(())
}