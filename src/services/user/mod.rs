pub mod implementation;

use entities::models::User;
use diesel::result::Error;

pub trait UserService {
    fn create(
        &self,
        name: String,
        email: String,
        role: String,
        password: String,
    ) -> Result<User, Error>;

    fn update(&self, id: i32, name: String, email: String, role: String) -> Result<User, Error>;

    fn delete(&self, id: i32) -> Result<User, Error>;

    fn update_password(&self, id: i32, password: String) -> Result<User, Error>;
}
