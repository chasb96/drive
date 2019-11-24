use super::Bearer;
use super::Error;
use super::Token;
use chrono::*;
use entities::models::User;
use entities::diesel::pool::DbPool;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use env::Env;
use schema::*;
use serde_json::json;
use serde_json::Value;
use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::str::FromStr;

impl Bearer for User {
    fn payload(&self) -> Value {
        json!({
            "user_id": &self.id(),
            "email": &self.email(),
            "name": &self.name(),
            // Returns None on overflow, however that's nearly 30 years in the future,
            //  nearly impossible for this to hit unless configured to insanity
            "expires": Utc::now()
                .checked_add_signed(Duration::hours(Env::session_expiry_hours()))
                .unwrap()
        })
    }

    fn verify(payload: Value) -> Result<Self, Error> {
        let expires: DateTime<Utc> = match payload.get("expires") {
            Some(expires) => match expires.as_str() {
                Some(expires) => match DateTime::<Utc>::from_str(expires) {
                    Ok(expires) => expires,
                    Err(_) => return Err(Error::JWTInvalid),
                },
                None => return Err(Error::JWTInvalid),
            },
            None => return Err(Error::JWTInvalid),
        };

        if Utc::now() > expires {
            return Err(Error::JWTInvalid);
        }

        let conn = DbPool::connection();

        let user_id: i32 = payload
            .get("user_id")
            .unwrap_or(&Value::Null)
            .as_u64()
            .unwrap_or(0) as i32;

        match Self::all().filter(users::id.eq(user_id)).first(&conn) {
            Ok(user) => Ok(user),
            Err(_) => Err(Error::JWTInvalid),
        }
    }
}

pub struct TryFromUserError(Error);

impl TryFromUserError {
    fn new(msg: Error) -> Self {
        Self(msg)
    }
}

impl error::Error for TryFromUserError {}

impl fmt::Display for TryFromUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Failed to create token: {}", self.0)
    }
}

impl fmt::Debug for TryFromUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Failed to create token: {}", self.0)
    }
}

impl TryFrom<User> for Token {
    type Error = TryFromUserError;

    fn try_from(try_from: User) -> Result<Self, Self::Error> {
        match try_from.encode() {
            Ok(token) => Ok(Token::new(token)),
            Err(e) => Err(TryFromUserError::new(e)),
        }
    }
}
