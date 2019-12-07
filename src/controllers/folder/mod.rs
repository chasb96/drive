pub mod implementation;

use entities::models::{Folder, User};
use crate::controllers::error::ControllerError as Error;

pub struct StoreRequest {
    pub name: String,
    pub user_id: i32,
    pub parent_id: Option<i32>
}

pub struct UpdateRequest {
    pub folder_id: i32,
    pub name: String,
    pub user_id: i32,
    pub parent_id: Option<i32>,
}

pub trait FolderController {
    fn index(&self, user: User, parent_id: Option<i32>) -> Result<Vec<Folder>, Error>;

    fn show(&self, user: User, folder_id: i32) -> Result<Folder, Error>;

    fn create(&self, user: User) -> Result<(), Error>;

    fn store(&self, user: User, request: StoreRequest) -> Result<Folder, Error>;

    fn edit(&self, user: User, folder_id: i32) -> Result<Folder, Error>;

    fn update(&self, user: User, request: UpdateRequest) -> Result<Folder, Error>;

    fn delete(&self, user: User, folder_id: i32) -> Result<Folder, Error>;
}
