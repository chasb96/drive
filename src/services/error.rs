use std::{fmt, error};
use diesel::result::Error;

#[derive(Clone)]
pub enum ServiceError {
    InvalidConnectionString,
    DatabaseError,
    NotFound,
    QueryBuilderError,
    SerializationError,
    AlreadyInTransaction,
}

impl error::Error for ServiceError {}

impl fmt::Debug for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::InvalidConnectionString => write!(f, "Invalid Connection String"),
            ServiceError::DatabaseError => write!(f, "DatabaseError"),
            ServiceError::NotFound => write!(f, "Not Found"),
            ServiceError::QueryBuilderError => write!(f, "Query Builder Error"),
            ServiceError::SerializationError => write!(f, "Serialization Error"),
            ServiceError::AlreadyInTransaction => write!(f, "Already In Transaction"),
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::InvalidConnectionString => write!(f, "Invalid Connection String"),
            ServiceError::DatabaseError => write!(f, "DatabaseError"),
            ServiceError::NotFound => write!(f, "Not Found"),
            ServiceError::QueryBuilderError => write!(f, "Query Builder Error"),
            ServiceError::SerializationError => write!(f, "Serialization Error"),
            ServiceError::AlreadyInTransaction => write!(f, "Already In Transaction"),
        }
    }
}

impl From<Error> for ServiceError {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidCString(_) => ServiceError::InvalidConnectionString,
            Error::DatabaseError(_, _) => ServiceError::DatabaseError,
            Error::NotFound => ServiceError::NotFound,
            Error::QueryBuilderError(_) => ServiceError::QueryBuilderError,
            Error::SerializationError(_) => ServiceError::SerializationError,
            Error::AlreadyInTransaction => ServiceError::AlreadyInTransaction,
            _ => unreachable!()
        }
    }
}
