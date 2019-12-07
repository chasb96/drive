use entities::traits::file::FileStore;
use entities::builders::{Builder, FileBuilder};
use entities::models::File;
use services::error::ServiceError;
use super::FileService;
use crate::services::file::CreateRequest;
use crate::services::file::UpdateRequest;

pub struct Service<T: FileStore> {
    file_store: T
}

impl<T: FileStore> Service<T> {
    pub fn new(file_store: T) -> Self {
        Self { file_store }
    }
}

impl<T: FileStore> FileService for Service<T> {
    fn all(&self, folder_id: i32) -> Result<Vec<File>, ServiceError> {
        match self.file_store.find_by_folder_id(folder_id) {
            Ok(files) => Ok(files),
            Err(e) => Err(ServiceError::from(e))
        }
    }

    fn find(&self, file_id: i32) -> Result<File, ServiceError> {
        match self.file_store.find_by_file_id(file_id) {
            Ok(file) => Ok(file),
            Err(e) => Err(ServiceError::from(e))
        }
    }

    fn create(&self, request: CreateRequest) -> Result<File, ServiceError> {
        // Create the File with the
        //  name, extension, internal name, public, and folder_id
        let file = FileBuilder::new()
            .with_name(request.name)
            .with_extension(request.extension)
            .with_file_name(request.file_name)
            .with_public(request.public)
            .with_folder_id(request.folder_id)
            .build();

        // Request that the DataStore save the File
        match self.file_store.save(&file) {
            Ok(file) => Ok(file),
            Err(e) => Err(ServiceError::from(e))
        }
    }

    fn update(&self, request: UpdateRequest) -> Result<File, ServiceError> {
        // Attempt to retrieve the File from the DataStore
        //  by the file's Id, if theres an error, throw it back
        let mut file = self.file_store.find_by_file_id(request.id)?;

        // Update the name, internal name,
        //  extension, folder_id and public
        //  of the File
        file.set_name(request.name);
        file.set_file_name(request.file_name);
        file.set_extension(request.extension);
        file.set_folder_id(request.folder_id);
        file.set_public(request.public);

        // Request the DataStore update the file record
        match self.file_store.update(&file) {
            Ok(file) => Ok(file),
            Err(e) => Err(ServiceError::from(e))
        }
    }

    fn delete(&self, id: i32) -> Result<File, ServiceError> {
        // Attempt to get the File from the DataStore
        //  by its File Id, if there's an error, throw it
        //  back
        let file = self.file_store.find_by_file_id(id)?;

        match self.file_store.delete(&file) {
            Ok(file) => Ok(file),
            Err(e) => Err(ServiceError::from(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Service;
    use crate::services::FileService;
    use crate::entities::builders::{ Builder, FileBuilder };
    use super::CreateRequest;
    use super::UpdateRequest;
    use crate::entities::traits::file::MockFileStore;

    #[test]
    fn test_create() {
        let expected = factory!(File, 1);
        let expected_save_result = expected.clone();

        let mut mock_file_store = MockFileStore::new();
        mock_file_store
            .expect_save()
            .returning(move |_| Ok(expected_save_result.clone()));

        let file_service = Service::new(mock_file_store);

        let request = CreateRequest {
            name: expected.name().to_string(),
            extension: expected.extension().to_string(),
            file_name: expected.file_name().to_string(),
            folder_id: expected.folder_id(),
            public: expected.public(),
        };

        let actual = file_service.create(request).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_update() {
        let expected = factory!(File, 1);
        let expected_update_result = expected.clone();
        let expected_find_by_file_id_result = expected.clone();

        let mut mock_file_store = MockFileStore::new();
        mock_file_store
            .expect_update()
            .returning(move |_| Ok(expected_update_result.clone()));
        mock_file_store
            .expect_find_by_file_id()
            .returning(move |_| Ok(expected_find_by_file_id_result.clone()));

        let file_service = Service::new(mock_file_store);

        let request = UpdateRequest {
            id: expected.id(),
            name: expected.name().to_string(),
            file_name: expected.file_name().to_string(),
            extension: expected.extension().to_string(),
            folder_id: expected.folder_id(),
            public: expected.public(),
        };

        let actual = file_service.update(request).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_delete() {
        let expected = factory!(File, 1);
        let expected_update_result = expected.clone();
        let expected_find_by_file_id_result = expected.clone();

        let mut mock_file_store = MockFileStore::new();
        mock_file_store
            .expect_delete()
            .returning(move |_| Ok(expected_update_result.clone()));
        mock_file_store
            .expect_find_by_file_id()
            .returning(move |_| Ok(expected_find_by_file_id_result.clone()));

        let file_service = Service::new(mock_file_store);

        let actual = file_service.delete(expected.id()).unwrap();

        assert_eq!(expected, actual);
    }
}
