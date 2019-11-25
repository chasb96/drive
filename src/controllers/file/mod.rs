pub mod implementation;

use crate::controllers::error::ControllerError;
use crate::entities::models::User;
use crate::entities::models::File;
use std::fs;

pub struct CreateRequest {
    pub name: String,
    pub extension: String,
    pub user_id: i32,
    pub folder_id: i32,
    pub public: bool,
    pub input: fs::File,
}

pub struct UpdateRequest {
    pub file_id: i32,
    pub name: String,
    pub extension: String,
    pub public: bool,
    pub folder_id: i32,
}

pub trait FileController {
    fn index(&self, user: User, folder_id: i32) -> Result<Vec<File>, ControllerError>;

    fn show(&self, user: User, file_id: i32) -> Result<File, ControllerError>;

    fn create(&self, user: User) -> Result<(), ControllerError>;

    fn store(&self, user: User, request: CreateRequest) -> Result<File, ControllerError>;

    fn edit(&self, user: User, file_id: i32) -> Result<File, ControllerError>;

    fn update(&self, user: User, request: UpdateRequest) -> Result<File, ControllerError>;

    fn delete(&self, user: User, file_id: i32) -> Result<File, ControllerError>;

    fn contents(&self, user: User, file_id: i32) -> Result<fs::File, ControllerError>;
}
