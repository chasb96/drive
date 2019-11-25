use std::error::Error;
use std::fmt::Display;
use crate::services::error::ServiceError;

#[derive(Debug)]
pub enum ControllerError {
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl ControllerError {
    pub fn level(&self) -> &str {
        match self {
            ControllerError::Unauthorized => "warn",
            ControllerError::Forbidden => "warn",
            ControllerError::NotFound => "debug",
            ControllerError::InternalServerError => "error",
        }
    }
}

impl Display for ControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let response = match self {
            ControllerError::Unauthorized => "Unauthorized",
            ControllerError::Forbidden => "Forbidden",
            ControllerError::NotFound => "Not Found",
            ControllerError::InternalServerError => "Internal Server Error",
        };

        write!(f, "{}", response)
    }
}

impl Error for ControllerError {}

impl From<ServiceError> for ControllerError {
    fn from(error: ServiceError) -> Self {
        // Controllers live at the top-level of the
        //  application - all web, db, etc are external
        //
        // We don't want to expose the internal errors
        //  back out of the application. Therefore,
        //  the serious errors are mapped abiguously
        match error {
            ServiceError::NotFound => ControllerError::NotFound,
            ServiceError::StorageError(_) => ControllerError::InternalServerError,
            ServiceError::DataStoreError(_) => ControllerError::InternalServerError,
        }
    }
}
