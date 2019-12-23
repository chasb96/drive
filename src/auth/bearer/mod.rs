mod error;
mod user;

pub use self::error::Error;

use super::authenticate::Authenticate;
use crate::env::Env;
use frank_jwt::{decode, encode, Algorithm, ValidationOptions};
use serde::ser::Serialize;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use std::fmt;

pub trait Bearer: Sized + Serialize {
    fn header(&self) -> Value {
        Value::Object(Map::new())
    }

    fn payload(&self) -> Value {
        json!(&self)
    }

    fn encode(&self) -> Result<String, Error> {
        let secret = Env::app_key();

        let header = self.header();
        let payload = self.payload();

        let token_string: String = encode(header, &secret, &payload, Algorithm::HS384)?;

        Ok(token_string)
    }

    fn decode(token: &str) -> Result<Value, Error> {
        let secret = Env::app_key();

        match decode(token, &secret, Algorithm::HS384, &ValidationOptions::default()) {
            Ok((_, payload)) => Ok(payload),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn verify(token: Value) -> Result<Self, Error>;
}

pub struct Token {
    token: String,
}

impl Token {
    pub fn new(token: String) -> Self {
        Token { token }
    }
}

impl<T: Bearer> Authenticate<T> for Token {
    type Error = Error;

    fn verify(&self) -> Result<T, Self::Error> {
        let decoded = T::decode(&self.token)?;

        T::verify(decoded)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}
