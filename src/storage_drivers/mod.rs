mod aws;
mod disk;
pub mod error;
pub mod storage_driver_option;
// pub mod storage_router;

// pub use self::storage_router::StorageRouter;

use crate::storage_drivers::error::StorageError;
use std::fs::File;
use std::path::Path;

pub trait StorageDriver {
    fn store(&self, path: &Path, contents: File) -> Result<(), StorageError>;

    fn read(&self, path: &Path) -> Result<File, StorageError>;

    fn delete(&self, path: &Path) -> Result<(), StorageError>;
}
