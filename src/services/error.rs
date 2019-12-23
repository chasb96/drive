use crate::storage_drivers::error::StorageError;
use std::{fmt, error};
use crate::entities::error::DataStoreError;

pub enum ServiceError {
    NotFound,
    DataStoreError(DataStoreError),
    StorageError(StorageError),
}

impl error::Error for ServiceError {}

impl fmt::Debug for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::NotFound => write!(f, "Not Found"),
            ServiceError::DataStoreError(e) => e.fmt(f),
            ServiceError::StorageError(e) => e.fmt(f),
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::NotFound => write!(f, "Not Found"),
            ServiceError::DataStoreError(e) => e.fmt(f),
            ServiceError::StorageError(e) => e.fmt(f),
        }
    }
}

impl From<DataStoreError> for ServiceError {
    fn from(error: DataStoreError) -> Self {
        if error == DataStoreError::NotFound {
            ServiceError::NotFound
        } else {
            ServiceError::DataStoreError(error)
        }
    }
}

impl From<StorageError> for ServiceError {
    fn from(error: StorageError) -> Self {
        ServiceError::StorageError(error)
    }
}
