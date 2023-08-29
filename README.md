# replit_db

An unofficial database adapater for Replit Database for Rust!

## Supports

- Synchronous
- Asynchronous
- Type Safety (:skull emoji:)

## Example

```rust
use replit_db::{self, Synchronous};

fn main() {
    let config = replit_db::Config::new();
    let db = replit_db::Database::new(config);
    let res = db.set("testings".to_string(), "testers".to_string());
    match res {
        Ok(()) => println!("Successful!"),
        Err(e) => println!("{}",e.to_string())
    }
   println!(db.get("testings")).unwrap();
}
```

All documentations will be in the comment and intellisense.  
Also for asynchronous version please use `replit_db::Asynchronous` trait.


