use crate::entities::traits::file::FileStore;
use crate::entities::models::File;
use crate::entities::error::DataStoreError;
use crate::entities::builders::{ Builder, FileBuilder };

pub struct FileStoreMock;

impl FileStoreMock {
    pub fn new() -> Self {
        Self
    }
}

impl FileStore for FileStoreMock {
    fn find_by_folder_id(&self, folder_id: i32) -> Result<Vec<File>, DataStoreError> {
        let files = vec![
            factory!(File, folder_id),
            factory!(File, folder_id),
            factory!(File, folder_id),
            factory!(File, folder_id),
            factory!(File, folder_id),
        ];

        Ok(files)
    }

    fn find_by_file_id(&self, file_id: i32) -> Result<File, DataStoreError> {
        let mut file = factory!(File, 1);

        file.set_id(file_id);

        Ok(file)
    }

    fn save(&self, file: &File) -> Result<File, DataStoreError> {
        Ok(file.clone())
    }

    fn update(&self, file: &File) -> Result<File, DataStoreError> {
        Ok(file.clone())
    }

    fn delete(&self, file: &File) -> Result<File, DataStoreError> {
        Ok(file.clone())
    }
}
