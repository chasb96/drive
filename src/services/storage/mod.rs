pub mod implementation;

use std::fs::File;
use std::error::Error;
use std::fmt;
use storage_drivers::storage_router::StorageRouterError;

pub trait StorageService {
    fn store(&self, directory: String, input: File) -> Result<String, StorageServiceError>;

    fn read(&self, directory: String, file_name: String) -> Result<File, StorageServiceError>;

    fn delete(&self, directory: String, file_name: String) -> Result<(), StorageServiceError>;
}

pub struct StorageServiceError(StorageRouterError);

impl Error for StorageServiceError {
    fn description(&self) -> &str {
        self.0.description()
    }
}

impl fmt::Display for StorageServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl fmt::Debug for StorageServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl From<StorageRouterError> for StorageServiceError {
    fn from(from: StorageRouterError) -> Self {
        Self(from)
    }
}
