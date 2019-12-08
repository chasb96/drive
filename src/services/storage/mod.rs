pub mod implementation;

use std::fs::File;
use crate::services::error::ServiceError;

#[cfg(test)]
use mockall::*;
#[cfg(test)]
use mockall::predicate::*;

#[cfg_attr(test, automock)]
pub trait StorageService {
    fn store(&self, directory: String, input: File) -> Result<String, ServiceError>;

    fn read(&self, directory: String, file_name: String) -> Result<File, ServiceError>;

    fn delete(&self, directory: String, file_name: String) -> Result<(), ServiceError>;
}
