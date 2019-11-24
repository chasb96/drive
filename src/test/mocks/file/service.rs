use crate::services::file::FileService;
use crate::entities::models::File;
use crate::services::error::ServiceError;
use crate::entities::builders::{Builder, FileBuilder};

pub struct FileServiceMock;

impl FileServiceMock {
    pub fn new() -> Self {
        Self
    }
}

impl FileService for FileServiceMock {
    fn all(&self, folder_id: i32) -> Result<Vec<File>, ServiceError> {
        let files = vec![
            factory!(File, folder_id),
            factory!(File, folder_id),
            factory!(File, folder_id),
            factory!(File, folder_id),
            factory!(File, folder_id),
        ];

        Ok(files)
    }

    fn find(&self, file_id: i32) -> Result<File, ServiceError> {
        let mut file = factory!(File, 1);

        file.set_id(file_id);

        Ok(file)
    }

    fn create(
        &self,
        name: String,
        file_name: String,
        extension: String,
        folder_id: i32,
        public: bool,
    ) -> Result<File, ServiceError> {
        let mut file = factory!(File, folder_id);

        file.set_name(name);
        file.set_file_name(file_name);
        file.set_extension(extension);
        file.set_public(public);

        Ok(file)
    }

    fn update(
        &self,
        id: i32,
        name: String,
        file_name: String,
        extension: String,
        folder_id: i32,
        public: bool,
    ) -> Result<File, ServiceError> {
        let mut file = factory!(File, folder_id);

        file.set_id(id);
        file.set_name(name);
        file.set_file_name(file_name);
        file.set_extension(extension);
        file.set_public(public);

        Ok(file)
    }

    fn delete(&self, id: i32) -> Result<File, ServiceError> {
        let mut file = factory!(File, 1);

        file.set_id(id);

        Ok(file)
    }
}
