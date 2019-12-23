use crate::auth::bearer::Token;
use rocket::http::{Cookie, Cookies, HeaderMap};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

pub enum TokenError {
    NotPresent,
    Invalid,
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            TokenError::NotPresent => write!(f, "Token Not Present"),
            TokenError::Invalid => write!(f, "Invalid Token"),
        }
    }
}

impl fmt::Debug for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            TokenError::NotPresent => write!(f, "Token Not Present"),
            TokenError::Invalid => write!(f, "Invalid Token"),
        }
    }
}

impl TryFrom<Cookies<'_>> for Token {
    type Error = TokenError;

    fn try_from(mut try_from: Cookies) -> Result<Self, Self::Error> {
        match try_from.get_private("token") {
            Some(cookie) => Ok(Token::from(cookie)),
            None => Err(TokenError::NotPresent),
        }
    }
}

impl From<Cookie<'_>> for Token {
    fn from(from: Cookie) -> Self {
        Token::new(from.value().to_string())
    }
}

impl TryFrom<&HeaderMap<'_>> for Token {
    type Error = TokenError;

    fn try_from(try_from: &HeaderMap) -> Result<Self, Self::Error> {
        match try_from.get_one("Authorization") {
            Some(header) => Token::try_from(header),
            None => Err(TokenError::NotPresent),
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = TokenError;

    fn try_from(try_from: &str) -> Result<Self, Self::Error> {
        let mut parts = try_from.split_whitespace();

        let token = match parts.next() {
            Some("Bearer") => match parts.next() {
                Some(token) => token,
                None => return Err(TokenError::Invalid),
            },
            _ => return Err(TokenError::Invalid),
        };

        Ok(Token::new(token.to_string()))
    }
}
