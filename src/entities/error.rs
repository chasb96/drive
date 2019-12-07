use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt;

#[derive(PartialEq)]
pub enum DataStoreError {
    NotFound,
    Diesel(DieselError)
}

impl Error for DataStoreError {}

impl fmt::Debug for DataStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataStoreError::NotFound => write!(f, "Not Found"),
            DataStoreError::Diesel(e) => e.fmt(f),
        }
    }
}

impl fmt::Display for DataStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataStoreError::NotFound => write!(f, "Not Found"),
            DataStoreError::Diesel(e) => e.fmt(f),
        }
    }
}

impl From<DieselError> for DataStoreError {
    fn from(error: DieselError) -> Self {
        if error == DieselError::NotFound {
            DataStoreError::NotFound
        } else {
            DataStoreError::Diesel(error)
        }
    }
}
