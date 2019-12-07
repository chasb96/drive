use crate::services::file::FileService;
use crate::entities::models::File;
use crate::services::error::ServiceError;
use crate::entities::builders::{Builder, FileBuilder};
use crate::services::file::CreateRequest;
use crate::services::file::UpdateRequest;

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

    fn create(&self, request: CreateRequest) -> Result<File, ServiceError> {
        let mut file = factory!(File, request.folder_id);

        file.set_name(request.name);
        file.set_file_name(request.file_name);
        file.set_extension(request.extension);
        file.set_public(request.public);

        Ok(file)
    }

    fn update(&self, request: UpdateRequest) -> Result<File, ServiceError> {
        let mut file = factory!(File, request.folder_id);

        file.set_id(request.id);
        file.set_name(request.name);
        file.set_file_name(request.file_name);
        file.set_extension(request.extension);
        file.set_public(request.public);

        Ok(file)
    }

    fn delete(&self, id: i32) -> Result<File, ServiceError> {
        let mut file = factory!(File, 1);

        file.set_id(id);

        Ok(file)
    }
}
