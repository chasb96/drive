pub mod controller;

use crate::entities::models::User;
use crate::controllers::error::ControllerError as Error;

pub struct StoreRequest {
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: String,
}

pub struct UpdateRequest {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
}

pub trait UserController {
    fn index(&self, user: User) -> Result<Vec<User>, Error>;

    fn show(&self, user: User, user_id: i32) -> Result<User, Error>;

    fn create(&self, user: User) -> Result<(), Error>;

    fn store(&self, user: User, request: StoreRequest) -> Result<User, Error>;

    fn edit(&self, user: User, user_id: i32) -> Result<User, Error>;

    fn update(&self, user: User, request: UpdateRequest) -> Result<User, Error>;

    fn delete(&self, user: User, user_id: i32) -> Result<User, Error>;

    fn update_password(&self, user: User, user_id: i32, password: String) -> Result<User, Error>;
}
