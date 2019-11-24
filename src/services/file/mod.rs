pub mod implementation;

use entities::models::File;
use services::error::ServiceError;

pub trait FileService {
    fn all(&self, folder_id: i32) -> Result<Vec<File>, ServiceError>;

    fn find(&self, file_id: i32) -> Result<File, ServiceError>;

    fn create(
        &self,
        name: String,
        extension: String,
        file_name: String,
        folder_id: i32,
        public: bool
    ) -> Result<File, ServiceError>;

    fn update(
        &self,
        id: i32,
        name: String,
        file_name: String,
        extension: String,
        folder_id: i32,
        public: bool,
    ) -> Result<File, ServiceError>;

    fn delete(&self, id: i32) -> Result<File, ServiceError>;
}
