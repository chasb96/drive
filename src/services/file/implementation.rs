use entities::traits::file::FileStore;
use entities::builders::{Builder, FileBuilder};
use entities::models::File;
use services::error::ServiceError;
use super::FileService;

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

    fn create(
        &self,
        name: String,
        extension: String,
        file_name: String,
        folder_id: i32,
        public: bool,
    ) -> Result<File, ServiceError> {
        let file = FileBuilder::new()
            .with_name(name)
            .with_extension(extension)
            .with_file_name(file_name)
            .with_public(public)
            .with_folder_id(folder_id)
            .build();

        match self.file_store.save(&file) {
            Ok(file) => Ok(file),
            Err(e) => Err(ServiceError::from(e))
        }
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
        let mut file = self.file_store.find_by_file_id(id)?;

        file.set_name(name);
        file.set_file_name(file_name);
        file.set_extension(extension);
        file.set_folder_id(folder_id);
        file.set_public(public);

        match self.file_store.update(&file) {
            Ok(file) => Ok(file),
            Err(e) => Err(ServiceError::from(e))
        }
    }

    fn delete(&self, id: i32) -> Result<File, ServiceError> {
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
    use crate::test::mocks::file::store::FileStoreMock;
    use crate::services::FileService;
    use crate::entities::builders::{ Builder, FileBuilder };

    #[test]
    fn test_create() {
        let file_store = FileStoreMock::new();
        let file_service = Service::new(file_store);

        let expected = factory!(File, 1);

        let actual = file_service.create(
            expected.name().to_string(),
            expected.extension().to_string(),
            expected.file_name().to_string(),
            expected.folder_id(),
            expected.public(),
        )
        .unwrap();

        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.extension(), actual.extension());
        assert_eq!(expected.file_name(), actual.file_name());
        assert_eq!(expected.folder_id(), actual.folder_id());
        assert_eq!(expected.public(), actual.public());
    }

    #[test]
    fn test_update() {
        let file_store = FileStoreMock::new();
        let file_service = Service::new(file_store);

        let expected = factory!(File, 1);

        let actual = file_service.update(
            expected.id(),
            expected.name().to_string(),
            expected.file_name().to_string(),
            expected.extension().to_string(),
            expected.folder_id(),
            expected.public(),
        )
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_delete() {
        let file_store = FileStoreMock::new();
        let file_service = Service::new(file_store);

        let expected = factory!(File, 1);

        let actual = file_service.delete(expected.id()).unwrap();

        assert_eq!(expected.id(), actual.id());
    }
}
