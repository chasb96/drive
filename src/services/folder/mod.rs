pub mod implementation;

use entities::models::Folder;
use crate::services::error::ServiceError;

pub struct CreateRequest {
    pub name: String,
    pub user_id: i32,
    pub parent_id: Option<i32>,
}

pub struct UpdateRequest {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub parent_id: Option<i32>
}

pub trait FolderService {
    fn all(&self, user_id: i32) -> Result<Vec<Folder>, ServiceError>;

    fn find(&self, folder_id: i32) -> Result<Folder, ServiceError>;

    fn create(&self, request: CreateRequest) -> Result<Folder, ServiceError>;

    fn update(&self, request: UpdateRequest) -> Result<Folder, ServiceError>;

    fn delete(&self, id: i32) -> Result<Folder, ServiceError>;
}
