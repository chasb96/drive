pub mod file;
pub mod folder;
pub mod user;

use diesel::result::Error;

pub trait Query: Sized {
    fn save(&self) -> Result<Self, Error>;

    fn update(&self) -> Result<Self, Error>;

    fn delete(&self) -> Result<Self, Error>;
}
