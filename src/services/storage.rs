use chrono::Utc;
use env::Env;
use rand::{self, distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::Path;
use storage_drivers::storage_router::StorageRouterError;
use storage_drivers::StorageDriver;
use storage_drivers::StorageRouter;

pub struct StorageService;

impl StorageService {
    pub fn store(directory: String, input: File) -> Result<String, StorageServiceError> {
        let timestamp = Utc::now().to_string();
        let random_bytes: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .collect();

        let file_name = format!(
            "{timestamp}_{random_bytes}",
            timestamp = timestamp,
            random_bytes = random_bytes
        );

        let path = format!(
            "{}/{directory}/{file_name}",
            Env::storage_dir(),
            directory = directory,
            file_name = &file_name
        );

        match StorageRouter::store(Path::new(&path), input) {
            Ok(_) => Ok(file_name),
            Err(e) => {
                log!("error", "Failed to store file: {}", e);
                Err(StorageServiceError::from(e))
            }
        }
    }

    pub fn read(directory: String, file_name: String) -> Result<File, StorageServiceError> {
        let path = format!(
            "{}/{directory}/{file_name}",
            Env::storage_dir(),
            directory = directory,
            file_name = &file_name
        );

        match StorageRouter::read(Path::new(&path)) {
            Ok(contents) => Ok(contents),
            Err(e) => {
                log!("error", "Failed to read file: {}", e);
                Err(StorageServiceError::from(e))
            }
        }
    }

    pub fn delete(directory: String, file_name: String) -> Result<(), StorageServiceError> {
        let path = format!(
            "{}/{directory}/{file_name}",
            Env::storage_dir(),
            directory = directory,
            file_name = &file_name
        );

        match StorageRouter::delete(Path::new(&path)) {
            Ok(_) => Ok(()),
            Err(e) => {
                log!("error", "Failed to delete file: {}", e);
                Err(StorageServiceError::from(e))
            }
        }
    }
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
