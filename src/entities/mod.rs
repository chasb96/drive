pub mod builders;
pub mod models;
pub mod presentation;
pub mod traits;
pub mod diesel;
pub mod error;

use self::models::Model;
pub use self::diesel::pool::DbPool;

pub trait Entity: Model {}

impl Entity for models::User {}
impl Entity for models::Folder {}
impl Entity for models::File {}

pub trait Identifiable {
    fn id(&self) -> i32;
}

impl Identifiable for models::User {
    fn id(&self) -> i32 {
        self.id()
    }
}
