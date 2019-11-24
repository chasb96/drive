use std::{fmt, error};
use diesel::result::Error;
use crate::storage_drivers::storage_router::StorageRouterError;

pub enum ServiceError {
    NotFound,
    DatabaseError(Error),
    StorageError(StorageRouterError),
}

impl error::Error for ServiceError {}

impl fmt::Debug for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::NotFound => write!(f, "Not Found"),
            ServiceError::DatabaseError(e) => e.fmt(f),
            ServiceError::StorageError(e) => e.fmt(f),
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::NotFound => write!(f, "Not Found"),
            ServiceError::DatabaseError(e) => e.fmt(f),
            ServiceError::StorageError(e) => e.fmt(f),
        }
    }
}

impl From<Error> for ServiceError {
    fn from(error: Error) -> Self {
        if error == Error::NotFound {
            ServiceError::NotFound
        } else {
            ServiceError::DatabaseError(error)
        }
    }
}

impl From<StorageRouterError> for ServiceError {
    fn from(error: StorageRouterError) -> Self {
        ServiceError::StorageError(error)
    }
}
