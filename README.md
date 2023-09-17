# replit_db

An unofficial database adapater for Replit Database for Rust!

## Installation

```sh
cargo add replit_db
```

## Supports

- Synchronous
- Asynchronous
- Type Safety (:skull emoji:)

## Example

```rust
use replit_db::{self, Synchronous};

fn main() {
    let config = replit_db::Config::new().unwrap();
    let db = replit_db::Database::new(config);
    let res = db.set("testings", 30);
    match res {
        Ok(()) => println!("Successful!"),
        Err(e) => println!("{}",e.to_string())
    }
   println!(db.get("testings").unwrap());
}
```

All [documentations](https://replit-db.doc.timelessnesses.me/) will be in the comment and intellisense.  (I hosted my own documentation since docs.rs is slow)
Also for asynchronous version please use `replit_db::Asynchronous` trait.

