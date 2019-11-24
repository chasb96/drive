pub mod file;
mod folder;
mod user;

pub use self::file::FileController;
pub use self::folder::FolderController;
pub use self::user::UserController;

use std::error::Error;
use std::fmt::Display;

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
