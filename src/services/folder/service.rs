use crate::services::file::FileService;
use crate::entities::traits::folder::FolderStore;
use crate::entities::builders::{Builder, FolderBuilder};
use crate::entities::models::Folder;
use crate::services::error::ServiceError;
use super::FolderService;
use super::CreateRequest;
use super::UpdateRequest;

pub struct Service<T: FolderStore, S: FileService> {
    folder_store: T,
    file_service: S,
}

impl<T: FolderStore, S: FileService> Service<T, S> {
    pub fn new(folder_store: T, file_service: S) -> Self {
        Self {
            folder_store,
            file_service
        }
    }
}

impl<T: FolderStore, S: FileService> FolderService for Service<T, S> {
    fn all(&self, user_id: i32) -> Result<Vec<Folder>, ServiceError> {
        Ok(self.folder_store.find_by_user_id(user_id)?)
    }

    fn find(&self, folder_id: i32) -> Result<Folder, ServiceError> {
        Ok(self.folder_store.find_by_folder_id(folder_id)?)
    }

    fn create(&self, request: CreateRequest) -> Result<Folder, ServiceError> {
        // Create a Folder with the name, user_id, and parent_id
        let folder = FolderBuilder::new()
            .with_name(request.name)
            .with_user_id(request.user_id)
            .with_parent_id(request.parent_id)
            .build();

        // Request the DataStore store the Folder
        let folder = self.folder_store.save(&folder)?;

        Ok(folder)
    }

    fn update(&self, request: UpdateRequest) -> Result<Folder, ServiceError> {
        // Attempt to get the Folder by Id,
        //  if it fails, throw it back
        let mut folder = self.folder_store.find_by_folder_id(request.id)?;

        // Update the Folder's name, user_id, and parent_id
        folder.set_name(request.name);
        folder.set_user_id(request.user_id);
        folder.set_parent_id(request.parent_id);

        // Request the DataStore update the Folder
        let folder = self.folder_store.update(&folder)?;

        Ok(folder)
    }

    fn delete(&self, id: i32) -> Result<Folder, ServiceError> {
        // Find the Folder by the Id, if an error is thrown,
        //  throw it back
        let folder = self.folder_store.find_by_folder_id(id)?;

        // Files have a dependency on Folders,
        //  a Folder can't be deleted without its files
        //  being deleted
        //
        // Iterate through all the Folder's Files,
        //  and delete them
        //
        // TODO: This is N+1, there should be a bulk delete
        for file in self.folder_store.files(&folder)? {
            self.file_service.delete(file.id())?;
        }

        // Request the DataStore delete the Folder
        let folder = self.folder_store.delete(&folder)?;

        Ok(folder)
    }
}

#[cfg(test)]
mod tests {
    use super::Service;
    use crate::test::mocks::file::service::FileServiceMock;
    use crate::test::mocks::folder::store::FolderStoreMock;
    use crate::services::FolderService;
    use crate::entities::builders::{ Builder, FolderBuilder };
    use super::CreateRequest;
    use super::UpdateRequest;

    #[test]
    fn test_create() {
        let folder_store = FolderStoreMock::new();
        let file_service = FileServiceMock::new();
        let folder_service = Service::new(folder_store, file_service);

        let expected = factory!(Folder, 1, None);

        let request = CreateRequest {
            name: expected.name().to_string(),
            user_id: expected.user_id(),
            parent_id: *expected.parent_id(),
        };

        let actual = folder_service.create(request).unwrap();

        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.user_id(), actual.user_id());
        assert_eq!(expected.parent_id(), actual.parent_id());
    }

    #[test]
    fn test_update() {
        let folder_store = FolderStoreMock::new();
        let file_service = FileServiceMock::new();
        let folder_service = Service::new(folder_store, file_service);

        let expected = factory!(Folder, 1, None);

        let request = UpdateRequest {
            id: expected.id(),
            name: expected.name().to_string(),
            user_id: expected.user_id(),
            parent_id: *expected.parent_id(),
        };

        let actual = folder_service.update(request).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_delete() {
        let folder_store = FolderStoreMock::new();
        let file_service = FileServiceMock::new();
        let folder_service = Service::new(folder_store, file_service);

        let expected = factory!(Folder, 1, None);

        let actual = folder_service.delete(expected.id()).unwrap();

        assert_eq!(expected.id(), actual.id());
    }
}
