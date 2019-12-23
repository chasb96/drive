use crate::services::error::ServiceError;
use crate::services::storage::StorageService;
use chrono::Utc;
use crate::env::Env;
use rand::{self, distributions::Alphanumeric, Rng};
use std::fs::File;
use std::path::Path;
use crate::storage_drivers::StorageDriver;

pub struct Service<T: StorageDriver> {
    storage_driver: T
}

impl<T: StorageDriver> Service<T> {
    pub fn new(storage_driver: T) -> Self {
        Self {
            storage_driver
        }
    }
}

impl<T: StorageDriver> StorageService for Service<T> {
    fn store(&self, directory: String, input: File) -> Result<String, ServiceError> {
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

        match self.storage_driver.store(Path::new(&path), input) {
            Ok(_) => Ok(file_name),
            Err(e) => {
                log!("error", "Failed to store file: {}", e);
                Err(ServiceError::from(e))
            }
        }
    }

    fn read(&self, directory: String, file_name: String) -> Result<File, ServiceError> {
        let path = format!(
            "{}/{directory}/{file_name}",
            Env::storage_dir(),
            directory = directory,
            file_name = &file_name
        );

        match self.storage_driver.read(Path::new(&path)) {
            Ok(contents) => Ok(contents),
            Err(e) => {
                log!("error", "Failed to read file: {}", e);
                Err(ServiceError::from(e))
            }
        }
    }

    fn delete(&self, directory: String, file_name: String) -> Result<(), ServiceError> {
        let path = format!(
            "{}/{directory}/{file_name}",
            Env::storage_dir(),
            directory = directory,
            file_name = &file_name
        );

        match self.storage_driver.delete(Path::new(&path)) {
            Ok(_) => Ok(()),
            Err(e) => {
                log!("error", "Failed to delete file: {}", e);
                Err(ServiceError::from(e))
            }
        }
    }
}
