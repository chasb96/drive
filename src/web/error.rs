use serde_derive::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct Error(String);

impl Error {
    pub fn new(message: String) -> Self {
        Self(message)
    }

    pub fn message(&self) -> &String {
        &self.0
    }
}

impl From<&str> for Error {
    fn from(from: &str) -> Self {
        Error(from.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
