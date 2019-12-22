use crate::entities::models::Folder;
use crate::entities::error::DataStoreError;
use crate::entities::models::File;

pub trait FolderStore {
    fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Folder>, DataStoreError>;

    fn find_by_folder_id(&self, id: i32) -> Result<Folder, DataStoreError>;

    fn save(&self, folder: &Folder) -> Result<Folder, DataStoreError>;

    fn update(&self, folder: &Folder) -> Result<Folder, DataStoreError>;

    fn delete(&self, folder: &Folder) -> Result<Folder, DataStoreError>;

    fn files(&self, folder: &Folder) -> Result<Vec<File>, DataStoreError>;
}
