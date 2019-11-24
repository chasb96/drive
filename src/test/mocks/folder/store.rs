use entities::traits::folder::FolderStore;
use entities::models::{ File, Folder };
use crate::entities::error::DataStoreError;
use crate::entities::builders::{ Builder, FolderBuilder, FileBuilder };

pub struct FolderStoreMock;

impl FolderStoreMock {
    pub fn new() -> Self {
        Self
    }
}

impl FolderStore for FolderStoreMock {
    fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Folder>, DataStoreError> {
        let folders = vec![
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
        ];

        Ok(folders)
    }

    fn find_by_folder_id(&self, folder_id: i32) -> Result<Folder, DataStoreError> {
        let mut folder = factory!(Folder, 1, None);

        folder.set_id(folder_id);

        Ok(folder)
    }

    fn save(&self, folder: &Folder) -> Result<Folder, DataStoreError> {
        Ok(folder.clone())
    }

    fn update(&self, folder: &Folder) -> Result<Folder, DataStoreError> {
        Ok(folder.clone())
    }

    fn delete(&self, folder: &Folder) -> Result<Folder, DataStoreError> {
        Ok(folder.clone())
    }

    fn files(&self, folder: &Folder) -> Result<Vec<File>, DataStoreError> {
        let files = vec![
            factory!(File, folder.id()),
            factory!(File, folder.id()),
            factory!(File, folder.id()),
            factory!(File, folder.id()),
            factory!(File, folder.id()),
        ];

        Ok(files)
    }
}
