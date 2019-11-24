use std::{fmt, error};
use crate::entities::error::DataStoreError;
use crate::storage_drivers::storage_router::StorageRouterError;

pub enum ServiceError {
    NotFound,
    DataStoreError(DataStoreError),
    StorageError(StorageRouterError),
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

impl From<StorageRouterError> for ServiceError {
    fn from(error: StorageRouterError) -> Self {
        ServiceError::StorageError(error)
    }
}
