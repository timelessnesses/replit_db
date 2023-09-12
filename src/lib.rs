//! # replit_db
//!
//! An unofficial database adapater for Replit Database for Rust!
//!
//! ## Usage
//!
//! You need to import [`Database`], [`Config`], and a trait ([`Synchronous`], [`Asynchronous`]).
//! Then initialize [`Database::new()`] with [`Config::new()`] then database will give you function in either synchronously or asynchronously based on trait you imported in to the scope.
//!
//! ## Possible Exceptions
//!
//! [`Error`] struct contain useful informations and both [`std::fmt::Display`] and [`std::error::Error`] (support "?").
//! Also there's different error kinds that can happens, here's the list of them.
//! - [`ErrorKind::HttpError`]
//!     Raised when there's something wrong when doing HTTP request.
//! - [`ErrorKind::NoItemFoundError`]
//!     Raised when item is not found
//! - [`ErrorKind::DecodeError`]
//!     Raised when the key name is undecodable to UTF-8 string.
//!
//! ## Examples
//!
//! ### Example (Synchronous)
//!
//! ```rust
//! use replit_db::{self, Synchronous};
//!
//! fn main() -> Result<(), Error> {
//!
//!     let db = replit_db::Database::new(replit_db::Config::new());
//!     db.get("Hello")?; // Get a value from key's name.
//!     db.set("Hello", "World")?; // Set a value to that key
//!     db.delete("Hello")?; // Delete a key
//!     db.list(replit_db::NONE)?; // List all keys
//!     db.get(Some("H"))?; // List keys with "H" prefix
//! }
//! ```
//!
//! ### Example (Asynchronous)
//!
//! ```rust
//! use replit_db::{self, Asynchronous};
//! use tokio;
//!
//! #[tokio::main]
//! fn main() -> Result<(), Error> {
//!
//!     let db = replit_db::Database::new(replit_db::Config::new());
//!     db.get("Hello").await?; // Get a value from key's name.
//!     db.set("Hello", "World").await?; // Set a value to that key
//!     db.delete("Hello").await?; // Delete a key
//!     db.list(replit_db::NONE).await?; // List all keys
//!     db.get(Some("H")).await?; // List keys with "H" prefix
//! }
//! ```

use async_trait;
use reqwest;
use std;
use urlencoding;

/// This constant is for storing replit's db's domain name. This would likely change by whatever the reason is.
const MAIN_DOMAIN: &str = "kv.replit.com";

/// This type is a shorthand for [`Option<&str>::None`] or [`None::<&str>`].
pub const NONE: Option<&str> = None;

/// Configuration struct that contains information needed for Database.
pub struct Config {
    url: String,
}

#[derive(Debug, Clone)]
/// Error kind. (Http Error, No Item Found Error, Decode String Error)
pub enum ErrorKind {
    ///  Any [`reqwest`]'s errors will be here.
    HttpError,
    /// That item specified isn't exists in the database.
    NoItemFoundError,
    /// Couldn't decode bytes to string UTF-8.
    DecodeError,
}

#[derive(Debug, Clone)]
/// Error struct for giving useful information about what goes wrong.
pub struct Error {
    /// Error kind (See [`ErrorKind`])
    pub kind: ErrorKind,
    /// Message
    pub message: String,
}

/// Database main struct.
/// Please use this database with traits. (Availables are [`Synchronous`] and [`Asynchronous`])
pub struct Database {
    config: Config,
}

