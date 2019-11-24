pub mod implementation;

use services::error::ServiceError;
use entities::models::User;

pub struct CreateRequest {
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: String,
}

pub struct UpdateRequest {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
}

pub trait UserService {
    fn create(&self, request: CreateRequest) -> Result<User, ServiceError>;

    fn update(&self, request: UpdateRequest) -> Result<User, ServiceError>;

    fn delete(&self, id: i32) -> Result<User, ServiceError>;

    fn update_password(&self, id: i32, password: String) -> Result<User, ServiceError>;
}
