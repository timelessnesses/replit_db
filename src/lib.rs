use std;
use reqwest;

// Configuration struct that contains information needed for Database.
pub struct Config {
    url: String
}

#[derive(Debug, Clone)]
// Error kind. (Http Error, No Item Found Error, Decode String Error)
pub enum ErrorKind {
    HttpError, // Http error (when something goes wrong in reqwest.)
    NoItemFoundError, // No item found error (when item isn't exists)
    DecodeError // Decoding string error (when the string is undecodable)
}

#[derive(Debug, Clone)]
// Error struct for giving useful information about what goes wrong.
pub struct Error {
    // Error kind (See `ErrorKind`)
    pub kind: ErrorKind,
    // Message
    pub message: String
}

// Synchronous Replit Database Adapter for normal uses.
// Contains same APIs as `AsynchronousReplitDatabase`
pub struct SynchronousReplitDatabase {
    config: Config
}

// Asynchronous Replit Database Adapter for asynchronous operation. (supports tokio(?), and async-std(?))
// Contains same APIs as `SynchronousReplitDatabase`
pub struct AsynchronousReplitDatabase {
    config: Config
}

impl Config {
    // Creating new `Config` struct with default configuration.
    // With a possibility of `std::env::VarError` due to enviroment variable isn't exists.
    // If that happens, You should use `Config::new_custom_url` for defining your own database URL instead.
    pub fn new() -> Result<Config, std::env::VarError> {
        let res = std::env::var("REPLIT_DATABASE_ENV");
        if res.is_err() {
            return Err(res.err().unwrap())
        }
        return Ok(Self {
            url: res.unwrap()
        })
    }

   // Creating a new `Config` struct with custom URL configuration
   // This function also checks if the `url` parameter is kv.replit.com or not
   // ## Parameters
   // - `url`: `String` = Replit database URL.
    pub fn new_custom_url(url: String) -> Config {
	if !url.contains("kv.replit.com") {
            panic!("Invalid URL for custom URL.: {}", url);
        }
        return Self {
            url: url
        }
    }

}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str(format!("{:#?}: {}", self.kind, self.message).as_str())
    }
}

impl SynchronousReplitDatabase {
    // Creating new `SynchronousReplitDatabase` with configuration you provided.
    pub fn new(config: Config) -> Self {
        return Self {config: config}
    }

    // Set a new key (or override the key) with a value.
    pub fn set(&self, key: String, value: String) -> Result<(), Error> {
        let client = reqwest::blocking::Client::new();
        let payload = format!("{}={}", key.to_string(), value.to_string());
        let response = client.post(self.config.url.as_str().to_string()).body(payload).send();
        if response.is_err() {
            return Err(Error{
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string()
            })
        }
        return Ok(())
    }

    pub fn get(&self, key: String) -> Result<String, Error> {
        let client = reqwest::blocking::Client::new();
        let response = client.get(self.config.url.as_str().to_string() + format!("/{}", key.to_string()).as_str()).send();
        if response.is_err() {
            return Err(
                Error {
                    kind: ErrorKind::HttpError,
                    message: response.unwrap_err().to_string()
                }
            )
        }
        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(
                Error { kind: ErrorKind::NoItemFoundError, message: "No items were found on the database.".to_string() }
            )
        }
        let content = response.text().unwrap();
        return Ok(content)
    }

    pub fn delete(&self, key: String) -> Result<(), Error> {
        let client = reqwest::blocking::Client::new();
        let response = client.delete(self.config.url.as_str().to_string() + format!("/{}", key.to_string()).as_str()).send();

        if response.is_err() {
            return Err(
                Error {
                    kind: ErrorKind::HttpError,
                    message: response.unwrap_err().to_string()
                }
            )
        }
        if !response.unwrap().status().is_success() {
            return Err(Error {
                kind: ErrorKind::NoItemFoundError,
                message: "No item with that name were found.".to_string()
            })
        }
        return Ok(())
    }
    pub fn list(&self, prefix: Option<String>) -> Result<Vec<String>, Error> {

        let prefix2: String;

        if prefix.is_none() {
            prefix2 = "".to_string();
        } else {
            prefix2 = prefix.unwrap();
        }
        let client = reqwest::blocking::Client::new();
        let response = client.get(self.config.url.as_str().to_string() + format!("?prefix={}", prefix2).as_str()).send();
        if response.is_err() {
            return Err(
                Error {
                    kind: ErrorKind::HttpError,
                    message: response.unwrap_err().to_string()
                }
            )
        }
        let content = response.unwrap().text();
        if content.is_err() {
            return Err(
                Error { kind: ErrorKind::DecodeError, message: content.unwrap_err().to_string() }
            )
        }
        let mut variables: std::vec::Vec<String> = std::vec::Vec::new();
        for v in content.unwrap().lines() {
            variables.push(v.to_string());
        }
        return Ok(variables)
    }
}

impl AsynchronousReplitDatabase {
    pub fn new(config: Config) -> Self {
        return Self {config: config}
    }
    pub async fn set(&self, key: String, value: String) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let payload = format!("{}={}", key.to_string(), value.to_string());
        let response = client.post(self.config.url.as_str().to_string()).body(payload).send().await;
        if response.is_err() {
            return Err(Error{
                kind: ErrorKind::HttpError,
                message: response.unwrap_err().to_string()
            })
        }
        return Ok(())
    }

    pub async fn get(&self, key: String) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let response = client.get(self.config.url.as_str().to_string() + format!("/{}", key.to_string()).as_str()).send().await;
        if response.is_err() {
            return Err(
                Error {
                    kind: ErrorKind::HttpError,
                    message: response.unwrap_err().to_string()
                }
            )
        }
        let response = response.unwrap();
        if !response.status().is_success() {
            return Err(
                Error { kind: ErrorKind::NoItemFoundError, message: "No items were found on the database.".to_string() }
            )
        }
        let content = response.text().await.unwrap();
        return Ok(content)
    }

    pub async fn delete(&self, key: String) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let response = client.delete(self.config.url.as_str().to_string() + format!("/{}", key.to_string()).as_str()).send().await;

        if response.is_err() {
            return Err(
                Error {
                    kind: ErrorKind::HttpError,
                    message: response.unwrap_err().to_string()
                }
            )
        }
        if !response.unwrap().status().is_success() {
            return Err(Error {
                kind: ErrorKind::NoItemFoundError,
                message: "No item with that name were found.".to_string()
            })
        }
        return Ok(())
    }
    pub async fn list(&self, prefix: Option<String>) -> Result<Vec<String>, Error> {

        let prefix2: String;

        if prefix.is_none() {
            prefix2 = "".to_string();
        } else {
            prefix2 = prefix.unwrap();
        }
        let client = reqwest::Client::new();
        let response = client.get(self.config.url.as_str().to_string() + format!("?prefix={}", prefix2).as_str()).send().await;
        if response.is_err() {
            return Err(
                Error {
                    kind: ErrorKind::HttpError,
                    message: response.unwrap_err().to_string()
                }
            )
        }
        let content = response.unwrap().text().await;
        if content.is_err() {
            return Err(
                Error { kind: ErrorKind::DecodeError, message: content.unwrap_err().to_string() }
            )
        }
        let mut variables: std::vec::Vec<String> = std::vec::Vec::new();
        for v in content.unwrap().lines() {
            variables.push(v.to_string());
        }
        return Ok(variables)
    }
}