/// Synchronous support for Database struct. Use this trait by import it then use it right away!
pub trait Synchronous {
    /// Set a variable. `key` and `value` MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exception is [`ErrorKind::HttpError`] for HttpError
    fn set(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<(), Error>;
    /// Get a variable you just set. `key` MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::NoItemFoundError`] for no items were found in the database
    fn get(&self, key: impl AsRef<str>) -> Result<String, Error>;
    /// Delete a variable you just set. MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::NoItemFoundError`] for no items were found in the database
    fn delete(&self, key: impl AsRef<str>) -> Result<(), Error>;
    /// List variables. Optionally finding variable that contains defined prefix by passing [`Some`] with anything that implements [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead of [`None`].
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::DecodeError`] Decoding string error.
    fn list(&self, prefix: Option<impl AsRef<str>>) -> Result<std::vec::Vec<String>, Error>;
}

/// Asynchronous support for Database struct. Use this trait by import it then use it right away!
#[async_trait::async_trait]
pub trait Asynchronous {
    /// Set a variable. `key` and `value` MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exception is [`ErrorKind::HttpError`] for HttpError
    async fn set<T>(&self, key: T, value: T) -> Result<(), Error>
    where
        T: AsRef<str> + Send;
    /// Get a variable you just set. `key` MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::NoItemFoundError`] for no items were found in the database
    async fn get<T>(&self, key: T) -> Result<String, Error>
    where
        T: AsRef<str> + Send;
    /// Delete a variable you just set. MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::NoItemFoundError`] for no items were found in the database
    async fn delete<T>(&self, key: T) -> Result<(), Error>
    where
        T: AsRef<str> + Send;
    /// List variables. Optionally finding variable that contains defined prefix by passing [`Some`] with anything that implements [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead of [`None`].
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::DecodeError`] Decoding string error.
    async fn list<T>(&self, prefix: Option<T>) -> Result<std::vec::Vec<String>, Error>
    where
        T: AsRef<str> + Send;
}

impl Config {
    /// Creating new [`Config`] struct with default configuration. (This will get Replit's Database URL through enviroment variable `REPLIT_DB_URL`)
    /// With a possibility of [`std::env::VarError`] due to enviroment variable isn't exists.
    /// If that happens, You should use [`Config`]'s `new_custom_url` for defining your own database URL instead.
    pub fn new() -> Result<Config, std::env::VarError> {
        let res = std::env::var("REPLIT_DB_URL");
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        return Ok(Self { url: res.unwrap() });
    }

    /// Creating a new [`Config`] struct with custom URL configuration.
    /// This function also checks if the `url` parameter is kv.replit.com or not. If kv.replit.com is not in the `url` argument, then it will panic.
    pub fn new_custom_url(url: &str) -> Config {
        if !url.contains(MAIN_DOMAIN) {
            panic!("Invalid URL for custom URL.: {}", url);
        }

        return Self {
            url: url.to_owned(),
        };
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str(format!("{:#?}: {}", self.kind, self.message).as_str());
    }
}

impl std::error::Error for Error {} // Thanks nox!

impl Database {
    /// Creating new Database instance with [`Config`] struct.
    /// You still need traits for this struct to work.
    pub fn new(config: Config) -> Self {
        return Self { config: config };
    }
}

impl Synchronous for Database {
    fn set(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<(), Error> {
        let client = reqwest::blocking::Client::new();
        let payload = format!(
            "{}={}",
            urlencoding::encode(key.as_ref()),
            urlencoding::encode(value.as_ref())
        );
        let response = client
            .post(self.config.url.as_str().to_string())
            .body(payload)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send();
        if response.is_err() {
            return Err(Error {
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string(),
            });
        }
        return Ok(());
    }

    fn get(&self, key: impl AsRef<str>) -> Result<String, Error> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(
                self.config.url.as_str().to_string()
                    + format!("/{}", urlencoding::encode(key.as_ref())).as_str(),
            )
            .send();
        // println!("{:#?}", response); debugging
        if response.is_err() {
            return Err(Error {
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string(),
            });
        }
        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(Error {
                kind: ErrorKind::NoItemFoundError,
                message: "No items were found on the database.".to_string(),
            });
        }
        let content = response.text().unwrap();
        return Ok(content);
    }

    fn delete(&self, key: impl AsRef<str>) -> Result<(), Error> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .delete(
                self.config.url.as_str().to_string()
                    + format!("/{}", urlencoding::encode(key.as_ref())).as_str(),
            )
            .send();

        if response.is_err() {
            return Err(Error {
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string(),
            });
        }
        if !response.unwrap().status().is_success() {
            return Err(Error {
                kind: ErrorKind::NoItemFoundError,
                message: "No item with that name were found.".to_string(),
            });
        }
        return Ok(());
    }
    fn list(&self, prefix: Option<impl AsRef<str>>) -> Result<Vec<String>, Error> {
        let prefix2 = match &prefix {
            Some(p) => p.as_ref(),
            None => "",
        };
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(
                self.config.url.as_str().to_string()
                    + format!("?prefix={}", urlencoding::encode(prefix2)).as_str(),
            )
            .send();
        if response.is_err() {
            return Err(Error {
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string(),
            });
        }
        let content = response.unwrap().text();
        if content.is_err() {
            return Err(Error {
                kind: ErrorKind::DecodeError,
                message: content.unwrap_err().to_string(),
            });
        }
        let mut variables: std::vec::Vec<String> = std::vec::Vec::new();
        for v in content.unwrap().lines() {
            variables.push(v.to_string());
        }
        return Ok(variables);
    }
}

#[async_trait::async_trait]
impl Asynchronous for Database {
    async fn set<T>(&self, key: T, value: T) -> Result<(), Error>
    where
        T: AsRef<str> + Send,
    {
        let client = reqwest::Client::builder().build().unwrap();
        let payload = format!(
            "{}={}",
            urlencoding::encode(key.as_ref()),
            urlencoding::encode(value.as_ref())
        );
        let response = client
            .post(self.config.url.as_str().to_string())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(payload)
            .send()
            .await;
        if response.is_err() {
            return Err(Error {
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string(),
            });
        }
        return Ok(());
    }

    async fn get<T>(&self, key: T) -> Result<String, Error>
    where
        T: AsRef<str> + Send,
    {
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(
                self.config.url.as_str().to_string()
                    + format!("/{}", urlencoding::encode(key.as_ref())).as_str(),
            )
            .send()
            .await;
        if response.is_err() {
            return Err(Error {
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string(),
            });
        }
        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(Error {
                kind: ErrorKind::NoItemFoundError,
                message: "No items were found on the database.".to_string(),
            });
        }
        let content = response.text().await.unwrap();
        return Ok(content);
    }

    async fn delete<T>(&self, key: T) -> Result<(), Error>
    where
        T: AsRef<str> + Send,
    {
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .delete(
                self.config.url.as_str().to_string()
                    + format!("/{}", urlencoding::encode(key.as_ref())).as_str(),
            )
            .send()
            .await;

        if response.is_err() {
            return Err(Error {
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string(),
            });
        }
        if !response.unwrap().status().is_success() {
            return Err(Error {
                kind: ErrorKind::NoItemFoundError,
                message: "No item with that name were found.".to_string(),
            });
        }
        return Ok(());
    }
    async fn list<T>(&self, prefix: Option<T>) -> Result<Vec<String>, Error>
    where
        T: AsRef<str> + Send,
    {
        let prefix2 = match &prefix {
            Some(p) => p.as_ref(),
            None => "",
        };
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(
                self.config.url.as_str().to_string()
                    + format!("?prefix={}", urlencoding::encode(prefix2)).as_str(),
            )
            .send()
            .await;
        if response.is_err() {
            return Err(Error {
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string(),
            });
        }
        let content = response.unwrap().text().await;
        if content.is_err() {
            return Err(Error {
                kind: ErrorKind::DecodeError,
                message: content.unwrap_err().to_string(),
            });
        }
        let mut variables: std::vec::Vec<String> = std::vec::Vec::new();
        for v in content.unwrap().lines() {
            variables.push(v.to_string());
        }
        return Ok(variables);
    }
}
