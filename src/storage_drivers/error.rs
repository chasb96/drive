use crate::storage_drivers::aws::rusoto::s3::S3Error;
use std::error::Error;
use std::fmt;
use std::io::Error as IOError;

pub enum StorageError {
    Aws(S3Error),
    Disk(IOError),
}

impl Error for StorageError {
    fn description(&self) -> &str {
        match self {
            StorageError::Aws(aws) => aws.description(),
            StorageError::Disk(disk) => disk.description(),
        }
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            StorageError::Aws(aws) => aws.fmt(f)?,
            StorageError::Disk(disk) => disk.fmt(f)?,
        };

        Ok(())
    }
}

impl fmt::Debug for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            StorageError::Aws(aws) => aws.fmt(f)?,
            StorageError::Disk(disk) => disk.fmt(f)?,
        };

        Ok(())
    }
}

impl From<IOError> for StorageError {
    fn from(from: IOError) -> Self {
        StorageError::Disk(from)
    }
}

impl From<S3Error> for StorageError {
    fn from(from: S3Error) -> Self {
        StorageError::Aws(from)
    }
}
