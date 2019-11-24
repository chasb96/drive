use services::file::FileService;
use entities::traits::folder::FolderStore;
use entities::builders::{Builder, FolderBuilder};
use entities::models::Folder;
use diesel::result::Error;
use super::FolderService;

pub struct Service<T: FolderStore, S: FileService> {
    folder_store: T,
    file_service: S,
}

impl<T: FolderStore, S: FileService> Service<T, S> {
    pub fn new(folder_store: T, file_service: S) -> Self {
        Self { folder_store, file_service }
    }
}

impl<T: FolderStore, S: FileService> FolderService for Service<T, S> {
    fn all(&self, user_id: i32) -> Result<Vec<Folder>, Error> {
        self.folder_store.find_by_user_id(user_id)
    }

    fn find(&self, folder_id: i32) -> Result<Folder, Error> {
        self.folder_store.find_by_folder_id(folder_id)
    }

    fn create(&self, name: String, user_id: i32, parent_id: Option<i32>) -> Result<Folder, Error> {
        let folder = FolderBuilder::new()
            .with_name(name)
            .with_user_id(user_id)
            .with_parent_id(parent_id)
            .build();

        self.folder_store.save(&folder)
    }

    fn update(
        &self,
        id: i32,
        name: String,
        user_id: i32,
        parent_id: Option<i32>,
    ) -> Result<Folder, Error> {
        let mut folder = self.folder_store.find_by_folder_id(id)?;

        folder.set_name(name);
        folder.set_user_id(user_id);
        folder.set_parent_id(parent_id);

        self.folder_store.update(&folder)
    }

    fn delete(&self, id: i32) -> Result<Folder, Error> {
        let folder = self.folder_store.find_by_folder_id(id)?;

        for file in self.folder_store.files(&folder)? {
            if let Err(e) = self.file_service.delete(file.id()) {
                log!("error", "Failed to delete file {}: {}", file.id(), e);
                // return Err(e);
            }
        }

        self.folder_store.delete(&folder)
    }
}

#[cfg(test)]
mod tests {
    use super::Service;
    use crate::test::mocks::file::service::FileServiceMock;
    use crate::test::mocks::folder::store::FolderStoreMock;
    use crate::services::FolderService;
    use crate::entities::builders::{ Builder, FolderBuilder };

    #[test]
    fn test_create() {
        let folder_store = FolderStoreMock::new();
        let file_service = FileServiceMock::new();
        let folder_service = Service::new(folder_store, file_service);

        let expected = factory!(Folder, 1, None);

        let actual = folder_service.create(
            expected.name().to_string(),
            expected.user_id(),
            *expected.parent_id(),
        )
        .unwrap();

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

        let actual = folder_service.update(
            expected.id(),
            expected.name().to_string(),
            expected.user_id(),
            *expected.parent_id(),
        )
        .unwrap();

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
