use super::bucket::Bucket;
use super::credentials::Credentials;
use super::region::Region;
use chrono::Utc;
use rand::{self, distributions::Alphanumeric, Rng};
use rusoto_core::credential::StaticProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::{ByteStream, RusotoError};
use rusoto_s3::S3 as RusotoS3;
use rusoto_s3::{
    DeleteObjectError, DeleteObjectRequest, GetObjectError, GetObjectRequest, PutObjectError,
    PutObjectRequest, S3Client,
};
use std::error;
use std::fmt;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::path::Path;
use crate::storage_drivers::StorageDriver;
use tokio::codec::{BytesCodec, FramedRead};
use tokio::fs::File as TokioFile;
use tokio::prelude::{Future, Stream};
use crate::storage_drivers::error::StorageError;

pub struct S3;

impl S3 {
    pub fn new() -> Self {
        Self
    }
}

impl StorageDriver for S3 {
    fn store(&self, path: &Path, contents: File) -> Result<(), StorageError> {
        let len = match contents.metadata() {
            Ok(metadata) => metadata.len(),
            Err(e) => return Err(StorageError::from(S3Error::from(e))),
        };

        let async_file = TokioFile::from_std(contents);

        let stream = FramedRead::new(async_file, BytesCodec::new()).map(|r| r.freeze());

        let region = Region::env().into();
        let bucket = Bucket::env().into();
        let provider: StaticProvider = Credentials::env().into();

        let client = S3Client::new_with(HttpClient::new().unwrap(), provider, region);

        let request = PutObjectRequest {
            body: Some(ByteStream::new(stream)),
            bucket,
            key: path.to_str().unwrap().to_string(),
            content_length: Some(len as i64),
            ..Default::default()
        };

        match client.put_object(request).sync() {
            Ok(_) => Ok(()),
            Err(e) => Err(StorageError::from(S3Error::from(e))),
        }
    }

    fn read(&self, path: &Path) -> Result<File, StorageError> {
        let region = Region::env().into();
        let bucket = Bucket::env().into();
        let provider: StaticProvider = Credentials::env().into();

        let client = S3Client::new_with(HttpClient::new().unwrap(), provider, region);

        let request = GetObjectRequest {
            bucket,
            key: path.to_str().unwrap().to_string(),
            ..Default::default()
        };

        let response = match client.get_object(request).sync() {
            Ok(response) => response,
            Err(e) => return Err(StorageError::from(S3Error::from(e))),
        };

        // Create a temp file for streaming the file back to the end user

        let timestamp = Utc::now().to_string();
        let random_bytes: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .collect();

        let tmp = format!(
            "/tmp/{timestamp}_{random_bytes}",
            timestamp = timestamp,
            random_bytes = random_bytes
        );

        // Creating scope block here to force the file to close, and reopen,
        //  meaning the file cannot be opened while still being owned by the process
        {
            let mut file = match File::create(Path::new(&tmp)) {
                Ok(file) => file,
                Err(e) => {
                    log!("error", "Failed to create tmp file: {}", e);
                    return Err(StorageError::from(S3Error::from(e)));
                }
            };

            response
                .body
                .unwrap()
                .for_each(|chunk| file.write_all(&chunk))
                .wait()
                .unwrap();
        }

        let file = match File::open(Path::new(&tmp)) {
            Ok(file) => file,
            Err(e) => {
                log!("error", "Failed to open tmp file: {}", e);
                return Err(StorageError::from(S3Error::from(e)));
            }
        };

        Ok(file)
    }

    fn delete(&self, path: &Path) -> Result<(), StorageError> {
        let region = Region::env().into();
        let bucket = Bucket::env().into();
        let provider: StaticProvider = Credentials::env().into();

        let client = S3Client::new_with(HttpClient::new().unwrap(), provider, region);

        let request = DeleteObjectRequest {
            bucket,
            key: path.to_str().unwrap().to_string(),
            ..Default::default()
        };

        match client.delete_object(request).sync() {
            Ok(_) => Ok(()),
            Err(e) => Err(StorageError::from(S3Error::from(e))),
        }
    }
}

pub enum S3Error {
    DeleteObject(RusotoError<DeleteObjectError>),
    GetObject(RusotoError<GetObjectError>),
    PutObject(RusotoError<PutObjectError>),
    IO(Error),
}

impl error::Error for S3Error {
    fn description(&self) -> &str {
        match self {
            S3Error::DeleteObject(s3) => s3.description(),
            S3Error::GetObject(s3) => s3.description(),
            S3Error::PutObject(s3) => s3.description(),
            S3Error::IO(io) => io.description(),
        }
    }
}

impl fmt::Display for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            S3Error::DeleteObject(s3) => s3.fmt(f),
            S3Error::GetObject(s3) => s3.fmt(f),
            S3Error::PutObject(s3) => s3.fmt(f),
            S3Error::IO(io) => io.fmt(f),
        }
    }
}

impl fmt::Debug for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            S3Error::DeleteObject(s3) => s3.fmt(f),
            S3Error::GetObject(s3) => s3.fmt(f),
            S3Error::PutObject(s3) => s3.fmt(f),
            S3Error::IO(io) => io.fmt(f),
        }
    }
}

impl From<RusotoError<PutObjectError>> for S3Error {
    fn from(from: RusotoError<PutObjectError>) -> Self {
        S3Error::PutObject(from)
    }
}

impl From<RusotoError<GetObjectError>> for S3Error {
    fn from(from: RusotoError<GetObjectError>) -> Self {
        S3Error::GetObject(from)
    }
}

impl From<RusotoError<DeleteObjectError>> for S3Error {
    fn from(from: RusotoError<DeleteObjectError>) -> Self {
        S3Error::DeleteObject(from)
    }
}

impl From<Error> for S3Error {
    fn from(from: Error) -> Self {
        S3Error::IO(from)
    }
}
