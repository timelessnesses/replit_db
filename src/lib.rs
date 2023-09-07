#![allow(dead_code)] // `new` Functions are not dead code you dumb fuck.

use async_trait;
use reqwest;
use std;
use urlencoding;

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
    fn set(&self, key: impl ToString, value: impl ToString) -> Result<(), Error>;
    /// Get a variable you just set. `key` MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::NoItemFoundError`] for no items were found in the database
    fn get(&self, key: impl ToString) -> Result<String, Error>;
    /// Delete a variable you just set. MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::NoItemFoundError`] for no items were found in the database
    fn delete(&self, key: impl ToString) -> Result<(), Error>;
    /// List variables. Optionally finding variable that contains defined prefix by passing [`Some`] with anything that implements [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead of [`None`]
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::DecodeError`] Decoding string error.
    fn list(&self, prefix: Option<impl ToString>) -> Result<std::vec::Vec<String>, Error>;
}

/// Asynchronous support for Database struct. Use this trait by import it then use it right away!
#[async_trait::async_trait]
pub trait Asynchronous {
    /// Set a variable. `key` and `value` MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exception is [`ErrorKind::HttpError`] for HttpError
    async fn set<T>(&self, key: T, value: T) -> Result<(), Error>
    where
        T: ToString + Send;
    /// Get a variable you just set. `key` MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::NoItemFoundError`] for no items were found in the database
    async fn get<T>(&self, key: T) -> Result<String, Error>
    where
        T: ToString + Send;
    /// Delete a variable you just set. MUST implement [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead.
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::NoItemFoundError`] for no items were found in the database
    async fn delete<T>(&self, key: T) -> Result<(), Error>
    where
        T: ToString + Send;
    /// List variables. Optionally finding variable that contains defined prefix by passing [`Some`] with anything that implements [`std::string::ToString`] trait OR you could convert them to [`std::string::String`] instead of [`None`]
    /// Possible Exceptions are [`ErrorKind::HttpError`] for HttpError, [`ErrorKind::DecodeError`] Decoding string error.
    async fn list<T>(&self, prefix: Option<T>) -> Result<std::vec::Vec<String>, Error>;
    where
	T: ToString + Send;
}

impl Config {
    /// Creating new [`Config`] struct with default configuration.
    /// With a possibility of [`std::env::VarError`] due to enviroment variable isn't exists.
    /// If that happens, You should use [`Config::new_custom_url`] for defining your own database URL instead.
    pub fn new() -> Result<Config, std::env::VarError> {
        let res = std::env::var("REPLIT_DB_URL");
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        return Ok(Self { url: res.unwrap() });
    }

    /// Creating a new [`Config`] struct with custom URL configuration
    /// This function also checks if the `url` parameter is kv.replit.com or not
    pub fn new_custom_url(url: String) -> Config {
        if !url.contains("kv.replit.com") {
            panic!("Invalid URL for custom URL.: {}", url);
        }

        return Self { url: url };
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str(format!("{:#?}: {}", self.kind, self.message).as_str());
    }
}

impl Database {
    
    /// Creating new Database instance with [`Config`] struct.
    /// You still need traits for this struct to work.
    pub fn new(config: Config) -> Self {
        return Self { config: config };
    }
}

impl Synchronous for Database {
    fn set(&self, key: impl ToString, value: impl ToString) -> Result<(), Error> {
        let client = reqwest::blocking::Client::new();
        let payload = format!(
            "{}={}",
            urlencoding::encode(key.to_string().as_str()),
            urlencoding::encode(value.to_string().as_str())
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

    fn get(&self, key: impl ToString) -> Result<String, Error> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(
                self.config.url.as_str().to_string()
                    + format!("/{}", urlencoding::encode(key.to_string().as_str())).as_str(),
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

    fn delete(&self, key: impl ToString) -> Result<(), Error> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .delete(
                self.config.url.as_str().to_string()
                    + format!("/{}", urlencoding::encode(key.to_string().as_str())).as_str(),
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
    fn list(&self, prefix: Option<impl ToString>) -> Result<Vec<String>, Error> {
        let prefix2: String;
	    
        if prefix.is_none() {
            prefix2 = "".to_string();
        } else {
            prefix2 = prefix.unwrap().to_string();
        }
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(
                self.config.url.as_str().to_string()
                    + format!("?prefix={}", urlencoding::encode(prefix2.as_str())).as_str(),
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
        T: ToString + Send,
    {
        let client = reqwest::Client::builder().build().unwrap();
        let payload = format!(
            "{}={}",
            urlencoding::encode(key.to_string().as_str()),
            urlencoding::encode(value.to_string().as_str())
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
        T: ToString + Send,
    {
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(
                self.config.url.as_str().to_string()
                    + format!("/{}", urlencoding::encode(key.to_string().as_str())).as_str(),
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
        T: ToString + Send,
    {
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .delete(
                self.config.url.as_str().to_string()
                    + format!(
                        "/{}",
                        urlencoding::encode(key.to_string().as_str())
                    ).as_str(),
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
	T: ToString + Send
    {
        let prefix2: String;
	    
        if prefix.is_none() {
            prefix2 = "".to_string();
        } else {
            prefix2 = prefix.unwrap().to_string();
        }
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(
                self.config.url.as_str().to_string()
                    + format!("?prefix={}", urlencoding::encode(prefix2.as_str())).as_str(),
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
