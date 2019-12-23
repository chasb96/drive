use crate::storage_drivers::StorageDriver;
use crate::storage_drivers::disk::Disk;
use crate::storage_drivers::aws::rusoto::s3::S3;
use crate::storage_drivers::error::StorageError;
use std::path::Path;
use std::fs::File;

pub enum StorageDriverOption {
    Aws(S3),
    Disk(Disk),
}

impl From<String> for StorageDriverOption {
    fn from(from: String) -> Self {
        match from.to_lowercase().as_str() {
            "aws" => StorageDriverOption::Aws(S3::new()),
            "disk" => StorageDriverOption::Disk(Disk::new()),
            _ => panic!("Not a valid storage driver"),
        }
    }
}

impl Default for StorageDriverOption {
    fn default() -> Self {
        StorageDriverOption::Disk(Disk::new())
    }
}

impl StorageDriver for StorageDriverOption {
    fn store(&self, path: &Path, contents: File) -> Result<(), StorageError> {
        match self {
            StorageDriverOption::Aws(s3) => s3.store(path, contents),
            StorageDriverOption::Disk(disk) => disk.store(path, contents),
        }
    }

    fn read(&self, path: &Path) -> Result<File, StorageError> {
        match self {
            StorageDriverOption::Aws(s3) => s3.read(path),
            StorageDriverOption::Disk(disk) => disk.read(path)
        }
    }

    fn delete(&self, path: &Path) -> Result<(), StorageError> {
        match self {
            StorageDriverOption::Aws(s3) => s3.delete(path),
            StorageDriverOption::Disk(disk) => disk.delete(path),
        }
    }
}
