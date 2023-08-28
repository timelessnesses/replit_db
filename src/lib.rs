use std;
use reqwest;

pub struct Config {
    url: String
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    HttpError,
    NoItemFoundError,
    DecodeError
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

pub struct SynchronousReplitDatabase {
    config: Config
}

pub struct AsynchronousReplitDatabase {
    config: Config
}

impl Config {
    pub fn new() -> Result<Config, std::env::VarError> {
        let res = std::env::var("REPLIT_DATABASE_ENV");
        if res.is_err() {
            return Err(res.err().unwrap())
        }
        return Ok(Self {
            url: res.unwrap()
        })
    }

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
    pub fn new(config: Config) -> Self {
        return Self {config: config}
    }
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
