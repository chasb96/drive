pub mod implementation;

use std::fs::File;
use crate::services::error::ServiceError;

pub trait StorageService {
    fn store(&self, directory: String, input: File) -> Result<String, ServiceError>;

    fn read(&self, directory: String, file_name: String) -> Result<File, ServiceError>;

    fn delete(&self, directory: String, file_name: String) -> Result<(), ServiceError>;
}
