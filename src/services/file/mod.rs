pub mod service;

use crate::entities::models::File;
use crate::services::error::ServiceError;

pub struct CreateRequest {
    pub name: String,
    pub extension: String,
    pub file_name: String,
    pub folder_id: i32,
    pub public: bool
}

pub struct UpdateRequest {
    pub id: i32,
    pub name: String,
    pub file_name: String,
    pub extension: String,
    pub folder_id: i32,
    pub public: bool,
}

pub trait FileService {
    fn all(&self, folder_id: i32) -> Result<Vec<File>, ServiceError>;

    fn find(&self, file_id: i32) -> Result<File, ServiceError>;

    fn create(&self, request: CreateRequest) -> Result<File, ServiceError>;

    fn update(&self, request: UpdateRequest) -> Result<File, ServiceError>;

    fn delete(&self, id: i32) -> Result<File, ServiceError>;
}
