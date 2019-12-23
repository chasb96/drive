use crate::logging::{LogLevel, LoggerOption};
use std::convert::TryFrom;
use std::env;
use crate::storage_drivers::storage_driver_option::StorageDriverOption;

#[allow(dead_code)]
const DATABASE_URL: &str = "DATABASE_URL";

#[allow(dead_code)]
const TEST_DATABASE_URL: &str = "TEST_DATABASE_URL";

#[allow(dead_code)]
const APP_KEY: &str = "APP_KEY";

#[allow(dead_code)]
const STORAGE_DIR: &str = "STORAGE_DIR";

#[allow(dead_code)]
const BCRYPT_COST: &str = "BCRYPT_COST";

#[allow(dead_code)]
const STREAM_CHUNK_SIZE: &str = "STREAM_CHUNK_SIZE";

#[allow(dead_code)]
const LOG_LEVEL: &str = "LOG_LEVEL";

#[allow(dead_code)]
const LOGGER: &str = "LOGGER";

#[allow(dead_code)]
const STORAGE_DRIVER: &str = "STORAGE_DRIVER";

#[allow(dead_code)]
const AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";

#[allow(dead_code)]
const AWS_ACCESS_KEY_SECRET: &str = "AWS_ACCESS_KEY_SECRET";

#[allow(dead_code)]
const AWS_SESSION_TOKEN: &str = "AWS_SESSION_TOKEN";

#[allow(dead_code)]
const AWS_BUCKET_NAME: &str = "AWS_BUCKET_NAME";

#[allow(dead_code)]
const AWS_BUCKET_REGION: &str = "AWS_BUCKET_REGION";

#[allow(dead_code)]
const SESSION_EXPIRY_HOURS: &str = "SESSION_EXPIRY_HOURS";

pub struct Env;

#[allow(dead_code)]
impl Env {
    #[cfg(not(test))]
    pub fn database_url() -> String {
        match env::var(DATABASE_URL) {
            Ok(database_url) => database_url,
            Err(e) => panic!(e),
        }
    }

    #[cfg(test)]
    pub fn database_url() -> String {
        match env::var(TEST_DATABASE_URL) {
            Ok(database_url) => database_url,
            Err(e) => panic!(e),
        }
    }

    pub fn app_key() -> String {
        match env::var(APP_KEY) {
            Ok(app_key) => app_key,
            Err(e) => panic!(e),
        }
    }

    pub fn storage_dir() -> String {
        match env::var(STORAGE_DIR) {
            Ok(storage_dir) => storage_dir,
            Err(_) => String::from("storage"),
        }
    }

    pub fn bcrypt_cost() -> u32 {
        match env::var(BCRYPT_COST) {
            Ok(bcrypt_cost) => bcrypt_cost.parse::<u32>().unwrap(),
            Err(_) => 12,
        }
    }

    pub fn chunk_size() -> u64 {
        match env::var(STREAM_CHUNK_SIZE) {
            Ok(bcrypt_cost) => bcrypt_cost.parse::<u64>().unwrap(),
            Err(_) => 256,
        }
    }

    pub fn log_level() -> LogLevel {
        match env::var(LOG_LEVEL) {
            Ok(result) => match LogLevel::try_from(result) {
                Ok(log_level) => log_level,
                Err(e) => panic!(e),
            },
            Err(_) => LogLevel::default(),
        }
    }

    pub fn logger() -> LoggerOption {
        match env::var(LOGGER) {
            Ok(logger) => match LoggerOption::try_from(logger) {
                Ok(logger) => logger,
                Err(e) => panic!(e),
            },
            Err(_) => LoggerOption::default(),
        }
    }

    pub fn storage_driver() -> StorageDriverOption {
        match env::var(STORAGE_DRIVER) {
            Ok(driver) => StorageDriverOption::from(driver),
            Err(_) => StorageDriverOption::default(),
        }
    }

    pub fn aws_access_key_id() -> String {
        match env::var(AWS_ACCESS_KEY_ID) {
            Ok(id) => id,
            Err(e) => panic!(e),
        }
    }

    pub fn aws_access_key_secret() -> String {
        match env::var(AWS_ACCESS_KEY_SECRET) {
            Ok(secret) => secret,
            Err(e) => panic!(e),
        }
    }

    pub fn aws_session_token() -> Option<String> {
        match env::var(AWS_SESSION_TOKEN) {
            Ok(token) => Some(token),
            Err(_) => None,
        }
    }

    pub fn aws_bucket_name() -> String {
        match env::var(AWS_BUCKET_NAME) {
            Ok(bucket) => bucket,
            Err(e) => panic!(e),
        }
    }

    pub fn aws_bucket_region() -> String {
        match env::var(AWS_BUCKET_REGION) {
            Ok(region) => region,
            Err(e) => panic!(e),
        }
    }

    pub fn session_expiry_hours() -> i64 {
        match env::var(SESSION_EXPIRY_HOURS) {
            Ok(hours) => hours.parse::<i64>().unwrap(),
            Err(_) => 12,
        }
    }
}
