use crate::auth::basic::Error as BasicError;
use crate::auth::bearer::Error as TokenError;
use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    #[allow(dead_code)] // For schemes not allowed, not allowed without mutual exclusion
    SchemeInvalid,
    CredentialsInvalid,
    KeyMissing,
    SignatureExpired,
    SignatureInvalid,
    JWTInvalid,
    IssuerInvalid,
    ExpirationInvalid,
    AudienceInvalid,
    FormatInvalid(String),
    IoError(String),
    OpenSslError(String),
    ProtocolError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SchemeInvalid => write!(f, "Authentication Scheme Invalid"),
            Error::CredentialsInvalid => write!(f, "Credentials Invalid"),
            Error::KeyMissing => write!(f, "App Key Missing"),
            Error::SignatureExpired => write!(f, "Signature expired."),
            Error::SignatureInvalid => write!(f, "Signature invalid."),
            Error::JWTInvalid => write!(f, "JWT invalid."),
            Error::IssuerInvalid => write!(f, "Issuer invalid."),
            Error::ExpirationInvalid => write!(f, "Expiration invalid."),
            Error::AudienceInvalid => write!(f, "Audience invalid."),
            Error::FormatInvalid(msg) => write!(f, "Format invalid: {}.", msg),
            Error::IoError(msg) => write!(f, "IO error: {}.", msg),
            Error::OpenSslError(msg) => write!(f, "Open SSL error: {}.", msg),
            Error::ProtocolError(msg) => write!(f, "Protocol error: {}.", msg),
        }
    }
}

impl error::Error for Error {}

impl From<TokenError> for Error {
    fn from(error: TokenError) -> Self {
        match error {
            TokenError::KeyMissing => Error::KeyMissing,
            TokenError::SignatureExpired => Error::SignatureExpired,
            TokenError::SignatureInvalid => Error::SignatureInvalid,
            TokenError::JWTInvalid => Error::JWTInvalid,
            TokenError::IssuerInvalid => Error::IssuerInvalid,
            TokenError::ExpirationInvalid => Error::ExpirationInvalid,
            TokenError::AudienceInvalid => Error::AudienceInvalid,
            TokenError::FormatInvalid(s) => Error::FormatInvalid(s),
            TokenError::IoError(s) => Error::IoError(s),
            TokenError::OpenSslError(s) => Error::OpenSslError(s),
            TokenError::ProtocolError(s) => Error::ProtocolError(s),
        }
    }
}

impl From<BasicError> for Error {
    fn from(_: BasicError) -> Self {
        Error::CredentialsInvalid
    }
}
