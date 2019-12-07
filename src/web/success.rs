use serde_derive::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct Success(String);

impl Success {
    pub fn new(message: String) -> Self {
        Self(message)
    }

    pub fn message(&self) -> &String {
        &self.0
    }
}

impl From<&str> for Success {
    fn from(from: &str) -> Self {
        Success(from.to_string())
    }
}

impl fmt::Display for Success {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
